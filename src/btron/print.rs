extern crate alloc;

use super::{console::*, types::*};
use core::fmt::{self, Error, Write};

struct Writer;

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut result;
        let mut conf: [UW; 4] = [0, 0, 0, 0];
        unsafe {
            result = cons_conf(CS_GETPORT, conf.as_mut_ptr());
        }
        if result < 0 {
            return Err(Error);
        }
        unsafe {
            result = console_out(conf[0] as W, s.as_ptr() as *const B, s.len() as UW);
        }
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

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
