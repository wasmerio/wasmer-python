//! The `Buffer` Python object to represent a WebAssembly memory
//! through the Python Buffer Protocol.

use pyo3::{
    class::buffer::PyBufferProtocol,
    exceptions::BufferError,
    ffi::{PyBUF_FORMAT, PyBUF_ND, PyBUF_STRIDES, PyBUF_WRITABLE, Py_buffer},
    prelude::*,
};
use std::{
    ffi::{c_void, CStr},
    mem,
    ops::Deref,
    os::raw::{c_char, c_int},
    ptr,
    rc::Rc,
};
use wasmer_runtime::memory::Memory;

#[pyclass]
pub struct Buffer {
    pub memory: Rc<Memory>,
}

#[pyproto]
impl PyBufferProtocol for Buffer {
    fn bf_getbuffer(slf: PyRefMut<Self>, view: *mut Py_buffer, flags: c_int) -> PyResult<()> {
        if view.is_null() {
            return Err(BufferError::py_err(
                "`Py_buffer` cannot be filled because it is null.",
            ));
        }

        let memory_view = slf.memory.view::<u8>();

        // Fill `Py_buffer` according to https://docs.python.org/3/c-api/buffer.html.
        unsafe {
            // A pointer to the start of the logical structure
            // described by the buffer fields. This can be any
            // location within the underlying physical memory block of
            // the exporter. For example, with negative strides the
            // value may point to the end of the memory block.
            //
            // For contiguous arrays, the value points to the
            // beginning of the memory block.
            (*view).buf = memory_view.deref().as_ptr() as *mut c_void;

            // A new reference to the exporting object. The reference
            // is owned by the consumer and automatically decremented
            // and set to `NULL` by `PyBuffer_Release()`. The field is the
            // equivalent of the return value of any standard C-API
            // function.
            //
            // As a special case, for temporary buffers that are
            // wrapped by `PyMemoryView_FromBuffer()` or
            // `PyBuffer_FillInfo()` this field is `NULL`. In general,
            // exporting objects MUST NOT use this scheme.
            (*view).obj = ptr::null_mut();

            // `product(shape) * itemsize`. For contiguous arrays,
            // this is the length of the underlying memory block. For
            // non-contiguous arrays, it is the length that the
            // logical structure would have if it were copied to a
            // contiguous representation.
            //
            // Accessing `((char *)buf)[0]` up to `((char *)buf)[len-1]`
            // is only valid if the buffer has been obtained by a
            // request that guarantees contiguity. In most cases such
            // a request will be `PyBUF_SIMPLE` or `PyBUF_WRITABLE`.
            (*view).len = memory_view.len() as isize;

            // An indicator of whether the buffer is read-only. This
            // field is controlled by the `PyBUF_WRITABLE` flag.
            (*view).readonly = if PyBUF_WRITABLE == (flags & PyBUF_WRITABLE) {
                0
            } else {
                1
            };

            // Item size in bytes of a single element. Same as the
            // value of `struct.calcsize()` called on non-`NULL`
            // format values.
            //
            // Important exception: If a consumer requests a buffer
            // without the `PyBUF_FORMAT` flag, format will be set to
            // `NULL`, but `itemsize` still has the value for the
            // original format.
            //
            // If `shape` is present, the equality `product(shape) *
            // itemsize == len` still holds and the consumer can use
            // `itemsize` to navigate the buffer.
            //
            // If `shape` is `NULL` as a result of a `PyBUF_SIMPLE` or
            // a `PyBUF_WRITABLE` request, the consumer must disregard
            // `itemsize` and assume `itemsize == 1`.
            (*view).itemsize = mem::size_of::<u8>() as isize;

            // A `NUL` terminated string in `struct` module style
            // syntax describing the contents of a single item. If
            // this is `NULL`, `"B"` (unsigned bytes) is assumed.
            //
            // This field is controlled by the `PyBUF_FORMAT` flag.
            (*view).format = if PyBUF_FORMAT == (flags & PyBUF_FORMAT) {
                let format = CStr::from_bytes_with_nul(b"B\0")
                    .expect("The format must be a valid `NUL` terminated string.");

                format.as_ptr() as *mut c_char
            } else {
                ptr::null_mut()
            };

            // The number of dimensions the memory represents as an
            // n-dimensional array. If it is `0`, `buf` points to a
            // single item representing a scalar. In this case,
            // `shape`, `strides` and `suboffsets` MUST be `NULL`.
            //
            // The macro `PyBUF_MAX_NDIM` limits the maximum number of
            // dimensions to 64. Exporters MUST respect this limit,
            // consumers of multi-dimensional buffers SHOULD be able
            // to handle up to `PyBUF_MAX_NDIM` dimensions.
            (*view).ndim = 1;

            // An array of `Py_ssize_t` of length `ndim` indicating
            // the shape of the memory as an n-dimensional array. Note
            // that `shape[0] * ... * shape[ndim-1] * itemsize` MUST
            // be equal to `len`.
            //
            // Shape values are restricted to `shape[n] >= 0`. The
            // case `shape[n] == 0` requires special attention. See
            // complex arrays for further information.
            //
            // The shape array is read-only for the consumer.
            (*view).shape = if PyBUF_ND == (flags & PyBUF_ND) {
                &((*view).len) as *const isize as *mut isize
            } else {
                ptr::null_mut()
            };

            // An array of `Py_ssize_t` of length `ndim` giving the
            // number of bytes to skip to get to a new element in each
            // dimension.
            //
            // Stride values can be any integer. For regular arrays,
            // strides are usually positive, but a consumer MUST be
            // able to handle the case `strides[n] <= 0`. See complex
            // arrays for further information.
            //
            // The stride array is read-only for the consumer.
            (*view).strides = if PyBUF_STRIDES == (flags & PyBUF_STRIDES) {
                &((*view).itemsize) as *const isize as *mut isize
            } else {
                ptr::null_mut()
            };

            // An array of `Py_ssize_t` of length `ndim`. If
            // `suboffsets[n] >= 0`, the values stored along the nth
            // dimension are pointers and the suboffset value dictates
            // how many bytes to add to each pointer after
            // de-referencing. A suboffset value that is negative
            // indicates that no de-referencing should occur (striding
            // in a contiguous memory block).
            //
            // If all suboffsets are negative (i.e. no de-referencing
            // is needed), then this field must be `NULL` (the default
            // value).
            //
            // This type of array representation is used by the Python
            // Imaging Library (PIL). See complex arrays for further
            // information how to access elements of such an array.
            //
            // The suboffsets array is read-only for the consumer.
            (*view).suboffsets = ptr::null_mut();

            // This is for use internally by the exporting object. For
            // example, this might be re-cast as an integer by the
            // exporter and used to store flags about whether or not
            // the shape, strides, and suboffsets arrays must be freed
            // when the buffer is released. The consumer MUST NOT
            // alter this value.
            (*view).internal = ptr::null_mut();
        }

        Ok(())
    }

    fn bf_releasebuffer(_slf: PyRefMut<Self>, _view: *mut Py_buffer) -> PyResult<()> {
        Ok(())
    }
}
