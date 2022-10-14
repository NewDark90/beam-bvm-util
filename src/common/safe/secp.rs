use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Scalar_alloc
pub fn secp_scalar_alloc() -> *mut Secp_scalar {
    unsafe { Env::Secp_Scalar_alloc() }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Scalar_free
pub fn secp_scalar_free(s: &mut Secp_scalar) {
    unsafe { Env::Secp_Scalar_free(s) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Scalar_import
pub fn secp_scalar_import(s: &mut Secp_scalar, data: &Secp_scalar_data) -> u8 {
    unsafe { Env::Secp_Scalar_import(s, data) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Scalar_export
pub fn secp_scalar_export(s: &Secp_scalar, data: &mut Secp_scalar_data) {
    unsafe { Env::Secp_Scalar_export(s, data) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Scalar_neg
pub fn secp_scalar_neg(dst: &mut Secp_scalar, src: &Secp_scalar) {
    unsafe { Env::Secp_Scalar_neg(dst, src) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Scalar_add
pub fn secp_scalar_add(dst: &mut Secp_scalar, a: &Secp_scalar, b: &Secp_scalar) {
    unsafe { Env::Secp_Scalar_add(dst, a, b) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Scalar_mul
pub fn secp_scalar_mul(dst: &mut Secp_scalar, a: &Secp_scalar, b: &Secp_scalar) {
    unsafe { Env::Secp_Scalar_mul(dst, a, b) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Scalar_inv
pub fn secp_scalar_inv(dst: &mut Secp_scalar, src: &Secp_scalar) {
    unsafe { Env::Secp_Scalar_inv(dst, src) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Scalar_set
pub fn secp_scalar_set(dst: &mut Secp_scalar, val: u64) {
    unsafe { Env::Secp_Scalar_set(dst, val) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Point_alloc
pub fn secp_point_alloc() -> *mut Secp_point {
    unsafe { Env::Secp_Point_alloc() }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Point_free
pub fn secp_point_free(p: &mut Secp_point) {
    unsafe { Env::Secp_Point_free(p) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Point_Import
pub fn secp_point_import(p: &mut Secp_point, pk: &PubKey) -> u8 {
    unsafe { Env::Secp_Point_Import(p, pk) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Point_Exportconst
pub fn secp_point_export(p: &Secp_point, pk: &mut PubKey) {
    unsafe { Env::Secp_Point_Export(p, pk) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Point_neg
pub fn secp_point_neg(dst: &mut Secp_point, src: &Secp_point) {
    unsafe { Env::Secp_Point_neg(dst, src) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Point_add
pub fn secp_point_add(dst: &mut Secp_point, a: &Secp_point, b: &Secp_point) {
    unsafe { Env::Secp_Point_add(dst, a, b) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Point_mul
pub fn secp_point_mul(dst: &mut Secp_point, p: &Secp_point, s: &Secp_scalar) {
    unsafe { Env::Secp_Point_mul(dst, p, s) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Point_IsZero
pub fn secp_point_is_zero(p: &Secp_point) -> u8 {
    unsafe { Env::Secp_Point_IsZero(p) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Point_mul_G
pub fn secp_point_mul_g(dst: &mut Secp_point, s: &Secp_scalar) {
    unsafe { Env::Secp_Point_mul_G(dst, s) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Point_mul_J
pub fn secp_point_mul_j(dst: &mut Secp_point, s: &Secp_scalar) {
    unsafe { Env::Secp_Point_mul_J(dst, s) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Point_mul_H
pub fn secp_point_mul_h(dst: &mut Secp_point, s: &Secp_scalar, aid: AssetID) {
    unsafe { Env::Secp_Point_mul_H(dst, s, aid) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Secp_Point_ExportEx
pub fn secp_point_export_ex(p: &Secp_point, res: &mut Secp_point_dataEx) {
    unsafe { Env::Secp_Point_ExportEx(p, res) }
}
