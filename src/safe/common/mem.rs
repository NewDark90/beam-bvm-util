use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/Memis0
pub fn mem_is_0<T>(ptr: *const T, size: u32) -> bool {
    unsafe { 
        let result: u8 = Env::Memis0(ptr as *const c_void, size);
        result == 1
    }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Memset
pub fn memset<TDest>(dst: *mut TDest, val: u8, size: u32) -> *mut c_void {
    unsafe { Env::Memset(dst as *mut c_void, val, size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Memcpy
pub fn memcpy<TDest, TSource>(dst: *mut TDest, src: *mut TSource, size: u32) -> *mut TDest {
    unsafe { Env::Memcpy(dst as *mut c_void, src as *mut c_void, size) as *mut TDest  }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Memcmp
pub fn memcmp<T1, T2>(p1: *const T1, p2: *const T2, size: u32) -> i32 {
    unsafe { Env::Memcmp(p1 as *const c_void, p2 as *const c_void, size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/StackAlloc
pub fn stack_alloc(size: u32) -> *mut c_void {
    unsafe { Env::StackAlloc(size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/StackAlloc
pub fn stack_free(size: u32) {
    unsafe { Env::StackFree(size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Heap_Alloc
pub fn heap_alloc(size: u32) -> *mut c_void {
    unsafe { Env::Heap_Alloc(size) }
}

/// https://github.com/BeamMW/shader-sdk/wiki/Heap_Free
pub fn heap_free(p: *mut c_void) {
    unsafe { Env::Heap_Free(p) }
}