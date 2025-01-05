pub use crate::opaque_impl::{
    server_login_finish, server_login_start, server_register_finish, server_register_start,
};
pub use crate::opaque_impl::{
    PasswordFile, ServerLoginFinishResult, ServerLoginStartResult, ServerRegistrationStartResult,
    ServerSetup,
};
pub use crate::opaque_impl::{
    LOGIN_SERVER_MESSAGE_LEN, LOGIN_SERVER_STATE_LEN, PASSWORD_FILE_LEN,
    REGISTER_SERVER_MESSAGE_LEN, SERVER_SETUP_LEN, SHARED_SECRET_LEN,
};

#[cfg(test)]
mod tests {
    use crate::encoded::{decode_string, encode_bytes};

    use super::*;

    #[test]
    fn server_register_output() {
        // password 'clientele'
        let setup = "C5HVEMyOKYglRys_3a58GHLeRM0oa_pjxSO6mu-WEnfIdOO5mE7GpCz_Z0xrntzbeMQI3GACQet9N_3lh1eaEWM18tqMDhUEJ_TwfSJNEXavKLc2DHlxWcd5Xd8aiPMJJ11dZmU76urlWHZw5xJuvDfLbdnt2tIj-fmY9PobZQg";
        let message = "dn17rg3EzeikxJ4rWay0DCnSax5JoQOEifmhfwj5Jjs";
        let cred_id = "someperson";

        let setup = decode_string(setup).unwrap();
        let setup = ServerSetup::deserialize(&setup).unwrap();
        let message = decode_string(message).unwrap();

        let response =
            server_register_start(&mut setup.view(), &message, cred_id.as_bytes()).unwrap();
        println!("{}", encode_bytes(&response.response));
        // example response
        // fDCnRbPyYdSCw_6cFCDzo5Zcd5OwV2TnWNg43eWQIyqASLH7HrrwUUQdYwcPA8Bigtj_ISL-GC9iHKheKl0rew
    }

    #[test]
    fn test_server_register_finish() {
        let client_message = "LJ0rg3mSZ-x1tDbobI0xvroBjAPQ5fnAgrnEmxc67giA0XDjR8pJaOuNGlWtRku5Hk57yBlL6YrjBUQJ--7OMhPZra40WvmWSu7yT8s-CBAsE0jobWK-9qXk3xDv7TlK-g_TF3JzR3s8MntBWjIuN5Ii7Le93coLGLvm7xjQtuYHbszz3HBv-gBu_xlj7YitpgyQzYpcJGslbezqxEvZz4Jz0R64np94JBDibI7syTw13ZJ74tbjWiJbvwvKb5a-";
        let password_file =
            server_register_finish(&decode_string(client_message).unwrap()).unwrap();
        println!("{}", encode_bytes(&password_file.serialize()))

        // example file:
        // LJ0rg3mSZ-x1tDbobI0xvroBjAPQ5fnAgrnEmxc67giA0XDjR8pJaOuNGlWtRku5Hk57yBlL6YrjBUQJ--7OMhPZra40WvmWSu7yT8s-CBAsE0jobWK-9qXk3xDv7TlK-g_TF3JzR3s8MntBWjIuN5Ii7Le93coLGLvm7xjQtuYHbszz3HBv-gBu_xlj7YitpgyQzYpcJGslbezqxEvZz4Jz0R64np94JBDibI7syTw13ZJ74tbjWiJbvwvKb5a-
    }

    #[test]
    fn test_server_login() {
        let setup = "C5HVEMyOKYglRys_3a58GHLeRM0oa_pjxSO6mu-WEnfIdOO5mE7GpCz_Z0xrntzbeMQI3GACQet9N_3lh1eaEWM18tqMDhUEJ_TwfSJNEXavKLc2DHlxWcd5Xd8aiPMJJ11dZmU76urlWHZw5xJuvDfLbdnt2tIj-fmY9PobZQg";
        let client_message = "8FZa7PGvxQgFwh3WGP-HXd0_f1EgeVKHcWMBfpwYNCsqnNKMtkDQcb9j-yw0d3POSd81f0cgQAZiTo6nIrpvUATmdt9VnHbQDazHEA-D0iJ4uzlQbjjGHQ1UgnCqaTBG";
        // password 'clientiele'
        let password_file = "LJ0rg3mSZ-x1tDbobI0xvroBjAPQ5fnAgrnEmxc67giA0XDjR8pJaOuNGlWtRku5Hk57yBlL6YrjBUQJ--7OMhPZra40WvmWSu7yT8s-CBAsE0jobWK-9qXk3xDv7TlK-g_TF3JzR3s8MntBWjIuN5Ii7Le93coLGLvm7xjQtuYHbszz3HBv-gBu_xlj7YitpgyQzYpcJGslbezqxEvZz4Jz0R64np94JBDibI7syTw13ZJ74tbjWiJbvwvKb5a-";
        let cred_id = "someperson";
        let setup = ServerSetup::deserialize(&decode_string(setup).unwrap()).unwrap();
        let password_file =
            PasswordFile::deserialize(&decode_string(password_file).unwrap()).unwrap();
        let client_message = decode_string(client_message).unwrap();
        let result =
            server_login_start(&mut setup.view(), &password_file, &client_message, cred_id)
                .unwrap();

        println!("resp={}", encode_bytes(&result.response));
        println!("state={}", encode_bytes(&result.state));
        // example response
        // lskLi18T8NM-WjY926___29u0RoY0XcKAz8-Wzu9gRMYWfgTuEk5qx4ZF6OZkTfpM_eufiKYIoKK2HNOTUwSf-bUsZRi9vydqe2yB3Wz5y3TiWI6CkVzACIFfbKynKGg0DQ4Sr5KYhsnMTzoF1Me27oq5sONK-R1muZ8JZpGXMB8l5mllx8-jfqFfe-8EEDIH0vyi9nzBKbzZSyexPiI00js1Vo5WU55jFWWdMldTg67WhPTgfITmgoGr-bQp-6wdwJGva12wMkvwFPptzk-0TMMu04YxIRzjC3OoKNxKtT8iOPTpq6SHFnVoMq3hwsYVFXxim36iickj0BzHeqebWVoo3FV9Da-ph8i6a7sKNGpe4Q4wN-0WpBgMurTkwvwcvhUCGMYvde0j7u1QOKDI_UjA9jeTlASlQHSmu0se7E
        // example state
        // B552YqssmUw1OOCGiXnnJB51DwX38aYMhxTl7elzLHbnVlX1cXdlXcT2nUlU3gw3IyH-6PsAhGXDv-X20Knt3d6PlUtCThpEuiH1RxehA1u9R_OBS8ctVeeHLHhzNys4vLeWBQzHh_-sW3erjRuMBUxQQwMcgQl-4Kh_RWfIp6M8a3A1fQVtPc3V0PdNwFH9pkn26_03KwZSi7POpigdJiOHZr9fje88PY_zv5MZxb_ohiblddOWYzwYlOmNidkI
    }

    #[test]
    fn test_server_login_finish() {
        // correspond to above
        let client_message = "J_aMsbRzBJYQcB839mopFnkzHgCsfCpxDCR3Q-WtYEHnFtDMyf3fVk4u7KUHPIVoZo8Fc6z2KQASLr2kTpUpjQ";
        let state = "B552YqssmUw1OOCGiXnnJB51DwX38aYMhxTl7elzLHbnVlX1cXdlXcT2nUlU3gw3IyH-6PsAhGXDv-X20Knt3d6PlUtCThpEuiH1RxehA1u9R_OBS8ctVeeHLHhzNys4vLeWBQzHh_-sW3erjRuMBUxQQwMcgQl-4Kh_RWfIp6M8a3A1fQVtPc3V0PdNwFH9pkn26_03KwZSi7POpigdJiOHZr9fje88PY_zv5MZxb_ohiblddOWYzwYlOmNidkI";
        let state = decode_string(state).unwrap();
        let client_message = decode_string(client_message).unwrap();
        let session = server_login_finish(&client_message, &state).unwrap();
        let session = encode_bytes(&session.shared_secret);
        let expected_key = "PGtwNX0FbT3N1dD3TcBR_aZJ9uv9NysGUouzzqYoHSYjh2a_X43vPD2P87-TGcW_6IYm5XXTlmM8GJTpjYnZCA";
        assert_eq!(expected_key, session);
    }
}
