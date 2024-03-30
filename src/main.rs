#![no_std]
#![no_main]
#![feature(c_size_t)]

extern crate alloc;

use core::ffi::c_char;
use core::panic::PanicInfo;
use core::str;

use alloc::ffi::CString;

mod btron;

use btron::allocator::Allocator;
use btron::rustify::*;
use btron::types::*;

extern "C" {
    fn printf(format: *const c_char, value: i32) -> i32;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const TK_A: TC = 0x2341;
const TK_B: TC = TK_A + 1;
const TK_T: TC = TK_A + 19;
const TK_R: TC = TK_A + 17;
const TK_O: TC = TK_A + 14;
const TK_N: TC = TK_A + 13;

const TK_a: TC = 0x2361;
const TK_e: TC = TK_a + 4;
const TK_s: TC = TK_a + 18;
const TK_t: TC = TK_a + 19;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator {};

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
    let c_string = CString::new(format).expect("CString::new failed");
    let ptr = c_string.as_ptr();
    unsafe {
        printf(ptr, value);
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
