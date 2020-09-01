use crate::wasmer_inner::wasmer;
use pyo3::prelude::*;

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
///
/// ## Example
///
/// ```py
/// from wasmer import Store
///
/// store = Store()
/// ```
#[pyclass]
#[text_signature = "(/)"]
pub struct Store {
    inner: wasmer::Store,
}

impl Store {
    pub(crate) fn inner(&self) -> &wasmer::Store {
        &self.inner
    }
}

#[pymethods]
impl Store {
    #[new]
    fn new() -> Self {
        Store {
            inner: wasmer::Store::default(),
        }
    }
}
