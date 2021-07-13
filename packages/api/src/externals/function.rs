use crate::{
    errors::to_py_err,
    store::Store,
    types::FunctionType,
    values::{to_py_object, to_wasm_value},
    wasmer_inner::wasmer,
};
use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
    prelude::*,
    types::{PyDict, PyTuple},
};
use std::{io, sync::Arc};

/// Represents a WebAssembly function instance.
///
/// A function instance is the runtime representation of a
/// function. It effectively is a closure of the original function
/// (defined in either the host or the WebAssembly module) over the
/// runtime `Instance` of its originating `Module`.
///
/// The module instance is used to resolve references to other
/// definitions during executing of the function.
///
/// Specification: https://webassembly.github.io/spec/core/exec/runtime.html#function-instances
///
/// Note that the function can be invoked/called by the host only when
/// it is an exported function (see `Exports` to see an example).
///
/// # Example
///
/// To build a `Function`, we need its type. It can either be inferred
/// from Python thanks to annotations, or be given with a
/// `FunctionType` value.
///
/// ## With Python annotations
///
/// First, let's see with Python annotations:
///
/// ```py
/// from wasmer import Store, Function, Type
///
/// def sum(x: int, y: int) -> int:
///     return x + y
///
/// store = Store()
/// function = Function(store, sum)
/// function_type = function.type
///
/// assert function_type.params == [Type.I32, Type.I32]
/// assert function_type.results == [Type.I32]
/// ```
///
/// Here is the mapping table:
///
/// | Annotations | WebAssembly type |
/// |-|-|
/// | `int`, `'i32'`, `'I32'` | `Type.I32` |
/// | `'i64'`, `'I64'` | `Type.I64` |
/// | `float`, `'f32'`, `'F32'` | `Type.F32` |
/// | `'f64'`, `'F64'` | `Type.F64` |
/// | `None` | none (only in `return` position) |
///
/// It is possible for a host function to return a tuple of the types above (except `None`), like:
///
/// ```py
/// from wasmer import Store, Function, Type
///
/// def swap(x: 'i32', y: 'i64') -> ('i64', 'i32'):
///     return (y, x)
///
/// store = Store()
/// function = Function(store, swap)
/// function_type = function.type
///
/// assert function_type.params == [Type.I32, Type.I64]
/// assert function_type.results == [Type.I64, Type.I32]
/// ```
///
/// ## With `FunctionType`
///
/// Second, the same code but without annotations and a `FunctionType`:
///
/// ```py
/// from wasmer import Store, Function, FunctionType, Type
///
/// def sum(x, y):
///     return x + y
///
/// store = Store()
/// function = Function(store, sum, FunctionType([Type.I32, Type.I32], [Type.I32]))
/// ```
#[pyclass(unsendable)]
#[text_signature = "(store, function, function_type)"]
pub struct Function {
    inner: wasmer::Function,
}

impl Function {
    pub fn raw_new(inner: wasmer::Function) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &wasmer::Function {
        &self.inner
    }
}

#[pymethods]
impl Function {
    #[new]
    fn new(
        py: Python,
        store: &Store,
        py_function: &PyAny,
        function_type: Option<&FunctionType>,
    ) -> PyResult<Self> {
        if !py_function.is_callable() {
            return Err(to_py_err::<PyValueError, _>("Function must be a callable"));
        }

        let (argument_types, result_types) = match function_type {
            Some(function_type) => {
                let function_type: wasmer::FunctionType = function_type.into();

                (
                    function_type.params().to_vec(),
                    function_type.results().to_vec(),
                )
            }

            None => {
                if !py_function.hasattr("__annotations__")? {
                    return Err(to_py_err::<PyValueError, _>(
                        "The function must have type annotations",
                    ));
                }

                let annotations = py_function
                    .getattr("__annotations__")?
                    .downcast::<PyDict>()
                    .map_err(PyErr::from)?;

                let mut argument_types = Vec::new();
                let mut result_types = Vec::new();

                for (annotation_name, annotation_value) in annotations {
                    let maybe_ty = to_wasm_type(annotation_value)?;

                    match (annotation_name.to_string().as_str(), maybe_ty) {
                        ("return", MappedType::None) => (),
                        ("return", MappedType::One(ty)) => result_types.push(ty),
                        ("return", MappedType::Many(mut tys)) => result_types.append(&mut tys),

                        (name, MappedType::None) => {
                            return Err(to_py_err::<PyRuntimeError, _>(format!(
                                "Variable `{}` cannot have type `None`",
                                name
                            )))
                        }
                        (_, MappedType::One(ty)) => argument_types.push(ty),
                        (name, MappedType::Many(_)) => {
                            return Err(to_py_err::<PyRuntimeError, _>(format!(
                                "Variable `{}` cannot receive a tuple (not supported yet)",
                                name
                            )))
                        }
                    }
                }

                (argument_types, result_types)
            }
        };

        #[derive(wasmer::WasmerEnv, Clone)]
        struct Environment {
            py_function: Arc<PyObject>,
            result_types: Vec<wasmer::Type>,
        }

        let environment = Environment {
            py_function: Arc::new(py_function.to_object(py)),
            result_types: result_types.clone(),
        };

        let host_function = wasmer::Function::new_with_env(
            store.inner(),
            &wasmer::FunctionType::new(argument_types, result_types),
            environment,
            |environment,
             arguments: &[wasmer::Value]|
             -> Result<Vec<wasmer::Value>, wasmer::RuntimeError> {
                let gil = Python::acquire_gil();
                let py = gil.python();

                let to_py_object = to_py_object(py);
                let arguments: Vec<PyObject> = arguments.iter().map(to_py_object).collect();

                let results = environment
                    .py_function
                    .call(py, PyTuple::new(py, arguments), None)
                    .map_err(|error| {
                        wasmer::RuntimeError::new(io::Error::from(error).to_string())
                    })?;

                let result_types = &environment.result_types;
                let has_result_types = !result_types.is_empty();

                Ok(if let Ok(results) = results.cast_as::<PyTuple>(py) {
                    results
                        .iter()
                        .zip(result_types)
                        .map(|(value, ty)| to_wasm_value((value, *ty)))
                        .collect::<PyResult<_>>()
                        .map_err(|error| {
                            wasmer::RuntimeError::new(io::Error::from(error).to_string())
                        })?
                } else if !results.is_none(py) && has_result_types {
                    vec![to_wasm_value((
                        results
                            .cast_as::<PyAny>(py)
                            .map_err(PyErr::from)
                            .map_err(|error| {
                                wasmer::RuntimeError::new(io::Error::from(error).to_string())
                            })?,
                        result_types[0],
                    ))
                    .map_err(|error| {
                        wasmer::RuntimeError::new(io::Error::from(error).to_string())
                    })?]
                } else {
                    Vec::new()
                })
            },
        );

        Ok(Self::raw_new(host_function))
    }

    /// Calls the function as a regular Python function.
    #[call]
    #[args(arguments = "*")]
    fn __call__<'p>(&self, py: Python<'p>, arguments: &PyTuple) -> PyResult<PyObject> {
        let arguments: Vec<wasmer::Value> = arguments
            .iter()
            .zip(self.inner.ty().params())
            .map(|(value, ty)| to_wasm_value((value, *ty)))
            .collect::<PyResult<_>>()?;

        let results = self
            .inner
            .call(&arguments)
            .map(<[_]>::into_vec)
            .map_err(to_py_err::<PyRuntimeError, _>)?;

        let to_py_object = to_py_object(py);

        Ok(match results.len() {
            0 => py.None(),
            1 => to_py_object(&results[0]),
            _ => PyTuple::new(
                py,
                results.iter().map(to_py_object).collect::<Vec<PyObject>>(),
            )
            .to_object(py),
        })
    }

    /// Returns the type of the function as a `FunctionType` object.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import Store, Module, Instance, FunctionType, Type
    ///
    /// module = Module(
    ///     Store(),
    ///     """
    ///     (module
    ///       (type (func (param i32 i32) (result i32)))
    ///       (func (type 0)
    ///         local.get 0
    ///         local.get 1
    ///         i32.add)
    ///       (export "sum" (func 0)))
    ///     """
    /// )
    /// instance = Instance(module)
    /// sum = instance.exports.sum
    /// sum_type = sum.type
    ///
    /// assert isinstance(sum_type, FunctionType)
    /// assert sum_type.params == [Type.I32, Type.I32]
    /// assert sum_type.results == [Type.I32]
    /// ```
    #[getter(type)]
    fn ty(&self) -> FunctionType {
        self.inner.ty().into()
    }
}

enum MappedType {
    None,
    One(wasmer::Type),
    Many(Vec<wasmer::Type>),
}

fn to_wasm_type(value: &PyAny) -> PyResult<MappedType> {
    enum Level {
        Top,
        Deeper,
    }

    fn inner(value: &PyAny, level: Level) -> PyResult<MappedType> {
        Ok(
            match (level, value.get_type().name()?, value.to_string().as_str()) {
                (_, "type", "<class 'int'>") => MappedType::One(wasmer::Type::I32),
                (_, "str", "i32" | "I32") => MappedType::One(wasmer::Type::I32),
                (_, "str", "i64" | "I64") => MappedType::One(wasmer::Type::I64),

                (_, "type", "<class 'float'>") => MappedType::One(wasmer::Type::F32),
                (_, "str", "f32" | "F32") => MappedType::One(wasmer::Type::F32),
                (_, "str", "f64" | "F64") => MappedType::One(wasmer::Type::F64),

                (Level::Top, "tuple", _) => {
                    let tuple = value.cast_as::<PyTuple>()?;
                    let mut types = Vec::with_capacity(tuple.len());

                    for tuple_value in tuple.iter() {
                        match inner(tuple_value, Level::Deeper)? {
                            MappedType::One(ty) => types.push(ty),
                            _ => {
                                return Err(to_py_err::<PyRuntimeError, _>(
                                    "A tuple cannot contain `None` or another tuple",
                                ))
                            }
                        }
                    }

                    MappedType::Many(types)
                }

                (Level::Deeper, "tuple", _) => {
                    return Err(to_py_err::<PyRuntimeError, _>(
                        "It is not possible to get a tuple inside a tuple yet",
                    ))
                }

                (_, "NoneType", "None") => MappedType::None,

                (_, ty, as_str) => {
                    return Err(to_py_err::<PyRuntimeError, _>(format!(
                        "Type `{}` (`{}`) is not a supported type",
                        ty, as_str,
                    )))
                }
            },
        )
    }

    inner(value, Level::Top)
}
