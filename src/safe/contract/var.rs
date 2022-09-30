use core::{mem::size_of, ops::{Deref}};

use beam_bvm_interface::root::*;

use crate::types::sized_result::SizedResult;

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

/// https://github.com/BeamMW/shader-sdk/wiki/LoadVar
/// Size parameters figured out internally, result instantiated and returned
pub fn load_var_simple<K, V>(
    key: *const K,
    tag: u8,
) -> SizedResult<V> where V: Default + Deref<Target = *mut V> {

    let val: V = Default::default();
    let size = load_var::<K,V>(
        key,
        size_of::<K>() as u32,
        *val,
        size_of::<V>() as u32,
        tag
    );

    return SizedResult::<V> {
        size: size,
        value: val 
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

/// https://github.com/BeamMW/shader-sdk/wiki/SaveVar
/// Size parameters figured out internally
pub fn save_var_simple<K, V>(key: *const K, val: *const V, tag: u8) -> u32 {
    save_var(
        key as *const c_void,
        size_of::<K>() as u32,
        val as *const c_void,
        size_of::<V>() as u32,
        tag,
    )
}


/// https://github.com/BeamMW/shader-sdk/wiki/SaveVar (used in a way to clear variable)
pub fn del_var<K>(key: *const K, key_size: u32) -> u32 {
    save_var(
        key as *const c_void,
        key_size,
        0 as *const c_void,
        0,
        KeyTag_Internal
    )
}

/// https://github.com/BeamMW/shader-sdk/wiki/SaveVar (used in a way to clear variable)
/// Size parameters figured out internally
pub fn del_var_simple<K>(key: *const K) -> u32 {
    del_var(key, size_of::<K>() as u32)
}
