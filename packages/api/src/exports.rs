use crate::{
    errors::to_py_err,
    externals::{Function, Global, Memory, Table},
    wasmer_inner::wasmer,
};
use pyo3::{
    class::{basic::PyObjectProtocol, sequence::PySequenceProtocol},
    exceptions::LookupError,
    prelude::*,
};

/// Represents all the exports of an instance. It is built by
/// `Instance.exports`.
///
/// Exports can be of kind `Function`, `Global`, `Table`, or `Memory`.
///
/// ## Example
///
/// ```py
/// from wasmer import Store, Module, Instance, Exports, Function, Global, Table, Memory
///
/// module = Module(
///     Store(),
///     """
///     (module
///       (func (export "func") (param i32 i64))
///       (global (export "glob") i32 (i32.const 7))
///       (table (export "tab") 0 funcref)
///       (memory (export "mem") 1))
///     """
/// )
/// instance = Instance(module)
/// exports = instance.exports
///
/// assert isinstance(exports, Exports)
/// assert isinstance(exports.func, Function)
/// assert isinstance(exports.glob, Global)
/// assert isinstance(exports.tab, Table)
/// assert isinstance(exports.mem, Memory)
/// ```
#[pyclass(unsendable)]
#[derive(Clone)]
pub struct Exports {
    inner: wasmer::Exports,
}

impl Exports {
    pub fn new(inner: wasmer::Exports) -> Self {
        Self { inner }
    }
}

#[pyproto]
impl PyObjectProtocol for Exports {
    fn __getattr__(&self, key: String) -> PyResult<PyObject> {
        let gil_guard = Python::acquire_gil();
        let py = gil_guard.python();

        Ok(match self.inner.get_extern(key.as_str()) {
            Some(wasmer::Extern::Function(function)) => {
                Py::new(py, Function::raw_new(function.clone()))?.to_object(py)
            }
            Some(wasmer::Extern::Global(global)) => {
                Py::new(py, Global::raw_new(global.clone()))?.to_object(py)
            }
            Some(wasmer::Extern::Memory(memory)) => {
                Py::new(py, Memory::raw_new(memory.clone()))?.to_object(py)
            }
            Some(wasmer::Extern::Table(table)) => {
                Py::new(py, Table::raw_new(table.clone()))?.to_object(py)
            }
            _ => {
                return Err(to_py_err::<LookupError, _>(format!(
                    "Export `{}` does not exist.",
                    key
                )))
            }
        })
    }
}

#[pyproto]
impl PySequenceProtocol for Exports {
    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
