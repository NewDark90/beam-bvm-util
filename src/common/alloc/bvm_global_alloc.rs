use core::alloc::{
    GlobalAlloc,
    Layout
};

use beam_bvm_interface::root::c_void;

use crate::common::safe::mem::*;

pub struct BvmGlobalAlloc { }

unsafe impl GlobalAlloc for BvmGlobalAlloc {

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        heap_alloc(layout.size() as u32) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        heap_free(ptr as *mut c_void)
    }

    /* 
    unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = layout.size();
        // SAFETY: the safety contract for `alloc` must be upheld by the caller.
        let ptr = unsafe { self.alloc(layout) };
        if !core::ptr.is_null() {
            // SAFETY: as allocation succeeded, the region from `ptr`
            // of size `size` is guaranteed to be valid for writes.
            unsafe { core::ptr::write_bytes(core::ptr, 0, size) };
        }
        core::ptr
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: core::alloc::Layout, new_size: usize) -> *mut u8 {
        // SAFETY: the caller must ensure that the `new_size` does not overflow.
        // `layout.align()` comes from a `Layout` and is thus guaranteed to be valid.
        let new_layout = unsafe { core::alloc::Layout::from_size_align_unchecked(new_size, layout.align()) };
        // SAFETY: the caller must ensure that `new_layout` is greater than zero.
        let new_ptr = unsafe { self.alloc(new_layout) };
        if !new_ptr.is_null() {
            // SAFETY: the previously allocated block cannot overlap the newly allocated block.
            // The safety contract for `dealloc` must be upheld by the caller.
            unsafe {
                core::ptr::copy_nonoverlapping(core::ptr, new_ptr, core::cmp::min(layout.size(), new_size));
                self.dealloc(core::ptr, layout);
            }
        }
        new_ptr
    }
    */
}