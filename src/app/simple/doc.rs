

use core::{ptr::{null}, mem::size_of_val};
use alloc::string::String;
use beam_bvm_interface::root::c_char;

use crate::{common::{types::sized_result::SizedResult }, app::safe};

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetText
pub fn doc_get_text(id: &str) -> Result<SizedResult<String>, u32> {
    let size = safe::doc_get_text(id, &mut null::<c_char>(), 0);
    if size == 0 {
        return Err(size);
    }

    let mut str_data: String = String::with_capacity(size as usize);
    safe::doc_get_text::<String>(id, &mut str_data, size);
    Ok(SizedResult::<String>::new(str_data, size))
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetBlob
/// Ok returns the size
/// Error returns blob size and the size of the calculated size_of_val in that order
pub fn doc_get_blob<TResult>(id: &str, result: &mut TResult) -> Result<u32, [u32; 2]> {

    let size = safe::doc_get_blob(id, result, 0);
    let generic_size = size_of_val(result) as u32;

    if size > 0 && size == generic_size {
        safe::doc_get_blob(id, result, size);
        Ok(size)
    }
    else {
        Err([size, generic_size])
    }
    
}