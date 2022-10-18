use alloc::{ffi::CString, borrow::ToOwned, string::String};

pub trait ToCString {
    fn to_c_string(&self) -> CString;
}

impl ToCString for String {
    fn to_c_string(&self) -> CString {
        CString::new(str::replace(&self.to_owned(), "\0", "")).expect("Removing nulls from string for CString::new failed")
    }
}

impl ToCString for &str {
    fn to_c_string(&self) -> CString {
        CString::new(str::replace(&self.to_owned(), "\0", "")).expect("Removing nulls from string for CString::new failed")
    }
}
