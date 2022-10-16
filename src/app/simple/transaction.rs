use core::{mem::size_of_val };

use beam_bvm_interface::root::*;

use crate::{app::safe, common::traits::to_sized_ptr::ToSizedPtr};

/// https://github.com/BeamMW/shader-sdk/wiki/GenerateKernel
/// Size parameters figured out internally
pub fn generate_kernel<TArg>(
    cid: &ContractID,
    method: u32,
    arg: &TArg,
    funds: Option<&FundsChange>,
    sigs: Option<&SigRequest>,
    comment: Option<&str>,
    charge: u32,
) {
    
    let arg_size = size_of_val::<TArg>(arg) as u32;
    let funds_sized = funds.to_sized_or_null_ptr();
    let sigs_sized= sigs.to_sized_or_null_ptr();
    
    safe::generate_kernel::<TArg>(
        cid,
        method,
        arg,
        arg_size,
        funds_sized.ptr(),
        funds_sized.size(),
        sigs_sized.ptr(),
        sigs_sized.size(),
        comment.unwrap_or("\0"),
        charge,
    );
}



/// https://github.com/BeamMW/shader-sdk/wiki/GenerateKernelAdvanced
/// Size parameters figured out internally
pub fn generate_kernel_advanced<TArg>(
    cid: &ContractID,
    method: u32,
    arg: &TArg,
    funds: Option<&FundsChange>,
    sigs: Option<&PubKey>,
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
    let arg_size = size_of_val::<TArg>(arg) as u32;
    let funds_sized = funds.to_sized_or_null_ptr();
    let sigs_sized = sigs.to_sized_or_null_ptr();

    safe::generate_kernel_advanced(
        cid,
        method,
        arg,
        arg_size,
        funds_sized.ptr(),
        funds_sized.size(),
        sigs_sized.ptr(),
        sigs_sized.size(),
        comment,
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