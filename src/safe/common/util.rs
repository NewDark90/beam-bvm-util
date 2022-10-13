use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/Halt
pub fn halt() {
    unsafe { Env::Halt() }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Write
pub fn write<T>(data: &T, data_size: u32, stream: u32) {
    unsafe { Env::Write(data as *const T as *const c_void, data_size, stream) }
}
