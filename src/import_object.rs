use pyo3::prelude::*;

#[pyclass(unsendable)]
pub struct ImportObject {
    inner: wasmer::ImportObject,
}

#[pymethods]
impl ImportObject {
    #[new]
    fn new() -> Self {
        ImportObject {
            inner: Default::default(),
        }
    }

    #[text_signature = "($self, namespace)"]
    fn contains_namespace(&self, namespace: &str) -> bool {
        self.inner.contains_namespace(namespace)
    }
}
