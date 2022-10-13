use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/FundsLock
pub fn funds_lock(aid: AssetID, amount: Amount) {
    unsafe { Env::FundsLock(aid, amount) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/FundsUnlock
pub fn funds_unlock(aid: AssetID, amount: Amount) {
    unsafe { Env::FundsUnlock(aid, amount) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/AddSig
pub fn add_sig(pubkey: &PubKey) {
    unsafe { Env::AddSig(pubkey) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/AssetCreate
/// https://github.com/BeamMW/beam/wiki/Asset-Descriptor-v1.0
pub fn asset_create(meta_ptr: *const c_void, meta_size: u32) -> AssetID {
    unsafe { Env::AssetCreate(meta_ptr, meta_size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/AssetEmit
pub fn asset_emit(id: AssetID, amount: Amount, emit: u8) -> u8 {
    unsafe { Env::AssetEmit(id, amount, emit) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/AssetDestroy
pub fn asset_destroy(id: AssetID) -> u8 {
    unsafe { Env::AssetDestroy(id) }
}
