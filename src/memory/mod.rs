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
    #[getter]
    fn buffer(&self, py: Python) -> PyResult<Py<buffer::Buffer>> {
        Py::new(
            py,
            buffer::Buffer {
                memory: self.memory.clone(),
            },
        )
    }

    #[args(offset = 0)]
    fn uint8_view(&self, py: Python, offset: usize) -> PyResult<Py<view::Uint8Array>> {
        Py::new(
            py,
            view::Uint8Array {
                memory: self.memory.clone(),
                offset,
            },
        )
    }

    #[args(offset = 0)]
    fn int8_view(&self, py: Python, offset: usize) -> PyResult<Py<view::Int8Array>> {
        Py::new(
            py,
            view::Int8Array {
                memory: self.memory.clone(),
                offset,
            },
        )
    }

    #[args(offset = 0)]
    fn uint16_view(&self, py: Python, offset: usize) -> PyResult<Py<view::Uint16Array>> {
        Py::new(
            py,
            view::Uint16Array {
                memory: self.memory.clone(),
                offset,
            },
        )
    }

    #[args(offset = 0)]
    fn int16_view(&self, py: Python, offset: usize) -> PyResult<Py<view::Int16Array>> {
        Py::new(
            py,
            view::Int16Array {
                memory: self.memory.clone(),
                offset,
            },
        )
    }

    #[args(offset = 0)]
    fn uint32_view(&self, py: Python, offset: usize) -> PyResult<Py<view::Uint32Array>> {
        Py::new(
            py,
            view::Uint32Array {
                memory: self.memory.clone(),
                offset,
            },
        )
    }

    #[args(offset = 0)]
    fn int32_view(&self, py: Python, offset: usize) -> PyResult<Py<view::Int32Array>> {
        Py::new(
            py,
            view::Int32Array {
                memory: self.memory.clone(),
                offset,
            },
        )
    }

    fn grow(&self, number_of_pages: u32) -> PyResult<u32> {
        self.memory
            .grow(Pages(number_of_pages))
            .map(|previous_pages| previous_pages.0)
            .map_err(|err| RuntimeError::py_err(format!("Failed to grow the memory: {}.", err)))
    }
}
