use beam_bvm_interface::root::{*, Env::KeyPrefix};

use super::var_reader::*;

type KeyFunds = Env::Key_T<AssetID>; // AssetID is in big-endian format

#[derive(Default)]
pub struct ValueFunds
{
    pub high: Amount,
    pub low: Amount
}

pub struct FundsWalker {
    asset_id: AssetID,
    funds: ValueFunds,
    reader: VarReaderEx<true>,
}

impl FundsWalker {

    pub fn asset_id(&self) -> u32 { self.asset_id }
    pub fn funds(&self) -> &ValueFunds { &self.funds }

    pub fn new() -> Self {
        Self {
            reader: VarReaderEx::<true> { handle: 0 },
            asset_id: Default::default(),
            funds: Default::default(),
        }
    }

    pub fn r#enum(&mut self, cid: &ContractID, asset_id: Option<AssetID>)
	{
		let mut k0: KeyFunds = KeyFunds {
            m_Prefix: KeyPrefix {
                m_Cid: *cid,
                m_Tag: KeyTag_LockedAmount
            },
            ..Default::default()
        };

        match asset_id {
            Some(aid) => {
                k0.m_KeyInContract = aid;
                self.reader.r#enum(&k0,&k0);
            },
            None => {
                k0.m_KeyInContract = 0;

                let k1: KeyFunds = KeyFunds {
                    m_Prefix: KeyPrefix {
                        m_Cid: *cid,
                        m_Tag: KeyTag_LockedAmount
                    },
                    m_KeyInContract: u32::MAX,
                    ..Default::default()
                };
    
                self.reader.r#enum(&k0, &k1);
            }
        }
	}

	pub fn move_next(&mut self) -> bool
	{
        let mut key: KeyFunds = Default::default();
		let mut val: ValueFunds = Default::default();

		if !self.reader.move_next_t(&mut key, &mut val) {	
            return false; 
        }

        self.asset_id = key.m_KeyInContract;
        self.funds = val;

		return true;
	}

    pub fn move_all<TFunc: Fn(AssetID, &ValueFunds) -> ()>(&mut self, on_move: TFunc) {
        while self.move_next() {
            on_move(self.asset_id, &self.funds);
        }
    }

}
