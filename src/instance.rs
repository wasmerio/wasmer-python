use crate::{module::Module, wasmer_inner::wasmer};
use pyo3::prelude::*;

#[pyclass(unsendable)]
#[text_signature = "(module)"]
pub struct Instance {
    inner: wasmer::Instance,
}

impl Instance {
    pub fn new(module: &Module) -> Result<Self, wasmer::InstantiationError> {
        Ok(Instance {
            inner: wasmer::Instance::new(module.inner(), &wasmer::imports! {})?,
        })
    }

    pub fn store(&self) -> &wasmer::Store {
        self.inner.module().store()
    }
}

#[pymethods]
impl Instance {}
