//! The `ExportedGlobals` and `ExportedGlobal` objects.

use pyo3::{
    class::basic::PyObjectProtocol,
    exceptions::{LookupError, RuntimeError},
    prelude::*,
    types::{PyAny, PyFloat, PyLong},
};
use std::rc::Rc;
use wasmer_runtime::{types::Type, Value as WasmValue};
use wasmer_runtime_core::global::Global;

#[pyclass]
/// `ExportedGlobal` is a Python class that represents a WebAssembly
/// exported global variable. Such a variable can be read and write
/// with the `value` getter and setter.
pub struct ExportedGlobal {
    /// The exported global name from the WebAssembly instance.
    global_name: String,

    /// The exported global from the WebAssembly instance.
    global: Rc<Global>,
}

#[pymethods]
/// Implement methods on the `ExportedGlobal` Python class.
impl ExportedGlobal {
    #[getter(value)]
    fn get_value(&self, py: Python) -> PyObject {
        match self.global.get() {
            WasmValue::I32(result) => result.to_object(py),
            WasmValue::I64(result) => result.to_object(py),
            WasmValue::F32(result) => result.to_object(py),
            WasmValue::F64(result) => result.to_object(py),
            WasmValue::V128(result) => result.to_object(py),
        }
    }

    #[setter(value)]
    fn set_value(&self, value: &PyAny) -> PyResult<()> {
        let descriptor = self.global.descriptor();

        if !descriptor.mutable {
            return Err(RuntimeError::py_err(format!(
                "The global variable `{}` is not mutable, cannot set a new value.",
                self.global_name
            )));
        }

        self.global.set(match descriptor.ty {
            Type::I32 => WasmValue::I32(value.downcast::<PyLong>()?.extract::<i32>()?),
            Type::I64 => WasmValue::I64(value.downcast::<PyLong>()?.extract::<i64>()?),
            Type::F32 => WasmValue::F32(value.downcast::<PyFloat>()?.extract::<f32>()?),
            Type::F64 => WasmValue::F64(value.downcast::<PyFloat>()?.extract::<f64>()?),
            Type::V128 => WasmValue::V128(value.downcast::<PyLong>()?.extract::<u128>()?),
        });

        Ok(())
    }

    #[getter]
    fn mutable(&self) -> bool {
        self.global.descriptor().mutable
    }
}

#[pyclass]
/// `ExportedGlobals` is a Python class that represents the set
/// of WebAssembly exported globals. It's basically a set of
/// `ExportedGlobal` classes.
///
/// # Examples
///
/// ```python
/// from wasmer import Instance
///
/// instance = Instance(wasm_bytes)
///
/// assert instance.globals.var1.value == 42
/// ```
pub struct ExportedGlobals {
    /// Available exported globals names from the WebAssembly module.
    pub(crate) globals: Vec<(String, Rc<Global>)>,
}

#[pyproto]
/// Implement the Python object protocol on the `ExportedGlobals`
/// Python class.
impl PyObjectProtocol for ExportedGlobals {
    /// A Python attribute in this context represents a WebAssembly
    /// exported global name.
    fn __getattr__(&self, key: String) -> PyResult<ExportedGlobal> {
        match self.globals.iter().find(|(name, _)| name == &key) {
            Some((global_name, global)) => Ok(ExportedGlobal {
                global_name: global_name.clone(),
                global: global.clone(),
            }),
            None => Err(LookupError::py_err(format!(
                "Global `{}` does not exist.",
                key
            ))),
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.globals))
    }
}
