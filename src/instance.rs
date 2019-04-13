//! The `Instance` Python object to build WebAssembly instances.

//use crate::{
//    error::new_runtime_error,
//    memory_view,
//    value::{get_wasm_value, wasm_value_into_python_object, Value},
//    Shell,
//};
//use cpython::{PyBytes, PyObject, PyResult, Python};
use crate::memory_view;
use pyo3::{
    prelude::*,
    types::{PyAny, PyBytes, PyTuple, PyDict},
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
};

#[pyclass]
pub struct ExportedFunction {
    function_name: String,
    instance: Rc<runtime::Instance>,
}

#[pymethods]
impl ExportedFunction {
     #[call]
     #[args(args="*")]
     fn __call__(&self, args: &PyTuple) -> PyResult<String> {
         println!("exported function has been called {:?}", args);
         Ok(self.function_name.clone())
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

//    fn call(&self, function_name: &str, function_arguments: Value) -> PyResult<usize> {
//        /*
//        let function_arguments: Vec<WasmValue> =
//            function_arguments
//                .into_iter()
//                .map(|value_object| value_object.value)
//                .collect();
//
//        let instance = self.instance;
//        let function = match instance.dyn_func(function_name) {
//            Ok(function) => function,
//            Err(_) => return Err(RuntimeError::py_err(format!("Function `{}` does not exist.", function_name)))
//        };
//
//        let results = match function.call(function_arguments.as_slice()) {
//            Ok(results) => results,
//            Err(e) => return Err(RuntimeError::py_err(format!("{}", e)))
//        };
//        */
//
//        Ok(42) //wasm_value_into_python_object(py, &results[0]))
//    }

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
