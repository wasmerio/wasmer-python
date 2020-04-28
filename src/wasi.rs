use crate::import::ImportObject;
use pyo3::prelude::*;
use std::{convert::TryFrom, path::PathBuf, rc::Rc, slice};
use wasmer_runtime as runtime;
use wasmer_wasi::{generate_import_object_for_version, WasiVersion};

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

impl ImportObject {
    pub fn new_wasi(
        module: Rc<runtime::Module>,
        version: Version,
        args: Vec<Vec<u8>>,
        envs: Vec<Vec<u8>>,
        preopened_files: Vec<PathBuf>,
        mapped_dirs: Vec<(String, PathBuf)>,
    ) -> Self {
        Self {
            inner: generate_import_object_for_version(
                version.into(),
                args,
                envs,
                preopened_files,
                mapped_dirs,
            ),
            module,
            host_function_references: Vec::new(),
        }
    }
}
