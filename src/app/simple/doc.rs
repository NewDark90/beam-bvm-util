use alloc::{ffi::CString, vec::Vec};
use beam_bvm_interface::root::c_char;
use core::{ffi::CStr, mem::size_of_val, ptr::null};

use crate::{app::safe, common::types::sized_result::SizedResult};


/// https://github.com/BeamMW/shader-sdk/wiki/DocGetText
pub fn doc_get_text(id: &CStr) -> Result<SizedResult<CString>, u32> {
    let size = safe::doc_get_text(id, &mut null::<c_char>(), 0);
    if size == 0 {
        return Err(size);
    }

    let mut str_data = Vec::<u8>::with_capacity(size as usize);
    safe::doc_get_text::<Vec<u8>>(id, &mut str_data, size);

    match CString::from_vec_with_nul(str_data) {
        Ok(c_str) => {
            Ok(SizedResult::<CString>::new(c_str, size))
        },
        Err(_) => Err(size)
    }
}


/// https://github.com/BeamMW/shader-sdk/wiki/DocGetBlob
/// Ok returns the size
/// Error returns blob size and the size of the calculated size_of_val in that order
pub fn doc_get_blob<TResult>(id: &CStr, result: &mut TResult) -> Result<u32, [u32; 2]> {
    let size = safe::doc_get_blob(id, result, 0);
    let generic_size = size_of_val(result) as u32;

    if size > 0 && size == generic_size {
        safe::doc_get_blob(id, result, size);
        Ok(size)
    } else {
        Err([size, generic_size])
    }
}
