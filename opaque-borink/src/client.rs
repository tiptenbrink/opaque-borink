pub use crate::opaque_impl::{
    client_login_finish, client_login_start, client_register_finish, client_register_start,
};
pub use crate::opaque_impl::{
    ClientLoginFinishResult, ClientLoginStartResult, ClientRegistrationFinishResult,
    ClientRegistrationStartResult, ClientStateLogin, ClientStateRegistration,
};
pub use crate::opaque_impl::{
    LOGIN_CLIENT_MESSAGE_LEN, LOGIN_CLIENT_STATE_LEN, LOGIN_FINISH_MESSAGE_LEN,
    REGISTER_CLIENT_MESSAGE_LEN, REGISTER_CLIENT_STATE_LEN, REGISTER_FINISH_MESSAGE_LEN,
    SHARED_SECRET_LEN,
};

#[cfg(test)]
mod tests {
    use crate::encoded::{decode_string, encode_bytes};

    use super::*;

    #[test]
    fn client_register_output() {
        // password 'clientele'
        let password = "clientele";
        let mut state = ClientStateRegistration::setup();
        let result = client_register_start(&mut state, password.as_bytes()).unwrap();
        println!("{}", encode_bytes(&result.response));
        // We don't make the state public now, fix if new test vectors are necessary
        // example response
        // dn17rg3EzeikxJ4rWay0DCnSax5JoQOEifmhfwj5Jjs
        // example state
        // 9klWX8NMmsKZmGw5i1HflGrDdwbrHXKJn2QnaKgKnA12fXuuDcTN6KTEnitZrLQMKdJrHkmhA4SJ-aF_CPkmOw
    }

    #[test]
    fn client_register_finish_output() {
        // password 'clientele'
        let state = "9klWX8NMmsKZmGw5i1HflGrDdwbrHXKJn2QnaKgKnA12fXuuDcTN6KTEnitZrLQMKdJrHkmhA4SJ-aF_CPkmOw";
        let password = "clientele";
        let server_message = "fDCnRbPyYdSCw_6cFCDzo5Zcd5OwV2TnWNg43eWQIyqASLH7HrrwUUQdYwcPA8Bigtj_ISL-GC9iHKheKl0rew";
        let mut state =
            ClientStateRegistration::deserialize(&decode_string(state).unwrap()).unwrap();
        let server_message = decode_string(server_message).unwrap();
        let response =
            client_register_finish(&mut state, password.as_bytes(), &server_message).unwrap();
        println!("{}", encode_bytes(&response.response));
        // example response
        // LJ0rg3mSZ-x1tDbobI0xvroBjAPQ5fnAgrnEmxc67giA0XDjR8pJaOuNGlWtRku5Hk57yBlL6YrjBUQJ--7OMhPZra40WvmWSu7yT8s-CBAsE0jobWK-9qXk3xDv7TlK-g_TF3JzR3s8MntBWjIuN5Ii7Le93coLGLvm7xjQtuYHbszz3HBv-gBu_xlj7YitpgyQzYpcJGslbezqxEvZz4Jz0R64np94JBDibI7syTw13ZJ74tbjWiJbvwvKb5a-
    }

    #[test]
    fn client_login_output() {
        // password 'clientele'
        let password = "clientele";
        let mut state = ClientStateLogin::setup();
        let result = client_login_start(&mut state, password.as_bytes()).unwrap();
        println!("resp={}", encode_bytes(&result.response));
        // We don't make the state public now, fix if new tests vectors are required
        // println!("state={}", state);
        // example response
        // 8FZa7PGvxQgFwh3WGP-HXd0_f1EgeVKHcWMBfpwYNCsqnNKMtkDQcb9j-yw0d3POSd81f0cgQAZiTo6nIrpvUATmdt9VnHbQDazHEA-D0iJ4uzlQbjjGHQ1UgnCqaTBG
        // example state
        // lMZg9wetFB01g4KL1laU9s4tWR9ICMptDcVJxnfAugXwVlrs8a_FCAXCHdYY_4dd3T9_USB5UodxYwF-nBg0Kyqc0oy2QNBxv2P7LDR3c85J3zV_RyBABmJOjqcium9QBOZ231WcdtANrMcQD4PSIni7OVBuOMYdDVSCcKppMEZwgZxe7bM3BLJHtj-bXiaUH7GW1YLGk3U0VpAa40p-BSqc0oy2QNBxv2P7LDR3c85J3zV_RyBABmJOjqcium9Q
    }

    #[test]
    fn client_login_finish_output() {
        // password 'clientele'
        let state = "lMZg9wetFB01g4KL1laU9s4tWR9ICMptDcVJxnfAugXwVlrs8a_FCAXCHdYY_4dd3T9_USB5UodxYwF-nBg0Kyqc0oy2QNBxv2P7LDR3c85J3zV_RyBABmJOjqcium9QBOZ231WcdtANrMcQD4PSIni7OVBuOMYdDVSCcKppMEZwgZxe7bM3BLJHtj-bXiaUH7GW1YLGk3U0VpAa40p-BSqc0oy2QNBxv2P7LDR3c85J3zV_RyBABmJOjqcium9Q";
        let password = "clientele";
        let server_message = "lskLi18T8NM-WjY926___29u0RoY0XcKAz8-Wzu9gRMYWfgTuEk5qx4ZF6OZkTfpM_eufiKYIoKK2HNOTUwSf-bUsZRi9vydqe2yB3Wz5y3TiWI6CkVzACIFfbKynKGg0DQ4Sr5KYhsnMTzoF1Me27oq5sONK-R1muZ8JZpGXMB8l5mllx8-jfqFfe-8EEDIH0vyi9nzBKbzZSyexPiI00js1Vo5WU55jFWWdMldTg67WhPTgfITmgoGr-bQp-6wdwJGva12wMkvwFPptzk-0TMMu04YxIRzjC3OoKNxKtT8iOPTpq6SHFnVoMq3hwsYVFXxim36iickj0BzHeqebWVoo3FV9Da-ph8i6a7sKNGpe4Q4wN-0WpBgMurTkwvwcvhUCGMYvde0j7u1QOKDI_UjA9jeTlASlQHSmu0se7E";
        let mut state = ClientStateLogin::deserialize(&decode_string(state).unwrap()).unwrap();
        let server_message = decode_string(server_message).unwrap();
        let result = client_login_finish(&mut state, password.as_bytes(), &server_message).unwrap();
        println!("{}", encode_bytes(&result.response));
        println!("{}", encode_bytes(&result.shared_secret));
        // example response
        // J_aMsbRzBJYQcB839mopFnkzHgCsfCpxDCR3Q-WtYEHnFtDMyf3fVk4u7KUHPIVoZo8Fc6z2KQASLr2kTpUpjQ
        // example session key
        // PGtwNX0FbT3N1dD3TcBR_aZJ9uv9NysGUouzzqYoHSYjh2a_X43vPD2P87-TGcW_6IYm5XXTlmM8GJTpjYnZCA
    }
}
