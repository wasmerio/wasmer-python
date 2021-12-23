use pyo3::{exceptions::PyRuntimeError, prelude::*, type_object::PyTypeObject};
use std::string::ToString;
use wasmer::RuntimeError;

pub fn to_py_err<PyError, Error>(error: Error) -> PyErr
where
    PyError: PyTypeObject,
    Error: ToString,
{
    PyErr::new::<PyError, _>(error.to_string())
}

pub fn runtime_error_to_py_err(error: RuntimeError) -> PyErr {
    match error.downcast::<PyErr>() {
        Ok(err) => err,
        Err(err) => to_py_err::<PyRuntimeError, _>(err),
    }
}
