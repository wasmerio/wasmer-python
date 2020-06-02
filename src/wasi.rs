use crate::{import::ImportObject, module::Module};
use pyo3::{
    exceptions::{RuntimeError, ValueError},
    prelude::*,
    pycell::PyCell,
    types::{PyDict, PyList},
};
use std::{
    convert::{TryFrom, TryInto},
    path::PathBuf,
    slice,
};
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

/// `Wasi` is a Python class that helps to build a WASI state, i.e. to
/// define WASI arguments, environments, preopened directories, mapped
/// directories etc.
#[pyclass]
#[text_signature = "(arguments=[], environments={}, preopen_directories=[], map_directories={})"]
pub struct Wasi {
    pub(crate) inner: state::WasiStateBuilder,
}

impl Wasi {
    pub fn self_arguments(&mut self, arguments: &PyList) {
        self.inner
            .args(arguments.iter().map(|any_item| any_item.to_string()));
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
            .map_err(|error| {
                RuntimeError::py_err(format!(
                    "Failed to configure preopened directories when creating the WASI state: {}",
                    error
                ))
            })?;

        Ok(())
    }

    pub fn self_preopen_directory(&mut self, preopen_directory: String) -> PyResult<()> {
        self.inner
            .preopen_dir(PathBuf::from(preopen_directory))
            .map_err(|error| {
                RuntimeError::py_err(format!(
                    "Failed to configure the preopened directory when creating the WASI state: {}",
                    error
                ))
            })?;

        Ok(())
    }

    pub fn self_map_directories(&mut self, map_directories: &PyDict) -> PyResult<()> {
        self.inner
            .map_dirs(map_directories.iter().map(|(any_key, any_value)| {
                (any_key.to_string(), PathBuf::from(any_value.to_string()))
            }))
            .map_err(|error| {
                RuntimeError::py_err(format!(
                    "Failed to configure map directories when creating the WASI state: {}",
                    error
                ))
            })?;

        Ok(())
    }

    pub fn self_map_directory(&mut self, alias: String, directory: String) -> PyResult<()> {
        self.inner
            .map_dir(alias.as_str(), PathBuf::from(directory.to_string()))
            .map_err(|error| {
                RuntimeError::py_err(format!(
                    "Failed to configure the map directory when creating the WASI state: {}",
                    error
                ))
            })?;

        Ok(())
    }
}

#[pymethods]
impl Wasi {
    /// Build a `Wasi` object. The constructor can be used to
    /// initialize its state, and its methods help to update its
    /// state.
    ///
    /// # Examples
    ///
    /// Thus, both next notations are equivalent and can be mixed:
    ///
    /// ```py
    /// wasi = Wasi(
    ///     program_name="wasi_test_program",
    ///     arguments=["--test"],
    ///     environments={"COLOR": "true", "APP_SHOULD_LOG": "false"},
    ///     map_directories={"the_host_current_dir": "."}
    /// )
    /// ```
    ///
    /// could be rewritten:
    ///
    /// ```py
    /// wasi = \
    ///     Wasi("wasi_test_program"). \
    ///         argument("--test"). \
    ///         environment("COLOR", "true"). \
    ///         environment("APP_SHOULD_LOG", "false"). \
    ///         map_directory("the_host_current_dir", ".")
    /// ```
    #[new]
    #[args(
        arguments = "PyList::empty(_py)",
        environments = "PyDict::new(_py)",
        preopen_directories = "PyList::empty(_py)",
        map_directories = "PyDict::new(_py)"
    )]
    fn new(
        program_name: String,
        arguments: &PyList,
        environments: &PyDict,
        preopen_directories: &PyList,
        map_directories: &PyDict,
    ) -> PyResult<Self> {
        let mut wasi = Self {
            inner: state::WasiState::new(program_name.as_str()),
        };

        if !arguments.is_empty() {
            wasi.self_arguments(arguments);
        }

        if !environments.is_empty() {
            wasi.self_environments(environments);
        }

        if !preopen_directories.is_empty() {
            wasi.self_preopen_directories(preopen_directories)?;
        }

        if !map_directories.is_empty() {
            wasi.self_map_directories(map_directories)?;
        }

        Ok(wasi)
    }

    /// Add a list of arguments to the program.
    /// The arguments must not contain the `nul` (`0x0`) byte.
    #[text_signature = "($self, arguments)"]
    pub fn arguments<'py>(
        slf: &'py PyCell<Self>,
        arguments: &PyList,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_arguments(arguments);

        Ok(slf)
    }

    /// Add a single argument to the program.
    /// The argument must not contain the `nul` (`0x0`) byte.
    #[text_signature = "($self, argument)"]
    pub fn argument<'py>(slf: &'py PyCell<Self>, argument: String) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_argument(argument);

        Ok(slf)
    }

    /// Add environment variables to the program.
    /// The pairs key and value must not contain the `=` (`0x3d`) or
    /// `nul` (`0x0)` byte.
    #[text_signature = "($self, environments)"]
    pub fn environments<'py>(
        slf: &'py PyCell<Self>,
        environments: &PyDict,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_environments(environments);

        Ok(slf)
    }

    /// Add a single environment variable to the program.
    /// The pair key and value must not contain the `=` (`0x3d`) or
    /// `nul` (`0x0)` byte.
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

    /// Add preopened directories to the program.
    #[text_signature = "($self, preopen_directories)"]
    pub fn preopen_directories<'py>(
        slf: &'py PyCell<Self>,
        preopen_directories: &PyList,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_preopen_directories(preopen_directories)?;

        Ok(slf)
    }

    /// Add a single preopened directory to the program.
    #[text_signature = "($self, preopen_directory)"]
    pub fn preopen_directory<'py>(
        slf: &'py PyCell<Self>,
        preopen_directory: String,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_preopen_directory(preopen_directory)?;

        Ok(slf)
    }

    /// Add preopened directories with different names.
    #[text_signature = "($self, map_directories)"]
    pub fn map_directories<'py>(
        slf: &'py PyCell<Self>,
        map_directories: &PyDict,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_map_directories(map_directories)?;

        Ok(slf)
    }

    /// Add a single preopened directory with a different name.
    #[text_signature = "($self, map_directory)"]
    pub fn map_directory<'py>(
        slf: &'py PyCell<Self>,
        alias: String,
        directory: String,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_map_directory(alias, directory)?;

        Ok(slf)
    }

    /// Transform this WASI object into an `ImportObject` object for a
    /// particular module. The WASI version is optional; if absent, it
    /// will be guessed with the strict parameter turned off (see
    /// `Module::wasi_version` to learn more).
    ///
    /// # Examples
    ///
    /// ```py
    /// module = Module(wasm_bytes)
    /// wasi = Wasi("test_program", arguments=["--foobar"], environments={"BAZ": "qux"})
    /// import_object = wasi.generate_import_object_for_module(module)
    /// instance = module.instantiate(import_object)
    /// ```
    #[text_signature = "($self, module, version=0)"]
    #[args(version = 0)]
    pub fn generate_import_object_for_module(
        &mut self,
        module: &Module,
        version: u8,
    ) -> PyResult<ImportObject> {
        let version: Version = if version == 0 {
            wasmer_wasi::get_wasi_version(&module.inner, false)
                .ok_or(())
                .map(Into::into)
                .map_err(|_| {
                    RuntimeError::py_err("Failed to generate an import object from a WASI state because the given module has no WASI imports")
                })?
        } else {
            version
                .try_into()
                .map_err(|e: String| ValueError::py_err(e))?
        };

        ImportObject::new_with_wasi(module.inner.clone(), version, self)
    }
}
