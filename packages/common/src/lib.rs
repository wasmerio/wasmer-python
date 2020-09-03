pub mod wasmer {
    pub use wasmer::*;
}

pub mod py {
    use pyo3::{exceptions::RuntimeError, prelude::*};

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
        inner: crate::wasmer::Store,
    }

    impl Store {
        pub fn inner(&self) -> &wasmer::Store {
            &self.inner
        }

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
    }

    #[pymethods]
    impl Store {
        #[new]
        fn new() -> Self {
            Self {
                inner: {
                    let engine = wasmer::JIT::headless().engine();

                    wasmer::Store::new(&engine)
                },
            }
        }

        #[staticmethod]
        #[text_signature = "($self, compiler_module)"]
        fn with_compiler(compiler_module: &PyModule) -> PyResult<Self> {
            let compiler = compiler_module.get("Compiler")?;
            let store = compiler.call_method0("into_store")?;
            let inner_store_ptr = store.call_method0("__inner_as_ptr")?.extract::<usize>()?;

            let inner_store_ptr: *const crate::wasmer::Store = inner_store_ptr as _;

            let store_inner_ref: &crate::wasmer::Store = unsafe {
                inner_store_ptr.as_ref().ok_or_else(|| {
                    RuntimeError::py_err("Failed to transfer the store from the compiler")
                })?
            };

            let store_inner: crate::wasmer::Store = store_inner_ref.clone();

            Ok(Store { inner: store_inner })
        }

        pub fn __inner_as_ptr(&self) -> usize {
            let store_ptr: *const crate::wasmer::Store = &self.inner;
            let store_usize: usize = store_ptr as _;

            store_usize
        }
    }
}
