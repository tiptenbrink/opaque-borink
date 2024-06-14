use opaque_borink::client::{
    client_login, client_login_finish, client_register, client_register_finish,
};
use opaque_borink::server::{
    login_server, login_server_finish, register_server, register_server_finish,
};
use opaque_borink::{create_setup, Error};
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
    let internal = PyModule::new_bound(m.py(), "_internal")?;
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
    create_setup()
}

#[pyfunction]
fn register_server_py(
    setup: String,
    client_request: String,
    credential_id: String,
) -> OpaquePyResult<String> {
    Ok(register_server(&setup, &client_request, &credential_id)?)
}

#[pyfunction]
fn register_server_finish_py(client_request_finish: String) -> OpaquePyResult<String> {
    Ok(register_server_finish(&client_request_finish)?)
}

#[pyfunction]
fn register_client_py(password: String) -> OpaquePyResult<(String, String)> {
    Ok(client_register(&password)?)
}

#[pyfunction]
fn register_client_finish_py(
    client_register_state: String,
    password: String,
    server_message: String,
) -> OpaquePyResult<String> {
    Ok(client_register_finish(
        &client_register_state,
        &password,
        &server_message,
    )?)
}

#[pyfunction]
fn login_server_py(
    setup: String,
    password_file: String,
    client_request: String,
    credential_id: String,
) -> OpaquePyResult<(String, String)> {
    Ok(login_server(
        &setup,
        &password_file,
        &client_request,
        &credential_id,
    )?)
}

#[pyfunction]
fn login_server_finish_py(
    client_request_finish: String,
    login_state: String,
) -> OpaquePyResult<String> {
    Ok(login_server_finish(&client_request_finish, &login_state)?)
}

#[pyfunction]
fn login_client_py(password: String) -> OpaquePyResult<(String, String)> {
    Ok(client_login(&password)?)
}

#[pyfunction]
fn login_client_finish_py(
    client_login_state: String,
    password: String,
    server_message: String,
) -> OpaquePyResult<(String, String)> {
    Ok(client_login_finish(
        &client_login_state,
        &password,
        &server_message,
    )?)
}
