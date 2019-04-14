//! The `Buffer` Python object to build WebAssembly values.

use pyo3::{
    class::PyMappingProtocol,
    exceptions::{IndexError, ValueError},
    prelude::*,
    types::{PyAny, PySlice},
};
use std::{mem::size_of, rc::Rc};
use wasmer_runtime::memory::Memory;

macro_rules! memory_view {
    ($class_name:ident over $wasm_type:ty | $bytes_per_element:expr) => {
        #[pyclass]
        pub struct $class_name {
            pub memory: Rc<Memory>,
            pub offset: usize,
        }

        #[pymethods]
        impl $class_name {
            #[getter]
            fn bytes_per_element(&self) -> PyResult<u8> {
                Ok($bytes_per_element)
            }
        }

        #[pyproto]
        impl PyMappingProtocol for $class_name {
            fn __len__(&self) -> PyResult<usize> {
                Ok(self.memory.view::<$wasm_type>()[self.offset..].len() / size_of::<$wasm_type>())
            }

            fn __getitem__(&self, index: &PyAny) -> PyResult<PyObject> {
                let view = self.memory.view::<$wasm_type>();
                let offset = self.offset;
                let range = if let Ok(slice) = index.cast_as::<PySlice>() {
                    let slice = slice.indices(view.len() as i64)?;

                    if slice.start >= slice.stop {
                        return Err(IndexError::py_err(format!(
                            "Slice `{}:{}` cannot be empty.",
                            slice.start, slice.stop
                        )));
                    } else if slice.step > 1 {
                        return Err(IndexError::py_err(format!(
                            "Slice must have a step of 1 for now; given {}.",
                            slice.step
                        )));
                    }

                    (offset + slice.start as usize)..(offset + slice.stop as usize)
                } else if let Ok(index) = index.extract::<isize>() {
                    if index < 0 {
                        return Err(IndexError::py_err(
                            "Out of bound: Index cannot be negative.",
                        ));
                    }

                    (offset + index as usize)..(offset + index as usize + 1)
                } else {
                    return Err(ValueError::py_err(
                        "Only integers and slices are valid to represent an index.",
                    ));
                };

                if view.len() <= (range.end - 1) {
                    return Err(IndexError::py_err(format!(
                        "Out of bound: Maximum index {} is larger than the memory size {}.",
                        range.end - 1,
                        view.len()
                    )));
                }

                let gil = GILGuard::acquire();
                let py = gil.python();

                if range.end - range.start == 1 {
                    Ok(view[range.start].get().into_object(py))
                } else {
                    Ok(view[range]
                        .iter()
                        .map(|cell| cell.get())
                        .collect::<Vec<$wasm_type>>()
                        .into_object(py))
                }
            }

            fn __setitem__(&mut self, index: isize, value: u8) -> PyResult<()> {
                let offset = self.offset;
                let view = self.memory.view::<u8>();

                if index < 0 {
                    return Err(IndexError::py_err(
                        "Out of bound: Index cannot be negative.",
                    ));
                }

                let index = index as usize;

                if view.len() <= offset + index {
                    Err(IndexError::py_err(format!(
                        "Out of bound: Absolute index {} is larger than the memory size {}.",
                        offset + index,
                        view.len()
                    )))
                } else {
                    view[offset + index].set(value);

                    Ok(())
                }
            }
        }
    };
}

memory_view!(Uint8Array over u8|1);
memory_view!(Int8Array over i8|1);
memory_view!(Uint16Array over u16|2);
memory_view!(Int16Array over i16|2);
memory_view!(Uint32Array over u32|4);
memory_view!(Int32Array over i32|4);
