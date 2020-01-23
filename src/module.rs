//! The `wasmer.Module` Python object to build WebAssembly modules.

use crate::{instance::exports::ExportedFunctions, instance::Instance, memory::Memory};
use pyo3::{
    exceptions::RuntimeError,
    prelude::*,
    types::{PyAny, PyBytes, PyDict, PyList},
    PyTryFrom,
};
use std::rc::Rc;
use wasmer_runtime::{self as runtime, imports, validate, Export};
use wasmer_runtime_core::{self as runtime_core, cache::Artifact, module::ExportIndex};

#[pyclass]
/// `Module` is a Python class that represents a WebAssembly module.
pub struct Module {
    /// The underlying Rust WebAssembly module.
    module: runtime::Module,
}

#[pymethods]
/// Implement methods on the `Module` Python class.
impl Module {
    /// Compile bytes into a WebAssembly module.
    #[new]
    #[allow(clippy::new_ret_no_self)]
    fn new(object: &PyRawObject, bytes: &PyAny) -> PyResult<()> {
        // Read the bytes.
        let bytes = <PyBytes as PyTryFrom>::try_from(bytes)?.as_bytes();

        // Compile the module.
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

        // Instantiate the module.
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
        let memory = instance.exports().find_map(|(_, export)| match export {
            Export::Memory(memory) => Some(Rc::new(memory)),
            _ => None,
        });

        // Instantiate the `Instance` Python class.
        Ok(Py::new(
            py,
            Instance {
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
            },
        )?)
    }

    #[getter]
    /// The `exports` getter.
    fn exports<'p>(&self, py: Python<'p>) -> PyResult<&'p PyList> {
        let mut items: Vec<&PyDict> = vec![];

        for (name, export_index) in self.module.info().exports.iter() {
            let dict = PyDict::new(py);

            dict.set_item("name", name)?;
            dict.set_item(
                "kind",
                match export_index {
                    ExportIndex::Func(_) => "function",
                    ExportIndex::Memory(_) => "memory",
                    ExportIndex::Global(_) => "global",
                    ExportIndex::Table(_) => "table",
                },
            )?;

            items.push(dict);
        }

        Ok(PyList::new(py, items))
    }

    /// Serialize the module into Python bytes.
    fn serialize<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        // Get the module artifact.
        match self.module.cache() {
            // Serialize the artifact.
            Ok(artifact) => match artifact.serialize() {
                Ok(serialized_artifact) => Ok(PyBytes::new(py, serialized_artifact.as_slice())),
                Err(_) => Err(RuntimeError::py_err(
                    "Failed to serialize the module artifact.",
                )),
            },
            Err(_) => Err(RuntimeError::py_err("Failed to get the module artifact.")),
        }
    }

    /// Deserialize Python bytes into a module instance.
    #[staticmethod]
    fn deserialize(bytes: &PyAny, py: Python) -> PyResult<Py<Module>> {
        // Read the bytes.
        let serialized_module = <PyBytes as PyTryFrom>::try_from(bytes)?.as_bytes();

        // Deserialize the artifact.
        match Artifact::deserialize(serialized_module) {
            Ok(artifact) => {
                // Get the module from the artifact.
                match unsafe {
                    runtime_core::load_cache_with(artifact, &runtime::default_compiler())
                } {
                    Ok(module) => Ok(Py::new(py, Self { module })?),
                    Err(_) => Err(RuntimeError::py_err(
                        "Failed to compile the serialized module.",
                    )),
                }
            }
            Err(_) => Err(RuntimeError::py_err("Failed to deserialize the module.")),
        }
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
