use beam_bvm_interface::root::*;

pub fn doc_add_text<V>(id: &str, val: *const V) {
    unsafe { Env::DocAddText(id.as_ptr() as *const c_char, val as *const c_char) }
}

pub fn doc_get_text<V>(id: &str, val: *mut V, val_size: u32) -> u32 {
    unsafe { Env::DocGetText(id.as_ptr() as *const c_char, val as *mut c_char, val_size) }
}

pub fn doc_add_num32(id: &str, val: u32) {
    unsafe { Env::DocAddNum32(id.as_ptr() as *const c_char, val) }
}

pub fn doc_get_num64(id: &str, out: *mut u64) -> u8 {
    unsafe { Env::DocGetNum64(id.as_ptr() as *const c_char, out) }
}

pub fn doc_get_num32(id: &str, out: *mut u32) -> u8 {
    unsafe { Env::DocGetNum32(id.as_ptr() as *const c_char, out) }
}

pub fn doc_add_num64(id: &str, val: u64) {
    unsafe { Env::DocAddNum64(id.as_ptr() as *const c_char, val) }
}

pub fn doc_add_blob<V>(id: &str, val: *const V, val_size: u32) {
    unsafe { Env::DocAddBlob(id.as_ptr() as *const c_char, val as *const c_void, val_size) }
}

pub fn doc_get_blob<V>(id: &str, val: *mut V, val_size: u32) -> u32 {
    unsafe { Env::DocGetBlob(id.as_ptr() as *const c_char, val as *mut c_void, val_size) }
}

pub fn doc_add_group(id: &str) {
    unsafe { Env::DocAddGroup(id.as_ptr() as *const c_char) }
}

pub fn doc_close_group() {
    unsafe { Env::DocCloseGroup() }
}

pub fn doc_add_array(id: &str) {
    unsafe { Env::DocAddArray(id.as_ptr() as *const c_char) }
}

pub fn doc_close_array() {
    unsafe { Env::DocCloseArray() }
}