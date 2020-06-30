//! This module contains a helper to build an `ImportObject` and build
//! the host function logic.

use crate::{instance::exports::ExportImportKind, wasi};
use pyo3::{
    exceptions::RuntimeError,
    prelude::*,
    types::{PyDict, PyList},
    PyObject,
};
use std::sync::Arc;
use wasmer_runtime as runtime;
use wasmer_wasi;

#[pyclass]
/// `ImportObject` is a Python class that represents the
/// `wasmer_runtime_core::import::ImportObject`.
pub struct ImportObject {
    pub(crate) inner: runtime::ImportObject,

    #[allow(unused)]
    pub(crate) module: Arc<runtime::Module>,

    /// This field is unused as is, but is required to keep a
    /// reference to host function `PyObject`.
    #[allow(unused)]
    pub(crate) host_function_references: Vec<PyObject>,
}

impl ImportObject {
    pub fn new(module: Arc<runtime::Module>) -> Self {
        Self {
            inner: runtime::ImportObject::new(),
            module,
            host_function_references: Vec::new(),
        }
    }

    pub fn new_with_wasi(
        module: Arc<runtime::Module>,
        version: wasi::Version,
        wasi: &mut wasi::Wasi,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: wasmer_wasi::generate_import_object_from_state(
                wasi.inner.build().map_err(|error| {
                    RuntimeError::py_err(format!("Failed to create the WASI state: {}", error))
                })?,
                version.into(),
            ),
            module,
            host_function_references: Vec::new(),
        })
    }

    #[cfg(not(all(unix, target_arch = "x86_64")))]
    pub fn extend_with_pydict(&mut self, _py: Python, imported_functions: &PyDict) -> PyResult<()> {
        if imported_functions.is_empty() {
            Ok(())
        } else {
            Err(RuntimeError::py_err(
                "Imported functions are not yet supported for this platform and architecture.",
            ))
        }
    }

    #[cfg(all(unix, target_arch = "x86_64"))]
    pub fn extend_with_pydict<'py>(
        &mut self,
        py: Python,
        imported_functions: &PyDict,
    ) -> PyResult<()> {
        use pyo3::{
            types::{PyFloat, PyLong, PyString, PyTuple},
            AsPyPointer,
        };
        use std::collections::HashMap;
        use wasmer_runtime::{
            types::{FuncIndex, FuncSig, Type},
            Value,
        };
        use wasmer_runtime_core::{
            import::Namespace, structures::TypedIndex, typed_func::DynamicFunc,
        };

        let module_info = &self.module.info();
        let import_descriptors: HashMap<(String, String), &FuncSig> = module_info
            .imported_functions
            .iter()
            .map(|(import_index, import_name)| {
                let namespace = module_info
                    .namespace_table
                    .get(import_name.namespace_index)
                    .to_string();
                let name = module_info
                    .name_table
                    .get(import_name.name_index)
                    .to_string();
                let signature = module_info
                    .signatures
                    .get(
                        *module_info
                            .func_assoc
                            .get(FuncIndex::new(import_index.index()))
                            .ok_or_else(|| {
                                RuntimeError::py_err(format!(
                                    "Failed to retrieve the signature index of the imported function {}.",
                                    import_index.index()
                                ))
                            })?,
                    )
                    .ok_or_else(|| {
                        RuntimeError::py_err(format!(
                            "Failed to retrieve the signature of the imported function {}.",
                            import_index.index()
                                ))
                    })?;

                Ok(((namespace, name), signature))
            })
            .collect::<PyResult<HashMap<(String, String), &FuncSig>>>()?;

        let mut host_function_references = Vec::with_capacity(imported_functions.len());

        for (namespace_name, namespace) in imported_functions.iter() {
            let namespace_name = namespace_name
                .downcast::<PyString>()
                .map_err(|_| RuntimeError::py_err("Namespace name must be a string.".to_string()))?
                .to_string()?;

            let mut import_namespace = Namespace::new();

            for (function_name, function) in namespace
                .downcast::<PyDict>()
                .map_err(|_| RuntimeError::py_err("Namespace must be a dictionary.".to_string()))?
                .into_iter()
            {
                let function_name = function_name
                    .downcast::<PyString>()
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

                let imported_function_signature = import_descriptors
                    .get(&(namespace_name.to_string(), function_name.to_string()))
                    .ok_or_else(|| {
                        RuntimeError::py_err(
                            format!(
                                "The imported function `{}.{}` does not have a signature in the WebAssembly module.",
                                namespace_name,
                                function_name
                            )
                        )
                    })?;

                let mut input_types = vec![];
                let mut output_types = vec![];

                if !function.hasattr("__annotations__")? {
                    return Err(RuntimeError::py_err(format!(
                        "Function `{}` must have type annotations for parameters and results.",
                        function_name
                    )));
                }

                let annotations = function
                    .getattr("__annotations__")?
                    .downcast::<PyDict>()
                    .map_err(|_| {
                        RuntimeError::py_err(format!(
                            "Failed to read annotations of function `{}`.",
                            function_name
                        ))
                    })?;

                if annotations.len() > 0 {
                    for ((annotation_name, annotation_value), expected_type) in
                        annotations.iter().zip(
                            imported_function_signature
                                .params()
                                .iter()
                                .chain(imported_function_signature.returns().iter()),
                        )
                    {
                        let ty = match annotation_value.to_string().as_str() {
                            "i32" | "I32" | "<class 'int'>" if expected_type == &Type::I32 => Type::I32,
                            "i64" | "I64" | "<class 'int'>" if expected_type == &Type::I64 => Type::I64,
                            "f32" | "F32" | "<class 'float'>" if expected_type == &Type::F32 => Type::F32,
                            "f64" | "F64" | "<class 'float'>" if expected_type == &Type::F64 => Type::F64,
                            t @ _ => {
                                return Err(RuntimeError::py_err(format!(
                                    "Type `{}` is not a supported type, or is not the expected type (`{}`).",
                                    t, expected_type
                                )))
                            }
                        };

                        match annotation_name.to_string().as_str() {
                            "return" => output_types.push(ty),
                            _ => input_types.push(ty),
                        }
                    }

                    if output_types.len() > 1 {
                        return Err(RuntimeError::py_err(
                            "Function must return only one type, many given.".to_string(),
                        ));
                    }
                } else {
                    input_types.extend(imported_function_signature.params());
                    output_types.extend(imported_function_signature.returns());
                }

                let function = function.to_object(py);

                host_function_references.push(function.clone_ref(py));

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

            self.inner.register(namespace_name, import_namespace);
        }

        self.host_function_references = host_function_references;

        Ok(())
    }
}

#[pymethods]
/// Implement methods on the `ImportObject` Python class.
impl ImportObject {
    /// Extend the `ImportObject` by adding host functions stored in a Python directory.
    ///
    /// # Examples
    ///
    /// ```py
    /// # Our host function.
    /// def sum(x: int, y: int) -> int:
    ///     return x + y
    ///
    /// module = Module(wasm_bytes)
    ///
    /// # Generate an import object for this module.
    /// import_object = module.generate_import_object()
    ///
    /// # Register the `env.sum` host function.
    /// import_object.extend({
    ///     "env": {
    ///         "sum": sum
    ///     }
    /// })
    ///
    /// # Ready to instantiate the module.
    /// instance = module.instantiate(import_object)
    /// ```
    #[text_signature = "($self, imported_functions)"]
    pub fn extend(&mut self, py: Python, imported_functions: &PyDict) -> PyResult<()> {
        self.extend_with_pydict(py, imported_functions)
    }

    /// Read the descriptors of the imports.
    ///
    /// A descriptor for an import a dictionary with the following
    /// entries:
    ///
    ///   1. `kind` of type `ImportKind`, to represent the kind of
    ///      imported entity,
    ///   2. `namespace` of type `String`, to represent the namespace
    ///      of the imported entity,
    ///   3. `name` of type `String`, to represent the name of the
    ///      imported entity.
    #[text_signature = "($self)"]
    pub fn import_descriptors<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        let iterator = self.inner.clone().into_iter();
        let mut items: Vec<&PyDict> = Vec::with_capacity(iterator.size_hint().0);

        for (namespace, name, import) in iterator {
            let dict = PyDict::new(py);

            dict.set_item("kind", ExportImportKind::from(&import) as u8)?;
            dict.set_item("namespace", namespace)?;
            dict.set_item("name", name)?;

            items.push(dict);
        }

        Ok(PyList::new(py, items))
    }
}
