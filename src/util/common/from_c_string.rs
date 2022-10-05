use core::str::Utf8Error;

use beam_bvm_interface::root::{c_char};
use cstr_core::{ CStr};

pub trait FromCString {
    fn from_c_string(&self) -> Result<&'static str, Utf8Error>;
}


impl FromCString for *const c_char {
    fn from_c_string(self: &*const c_char) -> Result<&'static str, Utf8Error> {
        unsafe {
            CStr::from_ptr((*self) as *const i8).to_str()
        }
    }
}

impl FromCString for *mut c_char {
    fn from_c_string(self: &*mut c_char) -> Result<&'static str, Utf8Error> {
        unsafe {
            CStr::from_ptr((*self) as *mut i8).to_str()
        }
    }
}




