use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/SelectContext
pub fn select_context(dependent: u8, charge_needed: u32) {
    unsafe { Env::SelectContext(dependent, charge_needed) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/GenerateKernel
pub fn generate_kernel<U>(
    cid: *const ContractID,
    method: u32,
    arg: *const U,
    arg_size: u32,
    funds: *const FundsChange,
    funds_size: u32,
    sigs: *const SigRequest,
    sigs_size: u32,
    comment: &str,
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
            comment.as_ptr(),
            charge,
        )
    }
}

/// https://github.com/BeamMW/shader-sdk/wiki/GenerateKernelAdvanced
pub fn generate_kernel_advanced<TArg>(
    cid: *const ContractID,
    method: u32,
    arg: *const TArg,
    arg_size: u32,
    funds: *const FundsChange,
    funds_size: u32,
    sigs: *const PubKey,
    sigs_size: u32,
    comment: &str,
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
            arg as *const TArg as *const c_void,
            arg_size,
            funds,
            funds_size,
            sigs,
            sigs_size,
            comment.as_ptr(),
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
