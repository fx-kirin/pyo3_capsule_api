use pyo3;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::PyNativeType;
use pyo3::{
    ffi, pyobject_native_type_named, AsPyPointer, FromPyObject, PyAny, PyDowncastError, PyObject,
    PyResult, PyTypeInfo, Python,
};
use std::cell::UnsafeCell;
use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use std::ptr;

static PYO3_CAPSULE_API_NAME: &std::ffi::CStr =
    unsafe { std::mem::transmute::<_, &std::ffi::CStr>(concat!("pyo3_capsule_api._API", "\0")) };

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyO3Example_CAPI {
    pub ExampleType: *mut ffi::PyTypeObject,
}

#[derive(Debug)]
#[repr(C)]
struct PyExample {
    inner: i64,
}

unsafe impl pyo3::type_object::PyTypeInfo for PyExample {
    type AsRefTarget = pyo3::PyCell<Self>;
    const NAME: &'static str = "PyExample";
    const MODULE: ::std::option::Option<&'static str> = Some("pyo3_capsule_api");
    #[inline]
    fn type_object_raw(py: pyo3::Python<'_>) -> *mut pyo3::ffi::PyTypeObject {
        ensure_example_api(py).ExampleType
    }
}

impl pyo3::impl_::pyclass::PyClassImpl for PyExample {
    const DOC: &'static str = "\u{0}";
    const IS_BASETYPE: bool = false;
    const IS_SUBCLASS: bool = false;
    const IS_MAPPING: bool = false;
    type Layout = pyo3::PyCell<Self>;
    type BaseType = pyo3::PyAny;
    type ThreadChecker = pyo3::impl_::pyclass::ThreadCheckerStub<PyExample>;
    fn for_all_items(visitor: &mut dyn ::std::ops::FnMut(&pyo3::impl_::pyclass::PyClassItems)) {
        use pyo3::impl_::pyclass::*;
        let collector = PyClassImplCollector::<Self>::new();
        static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
            methods: &[],
            slots: &[],
        };
        visitor(&INTRINSIC_ITEMS);
        visitor(collector.py_methods());
        visitor(collector.object_protocol_items());
        visitor(collector.number_protocol_items());
        visitor(collector.iter_protocol_items());
        visitor(collector.gc_protocol_items());
        visitor(collector.descr_protocol_items());
        visitor(collector.mapping_protocol_items());
        visitor(collector.sequence_protocol_items());
        visitor(collector.async_protocol_items());
        visitor(collector.buffer_protocol_items());
    }
}

impl pyo3::PyClass for PyExample {
    type Dict = pyo3::impl_::pyclass::PyClassDummySlot;
    type WeakRef = pyo3::impl_::pyclass::PyClassDummySlot;
    type BaseNativeType = pyo3::PyAny;
}
pub struct Wrapper(PyCell<PyExample>);
unsafe impl pyo3::PyNativeType for Wrapper {}

fn ensure_example_api(_py: Python<'_>) -> &'static PyO3Example_CAPI {
    unsafe {
        if PyExampleAPI().is_null() {
            PyExample_IMPORT();
        }

        &*PyExampleAPI()
    }
}

#[inline]
/// Check if `op` is a `PyDateTimeAPI.DateTimeType` or subtype.
pub unsafe fn PyExample_Check(op: *mut ffi::PyObject) -> c_int {
    ffi::PyObject_TypeCheck(
        op,
        ensure_example_api(Python::assume_gil_acquired()).ExampleType,
    ) as c_int
}

impl<'py> FromPyObject<'py> for PyExample {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        unsafe {
            if PyExample_Check(ob.as_ptr()) == 0 {
                return Err(PyDowncastError::new(ob, PyExample::NAME).into());
            }
            let _cell = Wrapper::unchecked_downcast(ob);
            let unwrapped: &PyExample = &_cell.0.try_borrow().unwrap();
            Ok(PyExample {
                inner: unwrapped.inner,
            })
        }
    }
}

impl pyo3::IntoPy<pyo3::PyObject> for PyExample {
    fn into_py(self, py: pyo3::Python) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(pyo3::Py::new(py, self).unwrap(), py)
    }
}

#[inline]
pub unsafe fn PyExampleAPI() -> *mut PyO3Example_CAPI {
    *PyExampleAPI_impl.0.get()
}

struct PyExampleAPISingleton(UnsafeCell<*mut PyO3Example_CAPI>);
unsafe impl Sync for PyExampleAPISingleton {}
static PyExampleAPI_impl: PyExampleAPISingleton =
    PyExampleAPISingleton(UnsafeCell::new(ptr::null_mut()));

/// Populates the `PyExampleAPI` object
pub unsafe fn PyExample_IMPORT() {
    let py_example_c_api = {
        // PyExample_CAPSULE_NAME is a macro in C
        //
        //PyCapsule_Import(PyO3Example_CAPSULE_NAME.as_ptr(), 0) as *mut PyO3Example_CAPI
        let module = CString::new("pyo3_capsule_api").unwrap();
        let capsule = CString::new("_API").unwrap();
        unsafe {
            let module = ffi::PyImport_ImportModule(module.as_ptr());
            assert!(!module.is_null(), "Failed to import PyExample module");
            let capsule = ffi::PyObject_GetAttrString(module as _, capsule.as_ptr());
            assert!(!capsule.is_null(), "Failed to get PyExample API capsule");
            let name = ffi::PyCapsule_GetName(capsule);
            ffi::PyCapsule_GetPointer(capsule, PYO3_CAPSULE_API_NAME.as_ptr())
                as *mut PyO3Example_CAPI
        }
    };
    *PyExampleAPI_impl.0.get() = py_example_c_api;
}

#[pyfunction]
/// Formats the sum of two numbers as string
fn sum_as_string(a: PyExample) -> PyResult<PyExample> {
    println!("{:?}", a.inner);
    Ok(a)
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn rust_binding(py: Python, m: &PyModule) -> PyResult<()> {
    println!("loading library");
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;

    Ok(())
}
