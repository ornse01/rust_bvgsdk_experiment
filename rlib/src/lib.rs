#![feature(lang_items)]
#![no_std]

#[no_mangle]
pub extern fn plus_one(x: i32) -> i32 {
    x + 1
}

#[no_mangle]
pub extern fn minus_one(x: i32) -> i32 {
    x - 1
}

extern {
    fn link_confirmation(value: i32) -> i32;
    fn chg_pri(id: i32, pri: i32, opt: i32) -> i32;
}

#[no_mangle]
pub extern fn sample_call(value: i32) -> i32 {
    unsafe {
        link_confirmation(value);
        let err = chg_pri(0, 0, 0);
        err
    }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }
