use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/DerivePk
pub fn derive_pk<T>(result_pubkey: &mut PubKey, id: *const T, id_size: u32) {
    unsafe { Env::DerivePk(result_pubkey, id as *const c_void, id_size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_Pk
pub fn get_pk(result_ptr: *mut Secp_point, id_ptr: *const usize, id_size: u32) {
    unsafe { Env::get_Pk(result_ptr, id_ptr as *const c_void, id_size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_PkEx
pub fn get_pk_ex(
    result_ptr: *mut Secp_point,
    generator_point: *const Secp_point,
    id_ptr: *const usize,
    id_size: u32,
) {
    unsafe {
        Env::get_PkEx(
            result_ptr,
            generator_point,
            id_ptr as *const c_void,
            id_size,
        )
    }
}
