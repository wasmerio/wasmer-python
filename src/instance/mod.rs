//! The `wasmer.Instance` Python object to build WebAssembly instances.
//!
//! The `Instance` class has the following declaration:
//!
//! * The constructor reads bytes from its first parameter, and it
//!   expects those bytes to represent a valid WebAssembly module,
//! * The `exports` getter, to get exported functions from the
//!   WebAssembly module, e.g. `instance.exports.sum(1, 2)` to call the
//!   exported function `sum` with arguments `1` and `2`,
//! * The `memory` getter, to get the exported memory (if any) from
//!   the WebAssembly module, .e.g. `instance.memory.uint8_view()`, see
//!   the `wasmer.Memory` class.

pub(crate) mod exports;
pub(crate) mod globals;
pub(crate) mod inspect;

use crate::memory::Memory;
use exports::ExportedFunctions;
use globals::ExportedGlobals;
use pyo3::{
    exceptions::RuntimeError,
    prelude::*,
    types::{PyAny, PyBytes, PyDict, PyFloat, PyLong, PyString, PyTuple},
    PyNativeType, PyTryFrom, Python,
};
use std::{rc::Rc, sync::Arc};
use wasmer_runtime::{
    instantiate,
    types::{FuncSig, Type},
    Export, ImportObject, Value,
};
use wasmer_runtime_core::{import::Namespace, typed_func::DynamicFunc};

#[pyclass]
/// `Instance` is a Python class that represents a WebAssembly instance.
///
/// # Examples
///
/// ```python
/// from wasmer import Instance
///
/// instance = Instance(wasm_bytes)
/// ```
pub struct Instance {
    /// All WebAssembly exported functions represented by an
    /// `ExportedFunctions` object.
    pub(crate) exports: Py<ExportedFunctions>,

    /// The WebAssembly exported memory represented by a `Memory`
    /// object.
    pub(crate) memory: Option<Py<Memory>>,

    /// All WebAssembly exported globals represented by an
    /// `ExportedGlobals` object.
    pub(crate) globals: Py<ExportedGlobals>,
}

#[pymethods]
/// Implement methods on the `Instance` Python class.
impl Instance {
    /// The constructor instantiates a new WebAssembly instance basde
    /// on WebAssembly bytes (represented by the Python bytes type).
    #[new]
    #[allow(clippy::new_ret_no_self)]
    fn new(
        object: &PyRawObject,
        bytes: &PyAny,
        imported_functions: &'static PyDict,
    ) -> PyResult<()> {
        // Read the bytes.
        let bytes = <PyBytes as PyTryFrom>::try_from(bytes)?.as_bytes();

        let mut import_object = ImportObject::new();

        for (namespace_name, namespace) in imported_functions.into_iter() {
            let namespace_name = namespace_name
                .downcast_ref::<PyString>()
                .map_err(|_| RuntimeError::py_err("Namespace name must be a string.".to_string()))?
                .to_string()?;

            let mut import_namespace = Namespace::new();

            for (function_name, function) in namespace
                .downcast_ref::<PyDict>()
                .map_err(|_| RuntimeError::py_err("Namespace must be a dictionnary.".to_string()))?
                .into_iter()
            {
                let function_name = function_name
                    .downcast_ref::<PyString>()
                    .map_err(|_| {
                        RuntimeError::py_err("Function name must be a string.".to_string())
                    })?
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
                    .downcast_ref::<PyDict>()
                    .map_err(|_| {
                        RuntimeError::py_err(format!(
                            "Failed to read annotations of function `{}`.",
                            function_name
                        ))
                    })?
                {
                    let ty = match value.to_string().as_str() {
                        "i32" => Type::I32,
                        "i64" => Type::I64,
                        "f32" => Type::F32,
                        "f64" => Type::F64,
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

                        let results = function
                            .call(PyTuple::new(py, inputs), None)
                            .expect("Oh dear, trap, quick");

                        let results = results
                            .downcast_ref::<PyTuple>()
                            .unwrap_or_else(|_| PyTuple::new(py, vec![results]));

                        let outputs = results
                            .iter()
                            .zip(output_types.iter())
                            .map(|(result, output)| match output {
                                Type::I32 => Value::I32(
                                    result
                                        .downcast_ref::<PyLong>()
                                        .unwrap()
                                        .extract::<i32>()
                                        .unwrap(),
                                ),
                                Type::I64 => Value::I64(
                                    result
                                        .downcast_ref::<PyLong>()
                                        .unwrap()
                                        .extract::<i64>()
                                        .unwrap(),
                                ),
                                Type::F32 => Value::F32(
                                    result
                                        .downcast_ref::<PyFloat>()
                                        .unwrap()
                                        .extract::<f32>()
                                        .unwrap(),
                                ),
                                Type::F64 => Value::F64(
                                    result
                                        .downcast_ref::<PyFloat>()
                                        .unwrap()
                                        .extract::<f64>()
                                        .unwrap(),
                                ),
                                Type::V128 => Value::V128(
                                    result
                                        .downcast_ref::<PyLong>()
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

        // Instantiate the WebAssembly module.
        let instance = match instantiate(bytes, &import_object) {
            Ok(instance) => Rc::new(instance),
            Err(e) => {
                return Err(RuntimeError::py_err(format!(
                    "Failed to instantiate the module:\n    {}",
                    e
                )))
            }
        };

        let exports = instance.exports();

        // Collect the exported functions, globals and memory from the
        // WebAssembly module.
        let mut exported_functions = Vec::new();
        let mut exported_globals = Vec::new();
        let mut exported_memory = None;

        for (export_name, export) in exports {
            match export {
                Export::Function { .. } => exported_functions.push(export_name),
                Export::Global(global) => exported_globals.push((export_name, Rc::new(global))),
                Export::Memory(memory) if exported_memory.is_none() => {
                    exported_memory = Some(Rc::new(memory))
                }
                _ => (),
            }
        }

        let py = object.py();

        // Instantiate the `Instance` Python class.
        object.init({
            Self {
                exports: Py::new(
                    py,
                    ExportedFunctions {
                        instance: instance.clone(),
                        functions: exported_functions,
                    },
                )?,
                memory: match exported_memory {
                    Some(memory) => Some(Py::new(py, Memory { memory })?),
                    None => None,
                },
                globals: Py::new(
                    py,
                    ExportedGlobals {
                        globals: exported_globals,
                    },
                )?,
            }
        });

        Ok(())
    }

    /// The `exports` getter.
    #[getter]
    fn exports(&self) -> &Py<ExportedFunctions> {
        &self.exports
    }

    /// The `memory` getter.
    #[getter]
    fn memory(&self, py: Python) -> PyResult<PyObject> {
        match &self.memory {
            Some(memory) => Ok(memory.into_py(py)),
            None => Ok(py.None()),
        }
    }

    /// The `globals` getter.
    #[getter]
    fn globals(&self) -> &Py<ExportedGlobals> {
        &self.globals
    }
}
