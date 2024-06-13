use base64::{engine::general_purpose as b64, Engine as _};
use opaque_ke::errors::ProtocolError;
use opaque_ke::{
    CredentialFinalization, CredentialRequest, RegistrationRequest, RegistrationUpload,
    ServerLogin, ServerLoginFinishResult, ServerLoginStartParameters, ServerLoginStartResult,
    ServerRegistration, ServerRegistrationStartResult, ServerSetup,
};
use rand::rngs::OsRng;

use crate::Cipher;
use crate::Error;

pub fn register_server(
    server_setup: String,
    client_request: String,
    credential_id: String,
) -> Result<String, Error> {
    let setup_bytes = b64::URL_SAFE_NO_PAD.decode(server_setup)?;
    let request_bytes = b64::URL_SAFE_NO_PAD.decode(client_request)?;
    let credential_bytes = credential_id.as_bytes();
    let setup = ServerSetup::<Cipher>::deserialize(&setup_bytes)?;
    let client_request: RegistrationRequest<Cipher> =
        RegistrationRequest::deserialize(&request_bytes)?;

    let s = opaque_server_register(setup, client_request, credential_bytes)?;

    let response_bytes = s.message.serialize();
    let response_encoded = b64::URL_SAFE_NO_PAD.encode(response_bytes);

    Ok(response_encoded)
}

pub fn register_server_finish(client_request_finish: String) -> Result<String, Error> {
    let request_bytes = b64::URL_SAFE_NO_PAD.decode(client_request_finish)?;
    let client_request_finish: RegistrationUpload<Cipher> =
        RegistrationUpload::deserialize(&request_bytes)?;

    let s = opaque_server_register_finish(client_request_finish)?;

    let password_file_bytes = s.serialize();
    let password_file_encoded = b64::URL_SAFE_NO_PAD.encode(password_file_bytes);

    Ok(password_file_encoded)
}

pub fn login_server(
    server_setup: String,
    password_file: String,
    client_request: String,
    credential_id: String,
) -> Result<(String, String), Error> {
    let password_file_bytes = b64::URL_SAFE_NO_PAD.decode(password_file)?;
    let setup_bytes = b64::URL_SAFE_NO_PAD.decode(server_setup)?;
    let request_bytes = b64::URL_SAFE_NO_PAD.decode(client_request)?;
    let credential_bytes = credential_id.as_bytes();
    let setup = ServerSetup::<Cipher>::deserialize(&setup_bytes)?;
    let password_file = ServerRegistration::<Cipher>::deserialize(&password_file_bytes)?;
    let client_request: Box<CredentialRequest<Cipher>> =
        Box::new(CredentialRequest::deserialize(&request_bytes)?);

    let s = opaque_server_login(setup, password_file, *client_request, credential_bytes)?;

    let response_bytes = s.message.serialize();
    let state_bytes = s.state.serialize();

    let response_encoded = b64::URL_SAFE_NO_PAD.encode(response_bytes);
    let state_encoded = b64::URL_SAFE_NO_PAD.encode(state_bytes);

    Ok((response_encoded, state_encoded))
}

pub fn login_server_finish(
    client_request_finish: String,
    login_state: String,
) -> Result<String, Error> {
    let request_bytes = b64::URL_SAFE_NO_PAD.decode(client_request_finish)?;
    let state_bytes = b64::URL_SAFE_NO_PAD.decode(login_state)?;
    let client_request_finish = Box::new(CredentialFinalization::<Cipher>::deserialize(
        &request_bytes,
    )?);
    let login_state = Box::new(ServerLogin::<Cipher>::deserialize(&state_bytes)?);

    let s = opaque_server_login_finish(*client_request_finish, *login_state)?;

    let session_key_bytes = s.session_key;
    let session_key_encoded = b64::URL_SAFE_NO_PAD.encode(session_key_bytes);

    Ok(session_key_encoded)
}

fn opaque_server_register(
    setup: ServerSetup<Cipher>,
    client_request: RegistrationRequest<Cipher>,
    credential_id_bytes: &[u8],
) -> Result<ServerRegistrationStartResult<Cipher>, ProtocolError> {
    ServerRegistration::<Cipher>::start(&setup, client_request, credential_id_bytes)
}

fn opaque_server_register_finish(
    client_request_finish: RegistrationUpload<Cipher>,
) -> Result<ServerRegistration<Cipher>, ProtocolError> {
    Ok(ServerRegistration::<Cipher>::finish(client_request_finish))
}

fn opaque_server_login(
    setup: ServerSetup<Cipher>,
    password_file: ServerRegistration<Cipher>,
    client_request: CredentialRequest<Cipher>,
    credential_id_bytes: &[u8],
) -> Result<ServerLoginStartResult<Cipher>, ProtocolError> {
    let mut server_rng = OsRng;
    ServerLogin::<Cipher>::start(
        &mut server_rng,
        &setup,
        Some(password_file),
        client_request,
        credential_id_bytes,
        ServerLoginStartParameters::default(),
    )
}

fn opaque_server_login_finish(
    client_request_finish: CredentialFinalization<Cipher>,
    login_state: ServerLogin<Cipher>,
) -> Result<ServerLoginFinishResult<Cipher>, ProtocolError> {
    login_state.finish(client_request_finish)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_register_output() {
        // password 'clientele'
        let setup = "C5HVEMyOKYglRys_3a58GHLeRM0oa_pjxSO6mu-WEnfIdOO5mE7GpCz_Z0xrntzbeMQI3GACQet9N_3lh1eaEWM18tqMDhUEJ_TwfSJNEXavKLc2DHlxWcd5Xd8aiPMJJ11dZmU76urlWHZw5xJuvDfLbdnt2tIj-fmY9PobZQg".to_string();
        let message = "3i3SEzJNZKvsIfADx1lf-zk4SNeitkTp41-kpxWOUxE".to_string();
        let cred_id = "someperson".to_string();

        let response = register_server(setup, message, cred_id).unwrap();
        println!("{}", response);
        // example response
        // mKbmMmzMVuq9r2yrfWtJXQCYTFVxAD3ZHkPLFhGY-hqASLH7HrrwUUQdYwcPA8Bigtj_ISL-GC9iHKheKl0rew
    }

    #[test]
    fn server_register_finish() {
        let client_message = "wBtSZIhSPTwEY13yNT6nfWhj0WVRnhiqsnAYhUu7nj_5mmv-Trgm3DULYEZLwYhQaadsk8rI8n0PD1mZi8AL7517p9b5wisa4TxrNDyifHLUI_P09Re5KTf8CUr_0I6vMYOhCBl7WgItYfj1h-lAZU5E77fmsl-6l4MIZ5oYIKENNClbtgX9GYz1WkrZpJdeDdnAA5AI-0cfh3AX8UjpD46BhzwxkFDhtMra4vpRFQSAvu7gVzsZSDQJoqTcYXjy".to_string();
        let password_file = register_server_finish(client_message).unwrap();
        println!("{}", password_file)

        // example file:
        // wBtSZIhSPTwEY13yNT6nfWhj0WVRnhiqsnAYhUu7nj_5mmv-Trgm3DULYEZLwYhQaadsk8rI8n0PD1mZi8AL7517p9b5wisa4TxrNDyifHLUI_P09Re5KTf8CUr_0I6vMYOhCBl7WgItYfj1h-lAZU5E77fmsl-6l4MIZ5oYIKENNClbtgX9GYz1WkrZpJdeDdnAA5AI-0cfh3AX8UjpD46BhzwxkFDhtMra4vpRFQSAvu7gVzsZSDQJoqTcYXjy
    }

    #[test]
    fn server_login() {
        let setup = "C5HVEMyOKYglRys_3a58GHLeRM0oa_pjxSO6mu-WEnfIdOO5mE7GpCz_Z0xrntzbeMQI3GACQet9N_3lh1eaEWM18tqMDhUEJ_TwfSJNEXavKLc2DHlxWcd5Xd8aiPMJJ11dZmU76urlWHZw5xJuvDfLbdnt2tIj-fmY9PobZQg".to_string();
        let client_message = "iIx1sW5pj2GlKnb4V1MRrCFmLTkW_hhyXQCGmczMZUJZ50HlQnVjuPyvv6oKpD7d0ZzPDbZcnliqDlN-GHdrwUpAvLRYzwk1earfTNefonfH9faSj7307IMRwrfGyOhZ".to_string();
        // password 'clientiele'
        let password_file = "wBtSZIhSPTwEY13yNT6nfWhj0WVRnhiqsnAYhUu7nj_5mmv-Trgm3DULYEZLwYhQaadsk8rI8n0PD1mZi8AL7517p9b5wisa4TxrNDyifHLUI_P09Re5KTf8CUr_0I6vMYOhCBl7WgItYfj1h-lAZU5E77fmsl-6l4MIZ5oYIKENNClbtgX9GYz1WkrZpJdeDdnAA5AI-0cfh3AX8UjpD46BhzwxkFDhtMra4vpRFQSAvu7gVzsZSDQJoqTcYXjy".to_string();
        let cred_id = "someperson".to_string();

        let (response, state) =
            login_server(setup, password_file, client_message, cred_id).unwrap();

        println!("{}", response);
        println!("{}", state);
        // example response
        // 1kHXX25U1NE0nki_rL-5KnPRre6-CD2P4ApqhDOXL2m5xgVIT1oO4M5n5r6b2mfGx7Xq4kiMFQH8pfrGLok2H0A20_p30giVapbzQL1QYRiG2jJK2AUlhOK6lTr5YshNJNOqHgn0eGFUWiYwZ606pYfIJDaiUa5p9Mhmb9lWTpzP9akkvjDkbaxViRLB_-T9QZyTjiy-67bkepYWTeCEnqrkdw-_ckKEcBRJrlVX0yMqHtzwMdoMX6LKdT02BCXMde7kdxT5mWUMcMozBg0SLrTAehZiHgqQYq94TAK-lmiePUljPSSHFERFm-0h7r2t2QL-hLi1DmY53upbKvTQFyN-IzuAmK7UsG4MgcYVcGKQXlrF4keZVJaa5dOz5YaFYp1oBXm26hUdtKzXf-9gVTtlDc3ep-G8GNyiDNLiVwU
        // example state
        // NjqD_19cS-XSDTfOdHj5g8wC1zMlTy-ydF4kiwKHUjbC9e_8CZb4pkLi9t0694FS_QgT6T_jktK1PbSoPqsTyas6rleW5Ttf0yA_a488YahFnBYHhsjXRWccL4Y6atjNeFcYOhc2f6t5oLU4p_FIA1eKQQFNRMXxEXEuHDl_DKanO3_4Iqs659gCx0IrOZoxnBDvxk8sHXNO19-gmzAVGgAQp-wERCoW3FP6h21PZwRA99AWSOe2YETc-VL_MtRT
    }

    #[test]
    fn server_login_finish() {
        // correspond to above
        let client_message = "eP_3skqnrkJXs-AZpXqaxihP4EsF1ek8eDxH4ktDflExfEvNF99-oVZ24hkahw05v-rH9B-1WCs91dGteIMH9Q".to_string();
        let state = "NjqD_19cS-XSDTfOdHj5g8wC1zMlTy-ydF4kiwKHUjbC9e_8CZb4pkLi9t0694FS_QgT6T_jktK1PbSoPqsTyas6rleW5Ttf0yA_a488YahFnBYHhsjXRWccL4Y6atjNeFcYOhc2f6t5oLU4p_FIA1eKQQFNRMXxEXEuHDl_DKanO3_4Iqs659gCx0IrOZoxnBDvxk8sHXNO19-gmzAVGgAQp-wERCoW3FP6h21PZwRA99AWSOe2YETc-VL_MtRT".to_string();

        let session = login_server_finish(client_message, state).unwrap();

        println!("{}", session)
        // example session key
        // pzt_-CKrOufYAsdCKzmaMZwQ78ZPLB1zTtffoJswFRoAEKfsBEQqFtxT-odtT2cEQPfQFkjntmBE3PlS_zLUUw
    }
}
