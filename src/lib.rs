#![deny(warnings)]

use pyo3::prelude::*;

mod instance;
mod memory;
mod module;
mod value;

use instance::Instance;
use module::Module;
use value::Value;

/// This extension allows to manipulate and to execute WebAssembly binaries.
#[pymodule]
fn wasmer(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_class::<Instance>()?;
    module.add_class::<Module>()?;
    module.add_class::<Value>()?;
    module.add_class::<memory::Memory>()?;
    module.add_class::<memory::view::Int16Array>()?;
    module.add_class::<memory::view::Int32Array>()?;
    module.add_class::<memory::view::Int8Array>()?;
    module.add_class::<memory::view::Uint16Array>()?;
    module.add_class::<memory::view::Uint32Array>()?;
    module.add_class::<memory::view::Uint8Array>()?;

    Ok(())
}
