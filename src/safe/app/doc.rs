use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddText
pub fn doc_add_text<V>(id: &str, val: *const V) {
    unsafe { Env::DocAddText(id.as_ptr() as *const c_char, val as *const c_char) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetText
pub fn doc_get_text<V>(id: &str, val: *mut V, val_size: u32) -> u32 {
    unsafe { Env::DocGetText(id.as_ptr() as *const c_char, val as *mut c_char, val_size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddNum32
pub fn doc_add_num32(id: &str, val: u32) {
    unsafe { Env::DocAddNum32(id.as_ptr() as *const c_char, val) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetNum32
pub fn doc_get_num32(id: &str, out: *mut u32) -> u8 {
    unsafe { Env::DocGetNum32(id.as_ptr() as *const c_char, out) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddNum64
pub fn doc_add_num64(id: &str, val: u64) {
    unsafe { Env::DocAddNum64(id.as_ptr() as *const c_char, val) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetNum64
pub fn doc_get_num64(id: &str, out: *mut u64) -> u8 {
    unsafe { Env::DocGetNum64(id.as_ptr() as *const c_char, out) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddBlob
pub fn doc_add_blob<V>(id: &str, val: *const V, val_size: u32) {
    unsafe { Env::DocAddBlob(id.as_ptr() as *const c_char, val as *const c_void, val_size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocGetBlob
pub fn doc_get_blob<V>(id: &str, val: *mut V, val_size: u32) -> u32 {
    unsafe { Env::DocGetBlob(id.as_ptr() as *const c_char, val as *mut c_void, val_size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddGroup
pub fn doc_add_group(id: &str) {
    unsafe { Env::DocAddGroup(id.as_ptr() as *const c_char) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocCloseGroup
pub fn doc_close_group() {
    unsafe { Env::DocCloseGroup() }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocAddArray
pub fn doc_add_array(id: &str) {
    unsafe { Env::DocAddArray(id.as_ptr() as *const c_char) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/DocCloseArray
pub fn doc_close_array() {
    unsafe { Env::DocCloseArray() }
}