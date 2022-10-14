use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/Comm_Listen
pub fn comm_listen<T>(key_ptr: &T, key_size: u32, cookie: u32) {
    unsafe { Env::Comm_Listen(key_ptr as *const T as *const c_void, key_size, cookie) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Comm_Send
pub fn comm_send<T>(pk_remote: &PubKey, buffer_ptr: &T, buffer_size: u32) {
    unsafe { Env::Comm_Send(pk_remote, buffer_ptr as *const T as *const c_void, buffer_size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Comm_Read
pub fn comm_read<T>(buffer_ptr: &T, buffer_size: u32, cookie: &mut u32, keep: u8) -> u32 {
    unsafe { Env::Comm_Read(buffer_ptr as *const T as *mut c_void, buffer_size, cookie, keep) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Comm_WaitMsg
pub fn comm_wait_msg(timeout_ms: u32) {
    unsafe { Env::Comm_WaitMsg(timeout_ms) }
}
