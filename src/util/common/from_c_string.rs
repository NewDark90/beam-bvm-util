use alloc::borrow::ToOwned;
use beam_bvm_interface::root::c_char;
use cstr_core::{ CStr};

pub trait FromCString {
    fn from_c_string(&self) -> &'static str;
}


impl FromCString for *const c_char {
    fn from_c_string(self: &*const i8) -> &'static str {
        unsafe {
            CStr::from_ptr(*self).to_str().expect("FromCString::from_c_string failed")
        }
    }
}



