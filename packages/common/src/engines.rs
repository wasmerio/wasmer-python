use crate::wasmer;
use pyo3::{exceptions::RuntimeError, prelude::*};
use std::sync::Arc;

/// JIT engine for Wasmer compilers.
///
/// Given an option compiler, it generates the compiled machine code,
/// and publishes it into memory so it can be used externally.
///
/// If the compiler is absent, it will generate a headless engine.
#[pyclass(unsendable)]
#[text_signature = "(/, compiler)"]
pub struct JIT {
    inner: wasmer::JITEngine,
}

impl JIT {
    pub(crate) fn inner(&self) -> &wasmer::JITEngine {
        &self.inner
    }

    pub(crate) fn raw_new(compiler: Option<&PyAny>) -> PyResult<Self> {
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

                    // Let's clone the `OpaqueCompilerInner` so that
                    // whatever happens to its parent `compiler`
                    // Python object, we own a reference to it.
                    let opaque_compiler_inner: OpaqueCompilerInner =
                        opaque_compiler_inner_ref.clone();

                    debug_assert_eq!(Arc::strong_count(&opaque_compiler_inner.compiler_config), 2);

                    wasmer::JIT::new(opaque_compiler_inner.compiler_config.as_ref()).engine()
                }
            },
        })
    }
}

#[pymethods]
impl JIT {
    #[new]
    fn new(compiler: Option<&PyAny>) -> PyResult<Self> {
        Self::raw_new(compiler)
    }
}

/// Native engine for Wasmer compilers.
///
/// Given an option compiler, it generates a shared object file
/// (`.so`, `.dylib` or `.dll` depending on the target), saves it
/// temporarily to disk and uses it natively via `dlopen` and `dlsym`.
/// and publishes it into memory so it can be used externally.
///
/// If the compiler is absent, it will generate a headless engine.
#[pyclass(unsendable)]
#[text_signature = "(/, compiler)"]
pub struct Native {
    inner: wasmer::NativeEngine,
}

impl Native {
    pub(crate) fn inner(&self) -> &wasmer::NativeEngine {
        &self.inner
    }

    pub(crate) fn raw_new(compiler: Option<&PyAny>) -> PyResult<Self> {
        Ok(Self {
            inner: match compiler {
                None => wasmer::Native::headless().engine(),
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

                    // Let's clone the `OpaqueCompilerInner` so that
                    // whatever happens to its parent `compiler`
                    // Python object, we own a reference to it.
                    let opaque_compiler_inner: OpaqueCompilerInner =
                        opaque_compiler_inner_ref.clone();

                    debug_assert_eq!(Arc::strong_count(&opaque_compiler_inner.compiler_config), 2);

                    // Since we've cloned the `OpaqueCompilerInner`
                    // previously, its strong count is equal to
                    // 2. Consequently, we can't get a mutable version
                    // of it, and we need one.
                    //
                    // However, we are ensure the original value won't
                    // be used, since the value exists only in this
                    // block of code. Thus, we believe it is safe to
                    // cast the pointer to a mutable refeference.

                    let compiler_config_ptr: *mut dyn wasmer_compiler::CompilerConfig =
                        Arc::as_ptr(&opaque_compiler_inner.compiler_config) as *mut _;
                    let compiler_config_ref: &mut dyn wasmer_compiler::CompilerConfig =
                        unsafe { &mut *compiler_config_ptr };

                    wasmer::Native::new(compiler_config_ref).engine()
                }
            },
        })
    }
}

#[pymethods]
impl Native {
    #[new]
    fn new(compiler: Option<&PyAny>) -> PyResult<Self> {
        Self::raw_new(compiler)
    }
}

#[derive(Clone)]
struct OpaqueCompilerInner {
    compiler_config: Arc<dyn wasmer_compiler::CompilerConfig + Send + Sync>,
}

/// Opaque compiler.
///
/// Internal use only.
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
