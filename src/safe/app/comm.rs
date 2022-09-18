use beam_bvm_interface::root::*;

pub fn comm_listen<T>(key_ptr: *const T, key_size: u32, cookie: u32) {
    unsafe { Env::Comm_Listen(key_ptr as *const c_void, key_size, cookie) }
}

//buffer type?
pub fn comm_send(pk_remote: *const PubKey, buffer_ptr: *const c_void, buffer_size: u32) {
    unsafe { Env::Comm_Send(pk_remote, buffer_ptr, buffer_size) }
}

//buffer type? (docs?)
pub fn comm_read(buffer_ptr: *mut c_void, buffer_size: u32, cookie: *mut u32, keep: u8) -> u32 {
    unsafe { Env::Comm_Read(buffer_ptr as *mut c_void, buffer_size, cookie, keep) }
}

pub fn comm_wait_msg(timeout_ms: u32) {
    unsafe { Env::Comm_WaitMsg(timeout_ms) }
}