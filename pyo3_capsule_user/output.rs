#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use pyo3;
use pyo3::ffi::PyCapsule_New;
use pyo3::prelude::*;
use pyo3::types::PyCapsule;
use pyo3::wrap_pyfunction;
use pyo3::{
    exceptions::{PyIndexError, PyValueError},
    ffi::PyCapsule_Import, ffi::{self, PyTuple_Size},
    pyobject_native_type_extract, pyobject_native_type_named,
    types::{PyDict, PyTuple, PyType},
    AsPyPointer, FromPyObject, FromPyPointer, IntoPyPointer, PyAny, PyDowncastError,
    PyNativeType, PyObject, PyResult, PyTypeInfo, Python, ToPyObject,
};
use std::cell::UnsafeCell;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
#[repr(C)]
pub struct PyO3Decimal_CAPI {
    pub DecimalType: *mut ffi::PyTypeObject,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for PyO3Decimal_CAPI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Self { DecimalType: ref __self_0_0 } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(
                    f,
                    "PyO3Decimal_CAPI",
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "DecimalType",
                    &&(*__self_0_0),
                );
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for PyO3Decimal_CAPI {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for PyO3Decimal_CAPI {
    #[inline]
    fn clone(&self) -> PyO3Decimal_CAPI {
        {
            let _: ::core::clone::AssertParamIsClone<*mut ffi::PyTypeObject>;
            *self
        }
    }
}
#[repr(transparent)]
pub struct PyExample(PyAny);
unsafe impl ::pyo3::PyNativeType for PyExample {}
impl ::std::fmt::Debug for PyExample {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        let s = self.repr().or(::std::result::Result::Err(::std::fmt::Error))?;
        f.write_str(&s.to_string_lossy())
    }
}
impl ::std::fmt::Display for PyExample {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        let s = self.str().or(::std::result::Result::Err(::std::fmt::Error))?;
        f.write_str(&s.to_string_lossy())
    }
}
impl ::pyo3::ToPyObject for PyExample {
    #[inline]
    fn to_object(&self, py: ::pyo3::Python<'_>) -> ::pyo3::PyObject {
        use ::pyo3::AsPyPointer;
        unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
    }
}
impl ::std::convert::AsRef<::pyo3::PyAny> for PyExample {
    #[inline]
    fn as_ref(&self) -> &::pyo3::PyAny {
        &self.0
    }
}
impl ::std::ops::Deref for PyExample {
    type Target = ::pyo3::PyAny;
    #[inline]
    fn deref(&self) -> &::pyo3::PyAny {
        &self.0
    }
}
impl ::pyo3::AsPyPointer for PyExample {
    /// Gets the underlying FFI pointer, returns a borrowed pointer.
    #[inline]
    fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
        self.0.as_ptr()
    }
}
impl ::pyo3::IntoPy<::pyo3::Py<PyExample>> for &'_ PyExample {
    #[inline]
    fn into_py(self, py: ::pyo3::Python<'_>) -> ::pyo3::Py<PyExample> {
        use ::pyo3::AsPyPointer;
        unsafe { ::pyo3::Py::from_borrowed_ptr(py, self.as_ptr()) }
    }
}
impl ::std::convert::From<&'_ PyExample> for ::pyo3::Py<PyExample> {
    #[inline]
    fn from(other: &PyExample) -> Self {
        use ::pyo3::AsPyPointer;
        use ::pyo3::PyNativeType;
        unsafe { ::pyo3::Py::from_borrowed_ptr(other.py(), other.as_ptr()) }
    }
}
impl<'a> ::std::convert::From<&'a PyExample> for &'a ::pyo3::PyAny {
    fn from(ob: &'a PyExample) -> Self {
        unsafe { &*(ob as *const PyExample as *const ::pyo3::PyAny) }
    }
}
unsafe impl PyTypeInfo for PyExample {
    type AsRefTarget = Self;
    const NAME: &'static str = "PyExample";
    const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
    #[inline]
    fn type_object_raw(py: Python) -> *mut ffi::PyTypeObject {
        ensure_decimal_api(py).DecimalType
    }
    fn is_type_of(ob: &PyAny) -> bool {
        unsafe {
            ffi::PyObject_TypeCheck(ob.as_ptr(), Self::type_object_raw(ob.py())) > 0
        }
    }
}
fn ensure_decimal_api(_py: Python<'_>) -> &'static PyO3Decimal_CAPI {
    unsafe {
        if PyDecimalAPI().is_null() {
            PyDecimal_IMPORT()
        }
        &*PyDecimalAPI()
    }
}
#[inline]
/// Check if `op` is a `PyDateTimeAPI.DateTimeType` or subtype.
pub unsafe fn PyDecimal_Check(op: *mut ffi::PyObject) -> c_int {
    ffi::PyObject_TypeCheck(op, (*PyDecimalAPI()).DecimalType) as c_int
}
impl<'py> FromPyObject<'py> for &'py PyExample {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        unsafe {
            if PyDecimal_Check(ob.as_ptr()) == 0 {
                return Err(PyDowncastError::new(ob, Self::NAME).into());
            }
            Ok(&*(ob as *const PyAny as *const Self))
        }
    }
}
#[inline]
pub unsafe fn PyDecimalAPI() -> *mut PyO3Decimal_CAPI {
    *PyDecimalAPI_impl.0.get()
}
struct PyDecimalAPISingleton(UnsafeCell<*mut PyO3Decimal_CAPI>);
unsafe impl Sync for PyDecimalAPISingleton {}
static PyDecimalAPI_impl: PyDecimalAPISingleton = PyDecimalAPISingleton(
    UnsafeCell::new(ptr::null_mut()),
);
/// Populates the `PyDecimalAPI` object
pub unsafe fn PyDecimal_IMPORT() {
    let py_decimal_c_api = {
        let PyO3Decimal_CAPSULE_NAME = CString::new("_pyo3_decimal._API").unwrap();
        PyCapsule_Import(PyO3Decimal_CAPSULE_NAME.as_ptr(), 1) as *mut PyO3Decimal_CAPI
    };
    *PyDecimalAPI_impl.0.get() = py_decimal_c_api;
}
/// This module is a python module implemented in Rust.
fn rust_binding(py: Python, m: &PyModule) -> PyResult<()> {
    Ok(())
}
#[no_mangle]
#[allow(non_snake_case)]
/// This autogenerated function is called by the python interpreter when importing
/// the module.
pub unsafe extern "C" fn PyInit_rust_binding() -> *mut ::pyo3::ffi::PyObject {
    unsafe { __PYO3_PYMODULE_DEF_RUST_BINDING.module_init() }
}
#[doc(hidden)]
static __PYO3_PYMODULE_DEF_RUST_BINDING: ::pyo3::impl_::pymodule::ModuleDef = unsafe {
    ::pyo3::impl_::pymodule::ModuleDef::new(
        "rust_binding\u{0}",
        "This module is a python module implemented in Rust.\u{0}",
        ::pyo3::impl_::pymodule::ModuleInitializer(rust_binding),
    )
};
