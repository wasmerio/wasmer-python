#![deny(warnings)]

use pyo3::{
    prelude::*,
    types::{PyAny, PyBytes},
    wrap_pyfunction, PyTryFrom,
};
use wasmer_runtime::validate as wasm_validate;

mod instance;
mod memory_view;
mod value;

use instance::Instance;
use value::Value;

/// This extension allows to manipulate and to execute WebAssembly binaries.
#[pymodule]
fn wasmer(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(validate))?;
    module.add_class::<Instance>()?;
    module.add_class::<memory_view::Uint8MemoryView>()?;
    module.add_class::<memory_view::Int8MemoryView>()?;
    module.add_class::<memory_view::Uint16MemoryView>()?;
    module.add_class::<memory_view::Int16MemoryView>()?;
    module.add_class::<memory_view::Uint32MemoryView>()?;
    module.add_class::<memory_view::Int32MemoryView>()?;
    module.add_class::<Value>()?;

    Ok(())
}

/// validate(bytes, /)
/// --
///
/// Check a WebAssembly module is valid.
#[pyfunction]
pub fn validate(bytes: &PyAny) -> PyResult<bool> {
    match <PyBytes as PyTryFrom>::try_from(bytes) {
        Ok(bytes) => Ok(wasm_validate(bytes.as_bytes())),
        _ => Ok(false),
    }
}
