use crate::{
    errors::to_py_err, import_object::ImportObject, module::Module, store::Store,
    wasmer_inner::wasmer_wasi,
};
use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
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
                return Err(to_py_err::<PyValueError, _>(
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

/// Convenient builder API for configuring WASI.
///
/// Use the constructor to pass the arguments, environments, preopen
/// directories and map directories, or use the associated methods to
/// build the state step-by-steps.
///
/// ## Example
///
/// ```py
/// from wasmer import wasi
///
/// wasi_state_builder = wasi.StateBuilder('test-program')
/// ```
#[pyclass]
#[pyo3(
    text_signature = "(arguments=[], environments={}, preopen_directories=[], map_directories={})"
)]
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
            .map_err(to_py_err::<PyRuntimeError, _>)?;

        Ok(())
    }

    pub fn self_preopen_directory(&mut self, preopen_directory: String) -> PyResult<()> {
        self.inner
            .preopen_dir(PathBuf::from(preopen_directory))
            .map_err(to_py_err::<PyRuntimeError, _>)?;

        Ok(())
    }

    pub fn self_map_directories(&mut self, map_directories: &PyDict) -> PyResult<()> {
        self.inner
            .map_dirs(map_directories.iter().map(|(any_key, any_value)| {
                (any_key.to_string(), PathBuf::from(any_value.to_string()))
            }))
            .map_err(to_py_err::<PyRuntimeError, _>)?;

        Ok(())
    }

    pub fn self_map_directory(&mut self, alias: String, directory: String) -> PyResult<()> {
        self.inner
            .map_dir(alias.as_str(), PathBuf::from(directory))
            .map_err(to_py_err::<PyRuntimeError, _>)?;

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

    /// Add multiple arguments.
    ///
    /// Arguments must not contain the nul (`0x0`) byte.
    ///
    /// This method returns `self`.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import wasi
    ///
    /// wasi_state_builder = \
    ///     wasi.StateBuilder('test-program'). \
    ///         arguments(['--verbose --help'])
    /// ```
    #[pyo3(text_signature = "($self, arguments)")]
    pub fn arguments<'py>(
        slf: &'py PyCell<Self>,
        arguments: &PyList,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_arguments(arguments);

        Ok(slf)
    }

    /// Add an argument.
    ///
    /// Arguments must not contain the nul (`0x0`) byte.
    ///
    /// This method returns `self`.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import wasi
    ///
    /// wasi_state_builder = \
    ///     wasi.StateBuilder('test-program'). \
    ///         argument('--verbose'). \
    ///         argument('--help')
    /// ```
    #[pyo3(text_signature = "($self, argument)")]
    pub fn argument<'py>(slf: &'py PyCell<Self>, argument: String) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_argument(argument);

        Ok(slf)
    }

    /// Add environment variable pairs.
    ///
    /// Environment variable keys and values must not contain the byte
    /// `=` (`0x3d`) or null (`0x0`).
    ///
    /// This method returns `self`.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import wasi
    ///
    /// wasi_state_builder = \
    ///     wasi.StateBuilder('test-program'). \
    ///         environments({"ABC": "DEF", "X": "YZ"})
    /// ```
    #[pyo3(text_signature = "($self, environments)")]
    pub fn environments<'py>(
        slf: &'py PyCell<Self>,
        environments: &PyDict,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_environments(environments);

        Ok(slf)
    }

    /// Add an environment variable pair.
    ///
    /// Environment variable keys and values must not contain the byte
    /// `=` (`0x3d`) or null (`0x0`).
    ///
    /// This method returns `self`.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import wasi
    ///
    /// wasi_state_builder = \
    ///     wasi.StateBuilder('test-program'). \
    ///         environment("ABC", "DEF"). \
    ///         environment("X", "YZ")
    /// ```
    #[pyo3(text_signature = "($self, key, value)")]
    pub fn environment<'py>(
        slf: &'py PyCell<Self>,
        key: String,
        value: String,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_environment(key, value);

        Ok(slf)
    }

    /// Preopen directories.
    ///
    /// This opens the given directories at the virtual root, `/`, and
    /// allows the WASI module to read and write to the given
    /// directories.
    ///
    /// This method returns `self`.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import wasi
    ///
    /// wasi_state_builder = \
    ///     wasi.StateBuilder('test-program'). \
    ///         preopen_directories(["."])
    /// ```
    #[pyo3(text_signature = "($self, preopen_directories)")]
    pub fn preopen_directories<'py>(
        slf: &'py PyCell<Self>,
        preopen_directories: &PyList,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_preopen_directories(preopen_directories)?;

        Ok(slf)
    }

    /// Preopen a directory.
    ///
    /// This opens the given directory at the virtual root, `/`, and
    /// allows the WASI module to read and write to the given
    /// directory.
    ///
    /// This method returns `self`.
    ///
    /// ## Example
    ///
    /// ```py,ignore
    /// from wasmer import wasi
    ///
    /// wasi_state_builder = \
    ///     wasi.StateBuilder('test-program'). \
    ///         preopen_directory(".")
    /// ```
    #[pyo3(text_signature = "($self, preopen_directory)")]
    pub fn preopen_directory<'py>(
        slf: &'py PyCell<Self>,
        preopen_directory: String,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_preopen_directory(preopen_directory)?;

        Ok(slf)
    }

    /// Preopen directories with different names exposed to the WASI.
    ///
    /// This method returns `self`.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import wasi
    ///
    /// wasi_state_builder = \
    ///     wasi.StateBuilder('test-program'). \
    ///         map_directories({"foo": "."})
    /// ```
    #[pyo3(text_signature = "($self, map_directories)")]
    pub fn map_directories<'py>(
        slf: &'py PyCell<Self>,
        map_directories: &PyDict,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_map_directories(map_directories)?;

        Ok(slf)
    }

    /// Preopen a directory with a different name exposed to the WASI.
    ///
    /// This method returns `self`.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import wasi
    ///
    /// wasi_state_builder = \
    ///     wasi.StateBuilder('test-program'). \
    ///         map_directory("foo", ".")
    /// ```
    #[pyo3(text_signature = "($self, alias, directory)")]
    pub fn map_directory<'py>(
        slf: &'py PyCell<Self>,
        alias: String,
        directory: String,
    ) -> PyResult<&'py PyCell<Self>> {
        let mut slf_mut = slf.try_borrow_mut()?;
        slf_mut.self_map_directory(alias, directory)?;

        Ok(slf)
    }

    /// Produces a WASI `Environment` based on this state builder.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import wasi
    ///
    /// wasi_env = \
    ///     wasi.StateBuilder('test-program'). \
    ///         argument('--foo'). \
    ///         finalize()
    /// ```
    #[pyo3(text_signature = "($self)")]
    pub fn finalize(&mut self) -> PyResult<Environment> {
        Ok(Environment::raw_new(
            self.inner
                .finalize()
                .map_err(to_py_err::<PyRuntimeError, _>)?,
        ))
    }
}

/// The environment provided to the WASI imports.
///
/// To build it, use `StateBuilder`. See `StateBuilder.finalize` to
/// learn more.
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
    /// Create an `wasmer.ImportObject` with an existing
    /// `Environment`. The import object will be different according
    /// to the WASI version.
    ///
    /// Use the `Version` enum to use a specific WASI version, or use
    /// `get_version` to read the WASI version from a `wasmer.Module`.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import wasi, Store
    ///
    /// store = Store()
    /// wasi_env = wasi.StateBuilder('test-program').argument('--foo').finalize()
    /// import_object = wasi_env.generate_import_object(store, wasi.Version.SNAPSHOT1)
    /// ```
    //#[pyo3(text_signature = "($self, store, wasi_version)")]
    fn generate_import_object(&self, store: &Store, wasi_version: Version) -> ImportObject {
        let import_object = wasmer_wasi::generate_import_object_from_env(
            store.inner(),
            self.inner.clone(),
            wasi_version.into(),
        );

        ImportObject::raw_new(import_object)
    }

    /// Create a dictionary of import with an existing
    /// `Environment`. The import object will be different according
    /// to the WASI version.
    ///
    /// Use the `Version` enum to use a specific WASI version, or use
    /// `get_version` to read the WASI version from a `wasmer.Module`.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import wasi, Store
    ///
    /// store = Store()
    /// wasi_env = wasi.StateBuilder('test-program').argument('--foo').finalize()
    /// imports = wasi_env.generate_imports(store, wasi.Version.SNAPSHOT1)
    /// ```
    //#[pyo3(text_signature = "($self, store, wasi_version)")]
    fn generate_imports(&self, store: &Store, wasi_version: Version) -> PyResult<PyObject> {
        self.generate_import_object(store, wasi_version).to_dict()
    }
}

pub fn get_version(module: &Module, strict: bool) -> Option<Version> {
    wasmer_wasi::get_wasi_version(&module.inner(), strict).map(Into::into)
}
