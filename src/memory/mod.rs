//! Memory API of an WebAssembly instance.

use pyo3::prelude::*;
use std::rc::Rc;
use wasmer_runtime::Memory as WasmMemory;

pub mod view;

#[pyclass]
pub struct Memory {
    pub memory: Rc<WasmMemory>,
}

#[pymethods]
impl Memory {
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
}
