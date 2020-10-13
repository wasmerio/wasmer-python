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
/// ## Example
///
/// Use the JIT engine with no compiler (headless mode):
///
/// ```py
/// from wasmer import engine, Store
///
/// store = Store(engine.JIT())
/// ```
///
/// Use the JIT engine with the LLVM compiler:
///
/// ```py
/// from wasmer import engine, Store
/// from wasmer_compiler_llvm import Compiler
///
/// store = Store(engine.JIT(Compiler))
/// ```
///
/// If the store is built without an engine, the JIT engine will be
/// used, with the first compiler found in this order:
/// `compiler_compiler_cranelift`, `compiler_compiler_llvm`,
/// `compiler_compiler_singlepass`, otherwise it will run in headless
/// mode.
#[pyclass]
#[text_signature = "(engine)"]
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
                if let Ok(jit) = engine.downcast::<PyCell<engines::JIT>>() {
                    let jit = jit.borrow();

                    (
                        wasmer::Store::new(jit.inner()),
                        engines::JIT::name(),
                        jit.compiler_name().cloned(),
                    )
                } else if let Ok(native) = engine.downcast::<PyCell<engines::Native>>() {
                    let native = native.borrow();

                    (
                        wasmer::Store::new(native.inner()),
                        engines::Native::name(),
                        native.compiler_name().cloned(),
                    )
                } else {
                    return Err(to_py_err::<PyTypeError, _>("Unknown engine"));
                }
            }

            // No engine?
            None => {
                // This package embeds the `JIT` engine, we are
                // going to use it. We want to load a
                // compiler with it.
                let compiler = py
                    // Which compiler is available?
                    .import("wasmer_compiler_cranelift")
                    .or_else(|_| py.import("wasmer_compiler_llvm"))
                    .or_else(|_| py.import("wasmer_compiler_singlepass"))
                    // If any, load the `Compiler` class.
                    .and_then(|compiler_module| compiler_module.get("Compiler"))
                    .ok();

                let target = None;
                let engine = engines::JIT::raw_new(compiler, target)?;

                (
                    wasmer::Store::new(engine.inner()),
                    engines::JIT::name(),
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
