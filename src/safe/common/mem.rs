use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/Memis0
pub fn mem_is_0<T>(ptr: *const T, size: u32) -> u8 {
    unsafe { Env::Memis0(ptr as *const c_void, size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Memset
pub fn memset<V>(dst: *mut V, val: u8, size: u32) -> *mut c_void {
    unsafe { Env::Memset(dst as *mut c_void, val, size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Memcpy
pub fn memcpy<S, D>(dst: *mut D, src: *mut S, size: u32) -> *mut c_void {
    unsafe { Env::Memcpy(dst as *mut c_void, src as *mut c_void, size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Memcmp
pub fn memcmp<S, D>(p1: *const S, p2: *const D, size: u32) -> i32 {
    unsafe { Env::Memcmp(p1 as *const c_void, p2 as *const c_void, size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/StackAlloc
pub fn stack_alloc(size: u32) -> *mut c_void {
    unsafe { Env::StackAlloc(size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/StackAlloc
pub fn stack_free<V>(size: u32) {
    unsafe { Env::StackFree(size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Heap_Alloc
pub fn heap_alloc(size: u32) -> *mut c_void {
    unsafe { Env::Heap_Alloc(size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Heap_Free
pub fn heap_free<V>(p: *mut V) {
    unsafe { Env::Heap_Free(p as *mut c_void) }
}
