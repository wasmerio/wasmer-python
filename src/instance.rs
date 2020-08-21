use crate::{
    errors::to_py_err, exports::Exports, import_object::ImportObject, module::Module,
    wasmer_inner::wasmer,
};
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
    pub fn raw_new(
        py: Python,
        module: &Module,
        import_object: Option<&ImportObject>,
    ) -> Result<Self, InstanceError> {
        let instance = match import_object {
            Some(import_object) => wasmer::Instance::new(module.inner(), import_object.inner()),
            None => wasmer::Instance::new(module.inner(), &wasmer::imports! {}),
        };
        let instance = instance.map_err(InstanceError::InstantiationError)?;

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
    fn new(py: Python, module: &Module, import_object: Option<&ImportObject>) -> PyResult<Self> {
        Instance::raw_new(py, &module, import_object).map_err(|error| match error {
            InstanceError::InstantiationError(error) => to_py_err::<RuntimeError, _>(error),
            InstanceError::PyErr(error) => error,
        })
    }

    #[getter]
    fn exports(&self, py: Python) -> Py<Exports> {
        self.exports.clone_ref(py)
    }
}
