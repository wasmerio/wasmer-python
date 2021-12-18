use crate::{errors::to_py_err, wasmer_inner::wasmer};
use pyo3::{
    class::PyMappingProtocol,
    exceptions::{PyIndexError, PyValueError},
    prelude::*,
    types::{PyAny, PySequence, PySlice},
};
use std::{convert::TryInto, iter::StepBy, ops::Range, os::raw::c_long};

enum ViewIndex {
    Slice(StepBy<Range<usize>>),
    Single(usize),
}

fn bounds_check(index: &PyAny, offset: usize, view_len: usize) -> PyResult<ViewIndex> {
    let actual_len = view_len
        .saturating_sub(offset)
        .try_into()
        .unwrap_or(c_long::MAX as isize);
    if let Ok(slice) = index.cast_as::<PySlice>() {
        let slice = slice.indices(actual_len as c_long)?;

        if slice.start > slice.stop {
            return Err(to_py_err::<PyIndexError, _>(format!(
                "Slice `{}:{}` cannot be empty",
                slice.start, slice.stop
            )));
        } else if slice.step < 1 {
            return Err(to_py_err::<PyIndexError, _>(format!(
                "Slice must have a positive step; given {}",
                slice.step
            )));
        } else if slice.start < 0 {
            return Err(to_py_err::<PyIndexError, _>(
                "Out of bound: Index cannot be negative",
            ));
        } else if slice.stop > actual_len {
            return Err(to_py_err::<PyIndexError, _>(format!(
                "Out of bound: Maximum index {} is larger than the view size {}",
                slice.stop - 1,
                actual_len
            )));
        }

        let range = (offset + slice.start as usize)..(offset + slice.stop as usize);
        Ok(ViewIndex::Slice(range.step_by(slice.step as usize)))
    } else if let Ok(index) = index.extract::<isize>() {
        if index < 0 {
            return Err(to_py_err::<PyIndexError, _>(
                "Out of bound: Index cannot be negative",
            ));
        } else if index >= actual_len {
            return Err(to_py_err::<PyIndexError, _>(format!(
                "Out of bound: Index {} is larger than the view size {}",
                index, actual_len
            )));
        }

        Ok(ViewIndex::Single(offset + index as usize))
    } else {
        Err(to_py_err::<PyValueError, _>(
            "Only integers and slices are valid to represent an index",
        ))
    }
}

macro_rules! memory_view {
    ($class_name:ident over $wasm_type:ty | $bytes_per_element:expr) => {
        /// Represents a read-and-write view over the data of a
        /// memory.
        ///
        /// It is built by the `Memory.uint8_view` and siblings getters.
        ///
        /// It implements the [Python mapping
        /// protocol][mapping-protocol], so it is possible to read and
        /// write bytes with a standard Python API.
        ///
        /// [mapping-protocol]: https://docs.python.org/3/c-api/mapping.html
        ///
        /// ## Example
        ///
        /// This is an example for the `Uint8Array` view, but it is
        /// the same for its siblings!
        ///
        /// ```py
        /// from wasmer import Store, Module, Instance, Uint8Array
        ///
        /// module = Module(Store(), open('tests/tests.wasm', 'rb').read())
        /// instance = Instance(module)
        /// exports = instance.exports
        ///
        /// pointer = exports.string()
        /// memory = exports.memory.uint8_view(offset=pointer)
        /// nth = 0
        /// string = ''
        ///
        /// while (0 != memory[nth]):
        ///     string += chr(memory[nth])
        ///     nth += 1
        ///
        /// assert string == 'Hello, World!'
        /// ```
        #[pyclass]
        pub struct $class_name {
            pub(crate) memory: wasmer::Memory,
            pub(crate) offset: usize,
        }

        #[pymethods]
        impl $class_name {
            /// Gets the number of bytes per element.
            #[getter]
            fn bytes_per_element(&self) -> u8 {
                $bytes_per_element
            }
        }

        #[pyproto]
        impl PyMappingProtocol for $class_name {
            /// Returns the length of the memory view.
            fn __len__(&self) -> PyResult<usize> {
                Ok(self.memory.view::<$wasm_type>()[self.offset..].len())
            }

            /// Returns one or more values from the memory view.
            ///
            /// The `index` can be either a slice or an integer.
            fn __getitem__(&self, index: &PyAny) -> PyResult<PyObject> {
                let gil = Python::acquire_gil();
                let py = gil.python();
                let view = self.memory.view::<$wasm_type>();
                match bounds_check(index, self.offset, view.len())? {
                    ViewIndex::Slice(iter) => Ok(iter
                        .map(|i| view[i].get())
                        .collect::<Vec<$wasm_type>>()
                        .into_py(py)),
                    ViewIndex::Single(index) => Ok(view[index].get().into_py(py)),
                }
            }

            /// Sets one or more values in the memory view.
            ///
            /// The `index` and `value` can only be of type slice and
            /// list, or integer and integer.
            fn __setitem__(&mut self, index: &PyAny, value: &PyAny) -> PyResult<()> {
                let view = self.memory.view::<$wasm_type>();
                match bounds_check(index, self.offset, view.len())? {
                    ViewIndex::Slice(iter) => {
                        let values = value.cast_as::<PySequence>()?;
                        let num_values = values.len()? as usize;
                        if num_values != iter.len() {
                            return Err(to_py_err::<PyIndexError, _>(format!(
                                "Sequence length {} doesn't match slice length {}",
                                num_values,
                                iter.len()
                            )));
                        }
                        for (src_idx, dst_idx) in iter.enumerate() {
                            let value = values.get_item(src_idx as isize)?;
                            let value = value.extract::<$wasm_type>()?;
                            view[dst_idx].set(value);
                        }
                    }
                    ViewIndex::Single(index) => {
                        let value = value.extract::<$wasm_type>()?;
                        view[index].set(value);
                    }
                }
                Ok(())
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
memory_view!(Uint64Array over u64|8);
memory_view!(Int64Array over i64|8);
memory_view!(Float32Array over f32|4);
memory_view!(Float64Array over f64|8);
