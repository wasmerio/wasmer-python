use crate::{types::TableType, wasmer_inner::wasmer};
use pyo3::prelude::*;

#[pyclass(unsendable)]
pub struct Table {
    inner: wasmer::Table,
}

impl Table {
    pub fn raw_new(inner: wasmer::Table) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Table {
    #[getter]
    fn size(&self) -> u32 {
        self.inner.size()
    }

    #[getter(type)]
    fn ty(&self) -> TableType {
        self.inner.ty().into()
    }
}
