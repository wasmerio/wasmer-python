use crate::{
    errors::to_py_err, externals::Memory, import_object::ImportObject, module::Module,
    store::Store, wasmer_inner::wasmer_wasi,
};
use pyo3::{
    exceptions::{RuntimeError, TypeError, ValueError},
    prelude::*,
    types::{PyDict, PyList},
};
use std::{path::PathBuf, slice};

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Version {
    Latest = 1,
    Snapshot0 = 2,
    Snapshot1 = 3,
}

impl Version {
    pub fn iter() -> slice::Iter<'static, Version> {
        static VARIANTS: [Version; 3] = [Version::Latest, Version::Snapshot0, Version::Snapshot1];

        VARIANTS.iter()
    }
}

impl From<&Version> for &'static str {
    fn from(value: &Version) -> Self {
        match value {
            Version::Latest => "LATEST",
            Version::Snapshot0 => "SNAPSHOT0",
            Version::Snapshot1 => "SNAPSHOT1",
        }
    }
}

impl ToPyObject for Version {
    fn to_object(&self, py: Python) -> PyObject {
        (*self as u8).into_py(py)
    }
}

impl IntoPy<PyObject> for Version {
    fn into_py(self, py: Python) -> PyObject {
        self.to_object(py)
    }
}

impl<'source> FromPyObject<'source> for Version {
    fn extract(obj: &'source PyAny) -> PyResult<Self> {
        let variant = u8::extract(obj)?;

        Ok(match variant {
            1 => Self::Latest,
            2 => Self::Snapshot0,
            3 => Self::Snapshot1,
            _ => {
                return Err(to_py_err::<ValueError, _>(
                    "Failed to extract `Version` from `PyAny`",
                ))
            }
        })
    }
}

impl From<wasmer_wasi::WasiVersion> for Version {
    fn from(value: wasmer_wasi::WasiVersion) -> Self {
        match value {
            wasmer_wasi::WasiVersion::Latest => Self::Latest,
            wasmer_wasi::WasiVersion::Snapshot0 => Self::Snapshot0,
            wasmer_wasi::WasiVersion::Snapshot1 => Self::Snapshot1,
        }
    }
}

impl Into<wasmer_wasi::WasiVersion> for Version {
    fn into(self) -> wasmer_wasi::WasiVersion {
        match self {
            Self::Latest => wasmer_wasi::WasiVersion::Latest,
            Self::Snapshot0 => wasmer_wasi::WasiVersion::Snapshot0,
            Self::Snapshot1 => wasmer_wasi::WasiVersion::Snapshot1,
        }
    }
}

#[pyclass]
#[text_signature = "(arguments=[], environments={}, preopen_directories=[], map_directories={})"]
pub struct StateBuilder {
    inner: wasmer_wasi::WasiStateBuilder,
}

impl StateBuilder {
    pub fn self_arguments(&mut self, arguments: &PyList) {
        self.inner.args(arguments.iter().map(ToString::to_string));
    }

    pub fn self_argument(&mut self, argument: String) {
        self.inner.arg(argument);
    }

    pub fn self_environments(&mut self, environments: &PyDict) {
        self.inner.envs(
            environments
                .iter()
                .map(|(any_key, any_value)| (any_key.to_string(), any_value.to_string())),
        );
    }

    pub fn self_environment(&mut self, key: String, value: String) {
        self.inner.env(key, value);
    }

    pub fn self_preopen_directories(&mut self, preopen_directories: &PyList) -> PyResult<()> {
        self.inner
            .preopen_dirs(
                preopen_directories
                    .iter()
                    .map(|any_item| PathBuf::from(any_item.to_string())),
            )
            .map_err(to_py_err::<RuntimeError, _>)?;

        Ok(())
    }

    pub fn self_preopen_directory(&mut self, preopen_directory: String) -> PyResult<()> {
        self.inner
            .preopen_dir(PathBuf::from(preopen_directory))
            .map_err(to_py_err::<RuntimeError, _>)?;

        Ok(())
    }

    pub fn self_map_directories(&mut self, map_directories: &PyDict) -> PyResult<()> {
        self.inner
            .map_dirs(map_directories.iter().map(|(any_key, any_value)| {
                (any_key.to_string(), PathBuf::from(any_value.to_string()))
            }))
            .map_err(to_py_err::<RuntimeError, _>)?;

        Ok(())
    }

    pub fn self_map_directory(&mut self, alias: String, directory: String) -> PyResult<()> {
        self.inner
            .map_dir(alias.as_str(), PathBuf::from(directory.to_string()))
            .map_err(to_py_err::<RuntimeError, _>)?;

        Ok(())
    }
}

#[pymethods]
impl StateBuilder {
    #[new]
    fn new(
        program_name: String,
        arguments: Option<&PyList>,
        environments: Option<&PyDict>,
        preopen_directories: Option<&PyList>,
        map_directories: Option<&PyDict>,
    ) -> PyResult<Self> {
        let mut wasi = Self {
            inner: wasmer_wasi::WasiState::new(program_name.as_str()),
        };

        if let Some(arguments) = arguments {
            wasi.self_arguments(arguments);
        }

        if let Some(environments) = environments {
            wasi.self_environments(environments);
        }

        if let Some(preopen_directories) = preopen_directories {
            wasi.self_preopen_directories(preopen_directories)?;
        }

        if let Some(map_directories) = map_directories {
            wasi.self_map_directories(map_directories)?;
        }

        Ok(wasi)
    }

    #[text_signature = "($self, arguments)"]
    pub fn arguments<'py>(
        slf: &'py PyCell<Self>,
        arguments: &PyList,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_arguments(arguments);

        Ok(slf)
    }

    #[text_signature = "($self, argument)"]
    pub fn argument<'py>(slf: &'py PyCell<Self>, argument: String) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_argument(argument);

        Ok(slf)
    }

    #[text_signature = "($self, environments)"]
    pub fn environments<'py>(
        slf: &'py PyCell<Self>,
        environments: &PyDict,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_environments(environments);

        Ok(slf)
    }

    #[text_signature = "($self, key, value)"]
    pub fn environment<'py>(
        slf: &'py PyCell<Self>,
        key: String,
        value: String,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_environment(key, value);

        Ok(slf)
    }

    #[text_signature = "($self, preopen_directories)"]
    pub fn preopen_directories<'py>(
        slf: &'py PyCell<Self>,
        preopen_directories: &PyList,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_preopen_directories(preopen_directories)?;

        Ok(slf)
    }

    #[text_signature = "($self, preopen_directory)"]
    pub fn preopen_directory<'py>(
        slf: &'py PyCell<Self>,
        preopen_directory: String,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_preopen_directory(preopen_directory)?;

        Ok(slf)
    }

    #[text_signature = "($self, map_directories)"]
    pub fn map_directories<'py>(
        slf: &'py PyCell<Self>,
        map_directories: &PyDict,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_map_directories(map_directories)?;

        Ok(slf)
    }

    #[text_signature = "($self, alias, directory)"]
    pub fn map_directory<'py>(
        slf: &'py PyCell<Self>,
        alias: String,
        directory: String,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_map_directory(alias, directory)?;

        Ok(slf)
    }

    #[text_signature = "($self)"]
    pub fn finalize(&mut self) -> PyResult<Environment> {
        Ok(Environment::raw_new(
            self.inner
                .finalize()
                .map_err(to_py_err::<RuntimeError, _>)?,
        ))
    }
}

#[pyclass(unsendable)]
pub struct Environment {
    inner: wasmer_wasi::WasiEnv,
}

impl Environment {
    fn raw_new(inner: wasmer_wasi::WasiEnv) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Environment {
    #[setter]
    fn memory(&mut self, memory: &PyAny) -> PyResult<()> {
        match memory.downcast::<PyCell<Memory>>() {
            Ok(memory) => {
                let memory = memory.borrow();

                self.inner.set_memory(memory.inner().clone());

                Ok(())
            }

            _ => Err(to_py_err::<TypeError, _>(
                "Can only set a `Memory` object to `Environment.memory`",
            )),
        }
    }

    fn generate_import_object(&self, store: &Store, wasi_version: Version) -> ImportObject {
        ImportObject::raw_new(wasmer_wasi::generate_import_object_from_env(
            store.inner(),
            self.inner.clone(),
            wasi_version.into(),
        ))
    }
}

pub fn get_version(module: &Module, strict: bool) -> Option<Version> {
    wasmer_wasi::get_wasi_version(module.inner(), strict).map(Into::into)
}
