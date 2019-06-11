//! The `wasmer.Module` Python object to build WebAssembly modules.

use pyo3::{
    prelude::*,
    types::{PyAny, PyBytes},
};
use wasmer_runtime::validate;

#[pyclass]
/// `Module` is a Python class that represents a WebAssembly module.
pub struct Module {}

#[pymethods]
/// Implement methods on the `Module` Python class.
impl Module {
    /// Check that given bytes represent a valid WebAssembly module.
    #[staticmethod]
    fn validate(bytes: &PyAny) -> PyResult<bool> {
        match <PyBytes as PyTryFrom>::try_from(bytes) {
            Ok(bytes) => Ok(validate(bytes.as_bytes())),
            _ => Ok(false),
        }
    }
}
