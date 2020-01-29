//! The `ExportedTables` and `ExportedTable` objects.

use pyo3::{
    class::basic::PyObjectProtocol,
    exceptions::{LookupError, RuntimeError},
    prelude::*,
};
use std::rc::Rc;
use wasmer_runtime_core::table::Table;

#[pyclass]
/// `ExportedTable` is a Python class that represents a WebAssembly
/// exported table.
pub struct ExportedTable {
    /// The exported table name from the WebAssembly instance.
    table_name: String,

    /// The exported table from the WebAssembly instance.
    table: Rc<Table>,
}

#[pymethods]
/// Implement methods on the `ExportedTable` Python class.
impl ExportedTable {
    #[getter]
    fn minimum(&self) -> u32 {
        let _ = self.table_name;

        self.table.descriptor().minimum
    }

    #[getter]
    fn maximum(&self) -> Option<u32> {
        self.table.descriptor().maximum
    }

    #[getter]
    fn size(&self) -> u32 {
        self.table.size()
    }

    fn grow(&self, delta: u32) -> PyResult<u32> {
        self.table
            .grow(delta)
            .map_err(|e| RuntimeError::py_err(format!("Failed to grow the memory: {}.", e)))
    }
}

#[pyclass]
/// `ExportedTables` is a Python class that represents the set
/// of WebAssembly exported tables. It's basically a set of
/// `ExportedTable` classes.
///
/// # Examples
///
/// ```python
/// from wasmer import Instance
///
/// instance = Instance(wasm_bytes)
///
/// assert instance.tables.tab1.??? // TODO
/// ```
pub struct ExportedTables {
    /// Available exported tables names from the WebAssembly module.
    pub(crate) tables: Vec<(String, Rc<Table>)>,
}

#[pyproto]
/// Implement the Python object protocol on the `ExportedTables`
/// Python class.
impl PyObjectProtocol for ExportedTables {
    /// A Python attribute in this context represents a WebAssembly
    /// exported table name.
    fn __getattr__(&self, key: String) -> PyResult<ExportedTable> {
        self.tables
            .iter()
            .find(|(name, _)| name == &key)
            .map(|(table_name, table)| ExportedTable {
                table_name: table_name.clone(),
                table: table.clone(),
            })
            .ok_or_else(|| LookupError::py_err(format!("Table `{}` does not exist.", key)))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.tables))
    }
}
