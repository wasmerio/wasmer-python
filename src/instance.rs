//! The `Instance` Python object to build WebAssembly instances.

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
pub struct ExportedFunction {
    function_name: String,
    instance: Rc<runtime::Instance>,
}

#[pymethods]
impl ExportedFunction {
    #[call]
    #[args(arguments = "*")]
    fn __call__(&self, py: Python, arguments: &PyTuple) -> PyResult<PyObject> {
        let function = match self.instance.dyn_func(&self.function_name) {
            Ok(function) => function,
            Err(_) => {
                return Err(RuntimeError::py_err(format!(
                    "Function `{}` does not exist.",
                    self.function_name
                )))
            }
        };

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
                },
            };

            function_arguments.push(value);
        }

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
pub struct ExportedFunctions {
    instance: Rc<runtime::Instance>,
    functions: Vec<String>,
}

#[pyproto]
impl PyObjectProtocol for ExportedFunctions {
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
pub struct Instance {
    exports: Py<ExportedFunctions>,
    memory: Py<Memory>,
}

#[pymethods]
impl Instance {
    #[new]
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
    fn exports(&self) -> PyResult<&Py<ExportedFunctions>> {
        Ok(&self.exports)
    }

    #[getter]
    fn memory(&self) -> PyResult<&Py<Memory>> {
        Ok(&self.memory)
    }
}
