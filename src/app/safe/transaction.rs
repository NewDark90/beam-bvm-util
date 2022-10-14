use core::mem::size_of_val;

use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/SelectContext
pub fn select_context(dependent: u8, charge_needed: u32) {
    unsafe { Env::SelectContext(dependent, charge_needed) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/GenerateKernel
pub fn generate_kernel<U>(
    cid: &ContractID,
    method: u32,
    arg: &U,
    arg_size: u32,
    funds: &FundsChange,
    funds_size: u32,
    sigs: &SigRequest,
    sigs_size: u32,
    comment: &str,
    charge: u32,
) {
    unsafe {
        Env::GenerateKernel(
            cid,
            method,
            arg as *const U as *const c_void,
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

/// https://github.com/BeamMW/shader-sdk/wiki/GenerateKernel
/// Size parameters figured out internally
pub fn generate_kernel_simple<TArg>(
    cid: &ContractID,
    method: u32,
    arg: &TArg,
    funds: &FundsChange,
    sigs: &SigRequest,
    comment: &str,
    charge: u32,
) where TArg: Sized {
    let arg_size = size_of_val::<TArg>(arg) as u32;
    let funds_size = size_of_val::<FundsChange>(funds) as u32;
    let sigs_size = size_of_val::<SigRequest>(sigs) as u32;
    generate_kernel::<TArg>(
        cid,
        method,
        arg,
        arg_size,
        funds,
        funds_size,
        sigs,
        sigs_size,
        comment,
        charge,
    );
}

/// https://github.com/BeamMW/shader-sdk/wiki/GenerateKernelAdvanced
pub fn generate_kernel_advanced<TArg>(
    cid: &ContractID,
    method: u32,
    arg: &TArg,
    arg_size: u32,
    funds: &FundsChange,
    funds_size: u32,
    sigs: &PubKey,
    sigs_size: u32,
    comment: &str,
    charge: u32,
    min: Height,
    max: Height,
    full_blind: &PubKey,
    full_nonce: &PubKey,
    foreign_sig: &Secp_scalar_data,
    slot_blind: u32,
    slot_nonce: u32,
    challenges: &mut Secp_scalar_data,
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

/// https://github.com/BeamMW/shader-sdk/wiki/GenerateKernelAdvanced
/// Size parameters figured out internally
pub fn generate_kernel_advanced_simple<TArg>(
    cid: &ContractID,
    method: u32,
    arg: &TArg,
    arg_size: u32,
    funds: &FundsChange,
    funds_size: u32,
    sigs: &PubKey,
    sigs_size: u32,
    comment: &str,
    charge: u32,
    min: Height,
    max: Height,
    full_blind: &PubKey,
    full_nonce: &PubKey,
    foreign_sig: &Secp_scalar_data,
    slot_blind: u32,
    slot_nonce: u32,
    challenges: &mut Secp_scalar_data,
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
