#![deny(warnings)]

#[macro_use]
extern crate cpython;

use cpython::PyBytes;

mod instance;
mod value;

use instance::{validate, Instance};
use value::Value;

// Declare the module.
py_module_initializer!(libwasm, initlibwasm, PyInit_wasm, |python, module| {
    module.add(
        python,
        "__doc__",
        "This extension exposes an API to manipulate and to execute WebAssembly binaries.",
    )?;
    module.add_class::<Instance>(python)?;
    module.add_class::<Value>(python)?;
    module.add(python, "validate", py_fn!(python, validate(bytes: PyBytes)))?;

    Ok(())
});
