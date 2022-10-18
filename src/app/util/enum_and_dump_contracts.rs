use crate::common::extensions::*;

use super::{contracts_walker::*, document_writer::*, key_sid_cid::*, var_reader::*};
use alloc::ffi::CString;
use beam_bvm_interface::root::{Env::*, *};
use core::mem::size_of_val;

pub fn enum_and_dump_contracts(sid: &ShaderID) {
    enum_and_dump_contracts_prop(&"contracts".to_c_string(), sid)
}

pub fn enum_and_dump_contracts_prop(array_prop_name: &CString, sid: &ShaderID) {
    let doc = DocumentWriter {};
    doc.array_prop(array_prop_name, |arr| {
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
                    &"cid".to_c_string(),
                    &wlk.key.m_KeyInContract.cid,
                    size_of_val(&wlk.key.m_KeyInContract.cid) as u32,
                )
                .u64_prop(&"height".to_c_string(), wlk.height);
            });
        }
    });
}

pub trait ContractDumpArrayPropWriter {
    fn contract_dump_prop(self: &Self, prop_name: &CString, sid: &ShaderID) -> &Self;
}

impl ContractDumpArrayPropWriter for ObjectFuncs {
    fn contract_dump_prop(self: &Self, prop_name: &CString, sid: &ShaderID) -> &Self {
        enum_and_dump_contracts_prop(prop_name, sid);
        self
    }
}
