use beam_bvm_interface::root::*;

pub fn halt_if(condition: bool) {
    unsafe {
        if condition {
            Env::Halt()
        }
    }
}

pub fn halt() {
    unsafe { Env::Halt() }
}