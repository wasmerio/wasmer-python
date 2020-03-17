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
    instance::exports::{call_dyn_func, ExportedFunctions},
    instance::globals::ExportedGlobals,
    memory::Memory,
};
use pyo3::{
    exceptions::RuntimeError,
    prelude::*,
    types::{PyAny, PyBytes, PyTuple},
    PyNativeType, PyTryFrom, Python,
};
use std::rc::Rc;
use wasmer_runtime::{self as runtime, imports, instantiate, Export};

#[pyclass]
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
}

#[pymethods]
/// Implement methods on the `Instance` Python class.
impl Instance {
    /// The constructor instantiates a new WebAssembly instance basde
    /// on WebAssembly bytes (represented by the Python bytes type).
    #[new]
    #[allow(clippy::new_ret_no_self)]
    fn new(object: &PyRawObject, bytes: &PyAny) -> PyResult<()> {
        // Read the bytes.
        let bytes = <PyBytes as PyTryFrom>::try_from(bytes)?.as_bytes();

        // Instantiate the WebAssembly module.
        let imports = imports! {};
        let instance = match instantiate(bytes, &imports) {
            Ok(instance) => Rc::new(instance),
            Err(e) => {
                return Err(RuntimeError::py_err(format!(
                    "Failed to instantiate the module:\n    {}",
                    e
                )))
            }
        };

        let py = object.py();

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

        // Instantiate the `Instance` Python class.
        object.init({
            Self {
                instance: instance.clone(),
                exports: Py::new(
                    py,
                    ExportedFunctions {
                        instance: instance.clone(),
                        functions: exported_functions,
                    },
                )?,
                memory: match exported_memory {
                    Some(memory) => Some(Py::new(py, Memory { memory })?),
                    None => None,
                },
                globals: Py::new(
                    py,
                    ExportedGlobals {
                        globals: exported_globals,
                    },
                )?,
            }
        });

        Ok(())
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

    /// Call a function by its index. It is useful for advanced usages.
    ///
    /// Arguments must be objects of type `Value`.
    ///
    /// # Examples
    ///
    /// ```python
    /// index = 42
    /// instance.call_function_by_index(index, Value.i32(x), Value.i32(y))
    /// ```
    #[args(arguments = "*")]
    fn call_function_by_index(
        &self,
        py: Python,
        index: usize,
        arguments: &PyTuple,
    ) -> PyResult<PyObject> {
        let function = self.instance.dyn_func_by_index(index).map_err(|_| {
            RuntimeError::py_err(format!("Function at index `{}` does not exist.", index))
        })?;

        call_dyn_func(py, &index.to_string(), function, arguments)
    }
}
