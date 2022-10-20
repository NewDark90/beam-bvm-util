//use alloc::{ffi::CString, borrow::ToOwned, string::String};

/* 
pub trait ToCString {
    fn to_c_string(&self) -> CString;
}

impl ToCString for String {
    fn to_c_string(&self) -> CString {
        CString::new(str::replace(&self.to_owned(), "\0", "")).expect("Removing nulls from string for CString::new failed")
    }
}

*/

use core::ffi::{CStr};

pub trait ToCStr {
    fn to_c_string(&self) -> &CStr;
}


impl ToCStr for &str {
    /// **MUST** include a null byte at the end or it is unsafe. 
    fn to_c_string(&self) -> &CStr {
        to_c_string(self)
    }
}

/// **MUST** include a null byte at the end or it is unsafe. 
pub const fn to_c_string(string: &str) -> &CStr {
    unsafe { 
        CStr::from_bytes_with_nul_unchecked(string.as_bytes()) 
    }
}
