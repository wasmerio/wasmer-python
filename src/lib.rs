use pyo3::{
    exceptions::RuntimeError,
    prelude::*,
    types::{PyBytes, PyTuple},
    wrap_pyfunction,
};

pub(crate) mod wasmer_inner {
    pub use wasmer;
}

mod r#extern;
mod module;
mod store;

/// This extension allows to manipulate and to execute WebAssembly binaries.
#[pymodule]
fn wasmer(py: Python, module: &PyModule) -> PyResult<()> {
    let enum_module = py.import("enum")?;

    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add("__core_version__", env!("WASMER_VERSION"))?;
    module.add_wrapped(wrap_pyfunction!(wat2wasm))?;
    module.add_wrapped(wrap_pyfunction!(wasm2wat))?;
    module.add_class::<module::Module>()?;
    module.add_class::<store::Store>()?;
    module.add(
        "ExternType",
        enum_module.call1(
            "IntEnum",
            PyTuple::new(
                py,
                &[
                    "ExternType",
                    r#extern::ExternType::iter()
                        .map(Into::into)
                        .collect::<Vec<&'static str>>()
                        .join(" ")
                        .as_str(),
                ],
            ),
        )?,
    )?;

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
