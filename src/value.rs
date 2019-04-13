//! The `Value` Python class to build WebAssembly values.

use pyo3::{class::basic::PyObjectProtocol, prelude::*};
use wasmer_runtime::Value as WasmValue;

#[pyclass]
/// The `Value` class represents a WebAssembly value.
pub struct Value {
    pub value: WasmValue,
}

#[pymethods]
impl Value {
    #[staticmethod]
    fn i32(value: i32) -> PyResult<Self> {
        Ok(Self {
            value: WasmValue::I32(value),
        })
    }

    #[staticmethod]
    fn i64(value: i64) -> PyResult<Self> {
        Ok(Self {
            value: WasmValue::I64(value),
        })
    }

    #[staticmethod]
    fn f32(value: f32) -> PyResult<Self> {
        Ok(Self {
            value: WasmValue::F32(value),
        })
    }

    #[staticmethod]
    fn f64(value: f64) -> PyResult<Self> {
        Ok(Self {
            value: WasmValue::F64(value),
        })
    }
}

#[pyproto]
impl PyObjectProtocol for Value {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.value))
    }
}
