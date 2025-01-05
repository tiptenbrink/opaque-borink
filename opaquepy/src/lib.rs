use opaque_borink::client::{
    client_login_finish, client_login_start, client_register_finish, client_register_start, ClientStateLogin, ClientStateRegistration
};
use opaque_borink::server::{
    server_login_finish, server_login_start, server_register_finish, server_register_start, PasswordFile, ServerSetup
};
use opaque_borink::encoded::{encode_bytes, decode_string};
use opaque_borink::Error;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

pub type OpaquePyResult<T> = Result<T, OpaquePyError>;

pub struct OpaquePyError(Error);

impl From<Error> for OpaquePyError {
    fn from(e: Error) -> Self {
        OpaquePyError(e)
    }
}

impl From<OpaquePyError> for PyErr {
    fn from(e: OpaquePyError) -> Self {
        match e.0 {
            Error::ProtocolError(oe) => PyValueError::new_err(format!("{:?}", oe)),
            Error::DecodeError(oe) => PyValueError::new_err(oe.to_string()),
        }
    }
}

#[pymodule]
fn opaquepy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let internal = PyModule::new(m.py(), "_internal")?;
    internal.add_function(wrap_pyfunction!(create_setup_py, &internal)?)?;
    internal.add_function(wrap_pyfunction!(register_server_py, &internal)?)?;
    internal.add_function(wrap_pyfunction!(register_server_finish_py, &internal)?)?;
    internal.add_function(wrap_pyfunction!(register_client_py, &internal)?)?;
    internal.add_function(wrap_pyfunction!(register_client_finish_py, &internal)?)?;
    internal.add_function(wrap_pyfunction!(login_server_py, &internal)?)?;
    internal.add_function(wrap_pyfunction!(login_server_finish_py, &internal)?)?;
    internal.add_function(wrap_pyfunction!(login_client_py, &internal)?)?;
    internal.add_function(wrap_pyfunction!(login_client_finish_py, &internal)?)?;

    m.add_submodule(&internal)?;

    Ok(())
}

#[pyfunction]
fn create_setup_py() -> String {
    encode_bytes(&ServerSetup::create().serialize())
}

#[pyfunction]
fn register_server_py(
    setup: &str,
    client_request: &str,
    credential_id: &str,
) -> OpaquePyResult<String> {
    let setup_bytes = decode_string(setup)?;
    let setup = ServerSetup::deserialize(&setup_bytes)?;
    let result = server_register_start(&mut setup.view(), &decode_string(client_request)?, credential_id.as_bytes())?;

    Ok(encode_bytes(&result.response))
}

#[pyfunction]
fn register_server_finish_py(client_request_finish: &str) -> OpaquePyResult<String> {
    let result = server_register_finish(&decode_string(client_request_finish)?)?;

    Ok(encode_bytes(&result.serialize()))
}

#[pyfunction]
fn register_client_py(password: &str) -> OpaquePyResult<(String, String)> {
    let mut client_state = ClientStateRegistration::setup();

    let result = client_register_start(&mut client_state, password.as_bytes())?;
    let message_encoded = encode_bytes(&result.response);
    let state_encoded = encode_bytes(&client_state.serialize());

    Ok((message_encoded, state_encoded))
}

#[pyfunction]
fn register_client_finish_py(
    client_register_state: &str,
    password: &str,
    server_message: &str,
) -> OpaquePyResult<String> {
    let client_register_state = decode_string(client_register_state)?;
    let mut client_state = ClientStateRegistration::deserialize(&client_register_state)?;
    let server_message = decode_string(server_message)?;
    let result = client_register_finish(&mut client_state, password.as_bytes(), &server_message)?;

    Ok(encode_bytes(&result.response))
}

#[pyfunction]
fn login_server_py(
    setup: &str,
    password_file: &str,
    client_request: &str,
    credential_id: &str,
) -> OpaquePyResult<(String, String)> {
    let setup = ServerSetup::deserialize(&decode_string(setup)?)?;
    let password_file = PasswordFile::deserialize(&decode_string(password_file)?)?;
    let client_request = decode_string(client_request)?;

    let result = server_login_start(
        &mut setup.view(),
        &password_file,
        &client_request,
        credential_id,
    )?;

    let response_encoded = encode_bytes(&result.response);
    let state_encoded = encode_bytes(&result.state);

    Ok((response_encoded, state_encoded))
}

#[pyfunction]
fn login_server_finish_py(
    client_request_finish: &str,
    login_state: &str,
) -> OpaquePyResult<String> {
    let client_request_finish = decode_string(client_request_finish)?;
    let login_state = decode_string(login_state)?;

    let result = server_login_finish(&client_request_finish, &login_state)?;

    Ok(encode_bytes(&result.shared_secret))
}

#[pyfunction]
fn login_client_py(password: &str) -> OpaquePyResult<(String, String)> {
    let mut client_state = ClientStateLogin::setup();

    let result = client_login_start(&mut client_state, password.as_bytes())?;

    let message_encoded = encode_bytes(&result.response);
    let state_encoded = encode_bytes(&client_state.serialize());

    Ok((message_encoded, state_encoded))
}

#[pyfunction]
fn login_client_finish_py(
    client_login_state: &str,
    password: &str,
    server_message: &str,
) -> OpaquePyResult<(String, String)> {

    let mut client_state = ClientStateLogin::deserialize(&decode_string(client_login_state)?)?;
    let server_message = decode_string(server_message)?;

    let result = client_login_finish(
        &mut client_state,
        password.as_bytes(),
        &server_message,
    )?;
    
    let message_encoded = encode_bytes(&result.response);
    let shared_secret_encoded = encode_bytes(&result.shared_secret);

    Ok((message_encoded, shared_secret_encoded))
}
