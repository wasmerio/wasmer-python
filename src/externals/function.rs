use crate::wasmer_inner::wasmer;
use pyo3::{prelude::*, types::PyTuple};

#[pyclass(unsendable)]
pub struct Function {
    inner: wasmer::Function,
}

impl Function {
    pub fn new(inner: wasmer::Function) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Function {
    #[call]
    #[args(arguments = "*")]
    fn __call__(&self, py: Python, arguments: &PyTuple) -> PyResult<PyObject> {
        unimplemented!()
    }
}
