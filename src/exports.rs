use crate::{errors::to_py_err, externals::Function, wasmer_inner::wasmer};
use pyo3::{
    class::{basic::PyObjectProtocol, sequence::PySequenceProtocol},
    exceptions::LookupError,
    prelude::*,
};

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct Exports {
    inner: wasmer::Exports,
}

impl Exports {
    pub fn new(inner: wasmer::Exports) -> Self {
        Self { inner }
    }
}

#[pyproto]
impl PyObjectProtocol for Exports {
    fn __getattr__(&self, key: String) -> PyResult<PyObject> {
        let gil_guard = Python::acquire_gil();
        let py = gil_guard.python();

        Ok(Py::new(
            py,
            match self.inner.get_extern(key.as_str()) {
                Some(wasmer::Extern::Function(function)) => Function::new(function.clone()),
                _ => {
                    return Err(to_py_err::<LookupError, _>(format!(
                        "Export `{}` does not exist.",
                        key
                    )))
                }
            },
        )?
        .to_object(py))
    }
}

#[pyproto]
impl PySequenceProtocol for Exports {
    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
