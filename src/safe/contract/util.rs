use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/EmitLog
pub fn emit_log<K, V>(
    key: &K,
    key_size: u32,
    value: &V,
    value_size: u32,
    tag: u8,
) -> u32 {
    unsafe {
        Env::EmitLog(
            key as *const K as *const c_void,
            key_size,
            value as *const V as *const c_void,
            value_size,
            tag,
        )
    }
}

/// https://github.com/BeamMW/shader-sdk/wiki/UpdateShader
pub fn update_shader<V>(
    val: &V, 
    size: u32
) {
    unsafe {
        Env::UpdateShader(
            val as *const V as *const c_void,
            size
        )
    }
}