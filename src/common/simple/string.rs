use crate::common::safe;

/// https://github.com/BeamMW/shader-sdk/wiki/Strlen
pub fn strlen(p: &str) -> u32 {
    safe::strlen(p.as_ptr())
}

/// https://github.com/BeamMW/shader-sdk/wiki/Strcmp
pub fn strcmp(sz1: &str, sz2: &str) -> i32 {
    safe::strcmp(sz1.as_ptr(), sz2.as_ptr())
}