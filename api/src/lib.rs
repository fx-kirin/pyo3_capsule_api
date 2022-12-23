use pyo3;
use pyo3::ffi::PyCapsule_New;
use pyo3::prelude::*;
use pyo3::types::PyCapsule;
use pyo3::{PyResult, PyTypeInfo, Python};
use std::os::raw::c_void;

use pyo3::ffi::PyTypeObject;

static PYO3_CAPSULE_API_NAME: &std::ffi::CStr =
    unsafe { std::mem::transmute::<_, &std::ffi::CStr>(concat!("pyo3_capsule_api._API", "\0")) };
static mut PYO3_EXAMPLE_CAPI: *const PyO3Example_CAPI = std::ptr::null();

#[pyclass]
#[repr(C)]
struct PyExample {
    inner: i64,
}

#[pymethods]
impl PyExample {
    #[new]
    fn new(value: i64) -> Self {
        Self { inner: value }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyO3Example_CAPI {
    pub ExampleType: *mut PyTypeObject,
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn pyo3_capsule_api(py: Python, m: &PyModule) -> PyResult<()> {
    let example_type = PyExample::type_object_raw(py);
    let mut _example_api = PyO3Example_CAPI {
        ExampleType: example_type,
    };
    let mut _example_api = Box::new(_example_api);
    unsafe {
        // leak the value, so it will never be dropped or freed
        PYO3_EXAMPLE_CAPI = Box::leak(_example_api) as *const PyO3Example_CAPI;
    }
    unsafe {
        let cap_ptr = PyCapsule_New(
            PYO3_EXAMPLE_CAPI as *mut c_void,
            (*PYO3_CAPSULE_API_NAME).as_ptr(),
            None,
        );
        let capsule: &PyCapsule = py.from_owned_ptr_or_err(cap_ptr)?;
        m.add("_API", capsule)?;
    }

    m.add_class::<PyExample>()?;

    Ok(())
}
