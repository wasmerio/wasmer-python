//! The `Instance` Python object to build WebAssembly instances.

//use crate::{
//    error::new_runtime_error,
//    memory_view,
//    value::{get_wasm_value, wasm_value_into_python_object, Value},
//    Shell,
//};
//use cpython::{PyBytes, PyObject, PyResult, Python};
use crate::{memory_view, value::Value};
use pyo3::{
    prelude::*,
    types::{PyAny, PyBytes, PyTuple, PyDict, PyLong, PyFloat},
    PyTryFrom,
    exceptions::RuntimeError,
    PyNativeType,
};
use std::rc::Rc;
use wasmer_runtime::{
    self as runtime,
    imports,
    instantiate,
    Export,
    Memory,
    Value as WasmValue,
};
use wasmer_runtime_core::types::Type;

#[pyclass]
pub struct ExportedFunction {
    function_name: String,
    instance: Rc<runtime::Instance>,
}

#[pymethods]
impl ExportedFunction {
    #[call]
    #[args(arguments="*")]
    fn __call__(&self, py: Python, arguments: &PyTuple) -> PyResult<PyObject> {
        let function = match self.instance.dyn_func(&self.function_name) {
            Ok(function) => function,
            Err(_) => return Err(RuntimeError::py_err(format!("Function `{}` does not exist.", self.function_name)))
        };

        let signature = function.signature();
        let parameters = signature.params();
        let number_of_parameters = parameters.len() as isize;
        let number_of_arguments = arguments.len() as isize;
        let diff: isize = number_of_parameters - number_of_arguments;

        if diff > 0 {
            return Err(
                RuntimeError::py_err(
                    format!(
                        "Missing {} argument(s) when calling `{}`: Expect {} argument(s), given {}.",
                        diff,
                        self.function_name,
                        number_of_parameters,
                        number_of_arguments
                    )
                )
            );
        } else if diff < 0 {
            return Err(
                RuntimeError::py_err(
                    format!(
                        "Given {} extra argument(s) when calling `{}`: Expect {} argument(s), given {}.",
                        diff.abs(),
                        self.function_name,
                        number_of_parameters,
                        number_of_arguments
                    )
                )
            );
        }

        let mut function_arguments = Vec::<WasmValue>::with_capacity(number_of_parameters as usize);

        for (parameter, argument) in parameters.iter().zip(arguments.iter()) {
            let value = match argument.downcast_ref::<Value>() {
                Ok(value) => value.value.clone(),
                Err(_) => match parameter {
                    Type::I32 => WasmValue::I32(argument.downcast_ref::<PyLong>()?.extract::<i32>()?),
                    Type::I64 => WasmValue::I64(argument.downcast_ref::<PyLong>()?.extract::<i64>()?),
                    Type::F32 => WasmValue::F32(argument.downcast_ref::<PyFloat>()?.extract::<f32>()?),
                    Type::F64 => WasmValue::F64(argument.downcast_ref::<PyFloat>()?.extract::<f64>()?),
                },
            };

            function_arguments.push(value);
        }

        let results = match function.call(function_arguments.as_slice()) {
            Ok(results) => results,
            Err(e) => return Err(RuntimeError::py_err(format!("{}", e))),
        };

        Ok(
            match results[0] {
                WasmValue::I32(result) => result.to_object(py),
                WasmValue::I64(result) => result.to_object(py),
                WasmValue::F32(result) => result.to_object(py),
                WasmValue::F64(result) => result.to_object(py),
            }
        )
    }
}

#[pyclass]
pub struct Instance {
    instance: Rc<runtime::Instance>,
    exports: PyObject,
}

#[pymethods]
impl Instance {
    #[new]
    fn new(object: &PyRawObject, bytes: &PyAny) -> PyResult<()> {
        let bytes = <PyBytes as PyTryFrom>::try_from(bytes)?.as_bytes();
        let imports = imports! {};
        let instance = match instantiate(bytes, &imports) {
            Ok(instance) => Rc::new(instance),
            Err(e) => return Err(RuntimeError::py_err(format!("Failed to instantiate the module:\n    {}", e))),
        };

        let py = object.py();

        let dict = PyDict::new(py);
        let function_name = String::from("sum");
        dict.set_item(
            function_name.clone(),
            Py::new(
                py,
                ExportedFunction {
                    function_name,
                    instance: instance.clone()
                }
            )?
        )?;

        object.init({
            Self {
                instance,
                exports: dict.to_object(py),
            }
        });

        Ok(())
    }

    #[getter]
    fn exports(&self) -> PyResult<&PyDict> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        Ok(self.exports.cast_as::<PyDict>(py)?)
    }

    #[args(offset=0)]
    fn uint8_memory_view(&self, py: Python, offset: usize) -> PyResult<Py<memory_view::Uint8MemoryView>> {
        get_instance_memory(&self)
            .map_or_else(
                || Err(RuntimeError::py_err("No memory exported.")),
                |memory| {
                    Py::new(py, memory_view::Uint8MemoryView { memory, offset })
                }
            )
    }

    #[args(offset=0)]
    fn int8_memory_view(&self, py: Python, offset: usize) -> PyResult<Py<memory_view::Int8MemoryView>> {
        get_instance_memory(&self)
            .map_or_else(
                || Err(RuntimeError::py_err("No memory exported.")),
                |memory| {
                    Py::new(py, memory_view::Int8MemoryView { memory, offset })
                }
            )
    }

    #[args(offset=0)]
    fn uint16_memory_view(&self, py: Python, offset: usize) -> PyResult<Py<memory_view::Uint16MemoryView>> {
        get_instance_memory(&self)
            .map_or_else(
                || Err(RuntimeError::py_err("No memory exported.")),
                |memory| {
                    Py::new(py, memory_view::Uint16MemoryView { memory, offset })
                }
            )
    }

    #[args(offset=0)]
    fn int16_memory_view(&self, py: Python, offset: usize) -> PyResult<Py<memory_view::Int16MemoryView>> {
        get_instance_memory(&self)
            .map_or_else(
                || Err(RuntimeError::py_err("No memory exported.")),
                |memory| {
                    Py::new(py, memory_view::Int16MemoryView { memory, offset })
                }
            )
    }

    #[args(offset=0)]
    fn uint32_memory_view(&self, py: Python, offset: usize) -> PyResult<Py<memory_view::Uint32MemoryView>> {
        get_instance_memory(&self)
            .map_or_else(
                || Err(RuntimeError::py_err("No memory exported.")),
                |memory| {
                    Py::new(py, memory_view::Uint32MemoryView { memory, offset })
                }
            )
    }

    #[args(offset=0)]
    fn int32_memory_view(&self, py: Python, offset: usize) -> PyResult<Py<memory_view::Int32MemoryView>> {
        get_instance_memory(&self)
            .map_or_else(
                || Err(RuntimeError::py_err("No memory exported.")),
                |memory| {
                    Py::new(py, memory_view::Int32MemoryView { memory, offset })
                }
            )
    }
}

fn get_instance_memory(instance: &Instance) -> Option<Memory> {
    instance
        .instance
        .exports()
        .find_map(|(_, export)| match export {
            Export::Memory(memory) => Some(memory),
            _ => None,
        })
}
