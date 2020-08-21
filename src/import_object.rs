use crate::externals::{Function, Global, Memory, Table};
use pyo3::{
    prelude::*,
    types::{PyDict, PyString},
};

#[pyclass(unsendable)]
pub struct ImportObject {
    inner: wasmer::ImportObject,
}

impl ImportObject {
    pub fn raw_new(inner: wasmer::ImportObject) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &wasmer::ImportObject {
        &self.inner
    }
}

#[pymethods]
impl ImportObject {
    #[new]
    fn new() -> Self {
        ImportObject {
            inner: Default::default(),
        }
    }

    #[text_signature = "($self, namespace_name)"]
    fn contains_namespace(&self, namespace_name: &str) -> bool {
        self.inner.contains_namespace(namespace_name)
    }

    #[text_signature = "($self, namespace_name, namespace)"]
    fn register(&mut self, namespace_name: &str, namespace: &PyDict) -> PyResult<()> {
        let mut wasmer_namespace = wasmer::Exports::new();

        for (name, item) in namespace.into_iter() {
            let name = name
                .downcast::<PyString>()
                .map_err(PyErr::from)?
                .to_string()?;

            if let Ok(function) = item.downcast::<PyCell<Function>>() {
                let function = function.borrow();

                wasmer_namespace.insert(name, function.inner().clone());
            } else if let Ok(memory) = item.downcast::<PyCell<Memory>>() {
                let memory = memory.borrow();

                wasmer_namespace.insert(name, memory.inner().clone());
            } else if let Ok(global) = item.downcast::<PyCell<Global>>() {
                let global = global.borrow();

                wasmer_namespace.insert(name, global.inner().clone());
            } else if let Ok(table) = item.downcast::<PyCell<Table>>() {
                let table = table.borrow();

                wasmer_namespace.insert(name, table.inner().clone());
            } else {
                unimplemented!("import object does not support the given type");
            }
        }

        self.inner.register(namespace_name, wasmer_namespace);

        Ok(())
    }
}
