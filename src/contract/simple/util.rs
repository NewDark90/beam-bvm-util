use core::mem::size_of_val;

use crate::contract::safe;

/// https://github.com/BeamMW/shader-sdk/wiki/EmitLog
pub fn emit_log<K, V>(
    key: &K,
    value: &V,
    tag: u8,
) -> u32 {
    safe::emit_log(
        key,
        size_of_val::<K>(key) as u32,
        value,
        size_of_val::<V>(value) as u32,
        tag,
    )
}