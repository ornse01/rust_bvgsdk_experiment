#![no_std]
#![no_main]
#![feature(c_size_t)]
#![feature(panic_info_message)]

extern crate alloc;

use core::panic::PanicInfo;

mod btron;

use alloc::string::ToString;
use btron::allocator::Allocator;
use btron::print::*;
use btron::rustify::*;
use btron::types::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    match info.message() {
        Some(v) => {
            println!("{}", &v.to_string());
        }
        None => {
            println!("panic");
        }
    }
    match info.location() {
        Some(v) => println!(
            "file: {}, line: {}, column: {}",
            v.file(),
            v.line(),
            v.column(),
        ),
        None => println!("no location info"),
    }
    ext_prc(0)
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
    return chg_pri(0, 0, 0).unwrap_err();
}

#[repr(C)]
pub struct MESSAGE {
    msg_type: i32,
    msg_size: i32,
}

#[no_mangle]
pub extern "C" fn MAIN(target: *mut MESSAGE) -> i32 {
    unsafe {
        print!("msg_type: {}\n", (*target).msg_type);
        print!("msg_size: {}\n", (*target).msg_size);
    }

    println!("test: {}", plus_one(2));
    println!("test: {}", minus_one(2));
    println!("test: {}", sample_call(2));

    return 0;
}
