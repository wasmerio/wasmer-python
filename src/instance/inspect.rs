use pyo3::prelude::*;

use wasmer_runtime_core::instance::DynFunc;

pub trait InspectExported {
    // A convenience method to move Wasmer runtime's dynamic function object
    // into scope for pyo3 constructors/callers
    fn move_runtime_func_obj(&self) -> Result<DynFunc, PyErr>;

    // Convenience functions to allow inspection of the exported function.
    // NOTE: Python provides the `inspect` module for this. Future improvements
    // can be made on this side to have a Trait for these functions, as developers
    // may need an interface to the underlying `Instance::dyn_func`.

    // return the signature of an exported function
    fn signature(&self) -> String;

    // return the parameters of an exporterd function
    fn params(&self) -> String;

}
