//! The `wasmer.Module` Python object to build WebAssembly modules.

use crate::{
    instance::{ExportedFunctions, Instance},
    memory::Memory,
};
use pyo3::{
    exceptions::RuntimeError,
    prelude::*,
    types::{PyAny, PyBytes},
    PyTryFrom,
};
use std::rc::Rc;
use wasmer_runtime::{self as runtime, imports, validate, Export};

#[pyclass]
/// `Module` is a Python class that represents a WebAssembly module.
pub struct Module {
    /// The underlying Rust WebAssembly module.
    #[allow(unused)]
    module: runtime::Module,
}

#[pymethods]
/// Implement methods on the `Module` Python class.
impl Module {
    /// Compile bytes into a WebAssembly module.
    #[new]
    fn new(object: &PyRawObject, bytes: &PyAny) -> PyResult<()> {
        // Read the bytes.
        let bytes = <PyBytes as PyTryFrom>::try_from(bytes)?.as_bytes();
        let module = runtime::compile(bytes).map_err(|error| {
            RuntimeError::py_err(format!("Failed to compile the module:\n    {}", error))
        })?;

        // Instantiate the `Module` Python clas.
        object.init({ Self { module } });

        Ok(())
    }

    // Instantiate the module into an `Instance` Python object.
    fn instantiate(&self, py: Python) -> PyResult<Py<Instance>> {
        let imports = imports! {};
        let instance = match self.module.instantiate(&imports) {
            Ok(instance) => Rc::new(instance),
            Err(e) => {
                return Err(RuntimeError::py_err(format!(
                    "Failed to instantiate the module:\n    {}",
                    e
                )))
            }
        };

        // Collect the exported functions from the WebAssembly module.
        let mut exported_functions = Vec::new();

        for (export_name, export) in instance.exports() {
            if let Export::Function { .. } = export {
                exported_functions.push(export_name);
            }
        }

        // Collect the exported memory from the WebAssembly module.
        let memory = instance
            .exports()
            .find_map(|(_, export)| match export {
                Export::Memory(memory) => Some(Rc::new(memory)),
                _ => None,
            })
            .ok_or_else(|| RuntimeError::py_err("No memory exported."))?;

        // Instantiate the `Instance` Python class.
        Ok(Py::new(
            py,
            Instance {
                exports: Py::new(
                    py,
                    ExportedFunctions {
                        instance: instance.clone(),
                        functions: exported_functions,
                    },
                )?,
                memory: Py::new(py, Memory { memory })?,
            },
        )?)
    }

    /// Check that given bytes represent a valid WebAssembly module.
    #[staticmethod]
    fn validate(bytes: &PyAny) -> PyResult<bool> {
        match <PyBytes as PyTryFrom>::try_from(bytes) {
            Ok(bytes) => Ok(validate(bytes.as_bytes())),
            _ => Ok(false),
        }
    }
}
