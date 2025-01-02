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
    server_setup: &str,
    client_request: &str,
    credential_id: &str,
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

pub fn register_server_finish(client_request_finish: &str) -> Result<String, Error> {
    let request_bytes = b64::URL_SAFE_NO_PAD.decode(client_request_finish)?;
    let client_request_finish: RegistrationUpload<Cipher> =
        RegistrationUpload::deserialize(&request_bytes)?;

    let s = opaque_server_register_finish(client_request_finish)?;

    let password_file_bytes = s.serialize();
    let password_file_encoded = b64::URL_SAFE_NO_PAD.encode(password_file_bytes);

    Ok(password_file_encoded)
}

pub fn login_server(
    server_setup: &str,
    password_file: &str,
    client_request: &str,
    credential_id: &str,
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
    client_request_finish: &str,
    login_state: &str,
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
        let message = "dn17rg3EzeikxJ4rWay0DCnSax5JoQOEifmhfwj5Jjs".to_string();
        let cred_id = "someperson".to_string();

        let response = register_server(&setup, &message, &cred_id).unwrap();
        println!("{}", response);
        // example response
        // fDCnRbPyYdSCw_6cFCDzo5Zcd5OwV2TnWNg43eWQIyqASLH7HrrwUUQdYwcPA8Bigtj_ISL-GC9iHKheKl0rew
    }

    #[test]
    fn server_register_finish() {
        let client_message = "LJ0rg3mSZ-x1tDbobI0xvroBjAPQ5fnAgrnEmxc67giA0XDjR8pJaOuNGlWtRku5Hk57yBlL6YrjBUQJ--7OMhPZra40WvmWSu7yT8s-CBAsE0jobWK-9qXk3xDv7TlK-g_TF3JzR3s8MntBWjIuN5Ii7Le93coLGLvm7xjQtuYHbszz3HBv-gBu_xlj7YitpgyQzYpcJGslbezqxEvZz4Jz0R64np94JBDibI7syTw13ZJ74tbjWiJbvwvKb5a-".to_string();
        let password_file = register_server_finish(&client_message).unwrap();
        println!("{}", password_file)

        // example file:
        // LJ0rg3mSZ-x1tDbobI0xvroBjAPQ5fnAgrnEmxc67giA0XDjR8pJaOuNGlWtRku5Hk57yBlL6YrjBUQJ--7OMhPZra40WvmWSu7yT8s-CBAsE0jobWK-9qXk3xDv7TlK-g_TF3JzR3s8MntBWjIuN5Ii7Le93coLGLvm7xjQtuYHbszz3HBv-gBu_xlj7YitpgyQzYpcJGslbezqxEvZz4Jz0R64np94JBDibI7syTw13ZJ74tbjWiJbvwvKb5a-
    }

    #[test]
    fn server_login() {
        let setup = "C5HVEMyOKYglRys_3a58GHLeRM0oa_pjxSO6mu-WEnfIdOO5mE7GpCz_Z0xrntzbeMQI3GACQet9N_3lh1eaEWM18tqMDhUEJ_TwfSJNEXavKLc2DHlxWcd5Xd8aiPMJJ11dZmU76urlWHZw5xJuvDfLbdnt2tIj-fmY9PobZQg".to_string();
        let client_message = "8FZa7PGvxQgFwh3WGP-HXd0_f1EgeVKHcWMBfpwYNCsqnNKMtkDQcb9j-yw0d3POSd81f0cgQAZiTo6nIrpvUATmdt9VnHbQDazHEA-D0iJ4uzlQbjjGHQ1UgnCqaTBG".to_string();
        // password 'clientiele'
        let password_file = "LJ0rg3mSZ-x1tDbobI0xvroBjAPQ5fnAgrnEmxc67giA0XDjR8pJaOuNGlWtRku5Hk57yBlL6YrjBUQJ--7OMhPZra40WvmWSu7yT8s-CBAsE0jobWK-9qXk3xDv7TlK-g_TF3JzR3s8MntBWjIuN5Ii7Le93coLGLvm7xjQtuYHbszz3HBv-gBu_xlj7YitpgyQzYpcJGslbezqxEvZz4Jz0R64np94JBDibI7syTw13ZJ74tbjWiJbvwvKb5a-".to_string();
        let cred_id = "someperson".to_string();

        let (response, state) =
            login_server(&setup, &password_file, &client_message, &cred_id).unwrap();

        println!("resp={}", response);
        println!("state={}", state);
        // example response
        // lskLi18T8NM-WjY926___29u0RoY0XcKAz8-Wzu9gRMYWfgTuEk5qx4ZF6OZkTfpM_eufiKYIoKK2HNOTUwSf-bUsZRi9vydqe2yB3Wz5y3TiWI6CkVzACIFfbKynKGg0DQ4Sr5KYhsnMTzoF1Me27oq5sONK-R1muZ8JZpGXMB8l5mllx8-jfqFfe-8EEDIH0vyi9nzBKbzZSyexPiI00js1Vo5WU55jFWWdMldTg67WhPTgfITmgoGr-bQp-6wdwJGva12wMkvwFPptzk-0TMMu04YxIRzjC3OoKNxKtT8iOPTpq6SHFnVoMq3hwsYVFXxim36iickj0BzHeqebWVoo3FV9Da-ph8i6a7sKNGpe4Q4wN-0WpBgMurTkwvwcvhUCGMYvde0j7u1QOKDI_UjA9jeTlASlQHSmu0se7E
        // example state
        // B552YqssmUw1OOCGiXnnJB51DwX38aYMhxTl7elzLHbnVlX1cXdlXcT2nUlU3gw3IyH-6PsAhGXDv-X20Knt3d6PlUtCThpEuiH1RxehA1u9R_OBS8ctVeeHLHhzNys4vLeWBQzHh_-sW3erjRuMBUxQQwMcgQl-4Kh_RWfIp6M8a3A1fQVtPc3V0PdNwFH9pkn26_03KwZSi7POpigdJiOHZr9fje88PY_zv5MZxb_ohiblddOWYzwYlOmNidkI
    }

    #[test]
    fn server_login_finish() {
        // correspond to above
        let client_message = "J_aMsbRzBJYQcB839mopFnkzHgCsfCpxDCR3Q-WtYEHnFtDMyf3fVk4u7KUHPIVoZo8Fc6z2KQASLr2kTpUpjQ".to_string();
        let state = "B552YqssmUw1OOCGiXnnJB51DwX38aYMhxTl7elzLHbnVlX1cXdlXcT2nUlU3gw3IyH-6PsAhGXDv-X20Knt3d6PlUtCThpEuiH1RxehA1u9R_OBS8ctVeeHLHhzNys4vLeWBQzHh_-sW3erjRuMBUxQQwMcgQl-4Kh_RWfIp6M8a3A1fQVtPc3V0PdNwFH9pkn26_03KwZSi7POpigdJiOHZr9fje88PY_zv5MZxb_ohiblddOWYzwYlOmNidkI".to_string();

        let session = login_server_finish(&client_message, &state).unwrap();

        let expected_key = "PGtwNX0FbT3N1dD3TcBR_aZJ9uv9NysGUouzzqYoHSYjh2a_X43vPD2P87-TGcW_6IYm5XXTlmM8GJTpjYnZCA";
        assert_eq!(expected_key, session);
    }
}
