use crate::wasmer_inner::wasmer;
use pyo3::prelude::*;

#[pyclass]
pub struct Store {
    inner: wasmer::Store,
}

impl Store {
    pub(crate) fn inner(&self) -> &wasmer::Store {
        &self.inner
    }
}

#[pymethods]
impl Store {
    #[new]
    fn new() -> Self {
        Store {
            inner: wasmer::Store::default(),
        }
    }
}
