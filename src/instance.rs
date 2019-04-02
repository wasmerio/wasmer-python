//! The `Instance` Python object to build WebAssembly instances.

use crate::value::{get_wasm_value, wasm_value_into_python_object, Value};
use cpython::{PyBytes, PyObject, PyResult};
use generational_arena::{Arena, Index};
use std::cell::RefCell;
use wasmer_runtime::{self as runtime, imports, instantiate, Value as WasmValue};

/// `wasmer_runtime::Instance` isn't thread-safe, and it's somewhat
/// complex to make it entirely thread-safe. Instead, this trick is
/// used: Create a thread local storage key, that contains an arena,
/// that itself contains `wasmer_runtime::Instance`s. Each entry is
/// indexed by a `generational_arena::Index` value. This index is held
/// by the Python `Instance` object.
thread_local! {
    pub static WASM_INSTANCES: RefCell<Arena<runtime::Instance>> = RefCell::new(Arena::new());
}

/// Holds all the Wasm instance information.
pub struct InnerInstance {
    /// Index of the Wasm instance in `WASM_INSTANCES`.
    index: Index,
}

/// When the Python object is dropped, this structure is dropped, and
/// then, the Wasm instance is removed from `WASM_INSTANCES`.
impl Drop for InnerInstance {
    fn drop(&mut self) {
        WASM_INSTANCES.with(|instances| {
            instances.borrow_mut().remove(self.index);
        });
    }
}

/// The Python `Instance` class.
///
/// # Examples
///
/// ```python,ignore
/// from wasm import Instance, Value
///
/// file = open('my_program.wasm', 'rb') # note the mode contains `b` to get bytes, and not UTF-8 characters.
/// bytes = file.read()
///
/// instance = Instance(bytes)
/// result = instance.call('add_one', [Value::from_i32(1)])
/// ```
py_class!(pub class Instance |py| {
    data instance: InnerInstance;

    def __new__(_cls, bytes: PyBytes) -> PyResult<Instance> {
        let bytes = bytes.data(py);
        let imports = imports! {};
        let instance = instantiate(bytes, &imports).unwrap();
        let index = WASM_INSTANCES.with(|f| f.borrow_mut().insert(instance));

        Instance::create_instance(
            py,
            InnerInstance {
                index
            }
        )
    }

    def call(&self, function_name: &str, function_arguments: Vec<Value> = Vec::new()) -> PyResult<PyObject> {
        let function_arguments: Vec<WasmValue> =
            function_arguments
                .into_iter()
                .map(|value_object| get_wasm_value(py, &value_object))
                .collect();

        let index = self.instance(py).index;
        let results = WASM_INSTANCES.with(
            |instances| {
                let instances = instances.borrow();
                let instance = instances.get(index).unwrap();

                instance.dyn_func(function_name).unwrap().call(function_arguments.as_slice()).unwrap()
            }
        );

        Ok(wasm_value_into_python_object(py, &results[0]))
    }
});
