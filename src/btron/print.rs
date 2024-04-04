extern crate alloc;

use super::brightv::*;
use core::fmt::{self, Error, Write};

struct Writer;

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let result;
        let c_string = alloc::ffi::CString::new(s).unwrap();
        let ptr = c_string.as_ptr();
        unsafe { result = _PutString(ptr) }
        if result >= 0 {
            Ok(())
        } else {
            Err(Error)
        }
    }
}

pub fn _print(args: core::fmt::Arguments) {
    let mut writer = Writer {};
    writer.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (_print(format_args!($($arg)*)));
}
