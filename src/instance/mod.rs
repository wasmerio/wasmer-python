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
pub(crate) mod inspect;

use crate::memory::Memory;
use exports::ExportedFunctions;
use pyo3::{
    exceptions::RuntimeError,
    prelude::*,
    types::{PyAny, PyBytes},
    PyNativeType, PyTryFrom, Python,
};
use std::rc::Rc;
use wasmer_runtime::{imports, instantiate, Export};

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
    /// All WebAssembly exported functions represented by an
    /// `ExportedFunctions` object.
    pub(crate) exports: Py<ExportedFunctions>,

    /// The WebAssembly exported memory represented by a `Memory`
    /// object.
    pub(crate) memory: Option<Py<Memory>>,
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

        // Collect the exported functions from the WebAssembly module.
        let mut exported_functions = Vec::new();

        for (export_name, export) in instance.exports() {
            if let Export::Function { .. } = export {
                exported_functions.push(export_name);
            }
        }

        // Collect the exported memory from the WebAssembly module.
        let memory = instance.exports().find_map(|(_, export)| match export {
            Export::Memory(memory) => Some(Rc::new(memory)),
            _ => None,
        });

        // Instantiate the `Instance` Python class.
        object.init({
            Self {
                exports: Py::new(
                    py,
                    ExportedFunctions {
                        instance,
                        functions: exported_functions,
                    },
                )?,
                memory: match memory {
                    Some(memory) => Some(Py::new(py, Memory { memory })?),
                    None => None,
                },
            }
        });

        Ok(())
    }

    #[getter]
    /// The `exports` getter.
    fn exports(&self) -> PyResult<&Py<ExportedFunctions>> {
        Ok(&self.exports)
    }

    #[getter]
    /// The `memory` getter.
    fn memory(&self) -> PyResult<PyObject> {
        let gil_guard = Python::acquire_gil();
        let py = gil_guard.python();

        match &self.memory {
            Some(memory) => Ok(memory.into_py(py)),
            None => Ok(py.None()),
        }
    }
}
