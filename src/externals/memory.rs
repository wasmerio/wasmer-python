use crate::{
    errors::to_py_err,
    memory::{Buffer, Int16Array, Int32Array, Int8Array, Uint16Array, Uint32Array, Uint8Array},
    store::Store,
    types::MemoryType,
    wasmer_inner::wasmer,
};
use pyo3::{exceptions::RuntimeError, prelude::*};

#[pyclass(unsendable)]
pub struct Memory {
    inner: wasmer::Memory,
}

impl Memory {
    pub fn raw_new(inner: wasmer::Memory) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &wasmer::Memory {
        &self.inner
    }
}

#[pymethods]
impl Memory {
    #[new]
    fn new(store: &Store, memory_type: &MemoryType) -> PyResult<Self> {
        Ok(Self::raw_new(
            wasmer::Memory::new(&store.cloned_inner(), memory_type.into())
                .map_err(to_py_err::<RuntimeError, _>)?,
        ))
    }

    #[getter]
    fn size(&self) -> u32 {
        self.inner.size().0
    }

    #[getter]
    fn data_size(&self) -> u64 {
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
