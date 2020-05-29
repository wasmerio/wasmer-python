use pyo3::{
    exceptions::RuntimeError,
    prelude::*,
    pycell::PyCell,
    types::{PyDict, PyList},
};
use std::{convert::TryFrom, path::PathBuf, slice};
use wasmer_wasi::{state, WasiVersion};

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Version {
    Snapshot0 = 1,
    Snapshot1 = 2,
    Latest = 3,
}

impl Version {
    pub fn iter() -> slice::Iter<'static, Version> {
        static VARIANTS: [Version; 3] = [Version::Snapshot0, Version::Snapshot1, Version::Latest];

        VARIANTS.iter()
    }
}

impl From<&Version> for &'static str {
    fn from(value: &Version) -> Self {
        match value {
            Version::Snapshot0 => "Snapshot0",
            Version::Snapshot1 => "Snapshot1",
            Version::Latest => "Latest",
        }
    }
}

impl TryFrom<u8> for Version {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Version::Snapshot0),
            2 => Ok(Version::Snapshot1),
            3 => Ok(Version::Latest),
            e => Err(format!("Unknown WASI version `{}`", e)),
        }
    }
}

impl From<WasiVersion> for Version {
    fn from(value: WasiVersion) -> Self {
        match value {
            WasiVersion::Snapshot0 => Version::Snapshot0,
            WasiVersion::Snapshot1 => Version::Snapshot1,
            WasiVersion::Latest => Version::Latest,
        }
    }
}

impl From<Version> for WasiVersion {
    fn from(value: Version) -> Self {
        match value {
            Version::Snapshot0 => WasiVersion::Snapshot0,
            Version::Snapshot1 => WasiVersion::Snapshot1,
            Version::Latest => WasiVersion::Latest,
        }
    }
}

impl ToPyObject for Version {
    fn to_object(&self, py: Python) -> PyObject {
        (*self as u8).into_py(py)
    }
}

#[pyclass]
pub struct WasiStateBuilder {
    pub(crate) inner: state::WasiStateBuilder,
}

#[pymethods]
impl WasiStateBuilder {
    #[new]
    fn new(program_name: String) -> Self {
        Self {
            inner: state::WasiState::new(program_name.as_str()),
        }
    }

    pub fn environments<'py>(
        slf: &'py PyCell<Self>,
        environments: &PyDict,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;

        slf_mut.inner.envs(
            environments
                .iter()
                .map(|(any_key, any_value)| (any_key.to_string(), any_value.to_string())),
        );

        Ok(slf)
    }

    pub fn environment<'py>(
        slf: &'py PyCell<Self>,
        key: String,
        value: String,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;

        slf_mut.inner.env(key, value);

        Ok(slf)
    }

    pub fn arguments<'py>(
        slf: &'py PyCell<Self>,
        arguments: &PyList,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;

        slf_mut
            .inner
            .args(arguments.iter().map(|any_item| any_item.to_string()));

        Ok(slf)
    }

    pub fn argument<'py>(slf: &'py PyCell<Self>, argument: String) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;

        slf_mut.inner.arg(argument);

        Ok(slf)
    }

    pub fn preopen_directories<'py>(
        slf: &'py PyCell<Self>,
        preopened_directories: &PyList,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;

        slf_mut
            .inner
            .preopen_dirs(
                preopened_directories
                    .iter()
                    .map(|any_item| PathBuf::from(any_item.to_string())),
            )
            .map_err(|error| {
                RuntimeError::py_err(format!(
                    "Failed to configure preopened directories when creating the WASI state: {}",
                    error
                ))
            })?;

        Ok(slf)
    }

    pub fn preopen_directory<'py>(
        slf: &'py PyCell<Self>,
        preopened_directory: String,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;

        slf_mut
            .inner
            .preopen_dir(PathBuf::from(preopened_directory))
            .map_err(|error| {
                RuntimeError::py_err(format!(
                    "Failed to configure the preopened directory when creating the WASI state: {}",
                    error
                ))
            })?;

        Ok(slf)
    }

    pub fn map_directories<'py>(
        slf: &'py PyCell<Self>,
        map_directories: &PyDict,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;

        slf_mut
            .inner
            .map_dirs(map_directories.iter().map(|(any_key, any_value)| {
                (any_key.to_string(), PathBuf::from(any_value.to_string()))
            }))
            .map_err(|error| {
                RuntimeError::py_err(format!(
                    "Failed to configure map directories when creating the WASI state: {}",
                    error
                ))
            })?;

        Ok(slf)
    }

    pub fn map_directory<'py>(
        slf: &'py PyCell<Self>,
        alias: String,
        directory: String,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;

        slf_mut
            .inner
            .map_dir(alias.as_str(), PathBuf::from(directory.to_string()))
            .map_err(|error| {
                RuntimeError::py_err(format!(
                    "Failed to configure the map directory when creating the WASI state: {}",
                    error
                ))
            })?;

        Ok(slf)
    }
}
