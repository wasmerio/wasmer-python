use pyo3::{
    exceptions::RuntimeError,
    prelude::*,
    types::{PyBytes, PyTuple},
    wrap_pyfunction,
};

pub(crate) mod wasmer_inner {
    pub use wasmer;
    pub use wasmer_types;
}

mod errors;
mod exports;
mod externals;
mod instance;
mod module;
mod store;
mod types;
mod values;

use errors::to_py_err;

/// This extension allows to compile and to execute WebAssembly.
#[pymodule]
fn wasmer(py: Python, module: &PyModule) -> PyResult<()> {
    let enum_module = py.import("enum")?;

    // Constants.
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add("__core_version__", env!("WASMER_VERSION"))?;

    // Functions.
    module.add_wrapped(wrap_pyfunction!(wat2wasm))?;
    module.add_wrapped(wrap_pyfunction!(wasm2wat))?;

    // Classes.
    module.add_class::<exports::Exports>()?;
    module.add_class::<externals::Function>()?;
    module.add_class::<externals::Global>()?;
    module.add_class::<instance::Instance>()?;
    module.add_class::<module::Module>()?;
    module.add_class::<store::Store>()?;
    module.add_class::<types::ExportType>()?;
    module.add_class::<types::FunctionType>()?;
    module.add_class::<types::GlobalType>()?;
    module.add_class::<types::ImportType>()?;
    module.add_class::<types::MemoryType>()?;
    module.add_class::<types::TableType>()?;

    // Enums.
    module.add(
        "Type",
        enum_module.call1(
            "IntEnum",
            PyTuple::new(
                py,
                &[
                    "Type",
                    types::Type::iter()
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
        .map_err(to_py_err::<RuntimeError, _>)
}

/// Disassemble WebAssembly binary to WebAssembly text format.
#[pyfunction]
#[text_signature = "(bytes)"]
pub fn wasm2wat(bytes: &PyBytes) -> PyResult<String> {
    wasmprinter::print_bytes(bytes.as_bytes()).map_err(to_py_err::<RuntimeError, _>)
}
