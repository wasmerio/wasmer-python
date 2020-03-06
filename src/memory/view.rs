//! The `*Array` Python objects to represent WebAsembly memory views.

use pyo3::{
    class::PyMappingProtocol,
    exceptions::{IndexError, RuntimeError, ValueError},
    prelude::*,
    types::{PyAny, PyInt, PyLong, PySequence, PySlice},
};
use std::{cmp::min, mem::size_of, ops::Range, rc::Rc};
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
            /// Returns the length of the memory view.
            fn __len__(&self) -> PyResult<usize> {
                Ok(self.memory.view::<$wasm_type>()[self.offset..].len() / size_of::<$wasm_type>())
            }

            /// Returns one or more values from the memory view.
            ///
            /// The `index` can be either a slice or an integer.
            fn __getitem__(&self, index: &PyAny) -> PyResult<PyObject> {
                let view = self.memory.view::<$wasm_type>();
                let offset = self.offset;
                let range = if let Ok(slice) = index.cast_as::<PySlice>() {
                    let slice = slice.indices(view.len() as _)?;

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

                    (offset + slice.start as usize)..(min(offset + slice.stop as usize, view.len()))
                } else if let Ok(index) = index.extract::<isize>() {
                    if index < 0 {
                        return Err(IndexError::py_err(
                            "Out of bound: Index cannot be negative.",
                        ));
                    }

                    let index = offset + index as usize;

                    #[allow(clippy::range_plus_one)]
                    // Writing `index..=index` makes Clippy happy but
                    // the type of this expression is
                    // `RangeInclusive`, when the type of `range` is
                    // `Range`.
                    {
                        index..index + 1
                    }
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
                    Ok(view[range.start].get().into_py(py))
                } else {
                    Ok(view[range]
                        .iter()
                        .map(|cell| cell.get())
                        .collect::<Vec<$wasm_type>>()
                        .into_py(py))
                }
            }

            /// Sets one or more values in the memory view.
            ///
            /// The `index` and `value` can only be of type slice and
            /// list, or integer and integer.
            fn __setitem__(&mut self, index: &PyAny, value: &PyAny) -> PyResult<()> {
                let offset = self.offset;
                let view = self.memory.view::<$wasm_type>();

                if let (Ok(slice), Ok(values)) = (
                    index.cast_as::<PySlice>(),
                    value
                        .cast_as::<PySequence>()
                        .map_err(|_| {
                            RuntimeError::py_err(
                                "Failed to downcast `value` to a Python sequence.",
                            )
                        })
                        .and_then(|sequence| sequence.list()),
                ) {
                    let slice = slice.indices(view.len() as _)?;

                    if slice.start >= slice.stop {
                        return Err(IndexError::py_err(format!(
                            "Slice `{}:{}` cannot be empty.",
                            slice.start, slice.stop
                        )));
                    } else if slice.step < 1 {
                        return Err(IndexError::py_err(format!(
                            "Slice must have a positive step; given {}.",
                            slice.step
                        )));
                    }

                    let iterator = Range {
                        start: slice.start,
                        end: slice.stop,
                    }
                    .step_by(slice.step as usize);

                    // Normally unreachable since the slice is bound
                    // to the size of the memory view.
                    if iterator.len() > view.len() {
                        return Err(IndexError::py_err(format!(
                            "Out of bound: The given key slice will write out of memory; memory length is {}, memory offset is {}, slice length is {}.",
                            view.len(),
                            offset,
                            iterator.len()
                        )));
                    }

                    for (index, value) in iterator.zip(values.iter()) {
                        let index = index as usize;
                        let value = value.extract::<$wasm_type>()?;

                        view[offset + index].set(value);
                    }

                    Ok(())
                } else if let (Ok(index), Ok(value)) = (
                    index
                        .cast_as::<PyLong>()
                        .map_err(|_| {
                            RuntimeError::py_err(
                                "Failed to downcast `index` to a Python long value.",
                            )
                        })
                        .and_then(|pylong| pylong.extract::<isize>()),
                    value
                        .cast_as::<PyInt>()
                        .map_err(|_| {
                            RuntimeError::py_err(
                                "Failed to downcast `value` to a Python int value.",
                            )
                        })
                        .and_then(|pyint| pyint.extract::<$wasm_type>()),
                ) {
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
                } else {
                    Err(RuntimeError::py_err("When setting data to the memory view, the index and the value can only have the following types: Either `int` and `int`, or `slice` and `sequence`."))
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
