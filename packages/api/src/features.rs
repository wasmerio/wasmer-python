use cfg_if::cfg_if;
use pyo3::prelude::*;

#[pyclass]
pub struct Features {}

#[pymethods]
impl Features {
    #[classattr]
    fn headless() -> bool {
        cfg_if! {
            if #[cfg(feature = "headless")] {
                true
            } else {
                false
            }
        }
    }

    #[classattr]
    fn default_compiler() -> bool {
        cfg_if! {
            if #[cfg(feature = "default-compiler")] {
                true
            } else {
                false
            }
        }
    }
}
