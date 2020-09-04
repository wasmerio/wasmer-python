use crate::wasmer;
use pyo3::{exceptions::RuntimeError, prelude::*};
use std::sync::Arc;

macro_rules! engine_basis {
    (pyclass = $pyclass:ident, engine = $engine:ident, builder = $engine_builder:ident) => {
        #[pyclass(unsendable)]
        #[text_signature = "(/, compiler)"]
        pub struct $pyclass {
            inner: wasmer::$engine,
        }

        impl $pyclass {
            pub(crate) fn inner(&self) -> &wasmer::$engine {
                &self.inner
            }
        }
    };
}

engine_basis!(pyclass = JIT, engine = JITEngine, builder = JIT);
engine_basis!(pyclass = Native, engine = NativeEngine, builder = Native);

#[pymethods]
impl JIT {
    #[new]
    fn new(compiler: Option<&PyAny>) -> PyResult<Self> {
        Ok(Self {
            inner: match compiler {
                None => wasmer::JIT::headless().engine(),
                Some(compiler) => {
                    let opaque_compiler = compiler.call_method0("into_opaque_compiler")?;
                    let opaque_compiler_inner_ptr = opaque_compiler
                        .call_method0("__inner_as_ptr")?
                        .extract::<usize>()?;

                    let opaque_compiler_inner_ptr: *const OpaqueCompilerInner =
                        opaque_compiler_inner_ptr as _;

                    let opaque_compiler_inner_ref: &OpaqueCompilerInner = unsafe {
                        opaque_compiler_inner_ptr.as_ref().ok_or_else(|| {
                            RuntimeError::py_err(
                                "Failed to transfer the opaque compiler from the compiler",
                            )
                        })?
                    };

                    let opaque_compiler_inner: OpaqueCompilerInner =
                        opaque_compiler_inner_ref.clone();

                    wasmer::JIT::new(opaque_compiler_inner.compiler_config.as_ref()).engine()
                }
            },
        })
    }
}

#[derive(Clone)]
struct OpaqueCompilerInner {
    compiler_config: Arc<dyn wasmer_compiler::CompilerConfig + Send + Sync>,
}

#[pyclass]
pub struct OpaqueCompiler {
    inner: OpaqueCompilerInner,
}

impl OpaqueCompiler {
    pub fn raw_with_compiler<C>(compiler_config: C) -> Self
    where
        C: wasmer_compiler::CompilerConfig + Send + Sync + 'static,
    {
        Self {
            inner: OpaqueCompilerInner {
                compiler_config: Arc::new(compiler_config),
            },
        }
    }
}

#[pymethods]
impl OpaqueCompiler {
    pub fn __inner_as_ptr(&self) -> usize {
        let inner_ptr: *const OpaqueCompilerInner = &self.inner;
        let inner_usize: usize = inner_ptr as _;

        inner_usize
    }
}
