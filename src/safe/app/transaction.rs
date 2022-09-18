use beam_bvm_interface::root::*;

pub fn select_context(dependent: u8, charge_needed: u32) {
    unsafe { Env::SelectContext(dependent, charge_needed) }
}

pub fn generate_random(result_ptr: *mut c_void, size: u32) {
    unsafe { Env::GenerateRandom(result_ptr, size) }
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

pub fn generate_kernel_advanced<U, V>(
    cid: *const ContractID,
    method: u32,
    arg: *const U,
    arg_size: u32,
    funds: *const FundsChange,
    funds_size: u32,
    sigs: *const PubKey,
    sigs_size: u32,
    comment: *const V,
    charge: u32,
    min: Height,
    max: Height,
    full_blind: *const PubKey,
    full_nonce: *const PubKey,
    foreign_sig: *const Secp_scalar_data,
    slot_blind: u32,
    slot_nonce: u32,
    challenges: *mut Secp_scalar_data,
) {
    unsafe {
        Env::GenerateKernelAdvanced(
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
            min,
            max,
            full_blind,
            full_nonce,
            foreign_sig,
            slot_blind,
            slot_nonce,
            challenges,
        )
    }
}
