use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/get_Height
pub fn get_height() -> Height {
    unsafe { Env::get_Height() }
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_HdrInfo
pub fn get_hdr_info(hdr: *mut BlockHeader_Info) {
    unsafe { Env::get_HdrInfo(hdr) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_HdrFull
pub fn get_hdr_full(hdr: *mut BlockHeader_Full) {
    unsafe { Env::get_HdrFull(hdr) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_RulesCfg
pub fn get_rules_cfg(h: Height, res: *mut HashValue) -> Height {
    unsafe { Env::get_RulesCfg(h, res) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_ForkHeight
pub fn get_fork_height(fork: u32) -> Height {
    unsafe { Env::get_ForkHeight(fork) }
}
