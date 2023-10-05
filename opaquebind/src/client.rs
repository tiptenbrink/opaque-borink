use base64::{Engine as _, engine::general_purpose as b64};
use opaque_ke::{ClientLogin, ClientLoginFinishParameters, ClientLoginFinishResult,
                ClientLoginStartResult, ClientRegistration,
                ClientRegistrationFinishParameters, ClientRegistrationFinishResult,
                ClientRegistrationStartResult, CredentialResponse, RegistrationResponse};
use opaque_ke::errors::ProtocolError;
use rand::rngs::OsRng;
use crate::Cipher;
use crate::Error;

pub fn client_register(password: &str) -> Result<(String, String), Error> {
    let pass_bytes = password.as_bytes();
    let c = opaque_client_register(pass_bytes)?;

    let message_bytes = c.message.serialize();
    let state_bytes = c.state.serialize();

    let message_encoded = b64::URL_SAFE_NO_PAD.encode(message_bytes);
    let state_encoded = b64::URL_SAFE_NO_PAD.encode(state_bytes);

    Ok((message_encoded, state_encoded))
}

pub fn client_register_finish(client_register_state: &str, password: &str, server_message: &str) -> Result<String, Error> {
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

pub fn client_login_finish(client_login_state: &str, password: &str, server_message: &str) -> Result<(String, String), Error> {
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

fn opaque_client_register(password: &[u8]) -> Result<ClientRegistrationStartResult<Cipher>, ProtocolError> {
    let mut client_rng = OsRng;
    ClientRegistration::<Cipher>::start(
        &mut client_rng,
        password,
    )
}

fn opaque_client_register_finish(client_register_state: ClientRegistration<Cipher>, password: &[u8], server_message: RegistrationResponse<Cipher>)
                                 -> Result<ClientRegistrationFinishResult<Cipher>, ProtocolError> {
    let mut client_rng = OsRng;
    client_register_state.finish(
        &mut client_rng,
        password,
        server_message,
        ClientRegistrationFinishParameters::default()
    )
}

fn opaque_client_login(password: &[u8]) -> Result<ClientLoginStartResult<Cipher>, ProtocolError> {
    let mut client_rng = OsRng;
    ClientLogin::<Cipher>::start(
        &mut client_rng,
        password,
    )
}

fn opaque_client_login_finish(client_login_state: Box<ClientLogin<Cipher>>, password: &[u8], server_message: Box<CredentialResponse<Cipher>>)
                              -> Result<ClientLoginFinishResult<Cipher>, ProtocolError> {
    client_login_state.finish(
        password,
        *server_message,
        ClientLoginFinishParameters::default()
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
        // 3i3SEzJNZKvsIfADx1lf-zk4SNeitkTp41-kpxWOUxE
        // example state
        // lhGS4SnWizb6BWPxsQfY513RQ8yt_ODBi_pQQP5hygzeLdITMk1kq-wh8APHWV_7OThI16K2ROnjX6SnFY5TEQ
    }

    #[test]
    fn client_register_finish_output() {
        // password 'clientele'
        let state = "lhGS4SnWizb6BWPxsQfY513RQ8yt_ODBi_pQQP5hygzeLdITMk1kq-wh8APHWV_7OThI16K2ROnjX6SnFY5TEQ".to_string();
        let password = "clientele".to_string();
        let server_message = "mKbmMmzMVuq9r2yrfWtJXQCYTFVxAD3ZHkPLFhGY-hqASLH7HrrwUUQdYwcPA8Bigtj_ISL-GC9iHKheKl0rew".to_string();

        let response = client_register_finish(&state, &password, &server_message).unwrap();
        println!("{}", response);
        // example response
        // wBtSZIhSPTwEY13yNT6nfWhj0WVRnhiqsnAYhUu7nj_5mmv-Trgm3DULYEZLwYhQaadsk8rI8n0PD1mZi8AL7517p9b5wisa4TxrNDyifHLUI_P09Re5KTf8CUr_0I6vMYOhCBl7WgItYfj1h-lAZU5E77fmsl-6l4MIZ5oYIKENNClbtgX9GYz1WkrZpJdeDdnAA5AI-0cfh3AX8UjpD46BhzwxkFDhtMra4vpRFQSAvu7gVzsZSDQJoqTcYXjy
    }

    #[test]
    fn client_login_output() {
        // password 'clientele'
        let password = "abcd".to_string();

        let (response, state) = client_login(&password).unwrap();
        println!("{}", response);
        println!("{}", state);
        // example response
        // iIx1sW5pj2GlKnb4V1MRrCFmLTkW_hhyXQCGmczMZUJZ50HlQnVjuPyvv6oKpD7d0ZzPDbZcnliqDlN-GHdrwUpAvLRYzwk1earfTNefonfH9faSj7307IMRwrfGyOhZ
        // example state
        // TuuU6PmCvQrP6qb4-5a7_bsKc7Jth_Rr6IxImMMmWgOIjHWxbmmPYaUqdvhXUxGsIWYtORb-GHJdAIaZzMxlQlnnQeVCdWO4_K-_qgqkPt3RnM8NtlyeWKoOU34Yd2vBSkC8tFjPCTV5qt9M15-id8f19pKPvfTsgxHCt8bI6FlHWb7wtii5WeSG7D3lKKM5VhExzBReT7223RWU60qeB1nnQeVCdWO4_K-_qgqkPt3RnM8NtlyeWKoOU34Yd2vB
    }

    #[test]
    fn client_login_finish_output() {
        // password 'clientele'
        let state = "TuuU6PmCvQrP6qb4-5a7_bsKc7Jth_Rr6IxImMMmWgOIjHWxbmmPYaUqdvhXUxGsIWYtORb-GHJdAIaZzMxlQlnnQeVCdWO4_K-_qgqkPt3RnM8NtlyeWKoOU34Yd2vBSkC8tFjPCTV5qt9M15-id8f19pKPvfTsgxHCt8bI6FlHWb7wtii5WeSG7D3lKKM5VhExzBReT7223RWU60qeB1nnQeVCdWO4_K-_qgqkPt3RnM8NtlyeWKoOU34Yd2vB".to_string();
        let password = "clientele".to_string();
        let server_message = "1kHXX25U1NE0nki_rL-5KnPRre6-CD2P4ApqhDOXL2m5xgVIT1oO4M5n5r6b2mfGx7Xq4kiMFQH8pfrGLok2H0A20_p30giVapbzQL1QYRiG2jJK2AUlhOK6lTr5YshNJNOqHgn0eGFUWiYwZ606pYfIJDaiUa5p9Mhmb9lWTpzP9akkvjDkbaxViRLB_-T9QZyTjiy-67bkepYWTeCEnqrkdw-_ckKEcBRJrlVX0yMqHtzwMdoMX6LKdT02BCXMde7kdxT5mWUMcMozBg0SLrTAehZiHgqQYq94TAK-lmiePUljPSSHFERFm-0h7r2t2QL-hLi1DmY53upbKvTQFyN-IzuAmK7UsG4MgcYVcGKQXlrF4keZVJaa5dOz5YaFYp1oBXm26hUdtKzXf-9gVTtlDc3ep-G8GNyiDNLiVwU".to_string();

        let (response, session_key) = client_login_finish(&state, &password, &server_message).unwrap();
        println!("{}", response);
        println!("{}", session_key);
        // example response
        // eP_3skqnrkJXs-AZpXqaxihP4EsF1ek8eDxH4ktDflExfEvNF99-oVZ24hkahw05v-rH9B-1WCs91dGteIMH9Q
        // example session key
        // pzt_-CKrOufYAsdCKzmaMZwQ78ZPLB1zTtffoJswFRoAEKfsBEQqFtxT-odtT2cEQPfQFkjntmBE3PlS_zLUUw
    }
}