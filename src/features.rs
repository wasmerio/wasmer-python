use pyo3::prelude::*;

#[pyclass]
pub struct Features {}

#[pymethods]
impl Features {
    #[staticmethod]
    pub fn host_functions() -> bool {
        if cfg!(all(unix, target_arch = "x86_64")) {
            true
        } else {
            false
        }
    }
}
