//! Memory API of an WebAssembly instance.

use pyo3::{exceptions::RuntimeError, prelude::*};
use std::rc::Rc;
use wasmer_runtime::Memory as WasmMemory;
use wasmer_runtime_core::units::Pages;

pub mod buffer;
pub mod view;

#[pyclass]
pub struct Memory {
    pub memory: Rc<WasmMemory>,
}

#[pymethods]
impl Memory {
    /// Return a Python buffer over the memory data.
    ///
    /// # Examples
    ///
    /// ```py
    /// instance = Instance(wasm_bytes)
    /// byte_array = bytearray(instance.memory.buffer)
    /// assert byte_array[0:6].decode() == 'Wasmer'
    /// ```
    #[getter]
    fn buffer(&self) -> buffer::Buffer {
        buffer::Buffer {
            memory: self.memory.clone(),
        }
    }

    /// Return a uint8 view over the memory data.
    #[text_signature = "($self, offset=0)"]
    #[args(offset = 0)]
    fn uint8_view(&self, offset: usize) -> view::Uint8Array {
        view::Uint8Array {
            memory: self.memory.clone(),
            offset,
        }
    }

    /// Return a int8 view over the memory data.
    #[text_signature = "($self, offset=0)"]
    #[args(offset = 0)]
    fn int8_view(&self, offset: usize) -> view::Int8Array {
        view::Int8Array {
            memory: self.memory.clone(),
            offset,
        }
    }

    /// Return a uint16 view over the memory data.
    #[text_signature = "($self, offset=0)"]
    #[args(offset = 0)]
    fn uint16_view(&self, offset: usize) -> view::Uint16Array {
        view::Uint16Array {
            memory: self.memory.clone(),
            offset,
        }
    }

    /// Return a int16 view over the memory data.
    #[text_signature = "($self, offset=0)"]
    #[args(offset = 0)]
    fn int16_view(&self, offset: usize) -> view::Int16Array {
        view::Int16Array {
            memory: self.memory.clone(),
            offset,
        }
    }

    /// Return a uint32 view over the memory data.
    #[text_signature = "($self, offset=0)"]
    #[args(offset = 0)]
    fn uint32_view(&self, offset: usize) -> view::Uint32Array {
        view::Uint32Array {
            memory: self.memory.clone(),
            offset,
        }
    }

    /// Return a int32 view over the memory data.
    #[text_signature = "($self, offset=0)"]
    #[args(offset = 0)]
    fn int32_view(&self, offset: usize) -> view::Int32Array {
        view::Int32Array {
            memory: self.memory.clone(),
            offset,
        }
    }

    /// Grow the memory by a number of pages.
    #[text_signature = "($self, number_of_pages)"]
    fn grow(&self, number_of_pages: u32) -> PyResult<u32> {
        self.memory
            .grow(Pages(number_of_pages))
            .map(|previous_pages| previous_pages.0)
            .map_err(|err| RuntimeError::py_err(format!("Failed to grow the memory: {}.", err)))
    }
}
