use beam_bvm_interface::root::*;

pub fn derive_pk<T>(pubkey: &mut PubKey, id: *const T, id_size: u32) {
    unsafe { Env::DerivePk(pubkey, id as *const c_void, id_size) }
}

pub fn generate_kernel<U, V>(
    cid: *const ContractID,
    method: u32,
    arg: *const U,
    arg_size: u32,
    funds: *const FundsChange,
    funds_size: u32,
    sigs: *const SigRequest,
    sigs_size: u32,
    comment: *const V,
    charge: u32,
) {
    unsafe {
        Env::GenerateKernel(
            cid,
            method,
            arg as *const c_void,
            arg_size,
            funds,
            funds_size,
            sigs,
            sigs_size,
            comment as *const c_char,
            charge,
        )
    }
}


pub fn get_pk(ptr: *mut Secp_point, id_ptr: *const usize, id_size: u32) {
    unsafe { Env::get_Pk(ptr, id_ptr as *const c_void, id_size) }
}
