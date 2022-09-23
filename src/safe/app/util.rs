
use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/GenerateRandom
pub fn generate_random(result_ptr: *mut c_void, size: u32) {
    unsafe { Env::GenerateRandom(result_ptr, size) }
}