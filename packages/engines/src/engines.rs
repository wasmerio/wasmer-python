use crate::target_lexicon::Target;
use pyo3::{exceptions::PyRuntimeError, prelude::*};
use std::mem::ManuallyDrop;

/// Universal engine for Wasmer compilers.
///
/// Given an optional compiler, it generates the compiled machine code,
/// and publishes it into memory so it can be used externally.
///
/// If the compiler is absent, it will generate a headless engine.
///
/// It is possible to specify a `Target` to possibly cross-compile for
/// a different target. It requires a compiler.
#[pyclass(unsendable, subclass)]
#[text_signature = "(/, compiler, target)"]
pub struct Universal {
    inner: wasmer::UniversalEngine,
    compiler_name: Option<String>,
}

impl Universal {
    pub fn raw_new(compiler: Option<&PyAny>, target: Option<&Target>) -> PyResult<Self> {
        let (inner, compiler_name) = match compiler {
            None => (wasmer::Universal::headless().engine(), None),
            Some(compiler) => {
                let opaque_compiler = compiler.call_method0("into_opaque_compiler")?;
                let opaque_compiler_inner_ptr = opaque_compiler
                    .call_method0("__inner_as_ptr")?
                    .extract::<usize>()?;

                let opaque_compiler_inner_ptr: *mut OpaqueCompilerInner =
                    opaque_compiler_inner_ptr as *const OpaqueCompilerInner as *mut _;

                let opaque_compiler_inner_ref: &mut OpaqueCompilerInner = unsafe {
                    opaque_compiler_inner_ptr.as_mut().ok_or_else(|| {
                        PyRuntimeError::new_err(
                            "Failed to transfer the opaque compiler from the compiler",
                        )
                    })?
                };

                // SAFETY: `ManuallyDrop::take` semantically moves out the contained value. The
                // danger here is when the container is used by someone else. It doesn't happen in
                // this codebase.
                let compiler_config =
                    unsafe { ManuallyDrop::take(&mut opaque_compiler_inner_ref.compiler_config) };

                let mut engine_builder = wasmer::Universal::new(compiler_config);

                if let Some(target) = target {
                    engine_builder = engine_builder.target(target.inner().clone());
                }

                (
                    engine_builder.engine(),
                    Some(
                        opaque_compiler
                            .getattr("name")?
                            .extract::<String>()
                            .map_err(PyErr::from)?,
                    ),
                )
            }
        };

        Ok(Self {
            inner,
            compiler_name,
        })
    }

    pub fn name() -> &'static str {
        "universal"
    }

    pub fn inner(&self) -> &wasmer::UniversalEngine {
        &self.inner
    }

    pub fn compiler_name(&self) -> Option<&String> {
        self.compiler_name.as_ref()
    }
}

#[pymethods]
impl Universal {
    #[new]
    fn new(compiler: Option<&PyAny>, target: Option<&Target>) -> PyResult<Self> {
        Self::raw_new(compiler, target)
    }
}

/// Dylib engine for Wasmer compilers.
///
/// Given an optional compiler, it generates a shared object file
/// (`.so`, `.dylib` or `.dll` depending on the target), saves it
/// temporarily to disk and uses it dylibly via `dlopen` and `dlsym`.
/// and publishes it into memory so it can be used externally.
///
/// If the compiler is absent, it will generate a headless engine.
///
/// It is possible to specify a `Target` to possibly cross-compile for
/// a different target. It requires a compiler.
#[pyclass(unsendable, subclass)]
#[text_signature = "(/, compiler, target)"]
pub struct Dylib {
    inner: wasmer::DylibEngine,
    compiler_name: Option<String>,
}

impl Dylib {
    pub fn raw_new(compiler: Option<&PyAny>, target: Option<&Target>) -> PyResult<Self> {
        let (inner, compiler_name) = match compiler {
            None => (wasmer::Dylib::headless().engine(), None),
            Some(compiler) => {
                let opaque_compiler = compiler.call_method0("into_opaque_compiler")?;
                let opaque_compiler_inner_ptr = opaque_compiler
                    .call_method0("__inner_as_ptr")?
                    .extract::<usize>()?;

                let opaque_compiler_inner_ptr: *mut OpaqueCompilerInner =
                    opaque_compiler_inner_ptr as *const OpaqueCompilerInner as *mut _;

                let opaque_compiler_inner_ref: &mut OpaqueCompilerInner = unsafe {
                    opaque_compiler_inner_ptr.as_mut().ok_or_else(|| {
                        PyRuntimeError::new_err(
                            "Failed to transfer the opaque compiler from the compiler",
                        )
                    })?
                };

                // SAFETY: `ManuallyDrop::take` semantically moves out the contained value. The
                // danger here is when the container is used by someone else. It doesn't happen in
                // this codebase.
                let compiler_config =
                    unsafe { ManuallyDrop::take(&mut opaque_compiler_inner_ref.compiler_config) };

                let mut engine_builder = wasmer::Dylib::new(compiler_config);

                if let Some(target) = target {
                    engine_builder = engine_builder.target(target.inner().clone());
                }

                (
                    engine_builder.engine(),
                    Some(
                        opaque_compiler
                            .getattr("name")?
                            .extract::<String>()
                            .map_err(PyErr::from)?,
                    ),
                )
            }
        };

        Ok(Self {
            inner,
            compiler_name,
        })
    }

    pub fn name() -> &'static str {
        "dylib"
    }

    pub fn inner(&self) -> &wasmer::DylibEngine {
        &self.inner
    }

    pub fn compiler_name(&self) -> Option<&String> {
        self.compiler_name.as_ref()
    }
}

#[pymethods]
impl Dylib {
    #[new]
    fn new(compiler: Option<&PyAny>, target: Option<&Target>) -> PyResult<Self> {
        Self::raw_new(compiler, target)
    }
}

struct OpaqueCompilerInner {
    compiler_config: ManuallyDrop<Box<dyn wasmer_compiler::CompilerConfig>>,
}

/// Opaque compiler.
///
/// Internal use only.
#[pyclass(unsendable)]
pub struct OpaqueCompiler {
    inner: OpaqueCompilerInner,
    compiler_name: String,
}

impl OpaqueCompiler {
    pub fn raw_with_compiler<C>(compiler_config: C, compiler_name: String) -> Self
    where
        C: wasmer_compiler::CompilerConfig + Send + Sync + 'static,
    {
        Self {
            inner: OpaqueCompilerInner {
                compiler_config: ManuallyDrop::new(Box::new(compiler_config)),
            },
            compiler_name,
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

    #[getter]
    fn name(&self) -> &String {
        &self.compiler_name
    }
}

/// Deprecated engine. Please use the parent engine instead,
/// i.e. `Universal`.
#[pyclass(extends=Universal)]
pub struct JIT {}

#[pymethods]
impl JIT {
    #[new]
    fn new(compiler: Option<&PyAny>, target: Option<&Target>) -> PyResult<(Self, Universal)> {
        Ok((Self {}, Universal::raw_new(compiler, target)?))
    }
}

/// Deprecated engine. Please use the parent engine instead,
/// i.e. `Dylib`.
#[pyclass(extends=Dylib)]
pub struct Native {}

#[pymethods]
impl Native {
    #[new]
    fn new(compiler: Option<&PyAny>, target: Option<&Target>) -> PyResult<(Self, Dylib)> {
        Ok((Self {}, Dylib::raw_new(compiler, target)?))
    }
}
