use opaque_ke::{ClientLogin, ClientLoginFinishParameters, ClientLoginFinishResult,
                ClientLoginStartParameters, ClientLoginStartResult, ClientRegistration,
                ClientRegistrationFinishParameters, ClientRegistrationFinishResult,
                ClientRegistrationStartResult, CredentialResponse, RegistrationResponse};
use opaque_ke::errors::ProtocolError;
use rand::rngs::OsRng;
use crate::Cipher;
use crate::Error;

pub fn client_register(password: &str) -> Result<(String, String), Error> {
    let c = opaque_client_register(password)?;

    let message_bytes = c.message.serialize();
    let state_bytes = c.state.serialize();

    let message_encoded = base64::encode_config(message_bytes, base64::URL_SAFE_NO_PAD);
    let state_encoded = base64::encode_config(state_bytes, base64::URL_SAFE_NO_PAD);

    Ok((message_encoded, state_encoded))
}

pub fn client_register_finish(client_register_state: &str, server_message: &str) -> Result<String, Error> {
    let state_bytes = base64::decode_config(client_register_state, base64::URL_SAFE_NO_PAD)?;
    let client_register_state = ClientRegistration::<Cipher>::deserialize(&state_bytes)?;

    let message_bytes = base64::decode_config(server_message, base64::URL_SAFE_NO_PAD)?;
    let server_message = RegistrationResponse::<Cipher>::deserialize(&message_bytes)?;

    let c = opaque_client_register_finish(client_register_state, server_message)?;

    let message_bytes = c.message.serialize();

    let message_encoded = base64::encode_config(message_bytes, base64::URL_SAFE_NO_PAD);

    Ok(message_encoded)
}

pub fn client_login(password: &str) -> Result<(String, String), Error> {
    let c = opaque_client_login(password)?;

    let message_bytes = c.message.serialize()?;
    let state_bytes = c.state.serialize()?;

    let message_encoded = base64::encode_config(message_bytes, base64::URL_SAFE_NO_PAD);
    let state_encoded = base64::encode_config(state_bytes, base64::URL_SAFE_NO_PAD);

    Ok((message_encoded, state_encoded))
}

pub fn client_login_finish(client_login_state: &str, server_message: &str) -> Result<(String, String), Error> {
    let state_bytes = base64::decode_config(client_login_state, base64::URL_SAFE_NO_PAD)?;
    let client_login_state = ClientLogin::<Cipher>::deserialize(&state_bytes)?;

    let message_bytes = base64::decode_config(server_message, base64::URL_SAFE_NO_PAD)?;
    let server_message = CredentialResponse::<Cipher>::deserialize(&message_bytes)?;

    // An InvalidLogin will be emitted in this step in the case of an incorrect password
    let c = opaque_client_login_finish(client_login_state, server_message)?;

    let message_bytes = c.message.serialize()?;
    let session_bytes = c.session_key;

    let message_encoded = base64::encode_config(message_bytes, base64::URL_SAFE_NO_PAD);
    let session_encoded = base64::encode_config(session_bytes, base64::URL_SAFE_NO_PAD);

    Ok((message_encoded, session_encoded))
}

fn opaque_client_register(password: &str) -> Result<ClientRegistrationStartResult<Cipher>, ProtocolError> {
    let mut client_rng = OsRng;
    ClientRegistration::<Cipher>::start(
        &mut client_rng,
        password.as_bytes(),
    )
}

fn opaque_client_register_finish(client_register_state: ClientRegistration<Cipher>, server_message: RegistrationResponse<Cipher>)
                                 -> Result<ClientRegistrationFinishResult<Cipher>, ProtocolError> {

    let mut client_rng = OsRng;
    client_register_state.finish(
        &mut client_rng,
        server_message,
        ClientRegistrationFinishParameters::Default
    )
}

fn opaque_client_login(password: &str) -> Result<ClientLoginStartResult<Cipher>, ProtocolError> {
    let mut client_rng = OsRng;
    ClientLogin::<Cipher>::start(
        &mut client_rng,
        password.as_bytes(),
        ClientLoginStartParameters::default()
    )
}

fn opaque_client_login_finish(client_login_state: ClientLogin<Cipher>, server_message: CredentialResponse<Cipher>)
                              -> Result<ClientLoginFinishResult<Cipher>, ProtocolError> {
    client_login_state.finish(
        server_message,
        ClientLoginFinishParameters::default()
    )
}