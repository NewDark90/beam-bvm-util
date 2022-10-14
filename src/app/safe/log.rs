use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/Logs_Close
pub fn logs_close(slot: u32) {
    unsafe { Env::Logs_Close(slot) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Logs_Enum
pub fn logs_enum<U, V>(
    key0: &U,
    key0_size: u32,
    key1: &V,
    key1_size: u32,
    pos_min: &HeightPos,
    pos_max: &HeightPos,
) -> u32 {
    unsafe {
        Env::Logs_Enum(
            key0 as *const U as *const c_void,
            key0_size,
            key1 as *const V as *const c_void,
            key1_size,
            pos_min,
            pos_max,
        )
    }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Logs_MoveNext
pub fn logs_move_next<K, V>(
    slot: u32,
    key: &mut K,
    key_size: &mut u32,
    val: &mut V,
    val_size: &mut u32,
    pos: &mut HeightPos,
    repeat: u8,
) -> u8 {
    unsafe {
        Env::Logs_MoveNext(
            slot,
            key as *mut K as *mut c_void,
            key_size,
            val as *mut V as *mut c_void,
            val_size,
            pos,
            repeat,
        )
    }
}

/// https://github.com/BeamMW/shader-sdk/wiki/LogGetProof
pub fn logs_get_proof(
    pos: &HeightPos,
    proof: *mut *const Merkle::Node
) -> u32 {
    unsafe { 
        Env::LogGetProof(pos, proof) 
    }
}