

use core::{ptr::{null}, slice::{from_raw_parts}, str::{from_utf8}, mem::size_of_val};
use beam_bvm_interface::root::{c_char, Env, c_void};

use crate::types::sized_result::SizedResult;

const NULL_STR_PTR: *const u8 = null::<c_char>();

fn prop_name(id: &str) -> *const u8 {
    if id.len() > 0 {
        id.as_ptr()
    } else {
        NULL_STR_PTR
    }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetText
pub fn doc_get_text(id: &str) -> Result<SizedResult<&str>, u32> {
    unsafe {
        let size = Env::DocGetText(prop_name(id), NULL_STR_PTR as *mut c_char, 0);
        if size > 0 {
            let buffer = Env::StackAlloc(size);
            Env::DocGetText(prop_name(id), buffer as *mut u8, size);
            let slice = from_raw_parts::<'static, u8>(buffer as *const u8, size as usize);
            match from_utf8(slice) {
                Ok(value) => { 
                    return Ok(SizedResult::<&str> {
                        value,
                        size
                    })
                },
                Err(err) => { 
                    Env::Halt(); 
                }
            }
        }
        return Err(size);
    }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetBlob
/// Ok returns the size
/// Error returns blob size and the size of the calculated size_of_val in that order
pub fn doc_get_blob<TResult>(id: &str, result: &mut TResult) -> Result<u32, [u32; 2]> {
    unsafe {
        let size = Env::DocGetBlob(prop_name(id), result as *mut TResult as *mut c_void, 0);
        let generic_size = size_of_val(result) as u32;

        if size > 0 && size == generic_size {
            Env::DocGetBlob(prop_name(id), result as *mut TResult as *mut c_void, size);
            Ok(size)
        }
        else {
            Err([size, generic_size])
        }
    }
}