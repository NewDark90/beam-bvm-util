use core::ffi::CStr;
use beam_bvm_interface::root::{c_void, AssetID};

use crate::contract::safe;
use crate::common::extensions::as_bvm_ptr::*;

/// https://github.com/BeamMW/shader-sdk/wiki/AssetCreate
/// https://github.com/BeamMW/beam/wiki/Asset-Descriptor-v1.0
pub fn asset_create(metadata: &CStr) -> AssetID {
    safe::asset_create(
        metadata.as_bvm_ptr() as *const c_void,
        metadata.to_bytes_with_nul().len() as u32
    )
}
