//! This module contains a helper to build an `ImportObject` and build
//! the host function logic.

use pyo3::{
    exceptions::RuntimeError,
    prelude::*,
    types::{PyDict, PyFloat, PyLong, PyString, PyTuple},
    AsPyPointer, PyObject, ToPyObject,
};
use std::sync::Arc;
use wasmer_runtime::{
    types::{FuncSig, Type},
    ImportObject, Value,
};
use wasmer_runtime_core::{import::Namespace, typed_func::DynamicFunc};

pub(crate) fn build_import_object(
    py: &Python,
    imported_functions: &'static PyDict,
) -> PyResult<(ImportObject, Vec<PyObject>)> {
    let mut import_object = ImportObject::new();
    let mut host_function_references = Vec::with_capacity(imported_functions.len());

    for (namespace_name, namespace) in imported_functions.iter() {
        let namespace_name = namespace_name
            .downcast::<PyString>()
            .map_err(|_| RuntimeError::py_err("Namespace name must be a string.".to_string()))?
            .to_string()?;

        let mut import_namespace = Namespace::new();

        for (function_name, function) in namespace
            .downcast::<PyDict>()
            .map_err(|_| RuntimeError::py_err("Namespace must be a dictionnary.".to_string()))?
            .into_iter()
        {
            let function_name = function_name
                .downcast::<PyString>()
                .map_err(|_| RuntimeError::py_err("Function name must be a string.".to_string()))?
                .to_string()?;

            if !function.is_callable() {
                return Err(RuntimeError::py_err(format!(
                    "Function for `{}` is not callable.",
                    function_name
                )));
            }

            if !function.hasattr("__annotations__")? {
                return Err(RuntimeError::py_err(format!(
                    "Function `{}` must have type annotations for parameters and results.",
                    function_name
                )));
            }

            let mut input_types = vec![];
            let mut output_types = vec![];

            for (name, value) in function
                .getattr("__annotations__")?
                .downcast::<PyDict>()
                .map_err(|_| {
                    RuntimeError::py_err(format!(
                        "Failed to read annotations of function `{}`.",
                        function_name
                    ))
                })?
            {
                let ty = match value.to_string().as_str() {
                    "i32" | "I32" => Type::I32,
                    "i64" | "I64" => Type::I64,
                    "f32" | "F32" => Type::F32,
                    "f64" | "F64" => Type::F64,
                    _ => {
                        return Err(RuntimeError::py_err(
                            "Type `{}` is not supported as a WebAssembly type.".to_string(),
                        ))
                    }
                };

                match name.to_string().as_str() {
                    "return" => output_types.push(ty),
                    _ => input_types.push(ty),
                }
            }

            if output_types.len() > 1 {
                return Err(RuntimeError::py_err(
                    "Function must return only one type, many given.".to_string(),
                ));
            }

            let function = function.to_object(*py);

            host_function_references.push(function.clone_ref(*py));

            let function_implementation = DynamicFunc::new(
                Arc::new(FuncSig::new(input_types, output_types.clone())),
                move |_, inputs: &[Value]| -> Vec<Value> {
                    let gil = GILGuard::acquire();
                    let py = gil.python();

                    let inputs = inputs
                        .iter()
                        .map(|input| match input {
                            Value::I32(value) => value.to_object(py),
                            Value::I64(value) => value.to_object(py),
                            Value::F32(value) => value.to_object(py),
                            Value::F64(value) => value.to_object(py),
                            Value::V128(value) => value.to_object(py),
                        })
                        .collect::<Vec<PyObject>>();

                    if function.as_ptr().is_null() {
                        panic!("Host function implementation is null. Maybe it has moved?");
                    }

                    let results = function
                        .call(py, PyTuple::new(py, inputs), None)
                        .expect("Oh dear, trap, quick");

                    let results = match results.cast_as::<PyTuple>(py) {
                        Ok(results) => results,
                        Err(_) => PyTuple::new(py, vec![results]),
                    };

                    let outputs = results
                        .iter()
                        .zip(output_types.iter())
                        .map(|(result, output)| match output {
                            Type::I32 => Value::I32(
                                result
                                    .downcast::<PyLong>()
                                    .unwrap()
                                    .extract::<i32>()
                                    .unwrap(),
                            ),
                            Type::I64 => Value::I64(
                                result
                                    .downcast::<PyLong>()
                                    .unwrap()
                                    .extract::<i64>()
                                    .unwrap(),
                            ),
                            Type::F32 => Value::F32(
                                result
                                    .downcast::<PyFloat>()
                                    .unwrap()
                                    .extract::<f32>()
                                    .unwrap(),
                            ),
                            Type::F64 => Value::F64(
                                result
                                    .downcast::<PyFloat>()
                                    .unwrap()
                                    .extract::<f64>()
                                    .unwrap(),
                            ),
                            Type::V128 => Value::V128(
                                result
                                    .downcast::<PyLong>()
                                    .unwrap()
                                    .extract::<u128>()
                                    .unwrap(),
                            ),
                        })
                        .collect();

                    outputs
                },
            );

            import_namespace.insert(function_name, function_implementation);
        }

        import_object.register(namespace_name, import_namespace);
    }

    Ok((import_object, host_function_references))
}
