use core::mem::size_of_val;

use beam_bvm_interface::root::{c_void, AssetID};

use crate::safe::contract::assets::asset_create as asset_create_safe;

/// https://github.com/BeamMW/shader-sdk/wiki/AssetCreate
/// https://github.com/BeamMW/beam/wiki/Asset-Descriptor-v1.0
pub fn asset_create(metadata: &str) -> AssetID {
    unsafe {
        asset_create_safe(
            metadata as *const str as *const c_void,
            size_of_val::<str>(metadata) as u32,
        )
    }
}
