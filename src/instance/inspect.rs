use pyo3::prelude::*;

use wasmer_runtime_core::instance::DynFunc;

/// Convenience functions to allow inspection of an exported
/// function. Note: Python provides the `inspect` module for
/// this. Future improvements can be made on this side to have a trait
/// for these functions, as developers may need an interface to the
/// underlying `Instance::dyn_func`.
pub trait InspectExportedFunction {
    // A convenience method to move Wasmer runtime's dynamic function
    // object into scope for pyo3 constructors/callers
    fn move_runtime_func_obj(&self) -> Result<DynFunc, PyErr>;

    // Returns the signature of an exported function.
    fn signature(&self) -> String;

    // Returns the parameters of an exporterd function.
    fn params(&self) -> String;
}
