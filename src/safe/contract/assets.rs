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