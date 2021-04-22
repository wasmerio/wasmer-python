use crate::{
    errors::to_py_err,
    store::Store,
    types::GlobalType,
    values::{to_py_object, to_wasm_value, Value},
    wasmer_inner::wasmer,
};
use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
    prelude::*,
};

/// Represents a WebAssembly global instance.
///
/// A global instance is the runtime representation of a global
/// variable. It consists of an individual value and a flag indicating
/// whether it is mutable.
///
/// Specification: https://webassembly.github.io/spec/core/exec/runtime.html#global-instances
///
/// ## Example
///
/// ```py
/// from wasmer import Store, Global, Value, Type
///
/// store = Store()
///
/// # Let's create an immutable global.
/// global_ = Global(store, Value.i32(42))
/// global_type = global_.type
///
/// assert global_.value == 42
/// assert global_type.type == Type.I32
/// assert global_type.mutable == False
///
/// # Let's create an mutable global.
/// global_ = Global(store, Value.i32(42), mutable=True)
///
/// assert global_.mutable == True
/// ```
#[pyclass(unsendable)]
#[text_signature = "(store, value, mutable)"]
pub struct Global {
    inner: wasmer::Global,
}

impl Global {
    pub fn raw_new(inner: wasmer::Global) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &wasmer::Global {
        &self.inner
    }
}

#[pymethods]
impl Global {
    #[new]
    fn new(store: &Store, value: &Value, mutable: Option<bool>) -> Self {
        let store = store.inner();
        let value = value.inner().clone();

        Self {
            inner: match mutable {
                Some(true) => wasmer::Global::new_mut(store, value),
                _ => wasmer::Global::new(store, value),
            },
        }
    }

    /// Checks whether the global is mutable.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import Store, Global, Value
    ///
    /// store = Store()
    /// global_ = Global(store, Value.i32(42), mutable=True)
    ///
    /// assert global_.mutable == True
    /// ```
    #[getter]
    fn mutable(&self) -> bool {
        self.inner.ty().mutability.is_mutable()
    }

    /// Get or set a custom value to the global instance.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import Store, Global, Value
    ///
    /// store = Store()
    /// global_ = Global(store, Value.i32(42), mutable=True)
    ///
    /// assert global_.value == 42
    ///
    /// global_.value = 153
    ///
    /// assert global_.value == 153
    /// ```
    #[getter(value)]
    fn get_value(&self, py: Python) -> PyObject {
        let to_py_object = to_py_object(py);

        to_py_object(&self.inner.get())
    }

    #[setter(value)]
    fn set_value(&self, value: &PyAny) -> PyResult<()> {
        let ty = self.inner.ty();

        if !ty.mutability.is_mutable() {
            return Err(to_py_err::<PyRuntimeError, _>(
                "The global variable is not mutable, cannot set a new value",
            ));
        }

        self.inner
            .set(to_wasm_value((value, ty.ty))?)
            .map_err(to_py_err::<PyValueError, _>)?;

        Ok(())
    }

    /// Returns the type of the global as a value of kind `GlobalType`.
    ///
    /// ## Example
    ///
    /// ```py
    /// from wasmer import Store, Global, Value, Type
    ///
    /// store = Store()
    ///
    /// global_ = Global(store, Value.i32(42), mutable=False)
    /// global_type = global_.type
    ///
    /// assert global_type.type == Type.I32
    /// assert global_type.mutable == False
    /// ```
    #[getter(type)]
    fn ty(&self) -> GlobalType {
        self.inner.ty().into()
    }
}
