use std::fmt::Debug;
use wasm_bindgen::prelude::*;
use opaquebind_core::Error;
use opaquebind_core::client::{client_register, client_register_finish, client_login, client_login_finish};
use opaquebind_core::{PakeError, ProtocolError};

pub type OpaqueJsResult<T> = Result<T, JsValue>;

fn make_js_val<T: Debug>(val: T, info: &str) -> JsValue {
    JsValue::from(format!("{:?} {}", val, info))
}

trait ToJsVal {
    fn to_jsval(&self, info: &str) -> JsValue;
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
                ProtocolError::VerificationError(pe) => {
                    let err = Error::PakeError(pe);
                    JsValue::from(OpaqueJsError::from(err))
                },
                _ => make_js_val(oe, info)
            }
            Error::PakeError(oe) => match oe {
                PakeError::InvalidLoginError => invalid_login(info),
                _ => make_js_val(oe, info)
            },
            Error::DecodeError(oe) => JsValue::from(format!("{} {}", oe, info)),
        }
    }
}

#[wasm_bindgen]
pub struct MessageState {
    message: String,
    state: String
}

#[wasm_bindgen]
impl MessageState {
    #[wasm_bindgen(constructor)]
    pub fn new(message: String, state: String) -> MessageState {
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
    session: String
}

#[wasm_bindgen]
impl MessageSession {
    #[wasm_bindgen(constructor)]
    pub fn new(message: String, state: String) -> MessageState {
        MessageState { message, state }
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
    Ok(client_register(password).and_then(|(message, state)| Ok(MessageState { message, state }))
        .map_err(|e| OpaqueJsError(e))?)
}

#[wasm_bindgen]
pub fn client_register_finish_wasm(client_register_state: &str, server_message: &str) -> OpaqueJsResult<String> {
    Ok(client_register_finish(client_register_state, server_message)
        .map_err(|e| OpaqueJsError(e))?)
}

#[wasm_bindgen]
pub fn client_login_wasm(password: &str) -> OpaqueJsResult<MessageState> {
    Ok(client_login(password).and_then(|(message, state)| Ok(MessageState { message, state }))
        .map_err(|e| OpaqueJsError(e))?)
}

#[wasm_bindgen]
pub fn client_login_finish_wasm(client_login_state: &str, server_message: &str) -> OpaqueJsResult<MessageSession> {
    Ok(client_login_finish(client_login_state, server_message).and_then(|(message, session)| Ok(MessageSession { message, session }))
        .map_err(|e| OpaqueJsError(e))?)
}