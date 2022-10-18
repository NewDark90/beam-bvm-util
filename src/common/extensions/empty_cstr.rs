use core::ffi::CStr;

use alloc::ffi::CString;

pub trait EmptyCString {
    fn empty() -> CString;
}

pub trait EmptyCStr {
    fn empty() -> &'static CStr;
}


impl EmptyCString for CString {
    /// Creates a new empty CString
    fn empty() -> CString {
        CString::new("").expect("CString::new(\"\") failed.")
    }
}

static EMPTY_CSTR: &'static CStr = unsafe { CStr::from_bytes_with_nul_unchecked(&[0; 1]) };

impl EmptyCStr for CStr {
    /// Returns a single static empty CStr
    fn empty() -> &'static CStr {
        return EMPTY_CSTR
    }
}