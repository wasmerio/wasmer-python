use crate::store::Store;
use crate::wasmer_inner::wasmer;
use pyo3::{
    prelude::*,
    types::{PyAny, PyBytes},
};

#[pyclass(unsendable)]
#[text_signature = "(bytes)"]
pub struct Module {
    inner: wasmer::Module,
}

#[pymethods]
impl Module {
    #[text_signature = "(bytes)"]
    #[staticmethod]
    fn validate(store: &Store, bytes: &PyAny) -> PyResult<bool> {
        match <PyBytes as PyTryFrom>::try_from(bytes) {
            Ok(bytes) => Ok(wasmer::Module::validate(store.inner(), bytes.as_bytes()).is_ok()),
            _ => Ok(false),
        }
    }
}
