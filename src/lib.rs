use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};



pub struct HeaderR {
    pub key: String,
    pub value: String,
}


pub struct  ResponseR {
    pub status: i32,
    pub body: String,
}




// Estructuras compatibles con C
#[repr(C)]
pub struct HeaderC {
    pub key: *const c_char,
    pub value: *const c_char,
}

#[repr(C)]
pub struct ResponseC {
    pub status: c_int,
    pub body: *const c_char,
}

// Conversión de HeaderR a HeaderC
impl HeaderC {
    pub fn from_rust(header: &HeaderR) -> HeaderC {
        HeaderC {
            key: CString::new(header.key.clone()).unwrap().into_raw(),
            value: CString::new(header.value.clone()).unwrap().into_raw(),
        }
    }

    pub fn into_rust(&self) -> HeaderR {
        HeaderR {
            key: unsafe { CStr::from_ptr(self.key).to_string_lossy().into_owned() },
            value: unsafe { CStr::from_ptr(self.value).to_string_lossy().into_owned() },
        }
    }
}

// Conversión de ResponseR a ResponseC
impl ResponseC {
    pub fn from_rust(response: &ResponseR) -> ResponseC {
        ResponseC {
            status: response.status,
            body: CString::new(response.body.clone()).unwrap().into_raw(),
        }
    }
}

// Interfaz en Rust
pub trait GetPostRustObjInterface {
    fn get_res(&self, url: String, headers: Vec<HeaderR>, body: String) -> ResponseR;
    fn post_res(&self, url: String, headers: Vec<HeaderR>, body: String) -> ResponseR;
}

// Objeto compatible con C
#[repr(C)]
pub struct GetPostObjC {
    rust_obj: Box<dyn GetPostRustObjInterface>,
}

impl GetPostObjC {
    pub fn new(rust_obj: Box<dyn GetPostRustObjInterface>) -> Self {
        Self { rust_obj }
    }
}

// // Implementación de funciones compatibles con C
// #[no_mangle]
// pub extern "C" fn get_post_obj_new(rust_obj: *mut dyn GetPostRustObjInterface) -> *mut GetPostObjC {
//     let rust_obj = unsafe { Box::from_raw(rust_obj) };
//     let wrapper = GetPostObjC::new(rust_obj);
//     Box::into_raw(Box::new(wrapper))
// }

#[no_mangle]
pub extern "C" fn get_post_obj_free(obj: *mut GetPostObjC) {
    if !obj.is_null() {
        unsafe {
            drop(Box::from_raw(obj));
        }
    }
}

#[no_mangle]
pub extern "C" fn get_res(
    obj: *mut GetPostObjC,
    url: *const c_char,
    headers: *const HeaderC,
    headers_len: c_int,
    body: *const c_char,
) -> ResponseC {
    let wrapper = unsafe { &*obj };

    let url = unsafe { CStr::from_ptr(url).to_string_lossy().into_owned() };
    let body = unsafe { CStr::from_ptr(body).to_string_lossy().into_owned() };
    let headers = unsafe {
        std::slice::from_raw_parts(headers, headers_len as usize)
            .iter()
            .map(|h| h.into_rust())
            .collect()
    };

    let response = wrapper.rust_obj.get_res(url, headers, body);
    ResponseC::from_rust(&response)
}

#[no_mangle]
pub extern "C" fn post_res(
    obj: *mut GetPostObjC,
    url: *const c_char,
    headers: *const HeaderC,
    headers_len: c_int,
    body: *const c_char,
) -> ResponseC {
    let wrapper = unsafe { &*obj };

    let url = unsafe { CStr::from_ptr(url).to_string_lossy().into_owned() };
    let body = unsafe { CStr::from_ptr(body).to_string_lossy().into_owned() };
    let headers = unsafe {
        std::slice::from_raw_parts(headers, headers_len as usize)
            .iter()
            .map(|h| h.into_rust())
            .collect()
    };

    let response = wrapper.rust_obj.post_res(url, headers, body);
    ResponseC::from_rust(&response)
}