//! The `Value` Python class to build WebAssembly values.

use pyo3::prelude::*;
use std::slice;
use wasmer_runtime_core::types::Type as WasmType;

#[repr(u8)]
pub enum Type {
    I32,
    I64,
    F32,
    F64,
    V128,
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
        match self {
            Type::I32 => (Type::I32 as u8).into_py(py),
            Type::I64 => (Type::I64 as u8).into_py(py),
            Type::F32 => (Type::F32 as u8).into_py(py),
            Type::F64 => (Type::F64 as u8).into_py(py),
            Type::V128 => (Type::V128 as u8).into_py(py),
        }
    }
}
