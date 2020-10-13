use crate::{
    errors::to_py_err, store::Store, types::TableType, values::Value, wasmer_inner::wasmer,
};
use pyo3::{exceptions::PyRuntimeError, prelude::*};

/// A WebAssembly table instance.
///
/// The `Table` class is an array-like structure representing a
/// WebAssembly table, which stores function references.
///
/// A table created by the host or in WebAssembly code will be
/// accessible and mutable from both host and WebAssembly.
///
/// Specification: https://webassembly.github.io/spec/core/exec/runtime.html#table-instances
#[pyclass(unsendable)]
#[text_signature = "(store, table_type, initial_value)"]
pub struct Table {
    inner: wasmer::Table,
}

impl Table {
    pub fn raw_new(inner: wasmer::Table) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &wasmer::Table {
        &self.inner
    }
}

#[pymethods]
impl Table {
    #[new]
    fn new(store: &Store, table_type: &TableType, initial_value: &Value) -> PyResult<Self> {
        Ok(Self {
            inner: wasmer::Table::new(
                store.inner(),
                table_type.into(),
                initial_value.inner().clone(),
            )
            .map_err(to_py_err::<PyRuntimeError, _>)?,
        })
    }

    /// Gets the table size (in elements).
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import Store, Module, Instance, Table
    ///
    /// module = Module(Store(), '(module (table (export "table") 2 funcref))')
    /// instance = Instance(module)
    /// table = instance.exports.table
    ///
    /// assert table.size == 2
    /// ```
    #[getter]
    fn size(&self) -> u32 {
        self.inner.size()
    }

    /// Gets the table type, as an object of kind `TableType`.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import Store, Module, Instance, Table, Type
    ///
    /// module = Module(Store(), '(module (table (export "table") 2 funcref))')
    /// instance = Instance(module)
    /// table = instance.exports.table
    /// table_type = table.type
    ///
    /// assert table_type == Type.FUNC_REF
    /// assert table_type.minimum == 0
    /// assert table_type.maximum == None
    /// ```
    #[getter(type)]
    fn ty(&self) -> TableType {
        self.inner.ty().into()
    }
}
