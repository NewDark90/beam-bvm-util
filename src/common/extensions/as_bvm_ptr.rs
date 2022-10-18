use core::{ffi::CStr};
use beam_bvm_interface::root::c_char;

pub trait AsBvmPointer {
    fn as_bvm_ptr(&self) -> *const c_char;
} 

impl AsBvmPointer for CStr {
    /// Simply casts as_ptr to bvm c_char
    /// 
    /// i8/u8 => u8
    fn as_bvm_ptr(&self) -> *const c_char {
        self.as_ptr() as *const c_char 
    }
}

