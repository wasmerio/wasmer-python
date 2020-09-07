use crate::{engines, wasmer};
use pyo3::{exceptions::TypeError, prelude::*};

/// The store represents all global state that can be manipulated by
/// WebAssembly programs. It consists of the runtime representation of
/// all instances of functions, tables, memories, and globals that
/// have been allocated during the lifetime of the abstract machine.
///
/// The `Store` holds the engine (that is —amongst many things— used
/// to compile the WebAssembly bytes into a valid module artifact), in
/// addition to the `Tunables` (that are used to create the memories,
/// tables and globals). The engine comes from the `wasmer.engines`
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
        Ok(Self {
            inner: match engine {
                Some(engine) => {
                    if let Ok(jit) = engine.downcast::<PyCell<engines::JIT>>() {
                        let jit = jit.borrow();

                        wasmer::Store::new(jit.inner())
                    } else if let Ok(native) = engine.downcast::<PyCell<engines::Native>>() {
                        let native = native.borrow();

                        wasmer::Store::new(native.inner())
                    } else {
                        return Err(TypeError::py_err("Unknown engine"));
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

                    let engine = engines::JIT::raw_new(compiler)?;

                    wasmer::Store::new(engine.inner())
                }
            },
        })
    }
}
