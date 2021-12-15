use crate::{
    errors::to_py_err, exports::Exports, import_object::ImportObject, module::Module,
    wasmer_inner::wasmer,
};
use pyo3::types::PyDict;
use pyo3::{exceptions::PyRuntimeError, prelude::*};
use std::borrow::Borrow;

/// A WebAssembly instance is a stateful, executable instance of a
/// WebAssembly `Module`.
///
/// Instance objects contain all the exported WebAssembly functions,
/// memories, tables and globals that allow interacting with
/// WebAssembly.
///
/// Specification: https://webassembly.github.io/spec/core/exec/runtime.html#module-instances
///
/// ## Example
///
/// Example without an import object. The following creates a module
/// with a `sum` exported function that sum two integers.
///
/// ```py
/// from wasmer import Store, Module, Instance
///
/// module = Module(
///     Store(),
///     """
///     (module
///       (type (func (param i32 i32) (result i32)))
///       (func (type 0)
///         local.get 0
///         local.get 1
///         i32.add)
///       (export "sum" (func 0)))
///     """
/// )
/// instance = Instance(module)
///
/// assert instance.exports.sum(1, 2) == 3
/// ```
///
/// Example with an import object. The following creates a module that
/// (i) imports a `sum` function from the `math` namespace, and (ii)
/// exports a `add_one` function that adds 1 to any given integer (by
/// using the `math.sum` function).
///
/// ```py
/// from wasmer import Store, Module, Instance, Function
/// from collections import defaultdict
///
/// # Let's define the `sum` function!
/// def sum(x: int, y: int) -> int:
///     return x + y
///
/// # Let's build a store, as usual.
/// store = Store()
///
/// # Let's compile the WebAssembly module.
/// module = Module(
///     store,
///     """
///     (module
///       (import "math" "sum" (func $sum (param i32 i32) (result i32)))
///       (func (export "add_one") (param i32) (result i32)
///         local.get 0
///         i32.const 1
///         call $sum))
///     """
/// )
///
/// # Now, let's create an import object, and register the `sum`
/// # function.
/// import_object = defaultdict(dict)
/// import_object["math"]["sum"] = Function(store, sum)
///
/// # Here we go, let's instantiate the module with the import object!
/// instance = Instance(module, import_object)
///
/// # Let's test it!
/// assert instance.exports.add_one(41) == 42
/// ```
#[pyclass(unsendable)]
#[pyo3(text_signature = "(module, import_object)")]
pub struct Instance {
    #[allow(unused)]
    inner: wasmer::Instance,

    /// The exports of the instance, as an object of kind `Exports`.
    ///
    /// ## Example
    ///
    /// See the `Exports` class.
    #[pyo3(get)]
    exports: Py<Exports>,
}

pub enum InstanceError {
    InstantiationError(wasmer::InstantiationError),
    PyErr(PyErr),
}

impl Instance {
    pub fn raw_new(
        py: Python,
        module: &Module,
        import_object: Option<&PyAny>,
    ) -> Result<Self, InstanceError> {
        let module = module.inner();

        let instance = match import_object {
            Some(import_object) => match import_object.downcast::<PyCell<ImportObject>>() {
                Ok(io) => wasmer::Instance::new(&module, io.borrow().inner()),
                Err(_e) => match import_object.downcast::<PyDict>() {
                    Ok(dict) => {
                        let io = ImportObject::from_pydict(dict).map_err(|e| InstanceError::PyErr(e.into()))?;
                        wasmer::Instance::new(&module, io.borrow().inner())
                    }
                    Err(e) => {
                        return Err(InstanceError::PyErr(e.into()));
                    }
                },
            },
            None => wasmer::Instance::new(&module, &wasmer::imports! {}),
        };
        let instance = instance.map_err(InstanceError::InstantiationError)?;

        let exports =
            Py::new(py, Exports::new(instance.exports.clone())).map_err(InstanceError::PyErr)?;

        Ok(Instance {
            inner: instance,
            exports,
        })
    }
}

#[pymethods]
impl Instance {
    #[new]
    fn new(py: Python, module: &Module, import_object: Option<&PyAny>) -> PyResult<Self> {
        Instance::raw_new(py, &module, import_object).map_err(|error| match error {
            InstanceError::InstantiationError(error) => to_py_err::<PyRuntimeError, _>(error),
            InstanceError::PyErr(error) => error,
        })
    }
}
