#![deny(warnings)]

mod import;
mod instance;
mod memory;
mod module;
mod r#type;
mod value;
mod wasi;

use import::ImportObject;
use instance::{exports::ExportImportKind, Instance};
use module::Module;
use pyo3::{prelude::*, types::PyTuple};
use r#type::Type;
use value::Value;

/// This extension allows to manipulate and to execute WebAssembly binaries.
#[pymodule]
fn wasmer(py: Python, module: &PyModule) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add("__core_version__", env!("WASMER_RUNTIME_CORE_VERSION"))?;
    module.add_class::<Instance>()?;
    module.add_class::<Module>()?;
    module.add_class::<Value>()?;
    module.add_class::<ImportObject>()?;
    module.add_class::<memory::Memory>()?;
    module.add_class::<memory::view::Int16Array>()?;
    module.add_class::<memory::view::Int32Array>()?;
    module.add_class::<memory::view::Int8Array>()?;
    module.add_class::<memory::view::Uint16Array>()?;
    module.add_class::<memory::view::Uint32Array>()?;
    module.add_class::<memory::view::Uint8Array>()?;
    module.add_class::<memory::buffer::Buffer>()?;
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
