use beam_bvm_interface::root::*;

pub fn slot_init(extra_seed_ptr: *const c_void, extra_seed_size: u32, slot: u32) {
    unsafe { Env::SlotInit(extra_seed_ptr, extra_seed_size, slot) }
}

pub fn get_slot_image(result_nonce: *mut Secp_point, slot: u32) {
    unsafe { Env::get_SlotImage(result_nonce, slot) }
}

pub fn get_slot_image_ex(
    result_nonce: *mut Secp_point,
    generator_ptr: *const Secp_point,
    slot: u32,
) {
    unsafe { Env::get_SlotImageEx(result_nonce, generator_ptr, slot) }
}

pub fn get_blind_sk(
    result_scalar: *mut Secp_scalar,
    id_ptr: *const usize,
    id_size: u32,
    multiplier: *const Secp_scalar,
    slot: u32,
) {
    unsafe {
        Env::get_BlindSk(
            result_scalar,
            id_ptr as *const c_void,
            id_size,
            multiplier,
            slot,
        )
    }
}
