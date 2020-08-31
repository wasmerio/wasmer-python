use crate::{
    errors::to_py_err, store::Store, types::TableType, values::Value, wasmer_inner::wasmer,
};
use pyo3::{exceptions::RuntimeError, prelude::*};

#[pyclass(unsendable)]
pub struct Table {
    inner: wasmer::Table,
}

impl Table {
    pub fn raw_new(inner: wasmer::Table) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &wasmer::Table {
        &self.inner
    }
}

#[pymethods]
impl Table {
    #[new]
    fn new(store: &Store, table_type: &TableType, initial_value: &Value) -> PyResult<Self> {
        Ok(Self {
            inner: wasmer::Table::new(
                store.inner(),
                table_type.into(),
                initial_value.inner().clone(),
            )
            .map_err(to_py_err::<RuntimeError, _>)?,
        })
    }

    #[getter]
    fn size(&self) -> u32 {
        self.inner.size()
    }

    #[getter(type)]
    fn ty(&self) -> TableType {
        self.inner.ty().into()
    }
}
