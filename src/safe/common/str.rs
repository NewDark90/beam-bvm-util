use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/Strlen
pub fn strlen<V>(p: *const V) -> u32 {
    unsafe { Env::Strlen(p as *const c_char) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Strcmp
pub fn strcmp<V>(sz1: *const c_char, sz2: *const c_char) -> i32 {
    unsafe { Env::Strcmp(sz1, sz2) }
}