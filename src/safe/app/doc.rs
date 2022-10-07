use core::{ptr::null, slice, str::*};

use crate::types::sized_result::SizedResult;
use beam_bvm_interface::root::*;

const NULL_STR_PTR: *const u8 = null::<c_char>();

fn prop_name(id: &str) -> *const u8 {
    if id.len() > 0 {
        id.as_ptr()
    } else {
        NULL_STR_PTR
    }
}

// -- DOC CREATE --

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddText
pub fn doc_add_text(id: &str, val: &str) {
    unsafe { Env::DocAddText(prop_name(id), val.as_ptr()) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddNum32
pub fn doc_add_num32(id: &str, val: u32) {
    unsafe { Env::DocAddNum32(prop_name(id), val) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddNum64
pub fn doc_add_num64(id: &str, val: u64) {
    unsafe { Env::DocAddNum64(prop_name(id), val) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddBlob
pub fn doc_add_blob<V>(id: &str, val: *const V, val_size: u32) {
    unsafe {
        Env::DocAddBlob(
            prop_name(id) as *const c_char,
            val as *const c_void,
            val_size,
        )
    }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddGroup
pub fn doc_add_group(id: &str) {
    unsafe { Env::DocAddGroup(prop_name(id)) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocCloseGroup
pub fn doc_close_group() {
    unsafe { Env::DocCloseGroup() }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddArray
pub fn doc_add_array(id: &str) {
    unsafe { Env::DocAddArray(prop_name(id)) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocCloseArray
pub fn doc_close_array() {
    unsafe { Env::DocCloseArray() }
}

// -- DOC GET --

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetText
pub fn doc_get_text(id: &str, val: *mut c_char, val_size: u32) -> u32 {
    unsafe { Env::DocGetText(prop_name(id), val, val_size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetNum64
pub fn doc_get_num64(id: &str, out: *mut u64) -> u8 {
    unsafe { Env::DocGetNum64(prop_name(id), out) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetNum32
pub fn doc_get_num32(id: &str, out: *mut u32) -> u8 {
    unsafe { Env::DocGetNum32(prop_name(id), out) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetBlob
pub fn doc_get_blob<V>(id: &str, val: *mut V, val_size: u32) -> u32 {
    unsafe { Env::DocGetBlob(prop_name(id), val as *mut c_void, val_size) }
}

// -- DOC GET SIMPLE --

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetText
pub fn doc_get_text_simple<'a, V>(id: &'a str) -> SizedResult<&'a str> {
    unsafe {
        let mut result: SizedResult<&str> = Default::default();

        result.size = Env::DocGetText(prop_name(id), NULL_STR_PTR as *mut c_char, 0);

        if result.size > 0 {
            let buffer = Env::StackAlloc(result.size) as *mut c_char;
            Env::DocGetText(prop_name(id), buffer, result.size);
            let slice = slice::from_raw_parts(buffer, result.size as usize);

            match from_utf8(slice) {
                Ok(r) => {
                    result.value = r;
                }
                Err(_) => {}
            }
        }

        result
    }
}

pub fn doc_get_blob_simple<V>(id: &str) -> SizedResult<*mut c_void> {
    unsafe {
        let mut result = SizedResult::<*mut c_void> {
            size: 0,
            value: NULL_STR_PTR as *mut c_void,
        };

        result.size = Env::DocGetBlob(prop_name(id), NULL_STR_PTR as *mut c_void, 0);

        if result.size > 0 {
            let buffer = Env::StackAlloc(result.size);
            Env::DocGetBlob(prop_name(id), buffer, result.size);
            result.value = buffer;
        }

        result
    }
}
