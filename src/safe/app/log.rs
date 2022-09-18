use beam_bvm_interface::root::*;

pub fn logs_close(slot: u32) {
    unsafe { Env::Logs_Close(slot) }
}

pub fn logs_enum<U, V>(
    key0: *const U,
    key0_size: u32,
    key1: *const V,
    key1_size: u32,
    pos_min: *const HeightPos,
    pos_max: *const HeightPos,
) -> u32 {
    unsafe {
        Env::Logs_Enum(
            key0 as *const c_void,
            key0_size,
            key1 as *const c_void,
            key1_size,
            pos_min,
            pos_max,
        )
    }
}

pub fn logs_move_next<K, V>(
    slot: u32,
    key: *mut K,
    key_size: *mut u32,
    val: *mut V,
    val_size: *mut u32,
    pos: *mut HeightPos,
    repeat: u8,
) -> u8 {
    unsafe {
        Env::Logs_MoveNext(
            slot,
            key as *mut c_void,
            key_size,
            val as *mut c_void,
            val_size,
            pos,
            repeat,
        )
    }
}