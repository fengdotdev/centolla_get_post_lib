mod models;


use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use crate::models::rust_obj::RustObject;


#[repr(C)]
pub struct CppCompatibleObject {
    obj: *mut RustObject,
    get_res: extern "C" fn(*mut RustObject, *const c_char) -> *mut c_char,
    post_res: extern "C" fn(*mut RustObject, *const c_char) -> *mut c_char,
}

impl CppCompatibleObject {
    pub fn new() -> Self {
        let obj = Box::into_raw(Box::new(RustObject::new()));
        Self {
            obj,
            get_res: Self::call_get_res,
            post_res: Self::call_post_res,
        }
    }

    extern "C" fn call_get_res(obj: *mut RustObject, call: *const c_char) -> *mut c_char {
        unsafe {
            let obj = &*obj;
            let call = CStr::from_ptr(call).to_string_lossy().into_owned();
            let result = obj.get_res(call);
            CString::new(result).unwrap().into_raw()
        }
    }

    extern "C" fn call_post_res(obj: *mut RustObject, call: *const c_char) -> *mut c_char {
        unsafe {
            let obj = &*obj;
            let call = CStr::from_ptr(call).to_string_lossy().into_owned();
            let result = obj.post_res(call);
            CString::new(result).unwrap().into_raw()
        }
    }
}

#[no_mangle]
pub extern "C" fn create_cpp_compatible_object() -> *mut CppCompatibleObject {
    let obj = CppCompatibleObject::new();
    Box::into_raw(Box::new(obj))
}

#[no_mangle]
pub extern "C" fn destroy_cpp_compatible_object(obj: *mut CppCompatibleObject) {
    if obj.is_null() {
        return;
    }
    unsafe {
        let obj = Box::from_raw(obj);
        if !obj.obj.is_null() {
            let _ = Box::from_raw(obj.obj); // Liberar RustObject
        }
    }
}
