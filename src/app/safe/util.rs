use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/GenerateRandom
pub fn generate_random<TResult>(result_ptr: &mut TResult, size: u32) {
    unsafe { Env::GenerateRandom(result_ptr as *mut TResult as *mut c_void, size) }
}
