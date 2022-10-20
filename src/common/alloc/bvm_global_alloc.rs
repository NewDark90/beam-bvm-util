use core::alloc::{
    GlobalAlloc,
    Layout
};

use beam_bvm_interface::root::c_void;

use crate::common::safe::{mem::*, halt};

pub struct BvmGlobalAlloc { }

unsafe impl GlobalAlloc for BvmGlobalAlloc {

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        heap_alloc(layout.size() as u32) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        heap_free(ptr as *mut c_void)
    }

}


#[global_allocator]
pub static BVM_ALLOCATOR: BvmGlobalAlloc = BvmGlobalAlloc{};

#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    halt();
}
