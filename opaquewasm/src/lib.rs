use opaque_borink::client::{
    client_login_finish, client_login_start, client_register_finish, client_register_start
};
use opaque_borink::encoded::{decode_string, encode_bytes};
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
pub struct ClientStateRegistration {
    state: opaque_borink::client::ClientStateRegistration,
    message: String
}

#[wasm_bindgen]
impl ClientStateRegistration {
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.to_owned()
    }
}

#[wasm_bindgen]
pub struct ClientStateLogin {
    state: opaque_borink::client::ClientStateLogin,
    message: String
}

#[wasm_bindgen(getter_with_clone)]
pub struct ClientLoginResult {
    pub message: String,
    pub shared_secret: String
}

#[wasm_bindgen]
impl ClientStateLogin {
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.to_owned()
    }
}

#[wasm_bindgen]
pub fn client_register_wasm(password: &str) -> OpaqueJsResult<ClientStateRegistration> {
    let mut state = opaque_borink::client::ClientStateRegistration::setup();

    let result = client_register_start(&mut state, password.as_bytes())
    .map_err(OpaqueJsError)?;

    Ok(ClientStateRegistration {
        message: encode_bytes(&result.response),
        state
    })
}

#[wasm_bindgen]
pub fn client_register_finish_wasm(
    mut client_register_state: ClientStateRegistration,
    password: &str,
    server_message: &str,
) -> OpaqueJsResult<String> {
    let server_message = decode_string(server_message).map_err(OpaqueJsError)?;

    let result = client_register_finish(&mut client_register_state.state, password.as_bytes(), &server_message)
    .map_err(OpaqueJsError)?;

    Ok(
        encode_bytes(&result.response)
    )
}

#[wasm_bindgen]
pub fn client_login_wasm(password: &str) -> OpaqueJsResult<ClientStateLogin> {
    let mut state = opaque_borink::client::ClientStateLogin::setup();

    let result = client_login_start(&mut state, password.as_bytes())
    .map_err(OpaqueJsError)?;

    Ok(ClientStateLogin {
        state,
        message: encode_bytes(&result.response)
    })
}

#[wasm_bindgen]
pub fn client_login_finish_wasm(
    mut client_login_state: ClientStateLogin,
    password: &str,
    server_message: &str,
) -> OpaqueJsResult<ClientLoginResult> {
    let server_message = decode_string(server_message).map_err(OpaqueJsError)?;

    let result = client_login_finish(&mut client_login_state.state, password.as_bytes(), &server_message)
    .map_err(OpaqueJsError)?;

    Ok(
        ClientLoginResult {
            message: encode_bytes(&result.response),
            shared_secret: encode_bytes(&result.shared_secret)
        }        
    )
}
