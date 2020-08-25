use crate::{errors::to_py_err, store::Store, types, wasmer_inner::wasmer};
use pyo3::{
    exceptions::{RuntimeError, TypeError},
    prelude::*,
    types::{PyAny, PyBytes, PyList, PyString},
};
use std::convert::TryInto;

#[pyclass(unsendable)]
#[text_signature = "(store, bytes)"]
pub struct Module {
    inner: wasmer::Module,
}

impl Module {
    pub fn store(&self) -> &wasmer::Store {
        self.inner.store()
    }

    pub fn inner(&self) -> &wasmer::Module {
        &self.inner
    }
}

#[pymethods]
impl Module {
    #[text_signature = "(bytes)"]
    #[staticmethod]
    fn validate(store: &Store, bytes: &PyAny) -> bool {
        match bytes.downcast::<PyBytes>() {
            Ok(bytes) => wasmer::Module::validate(store.inner(), bytes.as_bytes()).is_ok(),
            _ => false,
        }
    }

    #[new]
    fn new(store: &Store, bytes: &PyAny) -> PyResult<Self> {
        // Read the bytes as if there were real bytes or a WAT string.
        <PyBytes as PyTryFrom>::try_from(bytes)
            .map(|bytes| bytes.as_bytes())
            .or_else(|_| {
                <PyString as PyTryFrom>::try_from(bytes)
                    .map_err(|_| {
                        to_py_err::<TypeError, _>("`Module` accepts Wasm bytes or a WAT string")
                    })
                    .and_then(|string| string.as_bytes())
            })
            .and_then(|bytes| {
                Ok(Module {
                    inner: wasmer::Module::new(store.inner(), bytes)
                        .map_err(to_py_err::<RuntimeError, _>)?,
                })
            })
    }

    #[getter]
    fn name(&self) -> Option<&str> {
        self.inner.name()
    }

    #[setter(name)]
    fn set_name(&mut self, name: &str) -> PyResult<()> {
        self.inner.set_name(name);

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
        Ok(Module {
            inner: unsafe { wasmer::Module::deserialize(store.inner(), bytes.as_bytes()) }
                .map_err(to_py_err::<RuntimeError, _>)?,
        })
    }
}
