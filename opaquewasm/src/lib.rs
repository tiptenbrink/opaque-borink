use opaque_borink::client::{
    client_login, client_login_finish, client_register, client_register_finish,
};
use opaque_borink::{Error, ProtocolError};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

pub type OpaqueJsResult<T> = Result<T, JsValue>;

fn make_js_val<T: Debug>(val: T, info: &str) -> JsValue {
    JsValue::from(format!("{:?} {}", val, info))
}

pub struct OpaqueJsError(Error);

impl From<Error> for OpaqueJsError {
    fn from(e: Error) -> Self {
        OpaqueJsError(e)
    }
}

/// Create an InvalidLogin error for JavaScript
fn invalid_login(info: &str) -> JsValue {
    let err = js_sys::Error::new(info);
    err.set_name("InvalidLogin");
    JsValue::from(err)
}

impl From<OpaqueJsError> for JsValue {
    fn from(e: OpaqueJsError) -> Self {
        let info = "default error";
        match e.0 {
            Error::ProtocolError(oe) => match oe {
                ProtocolError::InvalidLoginError => invalid_login(info),
                _ => make_js_val(oe, info),
            },
            Error::DecodeError(oe) => JsValue::from(format!("{} {}", oe, info)),
        }
    }
}

#[wasm_bindgen]
pub struct MessageState {
    message: String,
    state: String,
}

#[wasm_bindgen]
impl MessageState {
    #[wasm_bindgen(constructor)]
    pub fn new(message: String, state: String) -> Self {
        MessageState { message, state }
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn state(&self) -> String {
        self.state.to_owned()
    }
}

#[wasm_bindgen]
pub struct MessageSession {
    message: String,
    session: String,
}

#[wasm_bindgen]
impl MessageSession {
    #[wasm_bindgen(constructor)]
    pub fn new(message: String, session: String) -> Self {
        Self { message, session }
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn session(&self) -> String {
        self.session.to_owned()
    }
}

#[wasm_bindgen]
pub fn client_register_wasm(password: &str) -> OpaqueJsResult<MessageState> {
    Ok(client_register(password).map(|(message, state)| MessageState { message, state })
        .map_err(OpaqueJsError)?)
}

#[wasm_bindgen]
pub fn client_register_finish_wasm(
    client_register_state: &str,
    password: &str,
    server_message: &str,
) -> OpaqueJsResult<String> {
    Ok(
        client_register_finish(client_register_state, password, server_message)
            .map_err(OpaqueJsError)?,
    )
}

#[wasm_bindgen]
pub fn client_login_wasm(password: &str) -> OpaqueJsResult<MessageState> {
    Ok(client_login(password).map(|(message, state)| MessageState { message, state })
        .map_err(OpaqueJsError)?)
}

#[wasm_bindgen]
pub fn client_login_finish_wasm(
    client_login_state: &str,
    password: &str,
    server_message: &str,
) -> OpaqueJsResult<MessageSession> {
    Ok(
        client_login_finish(client_login_state, password, server_message).map(|(message, session)| MessageSession { message, session })
            .map_err(OpaqueJsError)?,
    )
}
