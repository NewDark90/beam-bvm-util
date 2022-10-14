use beam_bvm_interface::root::{Env::Key_T, *};

#[repr(C, packed(1))]
pub struct SidCid {
    pub sid: ShaderID,
    pub cid: ContractID,
}

pub type KeySidCid = Key_T<SidCid>;