#![deny(warnings)]

mod features;
mod import;
mod instance;
mod memory;
mod module;
mod r#type;
mod value;
mod wasi;

use crate::{
    features::Features,
    import::ImportObject,
    instance::{exports::ExportImportKind, Instance},
    module::Module,
    r#type::Type,
    value::Value,
};
use pyo3::{
    exceptions::RuntimeError,
    prelude::*,
    types::{PyBytes, PyTuple},
    wrap_pyfunction,
};

/// This extension allows to manipulate and to execute WebAssembly binaries.
#[pymodule]
fn wasmer(py: Python, module: &PyModule) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add("__core_version__", env!("WASMER_RUNTIME_CORE_VERSION"))?;
    module.add_wrapped(wrap_pyfunction!(wat2wasm))?;
    module.add_wrapped(wrap_pyfunction!(wasm2wat))?;
    module.add_class::<Features>()?;
    module.add_class::<ImportObject>()?;
    module.add_class::<Instance>()?;
    module.add_class::<Module>()?;
    module.add_class::<Value>()?;
    module.add_class::<memory::Memory>()?;
    module.add_class::<memory::buffer::Buffer>()?;
    module.add_class::<memory::view::Int16Array>()?;
    module.add_class::<memory::view::Int32Array>()?;
    module.add_class::<memory::view::Int8Array>()?;
    module.add_class::<memory::view::Uint16Array>()?;
    module.add_class::<memory::view::Uint32Array>()?;
    module.add_class::<memory::view::Uint8Array>()?;
    module.add_class::<wasi::Wasi>()?;

    {
        let enum_module = py.import("enum")?;

        {
            let mut variants = String::new();

            for ty in Type::iter() {
                variants.push_str(ty.into());
                variants.push(' ');
            }

            module.add(
                "Type",
                enum_module.call1("IntEnum", PyTuple::new(py, &["Type", variants.as_str()]))?,
            )?;
        }

        {
            let mut variants = String::new();

            for kind in ExportImportKind::iter() {
                variants.push_str(kind.into());
                variants.push(' ');
            }

            module.add(
                "ExportKind",
                enum_module.call1(
                    "IntEnum",
                    PyTuple::new(py, &["ExportKind", variants.as_str()]),
                )?,
            )?;
            module.add(
                "ImportKind",
                enum_module.call1(
                    "IntEnum",
                    PyTuple::new(py, &["ImportKind", variants.as_str()]),
                )?,
            )?;
        }

        {
            let mut variants = String::new();

            for kind in wasi::Version::iter() {
                variants.push_str(kind.into());
                variants.push(' ');
            }

            module.add(
                "WasiVersion",
                enum_module.call1(
                    "IntEnum",
                    PyTuple::new(py, &["WasiVersion", variants.as_str()]),
                )?,
            )?;
        }
    }

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
