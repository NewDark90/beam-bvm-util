use core::ffi::CStr;

use alloc::{ffi::CString, vec::Vec};


pub trait EmptyCString {
    fn empty() -> CString;
}

impl EmptyCString for CString {
    /// Creates a new empty CString
    fn empty() -> CString {
        unsafe {
            CString::from_vec_unchecked(Vec::<u8>::with_capacity(1))
        }
    }
}

pub trait EmptyCStr {
    fn empty() -> &'static CStr;
}

pub static EMPTY_CSTR: &'static CStr = unsafe { CStr::from_bytes_with_nul_unchecked(&[0; 1]) };

impl EmptyCStr for CStr {
    /// Returns a single static empty CStr
    fn empty() -> &'static CStr {
        return EMPTY_CSTR
    }
}