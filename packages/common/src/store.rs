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
/// tables and globals).
///
/// Specification: https://webassembly.github.io/spec/core/exec/runtime.html#store
#[pyclass]
pub struct Store {
    inner: wasmer::Store,
}

impl Store {
    pub fn inner(&self) -> &wasmer::Store {
        &self.inner
    }

    /*
    pub fn raw_with_compiler(
        compiler_config: impl wasmer_compiler::CompilerConfig + Send + Sync,
    ) -> Self {
        Store {
            inner: {
                let engine = wasmer::JIT::new(&compiler_config).engine();

                wasmer::Store::new(&engine)
            },
        }
    }
    */
}

#[pymethods]
impl Store {
    #[new]
    fn new(engine: Option<&PyAny>) -> PyResult<Self> {
        Ok(Self {
            inner: match engine {
                None => wasmer::Store::new(&wasmer::JIT::headless().engine()),
                Some(engine) => {
                    if let Ok(jit) = engine.downcast::<PyCell<engines::JIT>>() {
                        let jit = jit.borrow();

                        wasmer::Store::new(jit.inner())
                    } else if let Ok(native) = engine.downcast::<PyCell<engines::Native>>() {
                        let native = native.borrow();

                        wasmer::Store::new(native.inner())
                    } else {
                        return Err(TypeError::py_err("…"));
                    }
                }
            },
        })
    }
}
