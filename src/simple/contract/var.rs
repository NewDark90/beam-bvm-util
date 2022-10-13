

use core::mem::size_of_val;

use crate::safe::contract::var::{
    load_var as load_var_safe,
    save_var as save_var_safe,
    del_var as del_var_safe
};

/// https://github.com/BeamMW/shader-sdk/wiki/LoadVar
/// Size parameters figured out internally
pub fn load_var<K, V>(
    key: &K,
    result: &mut V,
    tag: u8,
) -> u32 {
    load_var_safe::<K,V>(
        key,
        size_of_val::<K>(key) as u32,
        result,
        size_of_val::<V>(result) as u32,
        tag
    )
}



/// https://github.com/BeamMW/shader-sdk/wiki/SaveVar
/// Size parameters figured out internally
pub fn save_var<K, V>(key: &K, val: &V, tag: u8) -> u32 {
    save_var_safe(
        key,
        size_of_val::<K>(key) as u32,
        val,
        size_of_val::<V>(val) as u32,
        tag,
    )
}

/// https://github.com/BeamMW/shader-sdk/wiki/SaveVar (used in a way to clear variable)
/// Size parameters figured out internally
pub fn del_var<K>(key: &K) -> u32 {
    del_var_safe(key, size_of_val::<K>(key) as u32)
}
