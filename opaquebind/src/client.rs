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

    let message_encoded = base64::encode_config(message_bytes, base64::URL_SAFE_NO_PAD);
    let state_encoded = base64::encode_config(state_bytes, base64::URL_SAFE_NO_PAD);

    Ok((message_encoded, state_encoded))
}

pub fn client_register_finish(client_register_state: &str, password: &str, server_message: &str) -> Result<String, Error> {
    let state_bytes = base64::decode_config(client_register_state, base64::URL_SAFE_NO_PAD)?;
    let pass_bytes = password.as_bytes();
    let message_bytes = base64::decode_config(server_message, base64::URL_SAFE_NO_PAD)?;
    let client_register_state = ClientRegistration::<Cipher>::deserialize(&state_bytes)?;


    let server_message = RegistrationResponse::<Cipher>::deserialize(&message_bytes)?;

    let c = opaque_client_register_finish(client_register_state, pass_bytes, server_message)?;

    let message_bytes = c.message.serialize();
    let message_encoded = base64::encode_config(message_bytes, base64::URL_SAFE_NO_PAD);

    Ok(message_encoded)
}

pub fn client_login(password: &str) -> Result<(String, String), Error> {
    let pass_bytes = password.as_bytes();
    let c = opaque_client_login(pass_bytes)?;

    let message_bytes = c.message.serialize();
    let state_bytes = c.state.serialize();

    let message_encoded = base64::encode_config(message_bytes, base64::URL_SAFE_NO_PAD);
    let state_encoded = base64::encode_config(state_bytes, base64::URL_SAFE_NO_PAD);

    Ok((message_encoded, state_encoded))
}

pub fn client_login_finish(client_login_state: &str, password: &str, server_message: &str) -> Result<(String, String), Error> {
    let state_bytes = base64::decode_config(client_login_state, base64::URL_SAFE_NO_PAD)?;
    let pass_bytes = password.as_bytes();
    let message_bytes = base64::decode_config(server_message, base64::URL_SAFE_NO_PAD)?;
    let client_login_state = Box::new(ClientLogin::<Cipher>::deserialize(&state_bytes)?);
    let server_message = Box::new(CredentialResponse::<Cipher>::deserialize(&message_bytes)?);

    // An InvalidLogin will be emitted in this step in the case of an incorrect password
    let c = opaque_client_login_finish(client_login_state, pass_bytes, server_message)?;

    let message_bytes = c.message.serialize();
    let session_bytes = c.session_key;

    let message_encoded = base64::encode_config(message_bytes, base64::URL_SAFE_NO_PAD);
    let session_encoded = base64::encode_config(session_bytes, base64::URL_SAFE_NO_PAD);

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
        // cC90jRh3KZlP9XpFn_EhIEkjxAgTQ3szGS1pTxDZhjw
        // 1nE62MQOsSCan3raiuU8UKuPkmCv41rxb41QrVY6ZFk
        // example state
        // cC90jRh3KZlP9XpFn_EhIEkjxAgTQ3szGS1pTxDZhjwbFAoFCZImU7iEolxMv8HWukMubkcKehoYPy7fU03TDWNsaWVudGVsZQ
        // 1nE62MQOsSCan3raiuU8UKuPkmCv41rxb41QrVY6ZFm1jxJmYNUII1EBT1uMkJ4fforeQ0NbxwACX0FS5g56DGNsaWVudGVsZQ
    }

    #[test]
    fn client_register_finish_output() {
        // password 'clientele'
        let state = "1nE62MQOsSCan3raiuU8UKuPkmCv41rxb41QrVY6ZFm1jxJmYNUII1EBT1uMkJ4fforeQ0NbxwACX0FS5g56DGNsaWVudGVsZQ".to_string();
        let password = "clientele".to_string();
        let server_message = "GGnMPMzUGlKDTd0O4Yjw2S3sNrte4a1ybatXCr_-cRvyxVgYqutFLW3oUC5bmAczDl2DMzPRvmukMc-eKmSsZg".to_string();

        let response = client_register_finish(&state, &password, &server_message).unwrap();
        println!("{}", response);
        // example response
        // jjSusq9xeAzi6YYgc35gXEzv4nJipm0KbPogXDIheQkBp0uqgutLRJhfn37_U4aH7LMf8uXYsUK3Id6wh4P_ZDJaBp3SH7r-5_EsbvLaEA--CiAXjmEH7nn3Sl8uhRkW71JuANOjlb5AxbTaZGyzSBZEYdofpumQ7TMIVy7wQQpc--yaP48xGKG0S93fn-0KxImYaXoRZTdJ-TnnDIjxQvo
        // HokzHOdiLQ2BULIauK38OflkqCKpIPh9gZqCBUGxgTcBP4WnKHuZZWI6BMXPd7hTnOznBrPIKsG4CZFlqeNK6QuCHku1lM4fi8Ep-n8dguVb8dpvU9vVP2w9L6A3RDETmYv6wCdX3PJw7y7WoRafdZ-v2DZGR9D_NvPcKVHcH03KQudID2lnpf00R_M4CtmXXajttWVdd3eh40Xp0YW41n8
    }

    #[test]
    fn client_login_output() {
        // password 'clientele'
        let password = "clientele".to_string();

        let (response, state) = client_login(&password).unwrap();
        println!("{}", response);
        println!("{}", state);
        // example response
        // IBLgmoQ-rRjs9otxi8niNKXwEPnvqjfONz8IA6LzIjnwGqkLclrQy7fGi1doawiamM7ftIZaihkhNVKHeIx4IAAAxEhDt_NBTRwKsZNQ0noZfr5_tbI3ZzZfjf5L-yxv-38
        // example state
        // yZhnv12-ad8_zm3HDzZEh-2wgmNeeSjeSyS0nR2M9QcAYiAS4JqEPq0Y7PaLcYvJ4jSl8BD576o3zjc_CAOi8yI58BqpC3Ja0Mu3xotXaGsImpjO37SGWooZITVSh3iMeCAAAMRIQ7fzQU0cCrGTUNJ6GX6-f7WyN2c2X43-S_ssb_t_AEDnL1GomLyYFaiFJ-ha-68A9I8Lx6XJAdBBiUNfokrmDvAaqQtyWtDLt8aLV2hrCJqYzt-0hlqKGSE1Uod4jHggY2xpZW50ZWxl
    }

    #[test]
    fn client_login_finish_output() {
        // password 'clientele'
        let state = "yZhnv12-ad8_zm3HDzZEh-2wgmNeeSjeSyS0nR2M9QcAYiAS4JqEPq0Y7PaLcYvJ4jSl8BD576o3zjc_CAOi8yI58BqpC3Ja0Mu3xotXaGsImpjO37SGWooZITVSh3iMeCAAAMRIQ7fzQU0cCrGTUNJ6GX6-f7WyN2c2X43-S_ssb_t_AEDnL1GomLyYFaiFJ-ha-68A9I8Lx6XJAdBBiUNfokrmDvAaqQtyWtDLt8aLV2hrCJqYzt-0hlqKGSE1Uod4jHggY2xpZW50ZWxl".to_string();
        let password = "clientele".to_string();
        let server_message = "XE2e5baMjY3k342xZ5PgC9pyxOkFdSVlR_0EzzT-k2zyxVgYqutFLW3oUC5bmAczDl2DMzPRvmukMc-eKmSsZgE_hacoe5llYjoExc93uFOc7OcGs8gqwbgJkWWp40rpC4IeS7WUzh-LwSn6fx2C5Vvx2m9T29U_bD0voDdEMROZi_rAJ1fc8nDvLtahFp91n6_YNkZH0P8289wpUdwfTcpC50gPaWel_TRH8zgK2ZddqO21ZV13d6HjRenRhbjWf16FzOJAzS7mEtX0xyNJvjcA4r-MpHt7sA6PXHgZNZhzkHaurq_ZutvRWCWLzGHPjGs5t_nBf4oqIuL_hfQjT0wAAOZYJP4U2mUe3oT-3LXInys1LjAYGX1jvow53a-pJex6jcHOFPwEvOhn3CKXGrHL913o84s07IQNJ0TZ26zzC9M".to_string();

        let (response, session_key) = client_login_finish(&state, &password, &server_message).unwrap();
        println!("{}", response);
        println!("{}", session_key);
        // example response
        // YATUKRGRBXjup27rb8TeoHFw8AlyZ1Kx5FB2oa4HLohCyU-BDaPLWm9CiRRCGHvp-PV9PThsLtjDLJXDEtnoXA
        // example session key
        // zySjwa5CpddMzSydqKOvXZHQrtRK-VD83aOPMAB_1gEVxSscBywmS8XxZze3letN9whXUiRfSEfGel9e-5XGgQ
    }
}