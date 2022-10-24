use core::ffi::{CStr};

pub trait ToCStr {
    fn to_c_str(&self) -> &CStr;
}


impl ToCStr for &str {
    /// **MUST** include a null byte at the end or it is unsafe. 
    fn to_c_str(&self) -> &CStr {
        to_c_str(self)
    }
}

/// **MUST** include a null byte at the end or it is unsafe. 
pub const fn to_c_str(string: &str) -> &CStr {
    unsafe { 
        CStr::from_bytes_with_nul_unchecked(string.as_bytes()) 
    }
}
