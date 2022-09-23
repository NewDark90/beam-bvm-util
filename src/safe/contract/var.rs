use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/LoadVar
pub fn load_var<K, V>(
    key: *const K,
    key_size: u32,
    value: *mut V,
    value_size: u32,
    tag: u8,
) -> u32 {
    unsafe {
        Env::LoadVar(
            key as *const c_void,
            key_size,
            value as *mut c_void,
            value_size,
            tag,
        )
    }
}

/// https://github.com/BeamMW/shader-sdk/wiki/SaveVar (used in a way to clear variable)
pub fn del_var<K>(key: *const K, key_size: u32) -> u32 {
    unsafe {
        Env::SaveVar(
            key as *const c_void,
            key_size,
            0 as *const c_void,
            0,
            KeyTag_Internal,
        )
    }
}

/// https://github.com/BeamMW/shader-sdk/wiki/SaveVar
pub fn save_var<K, V>(key: *const K, key_size: u32, val: *const V, val_size: u32, tag: u8) -> u32 {
    unsafe {
        Env::SaveVar(
            key as *const c_void,
            key_size,
            val as *const c_void,
            val_size,
            tag,
        )
    }
}
