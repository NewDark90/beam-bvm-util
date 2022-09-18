use beam_bvm_interface::root::*;

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