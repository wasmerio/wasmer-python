use crate::errors::to_py_err;
use pyo3::{exceptions::RuntimeError, prelude::*, types::PyBytes};

pub fn wat2wasm<'py>(py: Python<'py>, wat: String) -> PyResult<&'py PyBytes> {
    wat::parse_str(wat)
        .map(|bytes| PyBytes::new(py, bytes.as_slice()))
        .map_err(to_py_err::<RuntimeError, _>)
}

pub fn wasm2wat(bytes: &PyBytes) -> PyResult<String> {
    wasmprinter::print_bytes(bytes.as_bytes()).map_err(to_py_err::<RuntimeError, _>)
}
