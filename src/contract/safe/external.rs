use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/CallFar
pub fn call_far<T>(
    contract_id: &ContractID,
    method: u32,
    args: &mut T,
    args_size: u32,
    inherit_context: u8,
) {
    unsafe {
        Env::CallFar(
            contract_id,
            method,
            args  as *mut T as *mut c_void,
            args_size,
            inherit_context,
        )
    }
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_CallDepth
pub fn get_call_depth() -> u32 {
    unsafe { Env::get_CallDepth() }
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_CallerCid
pub fn get_caller_cid(caller: u32, contract_id: &mut ContractID) {
    unsafe { Env::get_CallerCid(caller, contract_id) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/RefAdd
pub fn ref_add(contract_id: &ContractID) -> u8 {
    unsafe { Env::RefAdd(contract_id) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/RefRelease
pub fn ref_release(contract_id: &ContractID) -> u8 {
    unsafe { Env::RefRelease(contract_id) }
}
