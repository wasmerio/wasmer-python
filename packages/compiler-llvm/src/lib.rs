use pyo3::prelude::*;

use wasmer_common_py::OpaqueCompiler;

#[pymodule]
fn wasmer_compiler_llvm(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<Compiler>()?;

    Ok(())
}

#[pyclass]
struct Compiler {}

#[pymethods]
impl Compiler {
    #[staticmethod]
    fn into_opaque_compiler() -> Store {
        let opaque_compiler =
            OpaqueCompiler::raw_with_compiler(wasmer_compiler_llvm::LLVM::default());

        opaque_compiler
    }
}
