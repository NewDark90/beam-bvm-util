use beam_bvm_interface::root::*;

pub fn vars_close(slot: u32) {
    unsafe { Env::Vars_Close(slot) }
}

pub fn vars_enum<U, V>(key0: *const U, key0_size: u32, key1: *const V, key1_size: u32) -> u32 {
    unsafe {
        Env::Vars_Enum(
            key0 as *const c_void,
            key0_size,
            key1 as *const c_void,
            key1_size,
        )
    }
}

pub fn vars_move_next<K, V>(
    slot: u32,
    key: *mut K,
    key_size: *mut u32,
    val: *mut V,
    val_size: *mut u32,
    repeat: u8,
) -> u8 {
    unsafe {
        Env::Vars_MoveNext(
            slot,
            key as *mut c_void,
            key_size,
            val as *mut c_void,
            val_size,
            repeat,
        )
    }
}

pub fn var_get_proof<K, V>(
    key: *const K,
    key_size: u32,
    val: *mut *const V,
    val_size: *mut u32,
    proof: *mut *const Merkle::Node,
) -> u32 {
    unsafe {
        Env::VarGetProof(
            key as *const c_void,
            key_size,
            val as *mut *const c_void,
            val_size,
            proof,
        )
    }
}