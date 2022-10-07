use core::str::Utf8Error;

use cstr_core::{ CStr, c_char};

pub trait FromCString {
    fn from_c_string(&self) -> Result<&'static str, Utf8Error>;
}

impl FromCString for *const i8 {
    fn from_c_string(self: &*const i8) -> Result<&'static str, Utf8Error> {
        unsafe {
            CStr::from_ptr((*self) as *const c_char).to_str()
        }
    }
}

impl FromCString for *mut i8 {
    fn from_c_string(self: &*mut i8) -> Result<&'static str, Utf8Error> {
        unsafe {
            CStr::from_ptr((*self) as *mut c_char).to_str()
        }
    }
}

impl FromCString for *const u8 {
    fn from_c_string(self: &*const u8) -> Result<&'static str, Utf8Error> {
        unsafe {
            CStr::from_ptr((*self) as *const c_char).to_str()
        }
    }
}

impl FromCString for *mut u8 {
    fn from_c_string(self: &*mut u8) -> Result<&'static str, Utf8Error> {
        unsafe {
            CStr::from_ptr((*self) as *mut c_char).to_str()
        }
    }
}


