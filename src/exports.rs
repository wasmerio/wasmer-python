use crate::{
    errors::to_py_err,
    externals::{Function, Global, Memory, Table},
    wasmer_inner::wasmer,
};
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

        Ok(match self.inner.get_extern(key.as_str()) {
            Some(wasmer::Extern::Function(function)) => {
                Py::new(py, Function::raw_new(function.clone()))?.to_object(py)
            }
            Some(wasmer::Extern::Global(global)) => {
                Py::new(py, Global::raw_new(global.clone()))?.to_object(py)
            }
            Some(wasmer::Extern::Memory(memory)) => {
                Py::new(py, Memory::raw_new(memory.clone()))?.to_object(py)
            }
            Some(wasmer::Extern::Table(table)) => {
                Py::new(py, Table::raw_new(table.clone()))?.to_object(py)
            }
            _ => {
                return Err(to_py_err::<LookupError, _>(format!(
                    "Export `{}` does not exist.",
                    key
                )))
            }
        })
    }
}

#[pyproto]
impl PySequenceProtocol for Exports {
    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
