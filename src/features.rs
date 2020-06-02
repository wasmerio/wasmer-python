use pyo3::prelude::*;

/// `Features` is a Python class used to test whether a specific
/// feature is enabled or not.
#[pyclass]
pub struct Features {}

#[pymethods]
impl Features {
    /// Check whether host functions are enabled.
    #[text_signature = "()"]
    #[staticmethod]
    pub fn host_functions() -> bool {
        if cfg!(all(unix, target_arch = "x86_64")) {
            true
        } else {
            false
        }
    }

    /// Check wether WASI is enabled.
    #[text_signature = "()"]
    #[staticmethod]
    pub fn wasi() -> bool {
        if cfg!(target_arch = "x86_64") {
            true
        } else {
            false
        }
    }
}
