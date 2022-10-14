
use core::mem::size_of_val;

use beam_bvm_interface::root::{PubKey, Secp_point};

use crate::app::safe;

/// https://github.com/BeamMW/shader-sdk/wiki/DerivePk
pub fn derive_pk<TId>(result: &mut PubKey, id: &TId) {
    safe::derive_pk(result, id, size_of_val::<TId>(id) as u32)
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_Pk
pub fn get_pk<TId>(result: &mut Secp_point, id: &TId) {
    safe::get_pk(result, id, size_of_val::<TId>(id) as u32)
}

/// https://github.com/BeamMW/shader-sdk/wiki/get_PkEx
pub fn get_pk_ex<TId>(
    result: &mut Secp_point,
    generator_point: &Secp_point,
    id: &TId
) {
    safe::get_pk_ex(
        result,
        generator_point,
        id,
        size_of_val::<TId>(id) as u32
    )
}
