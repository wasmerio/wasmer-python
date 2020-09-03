use pyo3::prelude::*;

use wasmer_common_py::py::Store;

#[pymodule]
fn wasmer_compiler_cranelift(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<Compiler>()?;

    Ok(())
}

#[pyclass]
struct Compiler {}

#[pymethods]
impl Compiler {
    #[staticmethod]
    fn into_store() -> Store {
        let store = Store::raw_with_compiler(wasmer_compiler_cranelift::Cranelift::default());

        store
    }
}
