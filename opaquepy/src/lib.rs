use opaquebind::{Error, create_setup};
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
            Error::DecodeError(oe) => PyValueError::new_err(oe.to_string()),
        }
    }
}

#[pymodule]
fn _opaquepy(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_setup_py, m)?)?;
    m.add_function(wrap_pyfunction!(register_server_py, m)?)?;
    m.add_function(wrap_pyfunction!(register_server_finish_py, m)?)?;
    m.add_function(wrap_pyfunction!(register_client_py, m)?)?;
    m.add_function(wrap_pyfunction!(register_client_finish_py, m)?)?;
    m.add_function(wrap_pyfunction!(login_server_py, m)?)?;
    m.add_function(wrap_pyfunction!(login_server_finish_py, m)?)?;

    Ok(())
}

#[pyfunction]
fn create_setup_py() -> String {
    create_setup()
}

#[pyfunction]
fn register_server_py(setup: String, client_request: String, credential_id: String) -> OpaquePyResult<String> {
    Ok(register_server(setup, client_request, credential_id)?)
}

#[pyfunction]
fn register_server_finish_py(client_request_finish: String) -> OpaquePyResult<String> {
    Ok(register_server_finish(client_request_finish)?)
}

#[pyfunction]
fn register_client_py(password: String) -> OpaquePyResult<(String, String)> {
    Ok(client_register(&password)?)
}

#[pyfunction]
fn register_client_finish_py(client_register_state: String, password: String, server_message: String) -> OpaquePyResult<String> {
    Ok(client_register_finish(&client_register_state, &password, &server_message)?)
}

#[pyfunction]
fn login_server_py(setup: String, password_file: String, client_request: String, credential_id: String) -> OpaquePyResult<(String, String)> {
    Ok(login_server(setup, password_file, client_request, credential_id)?)
}

#[pyfunction]
fn login_server_finish_py(client_request_finish: String, login_state: String) -> OpaquePyResult<String> {
    Ok(login_server_finish(client_request_finish, login_state)?)
}