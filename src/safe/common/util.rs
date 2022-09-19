use beam_bvm_interface::root::*;

pub fn halt_if(condition: bool) {
    if condition {
        halt()
    }
}

pub fn halt() {
    unsafe { Env::Halt() }
}

pub fn write<T>(data_ptr: *const T, data_size: u32, stream: u32) {
    unsafe { Env::Write(data_ptr as *const c_void, data_size, stream) }
}
