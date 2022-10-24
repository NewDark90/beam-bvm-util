use alloc::{ffi::CString, string::String, vec::Vec};

pub trait ToCString {
    fn to_c_string(&self) -> CString;
}

impl ToCString for String {
    /// Ideally does **NOT** contain the trailing \0. 
    /// 
    /// However, it **is safe** to use any \0 within the string as it will strip them out and add it's own.
    fn to_c_string(&self) -> CString {
        to_c_string(self)
    }
}

impl ToCString for &str {
    /// Ideally does **NOT** contain the trailing \0. 
    /// 
    /// However, it **is safe** to use any \0 within the string as it will strip them out and add it's own.
    /// 
    /// Will create a new instance
    fn to_c_string(&self) -> CString {
        to_c_string(self)
    }
}

/// Ideally does **NOT** contain the trailing \0. 
/// 
/// However, it **is safe** to use any \0 within the string as it will strip them out and add it's own.
pub fn to_c_string(string: &str) -> CString {
    let len = string.len();
    let mut vec = Vec::<u8>::with_capacity(len.checked_add(1).unwrap_or(len));
    for byte in string.as_bytes() {
        if *byte != 0 { vec.push(*byte); }
    }
    vec.push(0);

    unsafe {
        //Above code safely removes all nulls with one pushed to the end. 
        CString::from_vec_with_nul_unchecked(vec)
    }
}

