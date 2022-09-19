use beam_bvm_interface::root::*;

pub fn get_height() -> Height {
    unsafe { Env::get_Height() }
}

pub fn get_hdr_info(hdr: *mut BlockHeader_Info) {
    unsafe { Env::get_HdrInfo(hdr) }
}

pub fn get_hdr_full(hdr: *mut BlockHeader_Full) {
    unsafe { Env::get_HdrFull(hdr) }
}

pub fn get_rules_cfg(h: Height, res: *mut HashValue) -> Height {
    unsafe { Env::get_RulesCfg(h, res) }
}

pub fn get_fork_height(fork: u32) -> Height {
    unsafe { Env::get_ForkHeight(fork) }
}
