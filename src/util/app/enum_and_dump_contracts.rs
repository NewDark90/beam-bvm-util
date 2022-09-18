use core::mem::size_of_val;
use beam_bvm_interface::root::{*, Env::*};
use crate::safe::app::doc::*;
use super::{contracts_walker::*, key_sid_cid::*, var_reader::*};


pub fn enum_and_dump_contracts(sid: &ShaderID) {
    doc_add_array("contracts\0");

    let mut wlk = ContractsWalker {
        key: KeySidCid {
            m_Prefix: KeyPrefix { 
                m_Tag: KeyTag_Internal, 
                m_Cid: Default::default() 
            },
            m_KeyInContract: SidCid {
                cid: [0; 32],
                sid: [1; 32], // to avoid memset lib call
            },
            _phantom_0: Default::default()
        },
        height: 0,
        reader: VarReaderEx::<true> { handle: 0 },
    };
    wlk.r#enum(&sid);
    while wlk.move_next() {
        doc_add_group("\0");
        doc_add_blob(
            "cid\0",
            &wlk.key.m_KeyInContract.cid,
            size_of_val(&wlk.key.m_KeyInContract.cid) as u32,
        );
        doc_add_num64("height\0", wlk.height);
        doc_close_group();
    }
    doc_close_array();
}
