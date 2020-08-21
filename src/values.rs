use crate::wasmer_inner::{wasmer, wasmer_types::NativeWasmType};
use pyo3::{
    class::basic::PyObjectProtocol,
    prelude::*,
    types::{PyFloat, PyLong},
};

pub trait NativeFromPyAny {
    type Native;

    fn from_pyany(any: &PyAny) -> PyResult<Self::Native>;
}

impl NativeFromPyAny for i32 {
    type Native = Self;

    fn from_pyany(any: &PyAny) -> PyResult<Self::Native> {
        any.downcast::<PyLong>()?.extract::<Self::Native>()
    }
}

impl NativeFromPyAny for i64 {
    type Native = Self;

    fn from_pyany(any: &PyAny) -> PyResult<Self::Native> {
        any.downcast::<PyLong>()?.extract::<Self::Native>()
    }
}

impl NativeFromPyAny for f32 {
    type Native = Self;

    fn from_pyany(any: &PyAny) -> PyResult<Self::Native> {
        any.downcast::<PyFloat>()?.extract::<Self::Native>()
    }
}

impl NativeFromPyAny for f64 {
    type Native = Self;

    fn from_pyany(any: &PyAny) -> PyResult<Self::Native> {
        any.downcast::<PyFloat>()?.extract::<Self::Native>()
    }
}

impl NativeFromPyAny for u128 {
    type Native = Self;

    fn from_pyany(any: &PyAny) -> PyResult<Self::Native> {
        any.downcast::<PyLong>()?.extract::<Self::Native>()
    }
}

pub trait TryFromPyAny {
    fn try_from<N>(&self) -> PyResult<N::Native>
    where
        N: NativeFromPyAny;
}

impl TryFromPyAny for PyAny {
    fn try_from<N>(&self) -> PyResult<N::Native>
    where
        N: NativeFromPyAny,
    {
        N::from_pyany(&self)
    }
}

pub(crate) fn to_wasm_value((any, ty): (&PyAny, wasmer::Type)) -> PyResult<wasmer::Value> {
    Ok(match ty {
        wasmer::Type::I32 => any.try_from::<i32>()?.to_value(),
        wasmer::Type::I64 => any.try_from::<i64>()?.to_value(),
        wasmer::Type::F32 => any.try_from::<f32>()?.to_value(),
        wasmer::Type::F64 => any.try_from::<f64>()?.to_value(),
        wasmer::Type::V128 => any.try_from::<u128>()?.to_value(),
        _ => unimplemented!(),
    })
}

pub(crate) fn to_py_object<'p>(py: Python<'p>) -> impl Fn(&wasmer::Value) -> PyObject + 'p {
    move |value: &wasmer::Value| -> PyObject {
        match value {
            wasmer::Value::I32(value) => value.to_object(py),
            wasmer::Value::I64(value) => value.to_object(py),
            wasmer::Value::F32(value) => value.to_object(py),
            wasmer::Value::F64(value) => value.to_object(py),
            wasmer::Value::V128(value) => value.to_object(py),
            _ => unimplemented!(),
        }
    }
}

#[pyclass(unsendable)]
pub struct Value {
    inner: wasmer::Value,
}

impl Value {
    pub(crate) fn inner(&self) -> &wasmer::Value {
        &self.inner
    }
}

#[pymethods]
impl Value {
    #[staticmethod]
    #[text_signature = "(value)"]
    fn i32(value: i32) -> Self {
        Self {
            inner: wasmer::Value::I32(value),
        }
    }

    #[staticmethod]
    #[text_signature = "(value)"]
    fn i64(value: i64) -> Self {
        Self {
            inner: wasmer::Value::I64(value),
        }
    }

    #[staticmethod]
    #[text_signature = "(value)"]
    fn f32(value: f32) -> Self {
        Self {
            inner: wasmer::Value::F32(value),
        }
    }

    /// Build a WebAssembly `f64` value.
    #[staticmethod]
    #[text_signature = "(value)"]
    fn f64(value: f64) -> Self {
        Self {
            inner: wasmer::Value::F64(value),
        }
    }

    /// Build a WebAssembly `v128` value.
    #[staticmethod]
    #[text_signature = "(value)"]
    fn v128(value: u128) -> Self {
        Self {
            inner: wasmer::Value::V128(value),
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Value {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner()))
    }
}
