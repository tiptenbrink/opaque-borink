use base64;
use opaque_ke::{CredentialFinalization, CredentialRequest,
                RegistrationRequest, RegistrationUpload, ServerLogin, ServerLoginFinishResult,
                ServerLoginStartParameters, ServerLoginStartResult, ServerRegistration,
                ServerRegistrationStartResult};
use opaque_ke::errors::ProtocolError;
use opaque_ke::keypair::Key;
use rand::rngs::OsRng;

use crate::Cipher;
use crate::Error;

pub fn register_server(client_request: String, public_key: String) -> Result<(String, String), Error> {
    let request_bytes = base64::decode_config(client_request, base64::URL_SAFE_NO_PAD)?;
    let client_request: RegistrationRequest<Cipher> = RegistrationRequest::deserialize(&request_bytes)?;
    let key_bytes = base64::decode_config(public_key, base64::URL_SAFE_NO_PAD)?;
    let public_key = Key::from_bytes(&key_bytes)?;

    let s = opaque_server_register(client_request, &public_key)?;

    let response_bytes = s.message.serialize();
    let state_bytes = s.state.serialize();

    let response_encoded = base64::encode_config(response_bytes, base64::URL_SAFE_NO_PAD);
    let state_encoded = base64::encode_config(state_bytes, base64::URL_SAFE_NO_PAD);

    Ok((response_encoded, state_encoded))
}

pub fn register_server_finish(client_request_finish: String, registration_state: String) -> Result<String, Error> {
    let request_bytes = base64::decode_config(client_request_finish, base64::URL_SAFE_NO_PAD)?;
    let client_request_finish: RegistrationUpload<Cipher> = RegistrationUpload::deserialize(&request_bytes)?;

    let state_bytes = base64::decode_config(registration_state, base64::URL_SAFE_NO_PAD)?;
    let registration_state: ServerRegistration<Cipher> = ServerRegistration::deserialize(&state_bytes)?;

    let s = opaque_server_register_finish(client_request_finish, registration_state)?;

    let password_file_bytes = s.serialize();

    let password_file_encoded = base64::encode_config(password_file_bytes, base64::URL_SAFE_NO_PAD);

    Ok(password_file_encoded)
}

pub fn login_server(password_file: String, client_request: String, private_key: String) -> Result<(String, String), Error> {
    let password_file_bytes = base64::decode_config(password_file, base64::URL_SAFE_NO_PAD)?;
    let password_file= ServerRegistration::<Cipher>::deserialize(&password_file_bytes)?;

    let request_bytes = base64::decode_config(client_request, base64::URL_SAFE_NO_PAD)?;
    let client_request: CredentialRequest<Cipher> = CredentialRequest::deserialize(&request_bytes)?;

    let key_bytes = base64::decode_config(private_key, base64::URL_SAFE_NO_PAD)?;
    let private_key = Key::from_bytes(&key_bytes)?;

    let s = opaque_server_login(password_file, &private_key, client_request)?;

    let response_bytes = s.message.serialize()?;
    let state_bytes = s.state.serialize()?;

    let response_encoded = base64::encode_config(response_bytes, base64::URL_SAFE_NO_PAD);
    let state_encoded = base64::encode_config(state_bytes, base64::URL_SAFE_NO_PAD);

    Ok((response_encoded, state_encoded))
}

pub fn login_server_finish(client_request_finish: String, login_state: String) -> Result<String, Error> {
    let request_bytes = base64::decode_config(client_request_finish, base64::URL_SAFE_NO_PAD)?;
    let client_request_finish: CredentialFinalization<Cipher> = CredentialFinalization::deserialize(&request_bytes)?;

    let state_bytes = base64::decode_config(login_state, base64::URL_SAFE_NO_PAD)?;
    let login_state: ServerLogin<Cipher> = ServerLogin::deserialize(&state_bytes)?;

    let s = opaque_server_login_finish(client_request_finish, login_state)?;

    let session_key_bytes = s.session_key;

    let session_key_encoded = base64::encode_config(session_key_bytes, base64::URL_SAFE_NO_PAD);

    Ok(session_key_encoded)
}

fn opaque_server_register(client_request: RegistrationRequest<Cipher>, public_key: &Key)
                          -> Result<ServerRegistrationStartResult<Cipher>, ProtocolError> {
    let mut server_rng = OsRng;
    ServerRegistration::<Cipher>::start(
        &mut server_rng,
        client_request,
        public_key,
    )
}

fn opaque_server_register_finish(client_request_finish: RegistrationUpload<Cipher>, registration_state: ServerRegistration<Cipher>)
                                 -> Result<ServerRegistration<Cipher>, ProtocolError> {

    registration_state.finish(
        client_request_finish
    )
}

fn opaque_server_login(password_file: ServerRegistration<Cipher>, private_key: &Key, client_request: CredentialRequest<Cipher>)
                       -> Result<ServerLoginStartResult<Cipher>, ProtocolError> {
    let mut server_rng = OsRng;
    ServerLogin::start(
        &mut server_rng,
        password_file,
        private_key,
        client_request,
        ServerLoginStartParameters::default(),
    )
}

fn opaque_server_login_finish(client_request_finish: CredentialFinalization<Cipher>, login_state: ServerLogin<Cipher>)
                              -> Result<ServerLoginFinishResult<Cipher>, ProtocolError> {
    login_state.finish(
        client_request_finish
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_register() {
        //password 'garbage'
        let message = "Pj8bFY58CZoyi9Rsp2KyS4HhA2vXcSEAFH7BViwxRzw".to_string();
        let pub_string = "OhKbj6rzdot9c9y_RCcFcIKYozF2OaOHW7A6-UhveQo".to_string();

        let (response, state) = register_server_py(message, pub_string).unwrap();
        println!("{}", response);
        println!("{}", state);
    }

    #[test]
    fn server_register_finish() {
        let client_message = "WiiA158EDDe0lHHfg0C8HrhAnAh3AUfqanzVbsajm0IBmDAbxVcbUp5MVFm759dCz5YvNYlpZw5NQoaQFAJHkPmufK3_FFdV87nQ7bfp7BZ5BURZgLp6O_b0FlE80IKksTQFXN2mo8QqrVlIQPJ1DiAtr5FGuXqaSkduYkJyGRLXy_RzSmkME8Fs1zYqTPM-fAzzRjbRJBfOKdcuiJSzyFU".to_string();
        let server_state = "bdbAbu6_ZGoJUShySB6qx8oQrpNXz3CCWd_7qC1J2ws".to_string();
        let password_file = register_server_finish_py(client_message, server_state).unwrap();
        println!("{}", password_file)

        // example file:
        // bdbAbu6_ZGoJUShySB6qx8oQrpNXz3CCWd_7qC1J2wtaKIDXnwQMN7SUcd-DQLweuECcCHcBR-pqfNVuxqObQgGYMBvFVxtSnkxUWbvn10LPli81iWlnDk1ChpAUAkeQ-a58rf8UV1XzudDtt-nsFnkFRFmAuno79vQWUTzQgqSxNAVc3aajxCqtWUhA8nUOIC2vkUa5eppKR25iQnIZEtfL9HNKaQwTwWzXNipM8z58DPNGNtEkF84p1y6IlLPIVQ
    }

    #[test]
    fn server_login() {
        let client_message = "UD06GXLMCcJr-EaYonw0zKGQ9FMeMJ55Mh_H5yJ2S1AuF_sQmykFADMj9vdgA1Umw2SwtH0Tai0lOdF1WAM0TAAA_gVx9nSVv9YgIw5aMsrg67LJTZBm7DDQG4O6XpK9Rlw".to_string();
        // password 'abc'
        let password_file = "bdbAbu6_ZGoJUShySB6qx8oQrpNXz3CCWd_7qC1J2wtaKIDXnwQMN7SUcd-DQLweuECcCHcBR-pqfNVuxqObQgGYMBvFVxtSnkxUWbvn10LPli81iWlnDk1ChpAUAkeQ-a58rf8UV1XzudDtt-nsFnkFRFmAuno79vQWUTzQgqSxNAVc3aajxCqtWUhA8nUOIC2vkUa5eppKR25iQnIZEtfL9HNKaQwTwWzXNipM8z58DPNGNtEkF84p1y6IlLPIVQ".to_string();
        let private_key = "QNxnQ_c-rx2nmuLAOTln5Ul60XYqNz_yws_WG8BoAAc".to_string();

        let (response, state) = login_server_py(password_file, client_message, private_key).unwrap();

        println!("{}", response);
        println!("{}", state);
    }

    #[test]
    fn server_login_finish() {
        // correspond to above
        let client_message = "YHLmz1dB6XXkFabmzSctR53HskpKEWcZVvXEcswegia2OVbC4NezY1jqhzGN-z7trO8SCe_IbEyeg1n04UkJXw".to_string();
        let state = "3soQ8dLh007sMpOUvyBM4o0FDp-sHHXMu-WU1rtofMtjT5veRMmrv3KmZDTaAzGTxP442NYS-0_XpjPyLN_O9_UKQV92Cv6YvpFWwrNJlye_XfrwUV9fm9JCCA5R0CHCN9PVcrarW_1-GmSd5KitAr57LeS0Ne6fWZsYtI6yM6GkphmMAcxzykJxtyqicpmF3gjesD-Nbgktp7A3d066kHUZ4DRredc9NaF-gdVg76PtE8dVuL9aVEN2reciq54U".to_string();

        let session = login_server_finish_py(client_message, state).unwrap();

        println!("{}", session)
    }
}