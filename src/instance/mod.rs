//! The `wasmer.Instance` Python object to build WebAssembly instances.
//!
//! The `Instance` class has the following declaration:
//!
//! * The constructor reads bytes from its first parameter, and it
//!   expects those bytes to represent a valid WebAssembly module,
//! * The `exports` getter, to get exported functions from the
//!   WebAssembly module, e.g. `instance.exports.sum(1, 2)` to call the
//!   exported function `sum` with arguments `1` and `2`,
//! * The `memory` getter, to get the exported memory (if any) from
//!   the WebAssembly module, .e.g. `instance.memory.uint8_view()`, see
//!   the `wasmer.Memory` class.

pub(crate) mod exports;
pub(crate) mod globals;
pub(crate) mod inspect;

use crate::{
    import::build_import_object, instance::exports::ExportedFunctions,
    instance::globals::ExportedGlobals, memory::Memory,
};
use pyo3::{
    exceptions::RuntimeError,
    prelude::*,
    types::{PyAny, PyBytes, PyDict},
    PyObject, PyTryFrom, Python,
};
use std::{collections::HashMap, rc::Rc};
use wasmer_runtime::{self as runtime, Export};

#[pyclass]
#[text_signature = "(bytes, imported_functions={})"]
/// `Instance` is a Python class that represents a WebAssembly instance.
///
/// # Examples
///
/// ```python
/// from wasmer import Instance
///
/// instance = Instance(wasm_bytes)
/// ```
pub struct Instance {
    pub(crate) instance: Rc<runtime::Instance>,

    /// All WebAssembly exported functions represented by an
    /// `ExportedFunctions` object.
    pub(crate) exports: Py<ExportedFunctions>,

    /// The WebAssembly exported memory represented by a `Memory`
    /// object.
    pub(crate) memory: Option<Py<Memory>>,

    /// All WebAssembly exported globals represented by an
    /// `ExportedGlobals` object.
    pub(crate) globals: Py<ExportedGlobals>,

    /// This field is unused as is, but is required to keep a
    /// reference to host function `PyObject`.
    #[allow(unused)]
    pub(crate) host_function_references: Vec<PyObject>,

    exports_index_to_name: Option<HashMap<usize, String>>,
}

impl Instance {
    pub(crate) fn inner_new(
        instance: Rc<runtime::Instance>,
        exports: Py<ExportedFunctions>,
        memory: Option<Py<Memory>>,
        globals: Py<ExportedGlobals>,
        host_function_references: Vec<PyObject>,
    ) -> Self {
        Self {
            instance,
            exports,
            memory,
            globals,
            exports_index_to_name: None,
            host_function_references,
        }
    }
}

#[pymethods]
/// Implement methods on the `Instance` Python class.
impl Instance {
    /// The constructor instantiates a new WebAssembly instance basde
    /// on WebAssembly bytes (represented by the Python bytes type).
    #[new]
    #[args(imported_functions = "PyDict::new(_py)")]
    fn new(py: Python, bytes: &PyAny, imported_functions: &PyDict) -> PyResult<Self> {
        // Read the bytes.
        let bytes = <PyBytes as PyTryFrom>::try_from(bytes)?.as_bytes();

        // Compile the module.
        let module = runtime::compile(bytes).map_err(|error| {
            RuntimeError::py_err(format!("Failed to compile the module:\n    {}", error))
        })?;

        let (import_object, host_function_references) =
            build_import_object(py, &module, imported_functions)?;

        // Instantiate the WebAssembly module.
        let instance = match module.instantiate(&import_object) {
            Ok(instance) => Rc::new(instance),
            Err(e) => {
                return Err(RuntimeError::py_err(format!(
                    "Failed to instantiate the module:\n    {}",
                    e
                )))
            }
        };

        let exports = instance.exports();

        // Collect the exported functions, globals and memory from the
        // WebAssembly module.
        let mut exported_functions = Vec::new();
        let mut exported_globals = Vec::new();
        let mut exported_memory = None;

        for (export_name, export) in exports {
            match export {
                Export::Function { .. } => exported_functions.push(export_name),
                Export::Global(global) => exported_globals.push((export_name, Rc::new(global))),
                Export::Memory(memory) if exported_memory.is_none() => {
                    exported_memory = Some(Rc::new(memory))
                }
                _ => (),
            }
        }

        Ok(Self::inner_new(
            instance.clone(),
            Py::new(
                py,
                ExportedFunctions {
                    instance: instance.clone(),
                    functions: exported_functions,
                },
            )?,
            match exported_memory {
                Some(memory) => Some(Py::new(py, Memory { memory })?),
                None => None,
            },
            Py::new(
                py,
                ExportedGlobals {
                    globals: exported_globals,
                },
            )?,
            host_function_references,
        ))
    }

    /// The `exports` getter.
    #[getter]
    fn exports(&self) -> &Py<ExportedFunctions> {
        &self.exports
    }

    /// The `memory` getter.
    #[getter]
    fn memory(&self, py: Python) -> PyResult<PyObject> {
        match &self.memory {
            Some(memory) => Ok(memory.into_py(py)),
            None => Ok(py.None()),
        }
    }

    /// The `globals` getter.
    #[getter]
    fn globals(&self) -> &Py<ExportedGlobals> {
        &self.globals
    }

    /// Find the export _name_ associated to an index if it is valid.
    #[text_signature = "($self, index)"]
    fn resolve_exported_function(&mut self, py: Python, index: usize) -> PyResult<String> {
        match &self.exports_index_to_name {
            Some(exports_index_to_name) => {
                exports_index_to_name.get(&index).cloned().ok_or_else(|| {
                    RuntimeError::py_err(format!("Function at index `{}` does not exist.", index))
                })
            }

            None => {
                self.exports_index_to_name = Some(
                    self.instance
                        .exports()
                        .filter(|(_, export)| match export {
                            Export::Function { .. } => true,
                            _ => false,
                        })
                        .map(|(name, _)| (self.instance.resolve_func(&name).unwrap(), name.clone()))
                        .collect(),
                );

                self.resolve_exported_function(py, index)
            }
        }
    }
}
