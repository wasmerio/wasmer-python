use pyo3::{
    prelude::*,
    types::{PyBytes, PyTuple},
    wrap_pymodule,
};

pub(crate) mod wasmer_inner {
    pub use wasmer_common_py::{self, wasmer};
    pub use wasmer_types;
    pub use wasmer_wasi;
}

mod errors;
mod exports;
mod externals;
mod features;
mod import_object;
mod instance;
mod memory;
mod module;
mod store;
mod types;
mod values;
mod wasi;
mod wat;

pub use store::Store;

/// # <img height="48" src="https://wasmer.io/static/icons/favicon-96x96.png" alt="Wasmer logo" valign="middle"> Wasmer Python [![PyPI version](https://badge.fury.io/py/wasmer.svg?)](https://badge.fury.io/py/wasmer) [![Wasmer Python Documentation](https://img.shields.io/badge/docs-read-green)](https://wasmerio.github.io/wasmer-python/api/) [![Wasmer PyPI downloads](https://pepy.tech/badge/wasmer)](https://pypi.org/project/wasmer/) [![Wasmer Slack Channel](https://img.shields.io/static/v1?label=chat&message=on%20Slack&color=green)](https://slack.wasmer.io)
///
/// A complete and mature WebAssembly runtime for Python based on [Wasmer](https://github.com/wasmerio/wasmer).
///
/// Features:
///
///   * **Easy to use**: The `wasmer` API mimics the standard WebAssembly API,
///   * **Fast**: `wasmer` executes the WebAssembly modules as fast as
///     possible, close to **native speed**,
///   * **Safe**: All calls to WebAssembly will be fast, but more
///     importantly, completely safe and sandboxed.
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
    module.add_class::<features::Features>()?;
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

/// Wasmer's [WASI](https://github.com/WebAssembly/WASI)
/// implementation.
///
/// From the user perspective, WASI is a bunch of imports. To generate
/// the appropriated imports, you can use `StateBuilder` to build an
/// `Environment`. This environment holds the WASI memory, and can be
/// used to generate a valid `wasmer.ImportObject`. This last one can
/// be passed to `wasmer.Instance` to instantiate a `wasmer.Module`
/// that needs WASI support.
///
/// ## Example
///
/// ```py
/// from wasmer import wasi, Store, Module, Instance
///
/// store = Store()
/// module = Module(store, open('tests/wasi.wasm', 'rb').read())
///
/// # Get the WASI version.
/// wasi_version = wasi.get_version(module, strict=True)
///
/// # Build a WASI environment for the imports.
/// wasi_env = wasi.StateBuilder('test-program').argument('--foo').finalize()
///
/// # Generate an `ImportObject` from the WASI environment.
/// import_object = wasi_env.generate_import_object(store, wasi_version)
///
/// # Now we are ready to instantiate the module.
/// instance = Instance(module, import_object)
///
/// # … But (!) WASI needs an access to the memory of the
/// # module. Simple, pass it.
/// wasi_env.memory = instance.exports.memory
///
/// # Here we go, let's start the program.
/// instance.exports._start()
/// ```
#[pymodule]
fn wasi(py: Python, module: &PyModule) -> PyResult<()> {
    let enum_module = py.import("enum")?;

    // Functions.

    /// Detect the version of WASI being used based on the import
    /// namespaces.
    ///
    /// A strict detection expects that all imports live in a single WASI
    /// namespace. A non-strict detection expects that at least one WASI
    /// namespace exits to detect the version. Note that the strict
    /// detection is faster than the non-strict one.
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
