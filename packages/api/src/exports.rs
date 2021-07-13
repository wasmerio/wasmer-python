use crate::{
    errors::to_py_err,
    externals::{Function, Global, Memory, Table},
    wasmer_inner::wasmer,
};
use pyo3::{
    class::{basic::PyObjectProtocol, iter::PyIterProtocol, sequence::PySequenceProtocol},
    exceptions::PyLookupError,
    prelude::*,
};

/// Represents all the exports of an instance. It is built by
/// `Instance.exports`.
///
/// Exports can be of kind `Function`, `Global`, `Table`, or `Memory`.
///
/// The `Exports` class implement [the Iterator
/// Protocol](https://docs.python.org/3/c-api/iter.html). Please see
/// the `ExportsIterator` class.
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
    fn __getattr__(&self, key: &str) -> PyResult<PyObject> {
        let gil_guard = Python::acquire_gil();
        let py = gil_guard.python();

        Ok(match self.inner.get_extern(key) {
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
                return Err(to_py_err::<PyLookupError, _>(format!(
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

#[pyproto]
impl PyIterProtocol for Exports {
    fn __iter__(slf: PyRef<Self>) -> ExportsIterator {
        ExportsIterator {
            vector: slf
                .inner
                .iter()
                .map(|(name, export)| (name.clone(), export.clone()))
                .collect(),
            index: 0,
        }
    }
}

/// Iterator over all the exports of an `Instance`.
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
///
/// assert [name for (name, export) in instance.exports] == ["func", "glob", "tab", "mem"]
/// ```
#[pyclass]
pub struct ExportsIterator {
    vector: Vec<(String, wasmer::Extern)>,
    index: usize,
}

#[pyproto]
impl PyIterProtocol for ExportsIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<(String, PyObject)>> {
        let gil_guard = Python::acquire_gil();
        let py = gil_guard.python();

        let (output, next_index) = match slf.vector.get(slf.index) {
            Some((name, export)) => (
                Ok(Some((
                    name.clone(),
                    match export {
                        wasmer::Extern::Function(function) => {
                            Py::new(py, Function::raw_new(function.clone()))?.to_object(py)
                        }
                        wasmer::Extern::Global(global) => {
                            Py::new(py, Global::raw_new(global.clone()))?.to_object(py)
                        }
                        wasmer::Extern::Memory(memory) => {
                            Py::new(py, Memory::raw_new(memory.clone()))?.to_object(py)
                        }
                        wasmer::Extern::Table(table) => {
                            Py::new(py, Table::raw_new(table.clone()))?.to_object(py)
                        }
                    },
                ))),
                slf.index + 1,
            ),

            None => (Ok(None), slf.index),
        };

        slf.index = next_index;

        output
    }
}
