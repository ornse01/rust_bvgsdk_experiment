#![no_std]
#![no_main]
#![feature(c_size_t)]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::{c_char, c_size_t};
use core::panic::PanicInfo;
use core::str;

use alloc::ffi::CString;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn b_chg_pri(id: i32, pri: i32, opt: i32) -> i32;

    fn malloc(size: c_size_t) -> *mut u8;
    fn free(p: *mut u8);
}

struct BTRONAllocator {}

#[global_allocator]
static ALLOCATOR: BTRONAllocator = BTRONAllocator {};

unsafe impl GlobalAlloc for BTRONAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        free(ptr)
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn minus_one(x: i32) -> i32 {
    x - 1
}

fn sample_call(_value: i32) -> i32 {
    unsafe {
        let err = b_chg_pri(0, 0, 0);
        err
    }
}

fn print(format: &str, value: i32) {
    unsafe {
        printf(format.as_ptr(), value);
    }
}

#[repr(C)]
pub struct MESSAGE {
    msg_type: i32,
    msg_size: i32,
}

#[no_mangle]
pub extern "C" fn MAIN(target: *mut MESSAGE) -> i32 {
    unsafe {
        print("msg_type: %d\n", (*target).msg_type);
        print("msg_size: %d\n", (*target).msg_size);
    }

    print("test: %d\n", plus_one(2));
    print("test: %d\n", minus_one(2));
    print("test: %08x\n", sample_call(2));

    return 0;
}
