use crate::{
    errors::to_py_err,
    wasmer_inner::{wasmer, wasmer_engines as engines},
};
use pyo3::{exceptions::PyTypeError, prelude::*};

/// The store represents all global state that can be manipulated by
/// WebAssembly programs. It consists of the runtime representation of
/// all instances of functions, tables, memories, and globals that
/// have been allocated during the lifetime of the abstract machine.
///
/// The `Store` holds the engine (that is —amongst many things— used
/// to compile the WebAssembly bytes into a valid module artifact), in
/// addition to the `Tunables` (that are used to create the memories,
/// tables and globals). The engine comes from the `wasmer.engine`
/// module.
///
/// Specification: https://webassembly.github.io/spec/core/exec/runtime.html#store
///
/// Read the documentation of the `engine` submodule to learn more.
///
/// ## Examples
///
/// Use the Universal engine with no compiler (headless mode):
///
/// ```py
/// from wasmer import engine, Store
///
/// store = Store(engine.Universal())
/// ```
///
/// Use the Universal engine with the LLVM compiler:
///
/// ```py,ignore
/// from wasmer import engine, Store
/// from wasmer_compiler_llvm import Compiler
///
/// store = Store(engine.Universal(Compiler))
/// ```
///
/// If the store is built without an engine, the Universal engine will be
/// used, with the first compiler found in this order:
/// `compiler_compiler_cranelift`, `compiler_compiler_llvm`,
/// `compiler_compiler_singlepass`, otherwise it will run in headless
/// mode.
#[pyclass]
#[pyo3(text_signature = "(engine)")]
pub struct Store {
    inner: wasmer::Store,
    engine_name: String,
    compiler_name: Option<String>,
}

impl Store {
    pub fn inner(&self) -> &wasmer::Store {
        &self.inner
    }
}

#[pymethods]
impl Store {
    #[new]
    fn new(py: Python, engine: Option<&PyAny>) -> PyResult<Self> {
        let (inner, engine_name, compiler_name) = match engine {
            Some(engine) => {
                if let Ok(universal) = engine.downcast::<PyCell<engines::Universal>>() {
                    let universal = universal.borrow();

                    (
                        wasmer::Store::new(universal.inner()),
                        engines::Universal::name(),
                        universal.compiler_name().cloned(),
                    )
                } else if let Ok(dylib) = engine.downcast::<PyCell<engines::Dylib>>() {
                    let dylib = dylib.borrow();

                    (
                        wasmer::Store::new(dylib.inner()),
                        engines::Dylib::name(),
                        dylib.compiler_name().cloned(),
                    )
                } else {
                    return Err(to_py_err::<PyTypeError, _>("Unknown engine"));
                }
            }

            // No engine?
            None => {
                // This package embeds the `Universal` engine, we are going
                // to use it. We may want to load a compiler with it,
                // otherwise it's going to be a headless engine.
                let compiler = py
                    // Which compiler is available?
                    .import("wasmer_compiler_cranelift")
                    .or_else(|_| py.import("wasmer_compiler_llvm"))
                    .or_else(|_| py.import("wasmer_compiler_singlepass"))
                    // If any, load the `Compiler` class.
                    .and_then(|compiler_module| compiler_module.getattr("Compiler"))
                    .ok();

                let target = None;
                let engine = engines::Universal::raw_new(compiler, target)?;

                (
                    wasmer::Store::new(engine.inner()),
                    engines::Universal::name(),
                    engine.compiler_name().cloned(),
                )
            }
        };

        Ok(Self {
            inner,
            engine_name: engine_name.to_string(),
            compiler_name,
        })
    }

    #[getter]
    fn engine_name(&self) -> &String {
        &self.engine_name
    }

    #[getter]
    fn compiler_name(&self) -> Option<&String> {
        self.compiler_name.as_ref()
    }
}
