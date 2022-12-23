#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use pyo3;
use pyo3::ffi::PyCapsule_New;
use pyo3::prelude::*;
use pyo3::types::PyCapsule;
use pyo3::{PyResult, PyTypeInfo, Python};
use std::os::raw::c_void;
use pyo3::ffi::PyTypeObject;
static PYO3_CAPSULE_API_NAME: &std::ffi::CStr = unsafe {
    std::mem::transmute::<_, &std::ffi::CStr>("pyo3_capsule_api._API\u{0}")
};
static mut PYO3_EXAMPLE_CAPI: *const PyO3Example_CAPI = std::ptr::null();
#[repr(C)]
struct PyExample {
    inner: i64,
}
const _: () = {
    use ::pyo3 as _pyo3;
    unsafe impl _pyo3::type_object::PyTypeInfo for PyExample {
        type AsRefTarget = _pyo3::PyCell<Self>;
        const NAME: &'static str = "PyExample";
        const MODULE: ::std::option::Option<&'static str> = ::core::option::Option::None;
        #[inline]
        fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
            use _pyo3::type_object::LazyStaticType;
            static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
            TYPE_OBJECT.get_or_init::<Self>(py)
        }
    }
    impl _pyo3::PyClass for PyExample {
        type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
        type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
        type BaseNativeType = _pyo3::PyAny;
    }
    impl<'a> _pyo3::derive_utils::ExtractExt<'a> for &'a PyExample {
        type Target = _pyo3::PyRef<'a, PyExample>;
    }
    impl<'a> _pyo3::derive_utils::ExtractExt<'a> for &'a mut PyExample {
        type Target = _pyo3::PyRefMut<'a, PyExample>;
    }
    impl _pyo3::IntoPy<_pyo3::PyObject> for PyExample {
        fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
            _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
        }
    }
    impl _pyo3::impl_::pyclass::PyClassImpl for PyExample {
        const DOC: &'static str = "\u{0}";
        const IS_BASETYPE: bool = false;
        const IS_SUBCLASS: bool = false;
        const IS_MAPPING: bool = false;
        type Layout = _pyo3::PyCell<Self>;
        type BaseType = _pyo3::PyAny;
        type ThreadChecker = _pyo3::impl_::pyclass::ThreadCheckerStub<PyExample>;
        fn for_all_items(
            visitor: &mut dyn ::std::ops::FnMut(&_pyo3::impl_::pyclass::PyClassItems),
        ) {
            use _pyo3::impl_::pyclass::*;
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
};
impl PyExample {
    fn new(value: i64) -> Self {
        Self { inner: value }
    }
}
const _: () = {
    use ::pyo3 as _pyo3;
    impl _pyo3::impl_::pyclass::PyMethods<PyExample>
    for _pyo3::impl_::pyclass::PyClassImplCollector<PyExample> {
        fn py_methods(self) -> &'static _pyo3::impl_::pyclass::PyClassItems {
            static ITEMS: _pyo3::impl_::pyclass::PyClassItems = _pyo3::impl_::pyclass::PyClassItems {
                methods: &[],
                slots: &[
                    {
                        impl PyExample {
                            #[doc(hidden)]
                            unsafe extern "C" fn __pymethod__new__(
                                subtype: *mut ::pyo3::ffi::PyTypeObject,
                                _args: *mut ::pyo3::ffi::PyObject,
                                _kwargs: *mut ::pyo3::ffi::PyObject,
                            ) -> *mut ::pyo3::ffi::PyObject {
                                use ::pyo3 as _pyo3;
                                use _pyo3::callback::IntoPyCallbackOutput;
                                let gil = _pyo3::GILPool::new();
                                let _py = gil.python();
                                _pyo3::callback::panic_result_into_callback_output(
                                    _py,
                                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                                        const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription = _pyo3::impl_::extract_argument::FunctionDescription {
                                            cls_name: ::std::option::Option::Some(
                                                <PyExample as _pyo3::type_object::PyTypeInfo>::NAME,
                                            ),
                                            func_name: "__new__",
                                            positional_parameter_names: &["value"],
                                            positional_only_parameters: 0usize,
                                            required_positional_parameters: 1usize,
                                            keyword_only_parameters: &[],
                                        };
                                        let mut output = [::std::option::Option::None; 1usize];
                                        let (_args, _kwargs) = DESCRIPTION
                                            .extract_arguments_tuple_dict::<
                                                _pyo3::impl_::extract_argument::NoVarargs,
                                                _pyo3::impl_::extract_argument::NoVarkeywords,
                                            >(_py, _args, _kwargs, &mut output)?;
                                        let arg0 = _pyo3::impl_::extract_argument::extract_argument(
                                            _pyo3::impl_::extract_argument::unwrap_required_argument(
                                                output[0usize],
                                            ),
                                            "value",
                                        )?;
                                        let result = PyExample::new(arg0);
                                        let initializer: _pyo3::PyClassInitializer<PyExample> = result
                                            .convert(_py)?;
                                        let cell = initializer
                                            .create_cell_from_subtype(_py, subtype)?;
                                        ::std::result::Result::Ok(cell as *mut _pyo3::ffi::PyObject)
                                    }),
                                )
                            }
                        }
                        _pyo3::ffi::PyType_Slot {
                            slot: _pyo3::ffi::Py_tp_new,
                            pfunc: PyExample::__pymethod__new__ as _pyo3::ffi::newfunc
                                as _,
                        }
                    },
                ],
            };
            &ITEMS
        }
    }
};
#[repr(C)]
pub struct PyO3Example_CAPI {
    pub ExampleType: *mut PyTypeObject,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for PyO3Example_CAPI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Self { ExampleType: ref __self_0_0 } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(
                    f,
                    "PyO3Example_CAPI",
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "ExampleType",
                    &&(*__self_0_0),
                );
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for PyO3Example_CAPI {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for PyO3Example_CAPI {
    #[inline]
    fn clone(&self) -> PyO3Example_CAPI {
        {
            let _: ::core::clone::AssertParamIsClone<*mut PyTypeObject>;
            *self
        }
    }
}
/// This module is a python module implemented in Rust.
fn pyo3_capsule_api(py: Python, m: &PyModule) -> PyResult<()> {
    let example_type = PyExample::type_object_raw(py);
    let mut _example_api = PyO3Example_CAPI {
        ExampleType: example_type,
    };
    let mut _example_api = Box::new(_example_api);
    unsafe {
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
#[no_mangle]
#[allow(non_snake_case)]
/// This autogenerated function is called by the python interpreter when importing
/// the module.
pub unsafe extern "C" fn PyInit_pyo3_capsule_api() -> *mut ::pyo3::ffi::PyObject {
    unsafe { __PYO3_PYMODULE_DEF_PYO3_CAPSULE_API.module_init() }
}
#[doc(hidden)]
static __PYO3_PYMODULE_DEF_PYO3_CAPSULE_API: ::pyo3::impl_::pymodule::ModuleDef = unsafe {
    ::pyo3::impl_::pymodule::ModuleDef::new(
        "pyo3_capsule_api\u{0}",
        "This module is a python module implemented in Rust.\u{0}",
        ::pyo3::impl_::pymodule::ModuleInitializer(pyo3_capsule_api),
    )
};
