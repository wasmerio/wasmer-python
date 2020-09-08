use pyo3::{prelude::*, type_object::PyTypeObject};
use std::string::ToString;

pub fn to_py_err<PyError, Error>(error: Error) -> PyErr
where
    PyError: PyTypeObject,
    Error: ToString,
{
    PyErr::new::<PyError, _>(error.to_string())
}
