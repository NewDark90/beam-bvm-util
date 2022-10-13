use crate::safe::common::str::{
    strlen as strlen_safe,
    strcmp as strcmp_safe,
};

/// https://github.com/BeamMW/shader-sdk/wiki/Strlen
pub fn strlen(p: &str) -> u32 {
    strlen_safe(p.as_ptr())
}

/// https://github.com/BeamMW/shader-sdk/wiki/Strcmp
pub fn strcmp(sz1: &str, sz2: &str) -> i32 {
    strcmp_safe(sz1.as_ptr(), sz2.as_ptr())
}