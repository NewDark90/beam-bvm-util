use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/SlotInit
pub fn slot_init<S>(extra_seed_ptr: &S, extra_seed_size: u32, slot: u32) {
    unsafe { Env::SlotInit(extra_seed_ptr as *const S as *const c_void, extra_seed_size, slot) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_SlotImage
pub fn get_slot_image(result_nonce: &mut Secp_point, slot: u32) {
    unsafe { Env::get_SlotImage(result_nonce, slot) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_SlotImageEx
pub fn get_slot_image_ex(
    result_nonce: &mut Secp_point,
    generator_ptr: &Secp_point,
    slot: u32,
) {
    unsafe { Env::get_SlotImageEx(result_nonce, generator_ptr, slot) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_BlindSk
pub fn get_blind_sk<TId>(
    result_scalar: &mut Secp_scalar,
    id_ptr: &TId,
    id_size: u32,
    multiplier: &Secp_scalar,
    slot: u32,
) {
    unsafe {
        Env::get_BlindSk(
            result_scalar,
            id_ptr as *const TId as *const c_void,
            id_size,
            multiplier,
            slot,
        )
    }
}
