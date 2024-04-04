use core::ffi::{c_char, c_int};

use super::types::*;

extern "C" {
    pub fn b_chg_pri(id: i32, pri: i32, opt: i32) -> i32;
    pub fn b_ext_prc(code: W);
    pub fn b_clr_msg(t_mask: W, last_mask: W) -> ERR;
    pub fn b_pdsp_msg(msg: *const TC) -> W;
    pub fn b_dopn_dat(lnk: *const LINK) -> W;
    pub fn b_gset_ptr(style: PTRSTL, img: *const PTRIMG, fgcol: COLOR, bgcol: COLOR) -> ERR;
    pub fn b_wopn_wnd(
        attr: UW,
        par: UW,
        r: *mut RECT,
        org: *const RECT,
        pict: W,
        tit: *const TC,
        bgpat: *const PAT,
        atr: *const WDDISP,
    ) -> W;
    pub fn b_wcls_wnd(wid: W, opt: W) -> ERR;
    pub fn b_wsta_dsp(wid: W, r: *mut RECT, rlst: *mut RLIST) -> W;
    pub fn b_wend_dsp(wid: W) -> W;
    pub fn b_wera_wnd(wid: W, r: *const RECT) -> ERR;
    pub fn b_wchk_dck(first: UW) -> W;
    pub fn b_wmov_drg(evt: *const WEVENT, newr: *mut RECT) -> W;
    pub fn b_wget_evt(evt: *mut WEVENT, mode: W) -> W;
    pub fn b_wreq_dsp(wid: W) -> ERR;
    pub fn b_wrsp_evt(evt: *mut WEVENT, nak: W) -> ERR;

    pub fn _PutString(s: *const c_char) -> c_int;
}
