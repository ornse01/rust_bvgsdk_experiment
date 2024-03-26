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

#[repr(C)]
pub struct MESSAGE {
    msg_type: i32,
    msg_size: i32,
}

#[no_mangle]
pub extern "C" fn MAIN(_target: *mut MESSAGE) -> i32 {
    let mes01: [TC; 6] = [TK_B, TK_T, TK_R, TK_O, TK_N, TNULL];
    let _ = pdsp_msg(&mes01);
    ext_prc(0);

    return 0;
}
