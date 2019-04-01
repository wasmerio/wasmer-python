#[macro_use]
extern crate cpython;

use cpython::PyResult;
use generational_arena::{Arena, Index};
use std::cell::RefCell;
use wasmer_runtime::{self as runtime, imports, instantiate, Value};

static WASM: &'static [u8] = &[
    // The module above compiled to bytecode goes here.
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x06, 0x01, 0x60, 0x01, 0x7f, 0x01, 0x7f,
    0x03, 0x02, 0x01, 0x00, 0x07, 0x0b, 0x01, 0x07, 0x61, 0x64, 0x64, 0x5f, 0x6f, 0x6e, 0x65, 0x00,
    0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x20, 0x00, 0x41, 0x01, 0x6a, 0x0b, 0x00, 0x1a, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x01, 0x0a, 0x01, 0x00, 0x07, 0x61, 0x64, 0x64, 0x5f, 0x6f, 0x6e, 0x65, 0x02,
    0x07, 0x01, 0x00, 0x01, 0x00, 0x02, 0x70, 0x30,
];

py_module_initializer!(libwasm, initlibwasm, PyInit_wasm, |python, module| {
    module.add(
        python,
        "__doc__",
        "This extension exposes an API to manipulate and execute WebAssembly binaries.",
    )?;
    module.add_class::<Instance>(python)?;

    Ok(())
});

thread_local! {
    pub static WASM_INSTANCES: RefCell<Arena<runtime::Instance>> = RefCell::new(Arena::new());
}

struct WasmInstance {
    index: Index,
}

py_class!(class Instance |py| {
    data instance: WasmInstance;

    def __new__(_cls) -> PyResult<Instance> {
        let imports = imports! {};
        let instance = instantiate(WASM, &imports).unwrap();
        let index = WASM_INSTANCES.with(|f| f.borrow_mut().insert(instance));

        Instance::create_instance(
            py,
            WasmInstance {
                index
            }
        )
    }

    def invoke_function(&self, function_name: &str) -> PyResult<i32> {
        let index = self.instance(py).index;
        let result = WASM_INSTANCES.with(
            |f| {
                let b = f.borrow();
                let instance = b.get(index).unwrap();
                let values = instance.dyn_func(function_name).unwrap().call(&[Value::I32(1)]).unwrap();

                if let Value::I32(value) = values[0] {
                    value
                } else {
                    0
                }
            }
        );

        Ok(result)
    }
});
