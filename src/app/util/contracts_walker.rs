use beam_bvm_interface::root::{*, Env::KeyPrefix};
use super::{var_reader::*, key_sid_cid::*};

#[repr(C)]
pub struct ContractsWalker {
    pub key: KeySidCid,
    pub height: Height,
    pub reader: VarReaderEx<true>,
}

impl ContractsWalker {
    pub fn r#enum(&mut self, sid: &ShaderID) {
        let k0 = KeySidCid {
            m_Prefix: KeyPrefix {
                m_Tag: KeyTag_SidCid,
                m_Cid: Default::default()
            },
            m_KeyInContract: SidCid {
                cid: Default::default(),
                sid: *sid,
            },
            _phantom_0: Default::default()
        };
        let k1 = KeySidCid {
            m_Prefix: KeyPrefix { ..k0.m_Prefix },
            m_KeyInContract: SidCid {
                cid: [0xff; 32],
                ..k0.m_KeyInContract
            },
            _phantom_0: Default::default()
        };
        self.reader.r#enum(&k0, &k1);
    }

    pub fn move_next(&mut self) -> bool {
        if !self.reader.move_next_t(&mut self.key, &mut self.height) {
            return false;
        }
        self.height = u64::from_be(self.height);
        return true;
    }
}