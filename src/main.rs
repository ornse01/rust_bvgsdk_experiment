#![no_std]
#![no_main]
#![feature(c_size_t)]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::{c_char, c_size_t};
use core::panic::PanicInfo;
use core::result::Result;
use core::str;

use alloc::ffi::CString;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub type B = i8;
pub type H = i16;
pub type W = i32;
pub type UB = u8;
pub type UH = u16;
pub type UW = u32;

pub type TC = UH;

pub type ERR = W;

pub type COLOR = UW;

const TNULL: TC = 0;

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

#[repr(C)]
pub struct SIZE {
    h: H,
    v: H,
}

#[repr(C)]
pub struct PNT {
    x: H,
    y: H,
}

#[repr(C)]
pub struct RECT {
    left: H,
    top: H,
    right: H,
    bottom: H,
}

impl RECT {
    pub fn lefttop(&self) -> PNT {
        PNT {
            x: self.left,
            y: self.top,
        }
    }
    pub fn rightbot(&self) -> PNT {
        PNT {
            x: self.right,
            y: self.bottom,
        }
    }
}

const L_FSNM: usize = 20;

#[repr(C)]
pub struct LINK {
    fs_name: [TC; L_FSNM],
    f_id: UH,
    atr1: UH,
    atr2: UH,
    atr3: UH,
    atr4: UH,
    atr5: UH,
}

const EV_NULL: W = 0;
const EV_BUTDWN: W = 1;
const EV_BUTUP: W = 2;
const EV_KEYDWN: W = 3;
const EV_KEYUP: W = 4;
const EV_AUTKEY: W = 5;
const EV_DEVICE: W = 6;

const MM_ALL: W = 0x7fffffff;
const MM_NULL: W = 0;

#[repr(C)]
pub struct RLIST {
    next: *mut RLIST,
    comp: RECT,
}

#[repr(C)]
pub struct PAT {
    kind: W, // TODO
}

const MAX_PTR_SIZE: usize = 10;
const P_SIZE: usize = (MAX_PTR_SIZE * MAX_PTR_SIZE) / 8;

#[repr(C)]
pub struct PTRIMG {
    hotpt: PNT,
    size: SIZE,
    data: [UB; P_SIZE],
    mask: [UB; P_SIZE],
}

pub type PTRSTL = W;

pub type WID = W;

const EV_SWITCH: W = 8;
const EV_REQUEST: W = 9;
const EV_MENU: W = 10;
const EV_RSWITCH: W = 16;
const EV_INACT: W = 17;
const EV_MSG: W = 18;
const EV_NOMSG: W = 19;

const ES_CMD: UW = 0x00000080;

const W_WORK: W = 0;
const W_FRAM: W = 1;
const W_PICT: W = 2;
const W_TITL: W = 3;
const W_LTHD: W = 4;
const W_RTHD: W = 5;
const W_LBHD: W = 6;
const W_RBHD: W = 7;
const W_RBAR: W = 8;
const W_BBAR: W = 9;
const W_LBAR: W = 10;
const W_SWITCH: W = 128;
const W_CLOSED: W = 129;

const W_REDISP: W = 0;
const W_PASTE: W = 1;
const W_DELETE: W = 2;
const W_FINISH: W = 3;
const W_OPENED: W = 5;

const WAIT: W = 0;
const NOWAIT: W = 1;
const NOMSG: W = 0x0010;
const RAWEVT: W = 0x0020;
const DRGEVT: W = 0x0030;

#[repr(C)]
pub struct WEVENT {
    r#type: W,
    data: [H; 4],
    cmd: H,
    wid: H,
    src: W,
}

impl WEVENT {
    fn s_stat(&self) -> UW {
        self.src as UW
    }
    fn s_time(&self) -> UW {
        ((self.data[1] as UW) << 16) | ((self.data[0] as UW) & 0xffff)
    }
}

#[repr(C)]
pub struct WDDISP {
    frame: UW,
    tlbg: UW,
    barpat: UW,
    barbg: UW,
    tlcol: COLOR,
}

extern "C" {
    fn b_chg_pri(id: i32, pri: i32, opt: i32) -> i32;
    fn b_ext_prc(code: W);
    fn b_clr_msg(t_mask: W, last_mask: W) -> ERR;
    fn b_pdsp_msg(msg: *const TC) -> W;
    fn b_dopn_dat(lnk: *const LINK) -> W;
    fn b_gset_ptr(style: PTRSTL, img: *const PTRIMG, fgcol: COLOR, bgcol: COLOR) -> ERR;
    fn b_wopn_wnd(
        attr: UW,
        par: UW,
        r: *mut RECT,
        org: *const RECT,
        pict: W,
        tit: *const TC,
        bgpat: *const PAT,
        atr: *const WDDISP,
    ) -> W;
    fn b_wcls_wnd(wid: W, opt: W) -> ERR;
    fn b_wsta_dsp(wid: W, r: *mut RECT, rlst: *mut RLIST) -> W;
    fn b_wend_dsp(wid: W) -> W;
    fn b_wera_wnd(wid: W, r: *const RECT) -> ERR;
    fn b_wchk_dck(first: UW) -> W;
    fn b_wmov_drg(evt: *const WEVENT, newr: *mut RECT) -> W;
    fn b_wget_evt(evt: *mut WEVENT, mode: W) -> W;
    fn b_wreq_dsp(wid: W) -> ERR;
    fn b_wrsp_evt(evt: *mut WEVENT, nak: W) -> ERR;

    fn printf(format: *const c_char, value: i32) -> i32;

    fn malloc(size: c_size_t) -> *mut u8;
    fn free(p: *mut u8);
}

pub fn ext_prc(code: W) {
    unsafe { b_ext_prc(code) }
}

pub fn clr_msg(t_mask: W, last_mask: W) -> Result<W, ERR> {
    let ret;
    unsafe { ret = b_clr_msg(t_mask, last_mask) }
    if ret >= 0 {
        Ok(ret)
    } else {
        Err(ret)
    }
}

pub fn pdsp_msg(msg: &[TC]) -> Result<W, W> {
    let err;
    let msg_ptr = msg.as_ptr();
    unsafe { err = b_pdsp_msg(msg_ptr) }
    if err >= 0 {
        Ok(err)
    } else {
        Err(err)
    }
}

pub enum StdPointerShape {
    PS_SELECT = 0,
    PS_MODIFY = 1,
    PS_MOVE = 2,
    PS_VMOVE = 3,
    PS_HMOVE = 4,
    PS_GRIP = 5,
    PS_VGRIP = 6,
    PS_HGRIP = 7,
    PS_RSIZ = 8,
    PS_VRSIZ = 9,
    PS_HRSIZ = 10,
    PS_PICK = 11,
    PS_VPICK = 12,
    PS_HPICK = 13,
    PS_BUSY = 14,
    PS_MENU = 15,
}

pub fn gset_ptr(
    style: Option<StdPointerShape>,
    img: Option<&PTRIMG>,
    fgcol: Option<COLOR>,
    bgcol: Option<COLOR>,
) -> Result<W, ERR> {
    let err: ERR;

    let style_val = match style {
        Some(v) => v as PTRSTL,
        None => -1,
    };
    let img_val = match img {
        Some(v) => v as *const PTRIMG,
        None => core::ptr::null(),
    };
    let fgcol_val = match fgcol {
        Some(v) => v,
        None => 0x80000000,
    };
    let bgcol_val = match bgcol {
        Some(v) => v,
        None => 0x80000000,
    };

    unsafe { err = b_gset_ptr(style_val, img_val, fgcol_val, bgcol_val) }
    if err >= 0 {
        Ok(0)
    } else {
        Err(err)
    }
}

pub fn dopn_dat(lnk: Option<&LINK>) -> Result<W, ERR> {
    let ret;
    let lnk_val = match lnk {
        Some(v) => v as *const LINK,
        None => core::ptr::null(),
    };
    unsafe { ret = b_dopn_dat(lnk_val) }
    if ret >= 0 {
        Ok(ret)
    } else {
        Err(ret)
    }
}

pub fn wopn_wnd(
    attr: UW,
    par: UW,
    r: &mut RECT,
    org: Option<&RECT>,
    pict: W,
    tit: &[TC],
    bgpat: Option<&PAT>,
    atr: Option<&WDDISP>,
) -> Result<WID, W> {
    let err;
    let r_val = r as *mut RECT;
    let org_val = match org {
        Some(v) => v as *const RECT,
        None => core::ptr::null(),
    };
    let tit_val = tit.as_ptr();
    let bgpat_val = match bgpat {
        Some(v) => v as *const PAT,
        None => core::ptr::null(),
    };
    let atr_val = match atr {
        Some(v) => v as *const WDDISP,
        None => core::ptr::null(),
    };

    unsafe { err = b_wopn_wnd(attr, par, r_val, org_val, pict, tit_val, bgpat_val, atr_val) }
    if err >= 0 {
        Ok(err)
    } else {
        Err(err)
    }
}

pub enum WClsWndOpt {
    CLR = 0,
    NOCLR = 8,
}

pub fn wcls_wnd(wid: WID, opt: WClsWndOpt) -> Result<W, ERR> {
    let ret;
    unsafe { ret = b_wcls_wnd(wid, opt as W) }
    if ret >= 0 {
        Ok(ret)
    } else {
        Err(ret)
    }
}

pub fn wreq_dsp(wid: Option<WID>) -> Result<W, ERR> {
    let ret;
    let wid_val = match wid {
        Some(v) => v,
        None => 0,
    };
    unsafe { ret = b_wreq_dsp(wid_val) }
    if ret >= 0 {
        Ok(ret)
    } else {
        Err(ret)
    }
}

pub fn wsta_dsp(wid: WID, r: Option<*mut RECT>, _: Option<&RLIST>) -> Result<W, ERR> {
    let ret;
    let r_val = match r {
        Some(v) => v,
        None => core::ptr::null_mut(),
    };
    unsafe { ret = b_wsta_dsp(wid, r_val, core::ptr::null_mut()) } // TODO: rlist
    if ret >= 0 {
        Ok(ret)
    } else {
        Err(ret)
    }
}

pub fn wend_dsp(wid: WID) -> Result<bool, ERR> {
    let ret;
    unsafe { ret = b_wend_dsp(wid) }
    if ret == 1 {
        Ok(true)
    } else if ret == 0 {
        Ok(false)
    } else {
        Err(ret)
    }
}

pub fn wera_wnd(wid: WID, r: Option<&RECT>) -> Result<W, ERR> {
    let ret;
    let r_val = match r {
        Some(v) => v,
        None => core::ptr::null(),
    };
    unsafe { ret = b_wera_wnd(wid, r_val) }
    if ret >= 0 {
        Ok(ret)
    } else {
        Err(ret)
    }
}

pub fn wmov_drg(evt: &WEVENT, newr: Option<*mut RECT>) -> Result<bool, ERR> {
    let ret;
    let newr_val = match newr {
        Some(v) => v,
        None => core::ptr::null_mut(),
    };
    unsafe { ret = b_wmov_drg(evt, newr_val) }
    if ret == 1 {
        Ok(true)
    } else if ret == 0 {
        Ok(false)
    } else {
        Err(ret)
    }
}

// TODO: mode flag and return value
pub fn wget_evt(evt: &mut WEVENT, mode: W) -> Result<W, ERR> {
    let ret;
    unsafe { ret = b_wget_evt(evt, mode) }
    if ret >= 0 {
        Ok(ret)
    } else {
        Err(ret)
    }
}

pub enum PD {
    W_PRESS = 0,
    W_QPRESS = 1,
    W_CLICK = 2,
    W_DCLICK = 3,
}

pub fn wchk_dck(first: UW) -> Result<PD, ERR> {
    let ret;
    unsafe { ret = b_wchk_dck(first) }
    if ret == PD::W_PRESS as W {
        Ok(PD::W_PRESS)
    } else if ret == PD::W_QPRESS as W {
        Ok(PD::W_QPRESS)
    } else if ret == PD::W_CLICK as W {
        Ok(PD::W_CLICK)
    } else if ret == PD::W_DCLICK as W {
        Ok(PD::W_DCLICK)
    } else {
        Err(ret)
    }
}

pub fn wrsp_evt(evt: &mut WEVENT, nak: bool) -> Result<W, ERR> {
    let ret;
    let nak_val = match nak {
        true => 1,
        false => 0,
    };
    unsafe { ret = b_wrsp_evt(evt, nak_val) }
    if ret >= 0 {
        Ok(ret)
    } else {
        Err(ret)
    }
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
