use crate::wasmer_inner::wasmer;
use pyo3::{prelude::*, ToPyObject};
use std::slice;

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum ExternType {
    Function = 1,
    Global = 2,
    Table = 3,
    Memory = 4,
}

impl ExternType {
    pub fn iter() -> slice::Iter<'static, ExternType> {
        static VARIANTS: [ExternType; 4] = [
            ExternType::Function,
            ExternType::Global,
            ExternType::Table,
            ExternType::Memory,
        ];

        VARIANTS.iter()
    }
}

impl From<&ExternType> for &'static str {
    fn from(value: &ExternType) -> Self {
        match value {
            ExternType::Function => "FUNCTION",
            ExternType::Memory => "MEMORY",
            ExternType::Global => "GLOBAL",
            ExternType::Table => "TABLE",
        }
    }
}

impl From<&wasmer::ExternType> for ExternType {
    fn from(value: &wasmer::ExternType) -> Self {
        match value {
            wasmer::ExternType::Function(..) => ExternType::Function,
            wasmer::ExternType::Memory(..) => ExternType::Memory,
            wasmer::ExternType::Global(..) => ExternType::Global,
            wasmer::ExternType::Table(..) => ExternType::Table,
        }
    }
}

impl ToPyObject for ExternType {
    fn to_object(&self, py: Python) -> PyObject {
        (*self as u8).into_py(py)
    }
}
