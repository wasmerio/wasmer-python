//! The `Buffer` Python object to build WebAssembly values.

use crate::Shell;
use cpython::{PyObject, PyResult, Python};
use std::mem::size_of;
use wasmer_runtime::memory::Memory;

/// The `Buffer` Python object represents a WebAssembly value.
///
/// # Examples
///
/// ```python,ignore
/// from wasm import Instance
///
/// instance = Instance(bytes)
/// memory = instance.memory_view()
/// memory.set(7, 42)
/// print(memory.get(7)) // 42!
/// ```
py_class!(pub class MemoryView |py| {
    data memory: Shell<Memory>;
    data offset: usize;

    def length(&self) -> PyResult<usize> {
        let offset = *self.offset(py);

        Ok(self.memory(py).view::<u8>()[offset..].len() / size_of::<u8>())
    }

    def get(&self, index: usize) -> PyResult<u8> {
        let offset = *self.offset(py);
        let index = index / size_of::<u8>();

        Ok(self.memory(py).view::<u8>()[offset + index].get() as u8)
    }

    def set(&self, index: usize, value: u8) -> PyResult<PyObject> {
        let offset = *self.offset(py);
        let index = index / size_of::<u8>();

        self.memory(py).view::<u8>()[offset + index].set(value);

        Ok(Python::None(py))
    }
});

pub fn new_memory_view(py: Python, memory: Memory, offset: usize) -> MemoryView {
    MemoryView::create_instance(py, Shell::new(memory), offset).unwrap()
}
