use crate::{errors::to_py_err, store::Store, types, wasmer_inner::wasmer};
use pyo3::{
    exceptions::{RuntimeError, TypeError},
    prelude::*,
    types::{PyAny, PyBytes, PyList, PyString},
};
use std::{convert::TryInto, sync::Arc};

#[pyclass(unsendable)]
#[text_signature = "(store, bytes)"]
pub struct Module {
    store: Arc<wasmer::Store>,
    inner: Arc<wasmer::Module>,
}

impl Module {
    pub fn store(&self) -> Arc<wasmer::Store> {
        Arc::clone(&self.store)
    }

    pub fn inner(&self) -> Arc<wasmer::Module> {
        Arc::clone(&self.inner)
    }
}

#[pymethods]
impl Module {
    #[text_signature = "(bytes)"]
    #[staticmethod]
    fn validate(store: &Store, bytes: &PyAny) -> bool {
        match bytes.downcast::<PyBytes>() {
            Ok(bytes) => wasmer::Module::validate(&store.inner(), bytes.as_bytes()).is_ok(),
            _ => false,
        }
    }

    #[new]
    fn new(store: &Store, bytes: &PyAny) -> PyResult<Self> {
        let store = store.inner();

        // Read the bytes as if there were real bytes or a WAT string.
        let module = if let Ok(bytes) = bytes.downcast::<PyBytes>() {
            wasmer::Module::new(&store, bytes.as_bytes())
        } else if let Ok(string) = bytes.downcast::<PyString>() {
            wasmer::Module::new(&store, string.to_string()?.as_bytes())
        } else {
            return Err(to_py_err::<TypeError, _>(
                "`Module` accepts Wasm bytes or a WAT string",
            ));
        };

        Ok(Module {
            store,
            inner: Arc::new(module.map_err(to_py_err::<RuntimeError, _>)?),
        })
    }

    #[getter]
    fn name(&self) -> Option<&str> {
        self.inner.name()
    }

    #[setter(name)]
    fn set_name(&mut self, name: &str) -> PyResult<()> {
        Arc::get_mut(&mut self.inner)
            .ok_or_else(|| to_py_err::<RuntimeError, _>("Value already shared with mutability"))?
            .set_name(name);

        Ok(())
    }

    #[getter]
    fn exports(&self) -> PyResult<Vec<types::ExportType>> {
        self.inner.exports().map(TryInto::try_into).collect()
    }

    #[getter]
    fn imports(&self) -> PyResult<Vec<types::ImportType>> {
        self.inner.imports().map(TryInto::try_into).collect()
    }

    #[text_signature = "($self, name)"]
    fn custom_sections<'p>(&self, py: Python<'p>, name: &str) -> &'p PyList {
        PyList::new(
            py,
            self.inner
                .custom_sections(name)
                .map(|custom_section| PyBytes::new(py, &*custom_section))
                .collect::<Vec<_>>(),
        )
    }

    #[text_signature = "($self)"]
    fn serialize<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(
            py,
            self.inner
                .serialize()
                .map_err(to_py_err::<RuntimeError, _>)?
                .as_slice(),
        ))
    }

    #[text_signature = "($self, bytes)"]
    #[staticmethod]
    fn deserialize(store: &Store, bytes: &PyBytes) -> PyResult<Self> {
        let store = store.inner();

        Ok(Module {
            store: Arc::clone(&store),
            inner: Arc::new(
                unsafe { wasmer::Module::deserialize(&Arc::clone(&store), bytes.as_bytes()) }
                    .map_err(to_py_err::<RuntimeError, _>)?,
            ),
        })
    }
}
