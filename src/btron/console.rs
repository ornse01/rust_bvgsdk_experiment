#![allow(dead_code)]

use core::ffi::{c_char, c_int};

use super::types::*;

extern "C" {
    pub fn console_out(port: W, buf: *const B, len: UW) -> ERR;
    pub fn _PutString(s: *const c_char) -> c_int;
    pub fn cons_put(port: W, buf: *const B, len: UW, tmout: W) -> W;
    pub fn cons_conf(req: W, arg: *mut UW) -> W;
}

pub const CS_CREATE: W = 0x11;
pub const CS_DELETE: W = 0x12;
pub const CS_SETCONF: W = 0x13;
pub const CS_GETCONF: W = 0x14;
pub const CS_GETPORT: W = 0x21;
pub const CS_SETPORT: W = 0x22;
pub const CS_SRCHPORT: W = 0x23;
