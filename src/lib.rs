use pyo3::{
    prelude::*,
    types::{PyBytes, PyTuple},
    wrap_pymodule,
};

pub(crate) mod wasmer_inner {
    pub use wasmer;
    pub use wasmer_types;
    pub use wasmer_wasi;
}

mod errors;
mod exports;
mod externals;
mod import_object;
mod instance;
mod memory;
mod module;
mod store;
mod types;
mod values;
mod wasi;
mod wat;

/// <p align="center">
///   <a href="https://wasmer.io" target="_blank" rel="noopener">
///     <img width="300" src="https://raw.githubusercontent.com/wasmerio/wasmer/master/assets/logo.png" alt="Wasmer logo">
///   </a>
/// </p>
///
/// <p align="center">
///   <a href="https://spectrum.chat/wasmer">
///     <img src="https://withspectrum.github.io/badge/badge.svg" alt="Join the Wasmer Community" valign="middle"></a>
///   <a href="https://pypi.org/project/wasmer/">
///       <img src="https://img.shields.io/pypi/format/wasmer.svg" alt="Pypi" valign="middle"/></a>
///   <a href="https://pypi.org/project/wasmer/">
///       <img src="https://pepy.tech/badge/wasmer" alt="Number of downloads on Pypi" valign="middle"/></a>
///   <a href="https://github.com/wasmerio/wasmer/blob/master/LICENSE">
///     <img src="https://img.shields.io/github/license/wasmerio/wasmer.svg" alt="License" valign="middle"></a>
/// </p>
/// Wasmer is an advanced and mature WebAssembly runtime. The `wasmer`
/// Python package is a native Python extension to embed Wasmer inside
/// Python.
///
/// ## Example
///
/// The very basic example is the following:
///
/// ```py
/// from wasmer import Store, Module, Instance
///
/// # Create a store, which holds the engine, the compiler etc.
/// store = Store()
///
/// # Let's assume we don't have WebAssembly bytes at hand. We will
/// # write WebAssembly manually.
/// module = Module(
///     store,
///     """
///     (module
///       (type (func (param i32 i32) (result i32)))
///       (func (type 0)
///         local.get 0
///         local.get 1
///         i32.add)
///       (export "sum" (func 0)))
///     """
/// )
///
/// # Instantiates the module.
/// instance = Instance(module)
///
/// # Now, let's execute the `sum` function.
/// assert instance.exports.sum(1, 2) == 3
/// ```
///
/// That's it. Now explore the API! Some pointers for the adventurers:
///
/// * The basic elements are `Module` and `Instance`,
/// * Exports of an instance are represented by the `Exports` object,
/// * Maybe your module needs to import `Function`, `Memory`, `Global`
///   or `Table`? Well, there is the `ImportObject` for that!
/// * It is possible to read and write `Memory` data with the Python
///   buffer protocol with `Buffer`.
///
/// Have fun!
#[pymodule]
fn wasmer(py: Python, module: &PyModule) -> PyResult<()> {
    let enum_module = py.import("enum")?;

    // Constants.
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add("__core_version__", env!("WASMER_VERSION"))?;

    // Functions.

    /// Translate WebAssembly text source to WebAssembly binary format.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import wat2wasm
    ///
    /// assert wat2wasm('(module)') == b'\x00asm\x01\x00\x00\x00'
    /// ```
    #[pyfn(module, "wat2wasm")]
    #[text_signature = "(wat)"]
    fn wat2wasm<'py>(py: Python<'py>, wat: String) -> PyResult<&'py PyBytes> {
        wat::wat2wasm(py, wat)
    }

    /// Disassemble WebAssembly binary to WebAssembly text format.
    ///
    /// ## Example
    ///
    /// ```py
    /// assert wasm2wat(b'\x00asm\x01\x00\x00\x00') == '(module)'
    /// ```
    #[pyfn(module, "wasm2wat")]
    #[text_signature = "(bytes)"]
    fn wasm2wat(bytes: &PyBytes) -> PyResult<String> {
        wat::wasm2wat(bytes)
    }

    // Classes.
    module.add_class::<exports::Exports>()?;
    module.add_class::<externals::Function>()?;
    module.add_class::<externals::Global>()?;
    module.add_class::<externals::Memory>()?;
    module.add_class::<externals::Table>()?;
    module.add_class::<import_object::ImportObject>()?;
    module.add_class::<instance::Instance>()?;
    module.add_class::<memory::Buffer>()?;
    module.add_class::<memory::Int16Array>()?;
    module.add_class::<memory::Int32Array>()?;
    module.add_class::<memory::Int8Array>()?;
    module.add_class::<memory::Uint16Array>()?;
    module.add_class::<memory::Uint32Array>()?;
    module.add_class::<memory::Uint8Array>()?;
    module.add_class::<module::Module>()?;
    module.add_class::<store::Store>()?;
    module.add_class::<types::ExportType>()?;
    module.add_class::<types::FunctionType>()?;
    module.add_class::<types::GlobalType>()?;
    module.add_class::<types::ImportType>()?;
    module.add_class::<types::MemoryType>()?;
    module.add_class::<types::TableType>()?;
    module.add_class::<values::Value>()?;

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

    // Modules.
    module.add_wrapped(wrap_pymodule!(wasi))?;

    Ok(())
}

/// This `wasi` module provides WASI supports to `wasmer`.
#[pymodule]
fn wasi(py: Python, module: &PyModule) -> PyResult<()> {
    let enum_module = py.import("enum")?;

    // Functions.

    /// Try to find the WASI version of the given module.
    #[pyfn(module, "get_version")]
    #[text_signature = "(module, strict)"]
    fn get_version(module: &module::Module, strict: bool) -> Option<wasi::Version> {
        wasi::get_version(module, strict)
    }

    // Classes.
    module.add_class::<wasi::Environment>()?;
    module.add_class::<wasi::StateBuilder>()?;

    // Enums.
    module.add(
        "Version",
        enum_module.call1(
            "IntEnum",
            PyTuple::new(
                py,
                &[
                    "Version",
                    wasi::Version::iter()
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
