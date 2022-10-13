use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/DerivePk
pub fn derive_pk<TId>(result: &mut PubKey, id: &TId, id_size: u32) {
    unsafe { Env::DerivePk(result, id as *const TId as *const c_void, id_size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_Pk
pub fn get_pk<TId>(result: &mut Secp_point, id: &TId, id_size: u32) {
    unsafe { Env::get_Pk(result, id as *const TId as *const c_void, id_size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_PkEx
pub fn get_pk_ex<TId>(
    result: &mut Secp_point,
    generator_point: &Secp_point,
    id_ptr: &TId,
    id_size: u32,
) {
    unsafe {
        Env::get_PkEx(
            result,
            generator_point,
            id_ptr as *const TId as *const c_void,
            id_size,
        )
    }
}
