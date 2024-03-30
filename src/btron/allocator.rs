use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_size_t;

extern "C" {
    fn malloc(size: c_size_t) -> *mut u8;
    fn free(p: *mut u8);
}

pub struct Allocator {}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        free(ptr)
    }
}
