//! Utils to manipulate Python errors.o

use cpython::{exc::RuntimeError, PyErr, Python, PythonObject, ToPyObject};

/// Create a `RuntimeError` error in Python.
///
/// # Examples
///
/// ```rs,ignore
/// fn f(py: Python) -> PyResult<()> {
///     let error = new_runtime_error(py, "foobar");
///     Err(error)
/// }
/// ```
pub fn new_runtime_error(py: Python, error_message: &str) -> PyErr {
    PyErr::new_lazy_init(
        py.get_type::<RuntimeError>(),
        Some(error_message.to_py_object(py).into_object()),
    )
}
