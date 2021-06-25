use crate::{errors::to_py_err, wasmer_inner::wasmer};
use pyo3::{
    class::basic::PyObjectProtocol,
    conversion::{FromPyObject, IntoPy},
    exceptions::PyValueError,
    prelude::*,
};
use std::{convert::TryFrom, slice};

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Type {
    I32 = 1,
    I64 = 2,
    F32 = 3,
    F64 = 4,
    V128 = 5,
    ExternRef = 6,
    FuncRef = 7,
}

impl Type {
    pub fn iter() -> slice::Iter<'static, Type> {
        static VARIANTS: [Type; 7] = [
            Type::I32,
            Type::I64,
            Type::F32,
            Type::F64,
            Type::V128,
            Type::ExternRef,
            Type::FuncRef,
        ];

        VARIANTS.iter()
    }
}

impl From<&Type> for &'static str {
    fn from(value: &Type) -> Self {
        match value {
            Type::I32 => "I32",
            Type::I64 => "I64",
            Type::F32 => "F32",
            Type::F64 => "F64",
            Type::V128 => "V128",
            Type::ExternRef => "EXTERN_REF",
            Type::FuncRef => "FUNC_REF",
        }
    }
}

impl ToPyObject for Type {
    fn to_object(&self, py: Python) -> PyObject {
        (*self as u8).into_py(py)
    }
}

impl IntoPy<PyObject> for Type {
    fn into_py(self, py: Python) -> PyObject {
        self.to_object(py)
    }
}

impl<'source> FromPyObject<'source> for Type {
    fn extract(obj: &'source PyAny) -> PyResult<Self> {
        let variant = u8::extract(obj)?;

        Ok(match variant {
            1 => Self::I32,
            2 => Self::I64,
            3 => Self::F32,
            4 => Self::F64,
            5 => Self::V128,
            6 => Self::ExternRef,
            7 => Self::FuncRef,
            _ => {
                return Err(to_py_err::<PyValueError, _>(
                    "Failed to extract `Type` from `PyAny`",
                ))
            }
        })
    }
}

impl From<&wasmer::Type> for Type {
    fn from(value: &wasmer::Type) -> Self {
        match value {
            wasmer::Type::I32 => Self::I32,
            wasmer::Type::I64 => Self::I64,
            wasmer::Type::F32 => Self::F32,
            wasmer::Type::F64 => Self::F64,
            wasmer::Type::V128 => Self::V128,
            wasmer::Type::ExternRef => Self::ExternRef,
            wasmer::Type::FuncRef => Self::FuncRef,
        }
    }
}

impl Into<wasmer::Type> for Type {
    fn into(self) -> wasmer::Type {
        match self {
            Self::I32 => wasmer::Type::I32,
            Self::I64 => wasmer::Type::I64,
            Self::F32 => wasmer::Type::F32,
            Self::F64 => wasmer::Type::F64,
            Self::V128 => wasmer::Type::V128,
            Self::ExternRef => wasmer::Type::ExternRef,
            Self::FuncRef => wasmer::Type::FuncRef,
        }
    }
}

/// Represents the signature of a function that is either implemented
/// in WebAssembly module or exposed to WebAssembly by the host.
///
/// WebAssembly functions can have 0 or more parameters and results.
///
/// ## Example
///
/// ```py
/// from wasmer import FunctionType, Type
///
/// # Type: (i32, i32) -> i32
/// function_type = FunctionType(
///     params=[Type.I32, Type.I32],
///     results=[Type.I32]
/// )
/// ```
#[pyclass]
#[text_signature = "(params, results)"]
pub struct FunctionType {
    /// Parameters, i.e. inputs, of the function.
    #[pyo3(get)]
    pub params: Vec<Type>,

    /// Results, i.e. outputs, of the function.
    #[pyo3(get)]
    pub results: Vec<Type>,
}

#[pymethods]
impl FunctionType {
    #[new]
    fn new(params: Vec<Type>, results: Vec<Type>) -> Self {
        Self { params, results }
    }
}

impl From<&wasmer::FunctionType> for FunctionType {
    fn from(value: &wasmer::FunctionType) -> Self {
        Self {
            params: value.params().iter().map(Into::into).collect(),
            results: value.results().iter().map(Into::into).collect(),
        }
    }
}

impl Into<wasmer::FunctionType> for &FunctionType {
    fn into(self) -> wasmer::FunctionType {
        wasmer::FunctionType::new(
            self.params
                .iter()
                .cloned()
                .map(Into::into)
                .collect::<Vec<_>>(),
            self.results
                .iter()
                .cloned()
                .map(Into::into)
                .collect::<Vec<_>>(),
        )
    }
}

#[pyproto]
impl PyObjectProtocol for FunctionType {
    fn __str__(&self) -> String {
        format!(
            "FunctionType(params: {:?}, results: {:?})",
            self.params, self.results,
        )
    }
}

/// A descriptor for a WebAssembly memory type.
///
/// Memories are described in units of pages (64Kb) and represent
/// contiguous chunks of addressable memory.
///
/// ## Example
///
/// ```py
/// from wasmer import MemoryType
///
/// memory_type = MemoryType(
///     minimum=1,
///     shared=True
/// )
/// ```
#[pyclass]
#[text_signature = "(minimum, maximum, shared)"]
pub struct MemoryType {
    /// The minimum number of pages in the memory.
    #[pyo3(get)]
    pub minimum: u32,

    /// The maximum number of pages in the memory. It is optional.
    #[pyo3(get)]
    pub maximum: Option<u32>,

    /// Whether the memory may be shared between multiple threads.
    #[pyo3(get)]
    pub shared: bool,
}

#[pymethods]
impl MemoryType {
    #[new]
    fn new(minimum: u32, maximum: Option<u32>, shared: bool) -> Self {
        Self {
            minimum,
            maximum,
            shared,
        }
    }
}

impl From<&wasmer::MemoryType> for MemoryType {
    fn from(value: &wasmer::MemoryType) -> Self {
        Self {
            minimum: value.minimum.0,
            maximum: value.maximum.map(|pages| pages.0),
            shared: value.shared,
        }
    }
}

impl From<wasmer::MemoryType> for MemoryType {
    fn from(value: wasmer::MemoryType) -> Self {
        Self::from(&value)
    }
}

impl Into<wasmer::MemoryType> for &MemoryType {
    fn into(self) -> wasmer::MemoryType {
        wasmer::MemoryType::new(self.minimum, self.maximum, self.shared)
    }
}

#[pyproto]
impl PyObjectProtocol for MemoryType {
    fn __str__(&self) -> String {
        format!(
            "MemoryType(minimum: {}, maximum: {:?}, shared: {})",
            self.minimum, self.maximum, self.shared,
        )
    }
}

/// A descriptor for a WebAssembly global.
///
/// ## Example
///
/// ```py
/// from wasmer import GlobalType, Type
///
/// # Describes a global of kind `i32` which is immutable.
/// global_type = GlobalType(Type.I32, mutable=False)
/// ```
#[pyclass]
#[text_signature = "(type, mutable)"]
pub struct GlobalType {
    /// The type of the value stored in the global.
    #[pyo3(get)]
    pub r#type: Type,

    /// A flag indicating whether the value may change at runtime.
    #[pyo3(get)]
    pub mutable: bool,
}

#[pymethods]
impl GlobalType {
    #[new]
    fn new(r#type: Type, mutable: bool) -> Self {
        Self { r#type, mutable }
    }
}

impl From<&wasmer::GlobalType> for GlobalType {
    fn from(value: &wasmer::GlobalType) -> Self {
        Self {
            r#type: (&value.ty).into(),
            mutable: value.mutability.is_mutable(),
        }
    }
}

#[pyproto]
impl PyObjectProtocol for GlobalType {
    fn __str__(&self) -> String {
        format!(
            "GlobalType(type: {:?}, mutable: {:?})",
            self.r#type, self.mutable,
        )
    }
}

/// A descriptor for a table in a WebAssembly module.
///
/// Tables are contiguous chunks of a specific element, typically a
/// `funcref` or `externref`. The most common use for tables is a
/// function table through which `call_indirect` can invoke other
/// functions.
///
/// ## Example
///
/// ```py
/// from wasmer import TableType, Type
///
/// table_type = TableType(Type.I32, minimum=7, maximum=42)
/// ```
#[pyclass]
#[text_signature = "(type, minium, maximum)"]
pub struct TableType {
    /// The type of data stored in elements of the table.
    #[pyo3(get)]
    pub r#type: Type,

    /// The minimum number of elements in the table.
    #[pyo3(get)]
    pub minimum: u32,

    /// The maximum number of elements in the table.
    #[pyo3(get)]
    pub maximum: Option<u32>,
}

#[pymethods]
impl TableType {
    #[new]
    fn new(r#type: Type, minimum: u32, maximum: Option<u32>) -> Self {
        Self {
            r#type,
            minimum,
            maximum,
        }
    }
}

impl From<&wasmer::TableType> for TableType {
    fn from(value: &wasmer::TableType) -> Self {
        Self {
            r#type: (&value.ty).into(),
            minimum: value.minimum,
            maximum: value.maximum,
        }
    }
}

impl Into<wasmer::TableType> for &TableType {
    fn into(self) -> wasmer::TableType {
        wasmer::TableType::new(self.r#type.into(), self.minimum, self.maximum)
    }
}

#[pyproto]
impl PyObjectProtocol for TableType {
    fn __str__(&self) -> String {
        format!(
            "TableType(type: {:?}, minimum: {}, maximum: {:?})",
            self.r#type, self.minimum, self.maximum,
        )
    }
}

/// Represents the type of a module's export (not to be confused with
/// an export of an instance). It is usually built from the
/// `Module.exports` getter.
///
/// ## Examples
///
/// ```py
/// from wasmer import Store, Module, ExportType, FunctionType, GlobalType, TableType, MemoryType, Type
///
/// module = Module(
///     Store(),
///     """
///     (module
///       (func (export "function") (param i32 i64))
///       (global (export "global") i32 (i32.const 7))
///       (table (export "table") 0 funcref)
///       (memory (export "memory") 1))
///     """
/// )
///
/// exports = module.exports
///
/// assert isinstance(exports[0], ExportType)
///
/// assert exports[0].name == "function"
/// assert isinstance(exports[0].type, FunctionType)
/// assert exports[0].type.params == [Type.I32, Type.I64]
/// assert exports[0].type.results == []
///
/// assert exports[1].name == "global"
/// assert isinstance(exports[1].type, GlobalType)
/// assert exports[1].type.type == Type.I32
/// assert exports[1].type.mutable == False
///
/// assert exports[2].name == "table"
/// assert isinstance(exports[2].type, TableType)
/// assert exports[2].type.type == Type.FUNC_REF
/// assert exports[2].type.minimum == 0
/// assert exports[2].type.maximum == None
///
/// assert exports[3].name == "memory"
/// assert isinstance(exports[3].type, MemoryType)
/// assert exports[3].type.minimum == 1
/// assert exports[3].type.maximum == None
/// assert exports[3].type.shared == False
/// ```
#[pyclass]
#[text_signature = "(name, type)"]
pub struct ExportType {
    /// The name of the export.
    #[pyo3(get)]
    pub name: String,

    /// The type of the export. Possible values are: `FunctionType`,
    /// `GlobalType`, `TableType` and `MemoryType`.
    #[pyo3(get)]
    pub r#type: PyObject,
}

#[pymethods]
impl ExportType {
    #[new]
    fn new(name: String, r#type: PyObject) -> Self {
        Self { name, r#type }
    }
}

impl TryFrom<wasmer::ExportType> for ExportType {
    type Error = PyErr;

    fn try_from(value: wasmer::ExportType) -> Result<Self, Self::Error> {
        let gil_guard = Python::acquire_gil();
        let py = gil_guard.python();

        Ok(Self {
            name: value.name().to_string(),
            r#type: extern_type_to_py_object(py, value.ty())?,
        })
    }
}

/// Represents the type of a module's import. It is usually built from
/// the `Module.imports` getter.
///
/// ## Example
///
/// ```py
/// from wasmer import Store, Module, ImportType, FunctionType, GlobalType, TableType, MemoryType, Type
///
/// module = Module(
///     Store(),
///     """
///     (module
///     (import "ns" "function" (func))
///     (import "ns" "global" (global f32))
///     (import "ns" "table" (table 1 2 anyfunc))
///     (import "ns" "memory" (memory 3 4)))
///     """
/// )
/// imports = module.imports
///
/// assert isinstance(imports[0], ImportType)
///
/// assert imports[0].module == "ns"
/// assert imports[0].name == "function"
/// assert isinstance(imports[0].type, FunctionType)
/// assert imports[0].type.params == []
/// assert imports[0].type.results == []
///
/// assert imports[1].module == "ns"
/// assert imports[1].name == "global"
/// assert isinstance(imports[1].type, GlobalType)
/// assert imports[1].type.type == Type.F32
/// assert imports[1].type.mutable == False
///
/// assert imports[2].module == "ns"
/// assert imports[2].name == "table"
/// assert isinstance(imports[2].type, TableType)
/// assert imports[2].type.type == Type.FUNC_REF
/// assert imports[2].type.minimum == 1
/// assert imports[2].type.maximum == 2
///
/// assert imports[3].module == "ns"
/// assert imports[3].name == "memory"
/// assert isinstance(imports[3].type, MemoryType)
/// assert imports[3].type.minimum == 3
/// assert imports[3].type.maximum == 4
/// assert imports[3].type.shared == False
/// ```
#[pyclass]
#[text_signature = "(module, name, type)"]
pub struct ImportType {
    /// The namespace name (also known as module name).
    #[pyo3(get)]
    pub module: String,

    /// The name of the import.
    #[pyo3(get)]
    pub name: String,

    /// The type of the import. Possible values are: `FunctionType`,
    /// `GlobalType`, `TableType` and `MemoryType`.
    #[pyo3(get)]
    pub r#type: PyObject,
}

#[pymethods]
impl ImportType {
    #[new]
    fn new(module: String, name: String, r#type: PyObject) -> Self {
        Self {
            module,
            name,
            r#type,
        }
    }
}

impl TryFrom<wasmer::ImportType> for ImportType {
    type Error = PyErr;

    fn try_from(value: wasmer::ImportType) -> Result<Self, Self::Error> {
        let gil_guard = Python::acquire_gil();
        let py = gil_guard.python();

        Ok(Self {
            module: value.module().to_string(),
            name: value.name().to_string(),
            r#type: extern_type_to_py_object(py, value.ty())?,
        })
    }
}

fn extern_type_to_py_object(py: Python, value: &wasmer::ExternType) -> PyResult<PyObject> {
    Ok(match value {
        wasmer::ExternType::Function(t) => Py::new(py, FunctionType::from(t))?.to_object(py),
        wasmer::ExternType::Global(t) => Py::new(py, GlobalType::from(t))?.to_object(py),
        wasmer::ExternType::Table(t) => Py::new(py, TableType::from(t))?.to_object(py),
        wasmer::ExternType::Memory(t) => Py::new(py, MemoryType::from(t))?.to_object(py),
    })
}
