use crate::{errors::to_py_err, types::MemoryType, wasmer_inner::wasmer};
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

    #[getter(type)]
    fn ty(&self) -> MemoryType {
        self.inner.ty().into()
    }
}
