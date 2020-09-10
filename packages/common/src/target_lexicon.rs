use crate::errors::to_py_err;
use enumset::EnumSet;
use pyo3::{exceptions::ValueError, prelude::*};
use std::str::FromStr;

#[pyclass]
pub struct Target {
    inner: wasmer_compiler::Target,
}

impl Target {
    pub(crate) fn inner(&self) -> &wasmer_compiler::Target {
        &self.inner
    }
}

#[pymethods]
impl Target {
    #[new]
    fn new(triple: &Triple, cpu_features: Option<&CpuFeatures>) -> Self {
        Self {
            inner: wasmer_compiler::Target::new(
                triple.inner().clone(),
                cpu_features.map_or_else(
                    || wasmer_compiler::CpuFeature::set(),
                    |cpu_features| cpu_features.inner().clone(),
                ),
            ),
        }
    }
}

/// A target “triple”.
///
/// Historically such things had three fields, though they have added
/// additional fields over time.
#[pyclass]
#[text_signature = "(triple)"]
pub struct Triple {
    inner: wasmer_compiler::Triple,
}

impl Triple {
    pub(crate) fn inner(&self) -> &wasmer_compiler::Triple {
        &self.inner
    }
}

#[pymethods]
impl Triple {
    #[new]
    fn new(triple: &str) -> PyResult<Self> {
        Ok(Self {
            inner: wasmer_compiler::Triple::from_str(triple).map_err(to_py_err::<ValueError, _>)?,
        })
    }

    /// Build the triple for the current host.
    #[staticmethod]
    fn host() -> Self {
        Self {
            inner: wasmer_compiler::Triple::host(),
        }
    }

    /// Returns the “architecture” (and sometimes the subarchitecture).
    #[getter]
    fn architecture(&self) -> String {
        self.inner.architecture.to_string()
    }

    /// Returns the “vendor” (whatever that means).
    #[getter]
    fn vendor(&self) -> String {
        self.inner.vendor.to_string()
    }

    /// Returns the "operating system” (sometimes also the environment).
    #[getter]
    fn operating_system(&self) -> String {
        self.inner.operating_system.to_string()
    }

    /// Returns the “binary format” (rarely used).
    #[getter]
    fn binary_format(&self) -> String {
        self.inner.binary_format.to_string()
    }

    /// Returns the “environment” on top of the operating system
    /// (often omitted for operating systems with a single predominant
    /// environment).
    #[getter]
    fn environment(&self) -> String {
        self.inner.environment.to_string()
    }

    /// Returns the endianness of this target's architecture.
    #[getter]
    fn endianness(&self) -> Option<&'static str> {
        self.inner
            .endianness()
            .ok()
            .map(|endianness| match endianness {
                wasmer_compiler::Endianness::Little => "little",
                wasmer_compiler::Endianness::Big => "big",
            })
    }

    /// Returns the pointer width (in bytes) of this target's
    /// architecture.
    #[getter]
    fn pointer_width(&self) -> Option<u8> {
        self.inner
            .pointer_width()
            .ok()
            .map(wasmer_compiler::PointerWidth::bytes)
    }

    /// Returns the default calling convention for the given target
    /// triple.
    #[getter]
    fn default_calling_convention(&self) -> Option<&'static str> {
        self.inner
            .default_calling_convention()
            .ok()
            .map(|convention| match convention {
                wasmer_compiler::CallingConvention::SystemV => "system_v",
                wasmer_compiler::CallingConvention::WasmBasicCAbi => "wasm_basic_c_abi",
                wasmer_compiler::CallingConvention::WindowsFastcall => "windows_fastcall",
            })
    }
}

/// Represents a set of CPU features.
#[pyclass]
#[text_signature = "()"]
pub struct CpuFeatures {
    inner: EnumSet<wasmer_compiler::CpuFeature>,
}

impl CpuFeatures {
    pub(crate) fn inner(&self) -> &EnumSet<wasmer_compiler::CpuFeature> {
        &self.inner
    }
}

#[pymethods]
impl CpuFeatures {
    #[new]
    fn new() -> Self {
        Self {
            inner: wasmer_compiler::CpuFeature::set(),
        }
    }

    /// Add a new CPU feature.
    #[text_signature = "($self, feature)"]
    fn add(&mut self, feature: &str) -> PyResult<()> {
        self.inner.insert(
            wasmer_compiler::CpuFeature::from_str(feature).map_err(to_py_err::<ValueError, _>)?,
        );

        Ok(())
    }
}
