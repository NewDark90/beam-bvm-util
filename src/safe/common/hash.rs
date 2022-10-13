use beam_bvm_interface::root::*;

/// https://github.com/BeamMW/shader-sdk/wiki/HashWrite
pub fn hash_write<T>(hash: &mut HashObj, p: &T, size: u32) { 
    unsafe { Env::HashWrite (hash, p as *const T as *const c_void, size)  }
}

/// https://github.com/BeamMW/shader-sdk/wiki/HashGetValue
pub fn hash_get_value<T>(hash: &mut HashObj, dst_ptr: &mut T, size: u32) { 
    unsafe { Env::HashGetValue (hash, dst_ptr as *mut T as *mut c_void, size)  }
}

/// https://github.com/BeamMW/shader-sdk/wiki/HashFree
pub fn hash_free(hash: &mut HashObj) { unsafe { Env::HashFree (hash) } }

/// https://github.com/BeamMW/shader-sdk/wiki/HashClone
pub fn hash_clone(hash_ptr: &mut HashObj) -> *mut HashObj { 
    unsafe { Env::HashClone(hash_ptr) }    
}

/// https://github.com/BeamMW/shader-sdk/wiki/HashCreateSha256
pub fn hash_create_sha256() -> *mut HashObj { unsafe { Env::HashCreateSha256() }  }

/// https://github.com/BeamMW/shader-sdk/wiki/HashCreateBlake2b
pub fn hash_create_blake2b<T>(
    personal: &T,
    personal_size: u32,
    result_size: u32
) -> *mut HashObj { 
    unsafe { 
        Env::HashCreateBlake2b (personal as *const T as *const c_void, personal_size, result_size) 
    } 
}

/// https://github.com/BeamMW/shader-sdk/wiki/HashCreateKeccak
pub fn hash_create_keccak(bits_size: u32) -> *mut HashObj {     
    unsafe { Env::HashCreateKeccak(bits_size) } 
}

/// https://github.com/BeamMW/shader-sdk/wiki/VerifyBeamHashIII
pub fn verify_beam_hash_3<I, N, S>(
    input: &I,
    input_size: u32,
    nonce: &N,
    nonce_size: u32,
    solution: &S,
    solution_size: u32
) -> bool { 
    unsafe { 
        let result: u8 = Env::VerifyBeamHashIII(
            input as *const I as *const c_void, 
            input_size,
            nonce as *const N as *const c_void, 
            nonce_size, 
            solution as *const S as *const c_void, 
            solution_size
        );
        
        result == 1
    } 
}

