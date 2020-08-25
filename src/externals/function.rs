use crate::{
    errors::to_py_err,
    store::Store,
    types::FunctionType,
    values::{to_py_object, to_wasm_value},
    wasmer_inner::wasmer,
};
use pyo3::{
    exceptions::{RuntimeError, ValueError},
    prelude::*,
    types::{PyDict, PyTuple},
};
use std::io;

#[pyclass(unsendable)]
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
            return Err(to_py_err::<ValueError, _>("Function must be a callable"));
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
                    return Err(to_py_err::<ValueError, _>(
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
                    let ty = match annotation_value.to_string().as_str() {
                        "i32" | "I32" | "<class 'int'>" => wasmer::Type::I32,
                        "i64" | "I64" => wasmer::Type::I64,
                        "f32" | "F32" | "<class 'float'>" => wasmer::Type::F32,
                        "f64" | "F64" => wasmer::Type::F64,
                        ty => {
                            return Err(to_py_err::<RuntimeError, _>(format!(
                                "Type `{}` is not a supported type",
                                ty,
                            )))
                        }
                    };

                    match annotation_name.to_string().as_str() {
                        "return" => result_types.push(ty),
                        _ => argument_types.push(ty),
                    }
                }

                (argument_types, result_types)
            }
        };

        struct Environment {
            py_function: PyObject,
        }

        let environment = Environment {
            py_function: py_function.to_object(py),
        };

        let host_function = wasmer::Function::new_with_env(
            store.inner(),
            &wasmer::FunctionType::new(argument_types, result_types.clone()),
            environment,
            move |environment,
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

                let result_types = result_types.clone();

                Ok(if let Ok(results) = results.cast_as::<PyTuple>(py) {
                    results
                        .iter()
                        .zip(result_types)
                        .map(to_wasm_value)
                        .collect::<PyResult<_>>()
                        .map_err(|error| {
                            wasmer::RuntimeError::new(io::Error::from(error).to_string())
                        })?
                } else if !results.is_none(py) && result_types.len() > 0 {
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

    #[call]
    #[args(arguments = "*")]
    fn __call__<'p>(&self, py: Python<'p>, arguments: &PyTuple) -> PyResult<PyObject> {
        let arguments: Vec<wasmer::Value> = arguments
            .iter()
            .zip(self.inner.ty().params().iter().cloned())
            .map(to_wasm_value)
            .collect::<PyResult<_>>()?;

        let results = self
            .inner
            .call(&arguments)
            .map(<[_]>::into_vec)
            .map_err(to_py_err::<RuntimeError, _>)?;

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

    #[getter(type)]
    fn ty(&self) -> FunctionType {
        self.inner.ty().into()
    }
}
