use beam_bvm_interface::root::*;

pub fn call_far<T>(
    contract_id: *const ContractID,
    method: u32,
    args_ptr: *mut T,
    args_size: u32,
    inherit_context: u8,
) {
    unsafe {
        Env::CallFar(
            contract_id,
            method,
            args_ptr as *mut c_void,
            args_size,
            inherit_context,
        )
    }
}

pub fn get_call_depth() -> u32 {
    unsafe { Env::get_CallDepth() }
}

pub fn get_caller_cid(caller: u32, contract_id: *mut ContractID) {
    unsafe { Env::get_CallerCid(caller, contract_id) }
}

pub fn ref_add(contract_id: *const ContractID) -> u8 {
    unsafe { Env::RefAdd(contract_id) }
}

pub fn ref_release(contract_id: *const ContractID) -> u8 {
    unsafe { Env::RefRelease(contract_id) }
}
