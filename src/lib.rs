use pyo3::{exceptions::RuntimeError, prelude::*, types::PyBytes, wrap_pyfunction};

/// This extension allows to manipulate and to execute WebAssembly binaries.
#[pymodule]
fn wasmer(py: Python, module: &PyModule) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add("__core_version__", env!("WASMER_RUNTIME_CORE_VERSION"))?;
    module.add_wrapped(wrap_pyfunction!(wat2wasm))?;
    module.add_wrapped(wrap_pyfunction!(wasm2wat))?;

    Ok(())
}

/// Translate WebAssembly text source to WebAssembly binary format.
#[pyfunction]
#[text_signature = "(wat)"]
pub fn wat2wasm<'py>(py: Python<'py>, wat: String) -> PyResult<&'py PyBytes> {
    wat::parse_str(wat)
        .map(|bytes| PyBytes::new(py, bytes.as_slice()))
        .map_err(|error| RuntimeError::py_err(error.to_string()))
}

/// Disassemble WebAssembly binary to WebAssembly text format.
#[pyfunction]
#[text_signature = "(bytes)"]
pub fn wasm2wat(bytes: &PyBytes) -> PyResult<String> {
    wasmprinter::print_bytes(bytes.as_bytes())
        .map_err(|error| RuntimeError::py_err(error.to_string()))
}
