#![no_std]
#![no_main]

use core::str;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern {
    fn b_chg_pri(id: i32, pri: i32, opt: i32) -> i32;
    fn printf(format: *const u8, value: i32) -> i32;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn minus_one(x: i32) -> i32 {
    x - 1
}

fn sample_call(value: i32) -> i32 {
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

#[no_mangle]
pub extern "C" fn MAIN() -> i32 {
	print("test: %d\n", plus_one(2));
	print("test: %d\n", minus_one(2));
	print("test: %08x\n", sample_call(2));

    return 0;
}
