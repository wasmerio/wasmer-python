use crate::{
    errors::to_py_err,
    types::FunctionType,
    values::{to_py_object, to_wasm_value},
    wasmer_inner::wasmer,
};
use pyo3::{exceptions::RuntimeError, prelude::*, types::PyTuple};

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
    fn __call__<'p>(&self, py: Python<'p>, arguments: &PyTuple) -> PyResult<PyObject> {
        let arguments: Vec<wasmer::Value> = arguments
            .iter()
            .zip(self.inner.ty().params().iter())
            .map(to_wasm_value)
            .collect::<PyResult<_>>()?;

        let results = self
            .inner
            .call(&arguments)
            .map(<[_]>::into_vec)
            .map_err(to_py_err::<RuntimeError, _>)?;

        let to_py_object = to_py_object(py);

        Ok(match results.len() {
            0 => py.None(),
            1 => to_py_object(&results[0]),
            _ => PyTuple::new(
                py,
                results.iter().map(to_py_object).collect::<Vec<PyObject>>(),
            )
            .to_object(py),
        })
    }

    #[getter(type)]
    fn ty(&self, py: Python) -> PyResult<Py<FunctionType>> {
        Py::new(py, FunctionType::from(self.inner.ty()))
    }
}
