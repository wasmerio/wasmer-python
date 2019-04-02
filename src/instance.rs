//! The `Instance` Python object to build WebAssembly instances.

use crate::value::{get_wasm_value, wasm_value_into_python_object, Value};
use cpython::{PyObject, PyResult};
use generational_arena::{Arena, Index};
use std::cell::RefCell;
use wasmer_runtime::{self as runtime, imports, instantiate, Value as WasmValue};

static WASM: &'static [u8] = &[
    // The module above compiled to bytecode goes here.
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x06, 0x01, 0x60, 0x01, 0x7f, 0x01, 0x7f,
    0x03, 0x02, 0x01, 0x00, 0x07, 0x0b, 0x01, 0x07, 0x61, 0x64, 0x64, 0x5f, 0x6f, 0x6e, 0x65, 0x00,
    0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x20, 0x00, 0x41, 0x01, 0x6a, 0x0b, 0x00, 0x1a, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x01, 0x0a, 0x01, 0x00, 0x07, 0x61, 0x64, 0x64, 0x5f, 0x6f, 0x6e, 0x65, 0x02,
    0x07, 0x01, 0x00, 0x01, 0x00, 0x02, 0x70, 0x30,
];

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

    def __new__(_cls) -> PyResult<Instance> {
        let imports = imports! {};
        let instance = instantiate(WASM, &imports).unwrap();
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
