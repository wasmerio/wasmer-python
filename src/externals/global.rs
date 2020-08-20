use crate::{
    errors::to_py_err,
    values::{to_py_object, to_wasm_value},
    wasmer_inner::wasmer,
};
use pyo3::{
    exceptions::{RuntimeError, ValueError},
    prelude::*,
};

#[pyclass(unsendable)]
pub struct Global {
    inner: wasmer::Global,
}

impl Global {
    pub fn new(inner: wasmer::Global) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Global {
    #[getter]
    fn mutable(&self) -> bool {
        self.inner.ty().mutability.is_mutable()
    }

    #[getter(value)]
    fn get_value(&self, py: Python) -> PyObject {
        let to_py_object = to_py_object(py);

        to_py_object(&self.inner.get())
    }

    #[setter]
    fn set_value(&self, value: &PyAny) -> PyResult<()> {
        let ty = self.inner.ty();

        if !ty.mutability.is_mutable() {
            return Err(to_py_err::<RuntimeError, _>(format!(
                "The global variable is not mutable, cannot set a new value",
            )));
        }

        self.inner
            .set(to_wasm_value((value, &ty.ty))?)
            .map_err(to_py_err::<ValueError, _>)?;

        Ok(())
    }
}
