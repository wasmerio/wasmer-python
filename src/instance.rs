//! The `Instance` Python object to build WebAssembly instances.

use crate::{
    error::new_runtime_error,
    memory_view,
    value::{get_wasm_value, wasm_value_into_python_object, Value},
    Shell,
};
use cpython::{PyBytes, PyObject, PyResult, Python};
use wasmer_runtime::{
    self as runtime, imports, instantiate, validate as wasm_validate, Export, Memory,
    Value as WasmValue,
};

/// The Python `Instance` class.
///
/// # Examples
///
/// ```python,ignore
/// from wasmer import Instance, Value
///
/// file = open('my_program.wasm', 'rb') # note the mode contains `b` to get bytes, and not UTF-8 characters.
/// bytes = file.read()
///
/// instance = Instance(bytes)
/// result = instance.call('add_one', [Value::from_i32(1)])
/// ```
py_class!(pub class Instance |py| {
    data instance: Shell<runtime::Instance>;

    def __new__(_cls, bytes: PyBytes) -> PyResult<Instance> {
        let bytes = bytes.data(py);
        let imports = imports! {};
        let instance = match instantiate(bytes, &imports) {
            Ok(instance) => instance,
            Err(e) => return Err(new_runtime_error(py, &format!("Failed to instantiate the module:\n    {}", e)))
        };

        Instance::create_instance(py, Shell::new(instance))
    }

    def call(&self, function_name: &str, function_arguments: Vec<Value> = Vec::new()) -> PyResult<PyObject> {
        let function_arguments: Vec<WasmValue> =
            function_arguments
                .into_iter()
                .map(|value_object| get_wasm_value(py, &value_object))
                .collect();

        let instance = self.instance(py);
        let function = match instance.dyn_func(function_name) {
            Ok(function) => function,
            Err(_) => return Err(new_runtime_error(py, &format!("Function `{}` does not exist.", function_name)))
        };

        let results = match function.call(function_arguments.as_slice()) {
            Ok(results) => results,
            Err(e) => return Err(new_runtime_error(py, &format!("{}", e)))
        };

        Ok(wasm_value_into_python_object(py, &results[0]))
    }

    def uint8_memory_view(&self, offset: usize = 0) -> PyResult<memory_view::Uint8MemoryView> {
        get_instance_memory(&self, py)
            .map_or_else(
                || Err(new_runtime_error(py, "No memory exported.")),
                |memory| Ok(memory_view::new_uint8_memory_view(py, memory, offset))
            )
    }

    def int8_memory_view(&self, offset: usize = 0) -> PyResult<memory_view::Int8MemoryView> {
        get_instance_memory(&self, py)
            .map_or_else(
                || Err(new_runtime_error(py, "No memory exported.")),
                |memory| Ok(memory_view::new_int8_memory_view(py, memory, offset))
            )
    }

    def uint16_memory_view(&self, offset: usize = 0) -> PyResult<memory_view::Uint16MemoryView> {
        get_instance_memory(&self, py)
            .map_or_else(
                || Err(new_runtime_error(py, "No memory exported.")),
                |memory| Ok(memory_view::new_uint16_memory_view(py, memory, offset))
            )
    }

    def int16_memory_view(&self, offset: usize = 0) -> PyResult<memory_view::Int16MemoryView> {
        get_instance_memory(&self, py)
            .map_or_else(
                || Err(new_runtime_error(py, "No memory exported.")),
                |memory| Ok(memory_view::new_int16_memory_view(py, memory, offset))
            )
    }

    def uint32_memory_view(&self, offset: usize = 0) -> PyResult<memory_view::Uint32MemoryView> {
        get_instance_memory(&self, py)
            .map_or_else(
                || Err(new_runtime_error(py, "No memory exported.")),
                |memory| Ok(memory_view::new_uint32_memory_view(py, memory, offset))
            )
    }

    def int32_memory_view(&self, offset: usize = 0) -> PyResult<memory_view::Int32MemoryView> {
        get_instance_memory(&self, py)
            .map_or_else(
                || Err(new_runtime_error(py, "No memory exported.")),
                |memory| Ok(memory_view::new_int32_memory_view(py, memory, offset))
            )
    }
});

/// The Python `validate` function.
///
///
pub fn validate(py: Python, bytes: PyBytes) -> PyResult<bool> {
    Ok(wasm_validate(bytes.data(py)))
}

fn get_instance_memory(instance: &Instance, py: Python) -> Option<Memory> {
    instance
        .instance(py)
        .exports()
        .find_map(|(_, export)| match export {
            Export::Memory(memory) => Some(memory),
            _ => None,
        })
}
