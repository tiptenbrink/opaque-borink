use opaquebind::{Error, generate_keys};
use opaquebind::server::{register_server, register_server_finish, login_server, login_server_finish};
use opaquebind::client::{client_register, client_register_finish};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

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
            Error::PakeError(oe) => PyValueError::new_err(format!("{:?}", oe)),
            Error::DecodeError(oe) => PyValueError::new_err(oe.to_string()),
        }
    }
}

#[pymodule]
fn opaquepy(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    let opqrust = PyModule::new(py, "_opqrust")?;
    opqrust.add_function(wrap_pyfunction!(generate_keys_py, opqrust)?)?;
    opqrust.add_function(wrap_pyfunction!(register_server_py, opqrust)?)?;
    opqrust.add_function(wrap_pyfunction!(register_server_finish_py, opqrust)?)?;
    opqrust.add_function(wrap_pyfunction!(register_client_py, m)?)?;
    opqrust.add_function(wrap_pyfunction!(register_client_finish_py, m)?)?;
    opqrust.add_function(wrap_pyfunction!(login_server_py, opqrust)?)?;
    opqrust.add_function(wrap_pyfunction!(login_server_finish_py, opqrust)?)?;

    m.add_submodule(opqrust)?;

    Ok(())
}

#[pyfunction]
fn generate_keys_py() -> (String, String) {
    generate_keys()
}

#[pyfunction]
fn register_server_py(client_request: String, public_key: String) -> OpaquePyResult<(String, String)> {
    Ok(register_server(client_request, public_key)?)
}

#[pyfunction]
fn register_server_finish_py(client_request_finish: String, registration_state: String) -> OpaquePyResult<String> {
    Ok(register_server_finish(client_request_finish, registration_state)?)
}

#[pyfunction]
fn register_client_py(password: String) -> OpaquePyResult<(String, String)> {
    Ok(client_register(&password)?)
}

#[pyfunction]
fn register_client_finish_py(client_register_state: String, server_message: String) -> OpaquePyResult<String> {
    Ok(client_register_finish(&client_register_state, &server_message)?)
}

#[pyfunction]
fn login_server_py(password_file: String, client_request: String, private_key: String) -> OpaquePyResult<(String, String)> {
    Ok(login_server(password_file, client_request, private_key)?)
}

#[pyfunction]
fn login_server_finish_py(client_request_finish: String, login_state: String) -> OpaquePyResult<String> {
    Ok(login_server_finish(client_request_finish, login_state)?)
}