//! allocator_api isn't released yet. This code will be unused until then.
//! https://github.com/rust-lang/rust/issues/32838 
//! 
//! DO NOT MODIFY UNTIL THE ABOVE FEATURE IS FINALIZED 
//! 
//! The BvmGlobalAlloc might be what you are looking for.
use core::{
    alloc::{
        Layout, Allocator, AllocError
    }, 
    ptr::{NonNull, slice_from_raw_parts_mut}
};

use beam_bvm_interface::root::c_void;

use crate::common::safe::mem::*;

pub struct BvmAllocator { }

unsafe impl Allocator for BvmAllocator {

    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let heap_ptr = heap_alloc(layout.size() as u32) as *mut u8;
        if heap_ptr.is_null()
        {
            return Result::Err(AllocError)
        }
        else {
            unsafe {
                let slice_ptr = slice_from_raw_parts_mut(heap_ptr, layout.size());
                return Result::Ok(NonNull::<[u8]>::new_unchecked(slice_ptr))
            }
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        heap_free(ptr.as_ptr() as *mut c_void)
    }
}