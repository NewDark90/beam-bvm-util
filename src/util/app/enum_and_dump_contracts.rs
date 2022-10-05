use super::{
    contracts_walker::*, document_writer::*, key_sid_cid::*,
    var_reader::*,
};
use beam_bvm_interface::root::{Env::*, *};
use core::mem::size_of_val;

pub fn enum_and_dump_contracts(sid: &ShaderID) {
    let doc = DocumentWriter {};
    doc.array_prop("contracts", |arr| {
        let mut wlk = ContractsWalker {
            key: KeySidCid {
                m_Prefix: KeyPrefix {
                    m_Tag: KeyTag_Internal,
                    m_Cid: Default::default(),
                },
                m_KeyInContract: SidCid {
                    cid: [0; 32],
                    sid: [1; 32], // to avoid memset lib call
                },
                _phantom_0: Default::default(),
            },
            height: 0,
            reader: VarReaderEx::<true> { handle: 0 },
        };
        wlk.r#enum(&sid);
        while wlk.move_next() {
            arr.object(|obj| {
                obj.blob_prop(
                    "cid",
                    &wlk.key.m_KeyInContract.cid,
                    size_of_val(&wlk.key.m_KeyInContract.cid) as u32
                ).u64_prop("height", wlk.height);
            });
        }
    });
}
