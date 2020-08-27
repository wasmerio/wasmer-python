use crate::wasmer_inner::wasmer;
use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass]
pub struct Store {
    inner: Arc<wasmer::Store>,
}

impl Store {
    pub(crate) fn inner(&self) -> Arc<wasmer::Store> {
        Arc::clone(&self.inner)
    }
}

#[pymethods]
impl Store {
    #[new]
    fn new() -> Self {
        Store {
            inner: Arc::new(wasmer::Store::default()),
        }
    }
}
