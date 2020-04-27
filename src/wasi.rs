use pyo3::prelude::*;
use std::slice;
use wasmer_wasi::WasiVersion;

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

impl From<WasiVersion> for Version {
    fn from(value: WasiVersion) -> Self {
        match value {
            WasiVersion::Snapshot0 => Version::Snapshot0,
            WasiVersion::Snapshot1 => Version::Snapshot1,
            WasiVersion::Latest => Version::Latest,
        }
    }
}

impl ToPyObject for Version {
    fn to_object(&self, py: Python) -> PyObject {
        (*self as u8).into_py(py)
    }
}
