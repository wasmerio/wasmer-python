use pyo3::prelude::*;

use wasmer_common_py::OpaqueCompiler;

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
    fn into_opaque_compiler() -> OpaqueCompiler {
        let opaque_compiler =
            OpaqueCompiler::raw_with_compiler(wasmer_compiler_cranelift::Cranelift::default());

        opaque_compiler
    }
}
