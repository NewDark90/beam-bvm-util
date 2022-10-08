use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/Strlen
pub fn strlen(p: *const c_char) -> u32 {
    unsafe { Env::Strlen(p) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Strlen
pub fn strlen_simple(p: &str) -> u32 {
    strlen(p.as_ptr())
}

/// https://github.com/BeamMW/shader-sdk/wiki/Strcmp
pub fn strcmp(sz1: *const c_char, sz2: *const c_char) -> i32 {
    unsafe { Env::Strcmp(sz1, sz2) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Strcmp
pub fn strcmp_simple(sz1: &str, sz2: &str) -> i32 {
    strcmp(sz1.as_ptr(), sz2.as_ptr())
}