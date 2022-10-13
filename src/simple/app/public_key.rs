
use core::mem::size_of_val;

use beam_bvm_interface::root::{PubKey, Secp_point};

use crate::safe::app::public_key::{
    derive_pk as derive_pk_safe,
    get_pk as get_pk_safe,
    get_pk_ex as get_pk_ex_safe
};

/// https://github.com/BeamMW/shader-sdk/wiki/DerivePk
pub fn derive_pk<TId>(result: &mut PubKey, id: &TId) {
    derive_pk_safe(result, id, size_of_val::<TId>(id) as u32)
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_Pk
pub fn get_pk<TId>(result: &mut Secp_point, id: &TId) {
    get_pk_safe(result, id, size_of_val::<TId>(id) as u32)
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_PkEx
pub fn get_pk_ex<TId>(
    result: &mut Secp_point,
    generator_point: &Secp_point,
    id: &TId
) {
    get_pk_ex_safe(
        result,
        generator_point,
        id,
        size_of_val::<TId>(id) as u32
    )
}
