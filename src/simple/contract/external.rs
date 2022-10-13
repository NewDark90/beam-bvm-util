use core::mem::size_of_val;

use beam_bvm_interface::root::ContractID;

use crate::safe::contract::external::call_far as call_far_safe;

/// https://github.com/BeamMW/shader-sdk/wiki/CallFar
/// Size parameters figured out internally
pub fn call_far<T>(
    contract_id: &ContractID,
    method: u32,
    args: &mut T,
    inherit_context: u8,
) {
    call_far_safe(
        contract_id,
        method,
        args,
        size_of_val::<T>(args) as u32,
        inherit_context
    )
}