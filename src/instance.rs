//! The `Instance` Python object to build WebAssembly instances.

use crate::value::{get_wasm_value, wasm_value_into_python_object, Value};
use cpython::{PyBytes, PyObject, PyResult};
use generational_arena::{Arena, Index};
use std::cell::RefCell;
use wasmer_runtime::{self as runtime, imports, instantiate, Value as WasmValue};

thread_local! {
    pub static WASM_INSTANCES: RefCell<Arena<runtime::Instance>> = RefCell::new(Arena::new());
}

pub struct InnerInstance {
    index: Index,
}

impl Drop for InnerInstance {
    fn drop(&mut self) {
        WASM_INSTANCES.with(|instances| {
            instances.borrow_mut().remove(self.index);
        });
    }
}

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

    def call(&self, function_name: &str, function_arguments: Vec<Value>) -> PyResult<PyObject> {
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
