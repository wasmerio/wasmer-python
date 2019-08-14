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

use crate::{memory::Memory, value::Value};
use pyo3::{
    class::basic::PyObjectProtocol,
    exceptions::{LookupError, RuntimeError},
    prelude::*,
    types::{PyAny, PyBytes, PyFloat, PyLong, PyTuple},
    PyNativeType, PyTryFrom, ToPyObject,
};
use std::rc::Rc;
use wasmer_runtime::{self as runtime, imports, instantiate, Export, Value as WasmValue};
use wasmer_runtime_core::types::Type;

#[pyclass]
/// `ExportedFunction` is a Python class that represents a WebAssembly
/// exported function. Such a function can be invoked from Python by using the
/// `__call__` Python class method.
pub struct ExportedFunction {
    /// The underlying Rust WebAssembly instance.
    instance: Rc<runtime::Instance>,

    /// The exported function name from the WebAssembly module.
    function_name: String,
}

#[pymethods]
/// Implement methods on the `ExportedFunction` Python class.
impl ExportedFunction {
    #[call]
    #[args(arguments = "*")]
    // The `ExportedFunction.__call__` method.
    // The `#[args(arguments = "*")]` means that the method has an
    // unfixed arity. All parameters will be received in the
    // `arguments` argument.
    fn __call__(&self, py: Python, arguments: &PyTuple) -> PyResult<PyObject> {
        // Get the exported function.
        let function = match self.instance.dyn_func(&self.function_name) {
            Ok(function) => function,
            Err(_) => {
                return Err(RuntimeError::py_err(format!(
                    "Function `{}` does not exist.",
                    self.function_name
                )))
            }
        };

        // Check the given arguments match the exported function signature.
        let signature = function.signature();
        let parameters = signature.params();

        let number_of_parameters = parameters.len() as isize;
        let number_of_arguments = arguments.len() as isize;
        let diff: isize = number_of_parameters - number_of_arguments;

        if diff > 0 {
            return Err(RuntimeError::py_err(format!(
                "Missing {} argument(s) when calling `{}`: Expect {} argument(s), given {}.",
                diff, self.function_name, number_of_parameters, number_of_arguments
            )));
        } else if diff < 0 {
            return Err(RuntimeError::py_err(format!(
                "Given {} extra argument(s) when calling `{}`: Expect {} argument(s), given {}.",
                diff.abs(),
                self.function_name,
                number_of_parameters,
                number_of_arguments
            )));
        }

        // Map Python arguments to WebAssembly values.
        let mut function_arguments = Vec::<WasmValue>::with_capacity(number_of_parameters as usize);

        for (parameter, argument) in parameters.iter().zip(arguments.iter()) {
            let value = match argument.downcast_ref::<Value>() {
                Ok(value) => value.value.clone(),
                Err(_) => match parameter {
                    Type::I32 => {
                        WasmValue::I32(argument.downcast_ref::<PyLong>()?.extract::<i32>()?)
                    }
                    Type::I64 => {
                        WasmValue::I64(argument.downcast_ref::<PyLong>()?.extract::<i64>()?)
                    }
                    Type::F32 => {
                        WasmValue::F32(argument.downcast_ref::<PyFloat>()?.extract::<f32>()?)
                    }
                    Type::F64 => {
                        WasmValue::F64(argument.downcast_ref::<PyFloat>()?.extract::<f64>()?)
                    }
                    Type::V128 => {
                        WasmValue::V128(argument.downcast_ref::<PyLong>()?.extract::<u128>()?)
                    }
                },
            };

            function_arguments.push(value);
        }

        // Call the exported function.
        let results = match function.call(function_arguments.as_slice()) {
            Ok(results) => results,
            Err(e) => return Err(RuntimeError::py_err(format!("{}", e))),
        };

        // Map the WebAssembly first result to a Python value.
        if results.len() > 0 {
            Ok(match results[0] {
                WasmValue::I32(result) => result.to_object(py),
                WasmValue::I64(result) => result.to_object(py),
                WasmValue::F32(result) => result.to_object(py),
                WasmValue::F64(result) => result.to_object(py),
                WasmValue::V128(result) => result.to_object(py),
            })
        } else {
            Ok(py.None())
        }
    }
}

#[pyclass]
/// `ExportedFunctions` is a Python class that represents the set
/// of WebAssembly exported functions. It's basically a set of
/// `ExportedFunction` classes.
///
/// # Examples
///
/// ```python
/// from wasmer import Instance
///
/// instance = Instance(wasm_bytes)
/// result = instance.exports.sum(1, 2)
/// ```
pub struct ExportedFunctions {
    /// The underlying Rust WebAssembly instance.
    pub(crate) instance: Rc<runtime::Instance>,

    /// Available exported function names from the WebAssembly module.
    pub(crate) functions: Vec<String>,
}

#[pyproto]
/// Implement the Python object protocol on the `ExportedFunctions`
/// Python class.
impl PyObjectProtocol for ExportedFunctions {
    /// An Python attribute in this context represents a WebAssembly
    /// exported function name.
    fn __getattr__(&self, key: String) -> PyResult<ExportedFunction> {
        if self.functions.contains(&key) {
            Ok(ExportedFunction {
                function_name: key,
                instance: self.instance.clone(),
            })
        } else {
            Err(LookupError::py_err(format!(
                "Function `{}` does not exist.",
                key
            )))
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.functions))
    }
}

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
    pub(crate) memory: Py<Memory>,
}

#[pymethods]
/// Implement methods on the `Instance` Python class.
impl Instance {
    #[new]
    /// The constructor instantiates a new WebAssembly instance basde
    /// on WebAssembly bytes (represented by the Python bytes type).
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
        let memory = instance
            .exports()
            .find_map(|(_, export)| match export {
                Export::Memory(memory) => Some(Rc::new(memory)),
                _ => None,
            })
            .ok_or_else(|| RuntimeError::py_err("No memory exported."))?;

        // Instantiate the `Instance` Python class.
        object.init({
            Self {
                exports: Py::new(
                    py,
                    ExportedFunctions {
                        instance: instance.clone(),
                        functions: exported_functions,
                    },
                )?,
                memory: Py::new(py, Memory { memory })?,
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
    fn memory(&self) -> PyResult<&Py<Memory>> {
        Ok(&self.memory)
    }
}
