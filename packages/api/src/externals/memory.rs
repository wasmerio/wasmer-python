use crate::{
    errors::to_py_err,
    memory::{Buffer, Int16Array, Int32Array, Int8Array, Uint16Array, Uint32Array, Uint8Array},
    store::Store,
    types::MemoryType,
    wasmer_inner::wasmer,
};
use pyo3::{exceptions::PyRuntimeError, prelude::*};

/// A WebAssembly memory instance.
///
/// A memory instance is the runtime representation of a linear
/// memory. It consists of a vector of bytes and an optional maximum
/// size.
///
/// The length of the vector always is a multiple of the WebAssembly
/// page size, which is defined to be the constant 65536 – abbreviated
/// 64Ki. Like in a memory type, the maximum size in a memory
/// instance is given in units of this page size.
///
/// A memory created by the host or in WebAssembly code will be accessible and
/// mutable from both host and WebAssembly.
///
/// Specification: https://webassembly.github.io/spec/core/exec/runtime.html#memory-instances
///
/// ## Example
///
/// Creates a `Memory` from scratch:
///
/// ```py
/// from wasmer import Store, Memory, MemoryType
///
/// store = Store()
/// memory_type = MemoryType(3, shared=False)
/// memory = Memory(store, memory_type)
///
/// assert memory.size == 3
/// ```
///
/// Gets a memory from the exports of an instance:
///
/// ```py
/// from wasmer import Store, Module, Instance, Memory
///
/// module = Module(Store(), open('tests/tests.wasm', 'rb').read())
/// instance = Instance(module)
///
/// memory = instance.exports.memory
///
/// assert isinstance(memory, Memory)
/// ```
#[pyclass(unsendable)]
#[text_signature = "(store, memory_type)"]
pub struct Memory {
    inner: wasmer::Memory,
}

impl Memory {
    pub fn raw_new(inner: wasmer::Memory) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &wasmer::Memory {
        &self.inner
    }
}

#[pymethods]
impl Memory {
    #[new]
    fn new(store: &Store, memory_type: &MemoryType) -> PyResult<Self> {
        Ok(Self::raw_new(
            wasmer::Memory::new(store.inner(), memory_type.into())
                .map_err(to_py_err::<PyRuntimeError, _>)?,
        ))
    }

    /// Returns the size (in pages) of the `Memory`.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import Store, Module, Instance
    ///
    /// module = Module(Store(), open('tests/tests.wasm', 'rb').read())
    /// instance = Instance(module)
    /// memory = instance.exports.memory
    ///
    /// assert memory.size == 17
    /// ```
    #[getter]
    fn size(&self) -> u32 {
        self.inner.size().0
    }

    /// Returns the size (in bytes) of the `Memory`.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import Store, Memory, MemoryType
    ///
    /// store = Store()
    /// memory_type = MemoryType(3, shared=False)
    /// memory = Memory(store, memory_type)
    ///
    /// assert memory.data_size == 196608
    /// ```
    #[getter]
    fn data_size(&self) -> u64 {
        self.inner.data_size()
    }

    /// Grow memory by the specified amount of WebAssembly pages.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import Store, Memory, MemoryType
    ///
    /// store = Store()
    /// memory_type = MemoryType(3, shared=False)
    /// memory = Memory(store, memory_type)
    ///
    /// assert memory.size == 3
    ///
    /// memory.grow(2)
    ///
    /// assert memory.size == 5
    /// ```
    #[text_signature = "($self, number_of_pages)"]
    fn grow(&self, number_of_pages: u32) -> PyResult<u32> {
        self.inner
            .grow(number_of_pages)
            .map(|pages| pages.0)
            .map_err(to_py_err::<PyRuntimeError, _>)
    }

    /// Creates a Python buffer to read and write the memory data. See
    /// the `Buffer` class to learn more.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import Store, Memory, MemoryType, Buffer
    ///
    /// store = Store()
    /// memory_type = MemoryType(3, shared=False)
    /// memory = Memory(store, memory_type)
    ///
    /// assert isinstance(memory.buffer, Buffer)
    /// ```
    #[getter]
    fn buffer(&self) -> Buffer {
        Buffer::new(self.inner.clone())
    }

    /// Creates a read-and-write view over the memory data where
    /// elements are of kind `uint8`. See the `Uint8Array` view to
    /// learn more.
    ///
    /// ## Examples
    ///
    /// ```py
    /// from wasmer import Store, Memory, MemoryType, Uint8Array
    ///
    /// store = Store()
    /// memory_type = MemoryType(3, shared=False)
    /// memory = Memory(store, memory_type)
    ///
    /// assert isinstance(memory.uint8_view(offset=42), Uint8Array)
    /// ```
    #[text_signature = "($self, /, offset=0)"]
    #[args(offset = 0)]
    fn uint8_view(&self, offset: usize) -> Uint8Array {
        Uint8Array {
            memory: self.inner.clone(),
            offset,
        }
    }

    /// Creates a read-and-write over the memory data where elements
    /// are of kind `int8`. See the `Int8Array` view to learn more,
    /// and the `Memory.uint8_view` method to see an example.
    #[text_signature = "($self, /, offset=0)"]
    #[args(offset = 0)]
    fn int8_view(&self, offset: usize) -> Int8Array {
        Int8Array {
            memory: self.inner.clone(),
            offset,
        }
    }

    /// Creates a read-and-write over the memory data where elements
    /// are of kind `uint16`. See the `Uint16Array` view to learn
    /// more, and the `Memory.uint8_view` method to see an example.
    #[text_signature = "($self, /, offset=0)"]
    #[args(offset = 0)]
    fn uint16_view(&self, offset: usize) -> Uint16Array {
        Uint16Array {
            memory: self.inner.clone(),
            offset,
        }
    }

    /// Creates a read-and-write over the memory data where elements
    /// are of kind `int16`. See the `Int16Array` view to learn more,
    /// and the `Memory.uint8_view` method to see an example.
    #[text_signature = "($self, /, offset=0)"]
    #[args(offset = 0)]
    fn int16_view(&self, offset: usize) -> Int16Array {
        Int16Array {
            memory: self.inner.clone(),
            offset,
        }
    }

    /// Creates a read-and-write over the memory data where elements
    /// are of kind `uint32`. See the `Uint32Array` view to learn
    /// more, and the `Memory.uint8_view` method to see an example.
    #[text_signature = "($self, /, offset=0)"]
    #[args(offset = 0)]
    fn uint32_view(&self, offset: usize) -> Uint32Array {
        Uint32Array {
            memory: self.inner.clone(),
            offset,
        }
    }

    /// Creates a read-and-write over the memory data where elements
    /// are of kind `int32`. See the `Int32Array` view to learn more,
    /// and the `Memory.uint8_view` method to see an example.
    #[text_signature = "($self, /, offset=0)"]
    #[args(offset = 0)]
    fn int32_view(&self, offset: usize) -> Int32Array {
        Int32Array {
            memory: self.inner.clone(),
            offset,
        }
    }

    /// Gets the memory type, of kind `MemoryType`.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import Store, Memory, Module, Instance
    ///
    /// module = Module(Store(), open('tests/tests.wasm', 'rb').read())
    /// instance = Instance(module)
    /// memory = instance.exports.memory
    /// memory_type = memory.type
    ///
    /// assert memory_type.minimum == 17
    /// assert memory_type.maximum == None
    /// assert memory_type.shared == False
    /// ```
    #[getter(type)]
    fn ty(&self) -> MemoryType {
        self.inner.ty().into()
    }
}
