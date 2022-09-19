use beam_bvm_interface::root::*;

pub fn funds_lock(aid: AssetID, amount: Amount) {
    unsafe { Env::FundsLock(aid, amount) }
}

pub fn funds_unlock(aid: AssetID, amount: Amount) {
    unsafe { Env::FundsUnlock(aid, amount) }
}

pub fn add_sig(pubkey: &PubKey) {
    unsafe { Env::AddSig(pubkey) }
}

pub fn asset_create(meta_ptr: *const c_void, meta_size: u32) -> AssetID {
    unsafe { Env::AssetCreate(meta_ptr, meta_size) }
}

pub fn asset_emit(id: AssetID, amount: Amount, emit: u8) -> u8 {
    unsafe { Env::AssetEmit(id, amount, emit) }
}

pub fn asset_destroy(id: AssetID) -> u8 {
    unsafe { Env::AssetDestroy(id) }
}
