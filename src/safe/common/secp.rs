use beam_bvm_interface::root::*;

pub fn secp_point_import(ptr: *mut Secp_point, pk: *const PubKey) -> u8 {
    unsafe { Env::Secp_Point_Import(ptr, pk) }
}

pub fn secp_point_export(ptr: *const Secp_point, pk: *mut PubKey) {
    unsafe { Env::Secp_Point_Export(ptr, pk) }
}

pub fn secp_point_add(dst: *mut Secp_point, a: *const Secp_point, b: *const Secp_point) {
    unsafe { Env::Secp_Point_add(dst, a, b) }
}