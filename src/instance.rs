use crate::{errors::to_py_err, exports::Exports, module::Module, wasmer_inner::wasmer};
use pyo3::{exceptions::RuntimeError, prelude::*};

#[pyclass(unsendable)]
#[text_signature = "(module)"]
pub struct Instance {
    inner: wasmer::Instance,
    exports: Py<Exports>,
}

pub enum InstanceError {
    InstantiationError(wasmer::InstantiationError),
    PyErr(PyErr),
}

impl Instance {
    pub fn raw_new(py: Python, module: &Module) -> Result<Self, InstanceError> {
        let instance = wasmer::Instance::new(module.inner(), &wasmer::imports! {})
            .map_err(InstanceError::InstantiationError)?;
        let exports =
            Py::new(py, Exports::new(instance.exports.clone())).map_err(InstanceError::PyErr)?;

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
    fn new(py: Python, module: &Module) -> PyResult<Self> {
        Instance::raw_new(py, &module).map_err(|error| match error {
            InstanceError::InstantiationError(error) => to_py_err::<RuntimeError, _>(error),
            InstanceError::PyErr(error) => error,
        })
    }

    #[getter]
    fn exports(&self, py: Python) -> Py<Exports> {
        self.exports.clone_ref(py)
    }
}
