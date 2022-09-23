use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/EmitLog
pub fn emit_log<K, V>(
    key: *const K,
    key_size: u32,
    value: *const V,
    value_size: u32,
    tag: u8,
) -> u32 {
    unsafe {
        Env::EmitLog(
            key as *const c_void,
            key_size,
            value as *const c_void,
            value_size,
            tag,
        )
    }
}

/// https://github.com/BeamMW/shader-sdk/wiki/UpdateShader
pub fn update_shader<V>(
    val_ptr: *const V, 
    val_size: u32
) {
    unsafe {
        Env::UpdateShader(
            val_ptr as *const c_void,
            val_size
        )
    }
}