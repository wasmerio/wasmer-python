//! The `ExportedFunction` and relative collection that encapsulate Wasmer
//!  memory and instances.

use crate::{instance::inspect::InspectExportedFunction, r#type::Type, value::Value};
use pyo3::{
    class::basic::PyObjectProtocol,
    exceptions::{LookupError, RuntimeError},
    prelude::*,
    types::{PyDict, PyFloat, PyLong, PyTuple},
    ToPyObject,
};
use std::{cmp::Ordering, convert::From, rc::Rc, slice};
use wasmer_runtime::{self as runtime, Value as WasmValue};
use wasmer_runtime_core::{instance::DynFunc, types::Type as WasmType};

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum ExportImportKind {
    Function = 1,
    Memory = 2,
    Global = 3,
    Table = 4,
}

impl ExportImportKind {
    pub fn iter() -> slice::Iter<'static, ExportImportKind> {
        static VARIANTS: [ExportImportKind; 4] = [
            ExportImportKind::Function,
            ExportImportKind::Memory,
            ExportImportKind::Global,
            ExportImportKind::Table,
        ];

        VARIANTS.iter()
    }
}

impl From<&ExportImportKind> for &'static str {
    fn from(value: &ExportImportKind) -> Self {
        match value {
            ExportImportKind::Function => "FUNCTION",
            ExportImportKind::Memory => "MEMORY",
            ExportImportKind::Global => "GLOBAL",
            ExportImportKind::Table => "TABLE",
        }
    }
}

impl ToPyObject for ExportImportKind {
    fn to_object(&self, py: Python) -> PyObject {
        (*self as u8).into_py(py)
    }
}

#[pyclass]
/// `ExportedFunction` is a Python class that represents a WebAssembly
/// exported function. Such a function can be invoked from Python by using the
/// `__call__` Python class method.
pub struct ExportedFunction {
    /// The underlying Rust WebAssembly instance.
    instance: Rc<runtime::Instance>,

    /// The exported function name from the WebAssembly module.
    function_name: String,
}

/// Implement the `InspectExportedFunction` trait.
impl InspectExportedFunction for ExportedFunction {
    fn function(&self) -> PyResult<DynFunc> {
        match self.instance.exports.get(&self.function_name) {
            Ok(function) => Ok(function),
            Err(_) => Err(RuntimeError::py_err(format!(
                "Function `{}` does not exist.",
                self.function_name
            ))),
        }
    }
}

pub(super) fn call_exported_func(
    py: Python,
    function_name_as_str: &str,
    function: DynFunc,
    arguments: &PyTuple,
) -> PyResult<PyObject> {
    // Check the given arguments match the exported function signature.
    let signature = function.signature();
    let parameters = signature.params();

    let number_of_parameters = parameters.len() as isize;
    let number_of_arguments = arguments.len() as isize;
    let diff: isize = number_of_parameters - number_of_arguments;

    match diff.cmp(&0) {
        Ordering::Greater => {
            return Err(RuntimeError::py_err(format!(
                "Missing {} argument(s) when calling `{}`: Expect {} argument(s), given {}.",
                diff, function_name_as_str, number_of_parameters, number_of_arguments,
            )))
        }
        Ordering::Less => {
            return Err(RuntimeError::py_err(format!(
                "Given {} extra argument(s) when calling `{}`: Expect {} argument(s), given {}.",
                diff.abs(),
                function_name_as_str,
                number_of_parameters,
                number_of_arguments,
            )))
        }
        Ordering::Equal => {}
    }

    // Map Python arguments to WebAssembly values.
    let mut function_arguments = Vec::<WasmValue>::with_capacity(number_of_parameters as usize);

    for (parameter, argument) in parameters.iter().zip(arguments.iter()) {
        let value = match argument.downcast::<Value>() {
            Ok(value) => value.value.clone(),
            Err(_) => match parameter {
                WasmType::I32 => WasmValue::I32(argument.downcast::<PyLong>()?.extract::<i32>()?),
                WasmType::I64 => WasmValue::I64(argument.downcast::<PyLong>()?.extract::<i64>()?),
                WasmType::F32 => WasmValue::F32(argument.downcast::<PyFloat>()?.extract::<f32>()?),
                WasmType::F64 => WasmValue::F64(argument.downcast::<PyFloat>()?.extract::<f64>()?),
                WasmType::V128 => {
                    WasmValue::V128(argument.downcast::<PyLong>()?.extract::<u128>()?)
                }
            },
        };

        function_arguments.push(value);
    }

    // Call the exported function.
    let results = function
        .call(function_arguments.as_slice())
        .map_err(|e| RuntimeError::py_err(format!("{}", e)))?;

    // Map the WebAssembly first result to a Python value.
    if !results.is_empty() {
        Ok(match results[0] {
            WasmValue::I32(result) => result.to_object(py),
            WasmValue::I64(result) => result.to_object(py),
            WasmValue::F32(result) => result.to_object(py),
            WasmValue::F64(result) => result.to_object(py),
            WasmValue::V128(result) => result.to_object(py),
        })
    } else {
        Ok(py.None())
    }
}

#[pymethods]
/// Implement methods on the `ExportedFunction` Python class.
impl ExportedFunction {
    // The `ExportedFunction.__call__` method.
    // The `#[args(arguments = "*")]` means that the method has an
    // unfixed arity. All parameters will be received in the
    // `arguments` argument.
    #[call]
    #[args(arguments = "*")]
    fn __call__(&self, py: Python, arguments: &PyTuple) -> PyResult<PyObject> {
        call_exported_func(py, &self.function_name, self.function()?, arguments)
    }

    // On the blueprint of Python's `inpect.getfullargspec`
    #[getter]
    fn getfullargspec(&self, py: Python) -> PyResult<PyObject> {
        let function = self.function()?;
        let signature = function.signature();
        let annotations = PyDict::new(py);

        for (nth, ty) in &signature
            .params()
            .iter()
            .enumerate()
            .map(|(nth, ty)| (nth, ty.into()))
            .collect::<Vec<(usize, Type)>>()
        {
            annotations.set_item(format!("x{}", nth), ty)?;
        }

        let args = annotations.keys();

        for ty in &signature
            .returns()
            .iter()
            .map(Into::into)
            .collect::<Vec<Type>>()
        {
            annotations.set_item("return", ty)?;
        }

        let inspect = py.import("inspect")?;
        let args: Py<PyTuple> = (
            args,
            py.None(), // varargs
            py.None(), // varkw
            py.None(), // defaults
            py.None(), // kwonlyargs
            py.None(), // kwonlydefaults
            annotations,
        )
            .into_py(py);

        Ok(inspect.call1("FullArgSpec", args)?.to_object(py))
    }
}

#[pyclass]
/// `ExportedFunctions` is a Python class that represents the set
/// of WebAssembly exported functions. It's basically a set of
/// `ExportedFunction` classes.
///
/// # Examples
///
/// ```python
/// from wasmer import Instance
///
/// instance = Instance(wasm_bytes)
/// result = instance.exports.sum(1, 2)
/// ```
pub struct ExportedFunctions {
    /// The underlying Rust WebAssembly instance.
    pub(crate) instance: Rc<runtime::Instance>,

    /// Available exported function names from the WebAssembly module.
    pub(crate) functions: Vec<String>,
}

#[pyproto]
/// Implement the Python object protocol on the `ExportedFunctions`
/// Python class.
impl PyObjectProtocol for ExportedFunctions {
    /// A Python attribute in this context represents a WebAssembly
    /// exported function name.
    fn __getattr__(&self, key: String) -> PyResult<ExportedFunction> {
        if self.functions.contains(&key) {
            Ok(ExportedFunction {
                instance: self.instance.clone(),
                function_name: key,
            })
        } else {
            Err(LookupError::py_err(format!(
                "Function `{}` does not exist.",
                key
            )))
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.functions))
    }
}
