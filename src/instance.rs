//! The `Instance` Python object to build WebAssembly instances,
//! used via `wasmer_runtime.Instance`
//!
//! *Reminder*:
//! > `WebAssembly.Instance` object is a stateful, executable instance of a
//! > `WebAssembly.Module`. Instance objects contain all the Exported
//! > WebAssembly functions that allow calling into WebAssembly code
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
/// Representation of an exported single function leveraging `pyo3`.
/// It is implemented using `pyo3` Python class that defines __call__
pub struct ExportedFunction {
    /// Rust References Counting, for attached `wasmer_runtime.Instance`
    instance: Rc<runtime::Instance>,
    /// Functions names as exported in the Python namespace
    function_name: String,
}

#[pymethods]
/// Methods attached to the single function
impl ExportedFunction {
    #[call]
    #[args(arguments = "*")]
    /// Function class shall declare __call__.
    /// `pyo3::prelude::Python` is a zero-size marker struct that is required
    /// for most Python operations; indicates that the GIL is currently held.
    fn __call__(&self, py: Python, arguments: &PyTuple) -> PyResult<PyObject> {
        // retrieve the function representation that can be called safely
        let function = match self.instance.dyn_func(&self.function_name) {
            Ok(function) => function,
            Err(_) => {
                return Err(RuntimeError::py_err(format!(
                    "Function `{}` does not exist.",
                    self.function_name
                )))
            }
        };

        // process function signature in `wasmer_runtime::DynFunc`
        let signature = function.signature();
        let parameters = signature.params();
        // match number of arguments against signature
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

        // safely allocate arguments as `wasmer_runtime.Value`
        let mut function_arguments = Vec::<WasmValue>::with_capacity(number_of_parameters as usize);

        // cast from `PyAny` to `T` using `pyo3::types::PyAny::downcast_ref`
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
                },
            };

            function_arguments.push(value);
        }

        // finally, call the function with `wasmer_runtime::DynFunc::call`
        let results = match function.call(function_arguments.as_slice()) {
            Ok(results) => results,
            Err(e) => return Err(RuntimeError::py_err(format!("{}", e))),
        };

        Ok(match results[0] {
            WasmValue::I32(result) => result.to_object(py),
            WasmValue::I64(result) => result.to_object(py),
            WasmValue::F32(result) => result.to_object(py),
            WasmValue::F64(result) => result.to_object(py),
        })
    }
}

#[pyclass]
/// Representation of exported **collection** of functions,
/// equivalent to a Python module-namespace
pub struct ExportedFunctions {
    /// Rust References Counting, for attached `wasmer_runtime.Instance`
    instance: Rc<runtime::Instance>,
    /// Functions names as exported in the Python namespace
    functions: Vec<String>,
}

#[pyproto]
/// A `pyo3` protocol to handle the ExportedFunctions namespace.
impl PyObjectProtocol for ExportedFunctions {
    /// Return the right function with a dedicated `wasmer_runtime.Instance`
    /// Comparable to `module.function_name` in Python
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
/// A struct that binds a collection of exported functions (a namespace) to
/// `wasmer_runtime.Memory`
/// as used from Python code, see example:
/// ```python
/// from wasmer import Instance
/// instance = Instance(wasm_bytes)
/// ```
pub struct Instance {
    /// PyO3 safe wrapper around `ffi::PyObject`, for exported functions
    exports: Py<ExportedFunctions>,
    /// Pyo3 safe wrapper around `ffi::PyObject`, for memory
    memory: Py<Memory>,
}

#[pymethods]
impl Instance {
    #[new]
    /// Constructor. Take a look to `pyo3::type_object::PyRawObject`
    fn new(object: &PyRawObject, bytes: &PyAny) -> PyResult<()> {
        let bytes = <PyBytes as PyTryFrom>::try_from(bytes)?.as_bytes();
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
        let mut exported_functions = Vec::new();

        for (export_name, export) in instance.exports() {
            if let Export::Function { .. } = export {
                exported_functions.push(export_name);
            }
        }

        let memory = instance
            .exports()
            .find_map(|(_, export)| match export {
                Export::Memory(memory) => Some(Rc::new(memory)),
                _ => None,
            })
            .ok_or_else(|| RuntimeError::py_err("No memory exported."))?;

        // initialize the new `ffi::PyObject`, with the required namespace
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
    /// Return exported functions bound to `wasmer_runtime.Memory`
    fn exports(&self) -> PyResult<&Py<ExportedFunctions>> {
        Ok(&self.exports)
    }

    #[getter]
    /// Return the `wasmer_runtime.Memory` object allocated for namespace
    fn memory(&self) -> PyResult<&Py<Memory>> {
        Ok(&self.memory)
    }
}
