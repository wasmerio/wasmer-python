use crate::{store::Store, types, wasmer_inner::wasmer};
use pyo3::{
    exceptions::RuntimeError,
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
    fn store(&self) -> &wasmer::Store {
        self.inner.store()
    }
}

#[pymethods]
impl Module {
    #[text_signature = "(bytes)"]
    #[staticmethod]
    fn validate(store: &Store, bytes: &PyAny) -> PyResult<bool> {
        match <PyBytes as PyTryFrom>::try_from(bytes) {
            Ok(bytes) => Ok(wasmer::Module::validate(store.inner(), bytes.as_bytes()).is_ok()),
            _ => Ok(false),
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
                        RuntimeError::py_err(
                            "`Module` accepts Wasm bytes or a WAT string".to_string(),
                        )
                    })
                    .and_then(|string| string.as_bytes())
            })
            .and_then(|bytes| {
                Ok(Module {
                    inner: wasmer::Module::new(store.inner(), bytes).map_err(|error| {
                        RuntimeError::py_err(format!("Failed to compile the module: {}", error))
                    })?,
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

    fn custom_sections<'p>(&self, py: Python<'p>, name: &str) -> &'p PyList {
        PyList::new(
            py,
            self.inner
                .custom_sections(name)
                .map(|custom_section| PyBytes::new(py, &*custom_section))
                .collect::<Vec<_>>(),
        )
    }
}
