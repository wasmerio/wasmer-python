use crate::{
    errors::to_py_err, exports::Exports, import_object::ImportObject, module::Module,
    wasmer_inner::wasmer,
};
use pyo3::{exceptions::RuntimeError, prelude::*};

#[pyclass(unsendable)]
#[text_signature = "(module, import_object)"]
pub struct Instance {
    inner: wasmer::Instance,
    #[pyo3(get)]
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
        let module = module.inner();

        let instance = match import_object {
            Some(import_object) => wasmer::Instance::new(&module, import_object.inner()),
            None => wasmer::Instance::new(&module, &wasmer::imports! {}),
        };
        let instance = instance.map_err(InstanceError::InstantiationError)?;

        let exports =
            Py::new(py, Exports::new(instance.exports.clone())).map_err(InstanceError::PyErr)?;

        Ok(Instance {
            inner: instance,
            exports,
        })
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
}
