use crate::Cipher;
use crate::Error;
use base64::{engine::general_purpose as b64, Engine as _};
use opaque_ke::errors::ProtocolError;
use opaque_ke::{
    ClientLogin, ClientLoginFinishParameters, ClientLoginFinishResult, ClientLoginStartResult,
    ClientRegistration, ClientRegistrationFinishParameters, ClientRegistrationFinishResult,
    ClientRegistrationStartResult, CredentialResponse, RegistrationResponse,
};
use rand::rngs::OsRng;

pub fn client_register(password: &str) -> Result<(String, String), Error> {
    let pass_bytes = password.as_bytes();
    let c = opaque_client_register(pass_bytes)?;

    let message_bytes = c.message.serialize();
    let state_bytes = c.state.serialize();

    let message_encoded = b64::URL_SAFE_NO_PAD.encode(message_bytes);
    let state_encoded = b64::URL_SAFE_NO_PAD.encode(state_bytes);

    Ok((message_encoded, state_encoded))
}

pub fn client_register_finish(
    client_register_state: &str,
    password: &str,
    server_message: &str,
) -> Result<String, Error> {
    let state_bytes = b64::URL_SAFE_NO_PAD.decode(client_register_state)?;
    let pass_bytes = password.as_bytes();
    let message_bytes = b64::URL_SAFE_NO_PAD.decode(server_message)?;
    let client_register_state = ClientRegistration::<Cipher>::deserialize(&state_bytes)?;

    let server_message = RegistrationResponse::<Cipher>::deserialize(&message_bytes)?;

    let c = opaque_client_register_finish(client_register_state, pass_bytes, server_message)?;

    let message_bytes = c.message.serialize();
    let message_encoded = b64::URL_SAFE_NO_PAD.encode(message_bytes);

    Ok(message_encoded)
}

pub fn client_login(password: &str) -> Result<(String, String), Error> {
    let pass_bytes = password.as_bytes();
    let c = opaque_client_login(pass_bytes)?;

    let message_bytes = c.message.serialize();
    let state_bytes = c.state.serialize();

    let message_encoded = b64::URL_SAFE_NO_PAD.encode(message_bytes);
    let state_encoded = b64::URL_SAFE_NO_PAD.encode(state_bytes);

    Ok((message_encoded, state_encoded))
}

pub fn client_login_finish(
    client_login_state: &str,
    password: &str,
    server_message: &str,
) -> Result<(String, String), Error> {
    let state_bytes = b64::URL_SAFE_NO_PAD.decode(client_login_state)?;
    let pass_bytes = password.as_bytes();
    let message_bytes = b64::URL_SAFE_NO_PAD.decode(server_message)?;
    let client_login_state = Box::new(ClientLogin::<Cipher>::deserialize(&state_bytes)?);
    let server_message = Box::new(CredentialResponse::<Cipher>::deserialize(&message_bytes)?);

    // An InvalidLogin will be emitted in this step in the case of an incorrect password
    let c = opaque_client_login_finish(client_login_state, pass_bytes, server_message)?;

    let message_bytes = c.message.serialize();
    let session_bytes = c.session_key;

    let message_encoded = b64::URL_SAFE_NO_PAD.encode(message_bytes);
    let session_encoded = b64::URL_SAFE_NO_PAD.encode(session_bytes);

    Ok((message_encoded, session_encoded))
}

fn opaque_client_register(
    password: &[u8],
) -> Result<ClientRegistrationStartResult<Cipher>, ProtocolError> {
    let mut client_rng = OsRng;
    ClientRegistration::<Cipher>::start(&mut client_rng, password)
}

fn opaque_client_register_finish(
    client_register_state: ClientRegistration<Cipher>,
    password: &[u8],
    server_message: RegistrationResponse<Cipher>,
) -> Result<ClientRegistrationFinishResult<Cipher>, ProtocolError> {
    let mut client_rng = OsRng;
    client_register_state.finish(
        &mut client_rng,
        password,
        server_message,
        ClientRegistrationFinishParameters::default(),
    )
}

fn opaque_client_login(password: &[u8]) -> Result<ClientLoginStartResult<Cipher>, ProtocolError> {
    let mut client_rng = OsRng;
    ClientLogin::<Cipher>::start(&mut client_rng, password)
}

fn opaque_client_login_finish(
    client_login_state: Box<ClientLogin<Cipher>>,
    password: &[u8],
    server_message: Box<CredentialResponse<Cipher>>,
) -> Result<ClientLoginFinishResult<Cipher>, ProtocolError> {
    client_login_state.finish(
        password,
        *server_message,
        ClientLoginFinishParameters::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_register_output() {
        // password 'clientele'
        let password = "clientele".to_string();

        let (response, state) = client_register(&password).unwrap();
        println!("{}", response);
        println!("{}", state);
        // example response
        // dn17rg3EzeikxJ4rWay0DCnSax5JoQOEifmhfwj5Jjs
        // example state
        // 9klWX8NMmsKZmGw5i1HflGrDdwbrHXKJn2QnaKgKnA12fXuuDcTN6KTEnitZrLQMKdJrHkmhA4SJ-aF_CPkmOw
    }

    #[test]
    fn client_register_finish_output() {
        // password 'clientele'
        let state = "9klWX8NMmsKZmGw5i1HflGrDdwbrHXKJn2QnaKgKnA12fXuuDcTN6KTEnitZrLQMKdJrHkmhA4SJ-aF_CPkmOw".to_string();
        let password = "clientele".to_string();
        let server_message = "fDCnRbPyYdSCw_6cFCDzo5Zcd5OwV2TnWNg43eWQIyqASLH7HrrwUUQdYwcPA8Bigtj_ISL-GC9iHKheKl0rew".to_string();

        let response = client_register_finish(&state, &password, &server_message).unwrap();
        println!("{}", response);
        // example response
        // LJ0rg3mSZ-x1tDbobI0xvroBjAPQ5fnAgrnEmxc67giA0XDjR8pJaOuNGlWtRku5Hk57yBlL6YrjBUQJ--7OMhPZra40WvmWSu7yT8s-CBAsE0jobWK-9qXk3xDv7TlK-g_TF3JzR3s8MntBWjIuN5Ii7Le93coLGLvm7xjQtuYHbszz3HBv-gBu_xlj7YitpgyQzYpcJGslbezqxEvZz4Jz0R64np94JBDibI7syTw13ZJ74tbjWiJbvwvKb5a-
    }

    #[test]
    fn client_login_output() {
        // password 'clientele'
        let password = "clientele".to_string();

        let (response, state) = client_login(&password).unwrap();
        println!("resp={}", response);
        println!("state={}", state);
        // example response
        // 8FZa7PGvxQgFwh3WGP-HXd0_f1EgeVKHcWMBfpwYNCsqnNKMtkDQcb9j-yw0d3POSd81f0cgQAZiTo6nIrpvUATmdt9VnHbQDazHEA-D0iJ4uzlQbjjGHQ1UgnCqaTBG
        // example state
        // lMZg9wetFB01g4KL1laU9s4tWR9ICMptDcVJxnfAugXwVlrs8a_FCAXCHdYY_4dd3T9_USB5UodxYwF-nBg0Kyqc0oy2QNBxv2P7LDR3c85J3zV_RyBABmJOjqcium9QBOZ231WcdtANrMcQD4PSIni7OVBuOMYdDVSCcKppMEZwgZxe7bM3BLJHtj-bXiaUH7GW1YLGk3U0VpAa40p-BSqc0oy2QNBxv2P7LDR3c85J3zV_RyBABmJOjqcium9Q
    }

    #[test]
    fn client_login_finish_output() {
        // password 'clientele'
        let state = "lMZg9wetFB01g4KL1laU9s4tWR9ICMptDcVJxnfAugXwVlrs8a_FCAXCHdYY_4dd3T9_USB5UodxYwF-nBg0Kyqc0oy2QNBxv2P7LDR3c85J3zV_RyBABmJOjqcium9QBOZ231WcdtANrMcQD4PSIni7OVBuOMYdDVSCcKppMEZwgZxe7bM3BLJHtj-bXiaUH7GW1YLGk3U0VpAa40p-BSqc0oy2QNBxv2P7LDR3c85J3zV_RyBABmJOjqcium9Q".to_string();
        let password = "clientele".to_string();
        let server_message = "lskLi18T8NM-WjY926___29u0RoY0XcKAz8-Wzu9gRMYWfgTuEk5qx4ZF6OZkTfpM_eufiKYIoKK2HNOTUwSf-bUsZRi9vydqe2yB3Wz5y3TiWI6CkVzACIFfbKynKGg0DQ4Sr5KYhsnMTzoF1Me27oq5sONK-R1muZ8JZpGXMB8l5mllx8-jfqFfe-8EEDIH0vyi9nzBKbzZSyexPiI00js1Vo5WU55jFWWdMldTg67WhPTgfITmgoGr-bQp-6wdwJGva12wMkvwFPptzk-0TMMu04YxIRzjC3OoKNxKtT8iOPTpq6SHFnVoMq3hwsYVFXxim36iickj0BzHeqebWVoo3FV9Da-ph8i6a7sKNGpe4Q4wN-0WpBgMurTkwvwcvhUCGMYvde0j7u1QOKDI_UjA9jeTlASlQHSmu0se7E".to_string();

        let (response, session_key) =
            client_login_finish(&state, &password, &server_message).unwrap();
        println!("{}", response);
        println!("{}", session_key);
        // example response
        // J_aMsbRzBJYQcB839mopFnkzHgCsfCpxDCR3Q-WtYEHnFtDMyf3fVk4u7KUHPIVoZo8Fc6z2KQASLr2kTpUpjQ
        // example session key
        // PGtwNX0FbT3N1dD3TcBR_aZJ9uv9NysGUouzzqYoHSYjh2a_X43vPD2P87-TGcW_6IYm5XXTlmM8GJTpjYnZCA
    }
}
