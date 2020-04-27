//! The `Value` Python class to build WebAssembly values.

use pyo3::prelude::*;
use std::slice;
use wasmer_runtime_core::types::Type as WasmType;

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Type {
    I32 = 1,
    I64 = 2,
    F32 = 3,
    F64 = 4,
    V128 = 5,
}

impl Type {
    pub fn iter() -> slice::Iter<'static, Type> {
        static VARIANTS: [Type; 5] = [Type::I32, Type::I64, Type::F32, Type::F64, Type::V128];

        VARIANTS.iter()
    }
}

impl From<&Type> for &'static str {
    fn from(value: &Type) -> Self {
        match value {
            Type::I32 => "I32",
            Type::I64 => "I64",
            Type::F32 => "F32",
            Type::F64 => "F64",
            Type::V128 => "V128",
        }
    }
}

impl From<&WasmType> for Type {
    fn from(ty: &WasmType) -> Self {
        match ty {
            WasmType::I32 => Type::I32,
            WasmType::I64 => Type::I64,
            WasmType::F32 => Type::F32,
            WasmType::F64 => Type::F64,
            WasmType::V128 => Type::V128,
        }
    }
}

impl ToPyObject for Type {
    fn to_object(&self, py: Python) -> PyObject {
        (*self as u8).into_py(py)
    }
}
