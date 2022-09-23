use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/HashWrite
pub fn hash_write(hash_ptr: *mut HashObj, p: *const c_void, size: u32) { 
    unsafe { Env::HashWrite (hash_ptr, p, size)  }
}

/// https://github.com/BeamMW/shader-sdk/wiki/HashGetValue
pub fn hash_get_value(hash_ptr: *mut HashObj, dst_ptr: *mut c_void, size: u32) { 
    unsafe { Env::HashGetValue (hash_ptr, dst_ptr, size)  }
}

/// https://github.com/BeamMW/shader-sdk/wiki/HashFree
pub fn hash_free(hash_ptr: *mut HashObj) { unsafe { Env::HashFree (hash_ptr) } }

/// https://github.com/BeamMW/shader-sdk/wiki/HashClone
pub fn hash_clone(hash_ptr: *mut HashObj) -> *mut HashObj { 
    unsafe { Env::HashClone(hash_ptr) }    
}

/// https://github.com/BeamMW/shader-sdk/wiki/HashCreateSha256
pub fn hash_create_sha256() -> *mut HashObj { unsafe { Env::HashCreateSha256() }  }

/// https://github.com/BeamMW/shader-sdk/wiki/HashCreateBlake2b
pub fn hash_create_blake2b(
    personal_ptr: *const c_void,
    personal_size: u32,
    result_size: u32
) -> *mut HashObj { 
    unsafe { 
        Env::HashCreateBlake2b (personal_ptr, personal_size, result_size) 
    } 
}

/// https://github.com/BeamMW/shader-sdk/wiki/HashCreateKeccak
pub fn hash_create_keccak(bits_size: u32) -> *mut HashObj {     
    unsafe { Env::HashCreateKeccak(bits_size) } 
}

/// https://github.com/BeamMW/shader-sdk/wiki/VerifyBeamHashIII
pub fn verify_beam_hash_3<I, N, S>(
    input_ptr: *const I,
    input_size: u32,
    nonce_ptr: *const N,
    nonce_size: u32,
    solution_ptr: *const S,
    solution_size: u32
) -> u8 { 
    unsafe { 
        Env::VerifyBeamHashIII(
            input_ptr as *const c_void, 
            input_size,
            nonce_ptr as *const c_void, 
            nonce_size, 
            solution_ptr as *const c_void, 
            solution_size
        )  
    } 
}

