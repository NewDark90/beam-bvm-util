use beam_bvm_interface::root::*;

pub fn secp_scalar_alloc() -> *mut Secp_scalar {
    unsafe { Env::Secp_Scalar_alloc() }
}

pub fn secp_scalar_free(s: *mut Secp_scalar) {
    unsafe { Env::Secp_Scalar_free(s) }
}

pub fn secp_scalar_import(s: *mut Secp_scalar, data: *const Secp_scalar_data) -> u8 {
    unsafe { Env::Secp_Scalar_import(s, data) }
}

pub fn secp_scalar_export(s: *const Secp_scalar, data: *mut Secp_scalar_data) {
    unsafe { Env::Secp_Scalar_export(s, data) }
}
pub fn secp_scalar_neg(dst: *mut Secp_scalar, src: *const Secp_scalar) {
    unsafe { Env::Secp_Scalar_neg(dst, src) }
}

pub fn secp_scalar_add(dst: *mut Secp_scalar, a: *const Secp_scalar, b: *const Secp_scalar) {
    unsafe { Env::Secp_Scalar_add(dst, a, b) }
}

pub fn secp_scalar_mul(dst: *mut Secp_scalar, a: *const Secp_scalar, b: *const Secp_scalar) {
    unsafe { Env::Secp_Scalar_mul(dst, a, b) }
}
pub fn secp_scalar_inv(dst: *mut Secp_scalar, src: *const Secp_scalar) {
    unsafe { Env::Secp_Scalar_inv(dst, src) }
}

pub fn secp_scalar_set(dst: *mut Secp_scalar, val: u64) {
    unsafe { Env::Secp_Scalar_set(dst, val) }
}

pub fn secp_point_alloc() -> *mut Secp_point {
    unsafe { Env::Secp_Point_alloc() }
}

pub fn secp_point_free(p: *mut Secp_point) {
    unsafe { Env::Secp_Point_free(p) }
}

pub fn secp_point_import(p: *mut Secp_point, pk: *const PubKey) -> u8 {
    unsafe { Env::Secp_Point_Import(p, pk) }
}

pub fn secp_point_export(p: *const Secp_point, pk: *mut PubKey) {
    unsafe { Env::Secp_Point_Export(p, pk) }
}

pub fn secp_point_neg(dst: *mut Secp_point, src: *const Secp_point) {
    unsafe { Env::Secp_Point_neg(dst, src) }
}

pub fn secp_point_add(dst: *mut Secp_point, a: *const Secp_point, b: *const Secp_point) {
    unsafe { Env::Secp_Point_add(dst, a, b) }
}

pub fn secp_point_mul(dst: *mut Secp_point, p: *const Secp_point, s: *const Secp_scalar) {
    unsafe { Env::Secp_Point_mul(dst, p, s) }
}

pub fn secp_point_is_zero(p: *const Secp_point) -> u8 {
    unsafe { Env::Secp_Point_IsZero(p) }
}

pub fn secp_point_mul_g(dst: *mut Secp_point, s: *const Secp_scalar) {
    unsafe { Env::Secp_Point_mul_G(dst, s) }
}

pub fn secp_point_mul_j(dst: *mut Secp_point, s: *const Secp_scalar) {
    unsafe { Env::Secp_Point_mul_J(dst, s) }
}

pub fn secp_point_mul_h(dst: *mut Secp_point, s: *const Secp_scalar, aid: AssetID) {
    unsafe { Env::Secp_Point_mul_H(dst, s, aid) }
}

pub fn secp_point_export_ex(p: *const Secp_point, res: *mut Secp_point_dataEx) {
    unsafe { Env::Secp_Point_ExportEx(p, res) }
}
