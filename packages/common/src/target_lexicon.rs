use crate::errors::to_py_err;
use enumset::EnumSet;
use pyo3::{class::basic::PyObjectProtocol, exceptions::ValueError, prelude::*};
use std::str::FromStr;

/// Represents a `Triple` + `CpuFeatures` pair.
///
/// If the `CpuFeatures` is ommited, an empty set of CPU feature will
/// be assumed.
///
/// ## Example
///
/// ```py
/// from wasmer import target
///
/// triple = target.Triple('x86_64-apple-darwin')
///
/// cpu_features = target.CpuFeatures()
/// cpu_features.add('sse2')
///
/// target = target.Target(triple, cpu_features)
/// ```
#[pyclass]
#[text_signature = "(triple, cpu_features)"]
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
///
/// ## Example
///
/// ```py
/// from wasmer import target
///
/// triple = target.Triple('x86_64-apple-darwin')
///
/// assert str(triple) == 'x86_64-apple-darwin'
/// assert triple.architecture == 'x86_64'
/// assert triple.vendor == 'apple'
/// assert triple.operating_system == 'darwin'
/// assert triple.binary_format == 'macho'
/// assert triple.environment == 'unknown'
/// assert triple.endianness == 'little'
/// assert triple.pointer_width == 8
/// assert triple.default_calling_convention == 'system_v'
/// ```
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
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import target
    ///
    /// this_triple = target.Triple.host()
    /// ```
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
    ///
    /// Possible returned values are `little` or `big`.
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
    ///
    /// The width of a pointer (in the default address space) can be
    /// of size `u16`, `u32` or `u64`, resp. 2, 4 or 8 bytes, which
    /// are the possible returned values.
    #[getter]
    fn pointer_width(&self) -> Option<u8> {
        self.inner
            .pointer_width()
            .ok()
            .map(wasmer_compiler::PointerWidth::bytes)
    }

    /// Returns the default calling convention for the given target
    /// triple.
    ///
    /// The calling convention specifies things like which registers
    /// are used for passing arguments, which registers are
    /// callee-saved, and so on.
    ///
    /// Possible returned values are:
    ///
    /// * `system_v`, “System V” which is used on most Unix-like platfoms. Note
    ///   that the specific conventions vary between hardware
    ///   architectures; for example, x86-32's “System V” is entirely
    ///   different from x86-64's “System V”.
    /// * `wasm_basic_c_abi`, [The WebAssembly C
    ///   ABI](https://github.com/WebAssembly/tool-conventions/blob/master/BasicCABI.md).
    /// * `windows_fastcall`, “Windows Fastcall” which is used on
    ///   Windows. Note that like “System V”, this varies between
    ///   hardware architectures. On x86-32 it describes what Windows
    ///   documentation calls “fastcall”, and on x86-64 it describes
    ///   what Windows documentation often just calls the Windows x64
    ///   calling convention (though the compiler still recognizes
    ///   “fastcall” as an alias for it).
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

#[pyproto]
impl PyObjectProtocol for Triple {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }
}

/// Represents a set of CPU features.
///
/// CPU features are identified by their stringified names. The
/// reference is the GCC options:
///
/// * https://gcc.gnu.org/onlinedocs/gcc/x86-Options.html
/// * https://gcc.gnu.org/onlinedocs/gcc/ARM-Options.html
/// * https://gcc.gnu.org/onlinedocs/gcc/AArch64-Options.html
///
/// At the time of writing this documentation (it might be outdated in
/// the future), the supported features are the following:
///
/// * `sse2`,
/// * `sse3`,
/// * `ssse3`,
/// * `sse4.1`,
/// * `sse4.2`,
/// * `popcnt`,
/// * `avx`,
/// * `bmi`,
/// * `bmi2`,
/// * `avx2`,
/// * `avx512dq`,
/// * `avx512vl`,
/// * `lzcnt`.
///
/// ## Example
///
/// ```py
/// from wasmer import target
///
/// cpu_features = target.CpuFeatures()
/// cpu_features.add('sse2')
/// ```
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
