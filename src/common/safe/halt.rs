use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/Halt
pub fn halt() -> () {
    unsafe { Env::Halt() }
}

pub fn halt_if(should_halt: bool) -> () {
    if should_halt { halt() }
}
