use crate::{exports::Exports, module::Module, wasmer_inner::wasmer};
use pyo3::{exceptions::RuntimeError, prelude::*};

#[pyclass(unsendable)]
#[text_signature = "(module)"]
pub struct Instance {
    inner: wasmer::Instance,
    exports: Exports,
}

impl Instance {
    pub fn raw_new(module: &Module) -> Result<Self, wasmer::InstantiationError> {
        let instance = wasmer::Instance::new(module.inner(), &wasmer::imports! {})?;
        let exports = Exports::new(instance.exports.clone());

        Ok(Instance {
            inner: instance,
            exports,
        })
    }

    pub fn store(&self) -> &wasmer::Store {
        self.inner.module().store()
    }
}

#[pymethods]
impl Instance {
    #[new]
    fn new(module: &Module) -> PyResult<Self> {
        Instance::raw_new(&module).map_err(|error| RuntimeError::py_err(error.to_string()))
    }

    #[getter]
    fn exports(&self) -> Exports {
        self.exports.clone()
    }
}
