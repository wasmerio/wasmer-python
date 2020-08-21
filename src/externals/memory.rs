use crate::{
    errors::to_py_err,
    memory::{Buffer, Int16Array, Int32Array, Int8Array, Uint16Array, Uint32Array, Uint8Array},
    types::MemoryType,
    wasmer_inner::wasmer,
};
use pyo3::{exceptions::RuntimeError, prelude::*};

#[pyclass(unsendable)]
pub struct Memory {
    inner: wasmer::Memory,
}

impl Memory {
    pub fn new(inner: wasmer::Memory) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Memory {
    #[getter]
    fn size(&self) -> u32 {
        self.inner.size().0
    }

    #[getter]
    fn data_size(&self) -> usize {
        self.inner.data_size()
    }

    #[text_signature = "($self, number_of_pages)"]
    fn grow(&self, number_of_pages: u32) -> PyResult<u32> {
        self.inner
            .grow(number_of_pages)
            .map(|pages| pages.0)
            .map_err(to_py_err::<RuntimeError, _>)
    }

    #[getter]
    fn buffer(&self) -> Buffer {
        Buffer::new(self.inner.clone())
    }

    #[text_signature = "($self, offset=0)"]
    #[args(offset = 0)]
    fn uint8_view(&self, offset: usize) -> Uint8Array {
        Uint8Array {
            memory: self.inner.clone(),
            offset,
        }
    }

    #[text_signature = "($self, offset=0)"]
    #[args(offset = 0)]
    fn int8_view(&self, offset: usize) -> Int8Array {
        Int8Array {
            memory: self.inner.clone(),
            offset,
        }
    }

    #[text_signature = "($self, offset=0)"]
    #[args(offset = 0)]
    fn uint16_view(&self, offset: usize) -> Uint16Array {
        Uint16Array {
            memory: self.inner.clone(),
            offset,
        }
    }

    #[text_signature = "($self, offset=0)"]
    #[args(offset = 0)]
    fn int16_view(&self, offset: usize) -> Int16Array {
        Int16Array {
            memory: self.inner.clone(),
            offset,
        }
    }

    #[text_signature = "($self, offset=0)"]
    #[args(offset = 0)]
    fn uint32_view(&self, offset: usize) -> Uint32Array {
        Uint32Array {
            memory: self.inner.clone(),
            offset,
        }
    }

    #[text_signature = "($self, offset=0)"]
    #[args(offset = 0)]
    fn int32_view(&self, offset: usize) -> Int32Array {
        Int32Array {
            memory: self.inner.clone(),
            offset,
        }
    }

    #[getter(type)]
    fn ty(&self) -> MemoryType {
        self.inner.ty().into()
    }
}
