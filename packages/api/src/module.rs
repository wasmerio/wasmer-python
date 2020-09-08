use crate::{errors::to_py_err, store::Store, types, wasmer_inner::wasmer};
use pyo3::{
    exceptions::{RuntimeError, TypeError},
    prelude::*,
    types::{PyAny, PyBytes, PyList, PyString},
};
use std::convert::TryInto;

/// A WebAssembly module contains stateless WebAssembly code that has
/// already been compiled and can be instantiated multiple times.
///
/// Creates a new WebAssembly Module given the configuration
/// in the store.
///
/// If the provided bytes are not WebAssembly-like (start with
/// `b"\0asm"`), this function will try to to convert the bytes
/// assuming they correspond to the WebAssembly text format.
///
/// ## Security
///
/// Before the code is compiled, it will be validated using the store
/// features.
///
/// ## Example
///
/// ```py
/// from wasmer import Store, Module
///
/// store = Store()
///
/// # Let's compile WebAssembly from bytes.
/// module = Module(store, open('tests/tests.wasm', 'rb').read())
///
/// # Let's compile WebAssembly from WAT.
/// module = Module(store, '(module)')
/// ```
#[pyclass(unsendable)]
#[text_signature = "(store, bytes)"]
pub struct Module {
    inner: wasmer::Module,
}

impl Module {
    pub(crate) fn inner(&self) -> &wasmer::Module {
        &self.inner
    }
}

#[pymethods]
impl Module {
    /// Validates a new WebAssembly Module given the configuration
    /// in the `Store`.
    ///
    /// This validation is normally pretty fast and checks the enabled
    /// WebAssembly features in the `Store` engine to assure deterministic
    /// validation of the `Module`.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import Store, Module
    ///
    /// assert Module.validate(Store(), wasm_bytes)
    /// ```
    #[text_signature = "(bytes)"]
    #[staticmethod]
    fn validate(store: &Store, bytes: &PyAny) -> bool {
        match bytes.downcast::<PyBytes>() {
            Ok(bytes) => wasmer::Module::validate(store.inner(), bytes.as_bytes()).is_ok(),
            _ => false,
        }
    }

    #[new]
    fn new(store: &Store, bytes: &PyAny) -> PyResult<Self> {
        let store = store.inner();

        // Read the bytes as if there were real bytes or a WAT string.
        let module = if let Ok(bytes) = bytes.downcast::<PyBytes>() {
            wasmer::Module::new(store, bytes.as_bytes())
        } else if let Ok(string) = bytes.downcast::<PyString>() {
            wasmer::Module::new(store, string.to_string()?.as_bytes())
        } else {
            return Err(to_py_err::<TypeError, _>(
                "`Module` accepts Wasm bytes or a WAT string",
            ));
        };

        Ok(Module {
            inner: module.map_err(to_py_err::<RuntimeError, _>)?,
        })
    }

    /// Get or set the current name of the module.
    ///
    /// This name is normally set in the WebAssembly bytecode by some
    /// compilers, but can be also overwritten.
    ///
    /// Not all modules have a name.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import Store, Module
    ///
    /// store = Store()
    ///
    /// # Module with an existing name.
    /// assert Module(store, '(module $moduleName)').name == 'moduleName'
    ///
    /// # Module with no name.
    /// assert Module(store, '(module)').name == None
    ///
    /// # Change the module's name.
    /// module = Module(store, '(module $moduleName)')
    /// module.name = 'hello'
    /// assert module.name == 'hello'
    /// ```
    #[getter]
    fn name(&self) -> Option<&str> {
        self.inner.name()
    }

    #[setter(name)]
    fn set_name(&mut self, name: &str) -> PyResult<()> {
        self.inner.set_name(name);

        Ok(())
    }

    /// Returns a list of `ExportType` objects, which represents all
    /// the exports of this module.
    ///
    /// The order of the exports is guaranteed to be the same as in
    /// the WebAssembly bytecode.
    ///
    /// ## Example
    ///
    /// See the `ExportType` class to learn more.
    #[getter]
    fn exports(&self) -> PyResult<Vec<types::ExportType>> {
        self.inner.exports().map(TryInto::try_into).collect()
    }

    /// Returns a list of `ImportType` objects, which represents all
    /// the imports of this module.
    ///
    /// The order of the imports is guaranteed to be the same as in
    /// the WebAssembly bytecode.
    ///
    /// ## Example
    ///
    /// See the `ImportType` class to learn more.
    #[getter]
    fn imports(&self) -> PyResult<Vec<types::ImportType>> {
        self.inner.imports().map(TryInto::try_into).collect()
    }

    /// Get the custom sections of the module given a `name`.
    ///
    /// ## Important
    ///
    /// Following the WebAssembly specification, one name can have
    /// multiple custom sections. That's why a list of bytes is
    /// returned rather than bytes.
    ///
    /// Consequently, the empty list represents the absence of a
    /// custom section for the given name.
    ///
    /// ## Examples
    ///
    /// ```py
    /// from wasmer import Store, Module
    ///
    /// module = Module(Store(), open('tests/custom_sections.wasm', 'rb').read())
    ///
    /// assert module.custom_sections('easter_egg') == [b'Wasmer']
    /// assert module.custom_sections('hello') == [b'World!']
    /// assert module.custom_sections('foo') == []
    /// ```
    #[text_signature = "($self, name)"]
    fn custom_sections<'p>(&self, py: Python<'p>, name: &str) -> &'p PyList {
        PyList::new(
            py,
            self.inner
                .custom_sections(name)
                .map(|custom_section| PyBytes::new(py, &*custom_section))
                .collect::<Vec<_>>(),
        )
    }

    /// Serializes a module into a binary representation that the
    /// `Engine` can later process via `Module.deserialize`.
    ///
    /// ## Examples
    ///
    /// ```py
    /// from wasmer import Store, Module
    ///
    /// store = Store()
    /// module = Module(Store(), '(module)')
    /// serialized_module = module.serialize()
    ///
    /// assert type(serialized_module) == bytes
    /// ```
    #[text_signature = "($self)"]
    fn serialize<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(
            py,
            self.inner
                .serialize()
                .map_err(to_py_err::<RuntimeError, _>)?
                .as_slice(),
        ))
    }

    /// Deserializes a serialized module binary into a `Module`.
    ///
    /// **Note**: the module has to be serialized before with the
    /// `serialize` method.
    ///
    /// ## Safety
    ///
    /// This function is inherently **unsafe** as the provided bytes:
    ///
    /// 1. Are going to be deserialized directly into Rust objects.
    /// 2. Contains the function assembly bodies and, if intercepted,
    ///    a malicious actor could inject code into executable
    ///    memory.
    ///
    /// And as such, the `deserialize` method is unsafe.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import Store, Module
    ///
    /// store = Store()
    /// module = Module(
    ///     store,
    ///     """
    ///     (module
    ///       (func (export "function") (param i32 i64)))
    ///     """
    /// )
    /// serialized_module = module.serialize()
    ///
    /// del module
    ///
    /// module = Module.deserialize(store, serialized_module)
    ///
    /// del serialized_module
    ///
    /// assert isinstance(module, Module)
    /// ```
    #[text_signature = "($self, bytes)"]
    #[staticmethod]
    fn deserialize(store: &Store, bytes: &PyBytes) -> PyResult<Self> {
        let module = unsafe { wasmer::Module::deserialize(store.inner(), bytes.as_bytes()) }
            .map_err(to_py_err::<RuntimeError, _>)?;

        Ok(Module { inner: module })
    }
}
