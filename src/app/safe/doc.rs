
use core::ffi::CStr;

use beam_bvm_interface::root::*;

use crate::common::extensions::*;

// -- DOC CREATE --

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddText
pub fn doc_add_text(id: &CStr, val: &CStr) {
    unsafe { Env::DocAddText(id.as_bvm_ptr(), val.as_bvm_ptr()) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddNum32
pub fn doc_add_num32(id: &CStr, val: u32) {
    unsafe { Env::DocAddNum32(id.as_bvm_ptr(), val) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddNum64
pub fn doc_add_num64(id: &CStr, val: u64) {
    unsafe { Env::DocAddNum64(id.as_bvm_ptr(), val) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddBlob
pub fn doc_add_blob<V>(id: &CStr, val: &V, val_size: u32) {
    unsafe {
        Env::DocAddBlob(
            id.as_bvm_ptr(),
            val as *const V as *const c_void,
            val_size,
        )
    }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddGroup
pub fn doc_add_group(id: &CStr) {
    unsafe { Env::DocAddGroup(id.as_bvm_ptr()) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocCloseGroup
pub fn doc_close_group() {
    unsafe { Env::DocCloseGroup() }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddArray
pub fn doc_add_array(id: &CStr) {
    unsafe { Env::DocAddArray(id.as_bvm_ptr()) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocCloseArray
pub fn doc_close_array() {
    unsafe { Env::DocCloseArray() }
}

// -- DOC GET --

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetText
/// Should set a C style array of u8s as a buffer. 
pub fn doc_get_text<V>(id: &CStr, val: &mut V, val_size: u32) -> u32 {
    unsafe { Env::DocGetText(id.as_bvm_ptr(), val as *mut V as *mut c_char, val_size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetNum64
pub fn doc_get_num64(id: &CStr, out: &mut u64) -> u8 {
    unsafe { Env::DocGetNum64(id.as_bvm_ptr(), out) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetNum32
pub fn doc_get_num32(id: &CStr, out: &mut u32) -> u8 {
    unsafe { Env::DocGetNum32(id.as_bvm_ptr(), out) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetBlob
pub fn doc_get_blob<V>(id: &CStr, val: &mut V, val_size: u32) -> u32 {
    unsafe { Env::DocGetBlob(id.as_bvm_ptr(), val as *mut V as *mut c_void, val_size) }
}

