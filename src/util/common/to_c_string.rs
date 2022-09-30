use alloc::{borrow::ToOwned};
use beam_bvm_interface::root::c_char;
use cstr_core::CString;

pub trait ToCString {
    fn to_c_string(&self) -> *const c_char;
}


impl ToCString for str {
    fn to_c_string(&self) -> *const c_char {
        (
            //Ensure terminator at the end and no other spots
            self.to_owned().replace("\0", "") + "\0"
        ).as_ptr() as *const c_char
    }
}



