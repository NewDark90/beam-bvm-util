use core::ffi::CStr;

use crate::common::{safe, extensions::*};

/// https://github.com/BeamMW/shader-sdk/wiki/Strlen
pub fn strlen(p: &CStr) -> u32 {
    safe::strlen(p.as_bvm_ptr())
}

/// https://github.com/BeamMW/shader-sdk/wiki/Strcmp
pub fn strcmp(sz1: &CStr, sz2: &CStr) -> i32 {
    safe::strcmp(sz1.as_bvm_ptr(), sz2.as_bvm_ptr())
}