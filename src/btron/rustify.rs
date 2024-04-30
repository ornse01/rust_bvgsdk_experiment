use super::brightv::*;
use super::types::*;

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

impl WEVENT {
    pub fn s_stat(&self) -> UW {
        self.src as UW
    }
    pub fn s_time(&self) -> UW {
        ((self.data[1] as UW) << 16) | ((self.data[0] as UW) & 0xffff)
    }
}

pub fn chg_pri(id: i32, pri: i32, opt: i32) -> Result<W, ERR> {
    let ret: i32;
    unsafe { ret = b_chg_pri(id, pri, opt) }
    if ret >= 0 {
        Ok(ret)
    } else {
        Err(ret)
    }
}

pub fn ext_prc(code: W) -> ! {
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

    let style_val = style.map_or(-1, |v| v as PTRSTL);
    let img_val = img.map_or(core::ptr::null(), |v| v  as *const PTRIMG);
    let fgcol_val =  fgcol.unwrap_or(0x80000000);
    let bgcol_val = bgcol.unwrap_or(0x80000000);

    unsafe { err = b_gset_ptr(style_val, img_val, fgcol_val, bgcol_val) }
    if err >= 0 {
        Ok(0)
    } else {
        Err(err)
    }
}

pub fn dopn_dat(lnk: Option<&LINK>) -> Result<W, ERR> {
    let ret;
    let lnk_val = lnk.map_or(core::ptr::null(), |v|  v as *const LINK);
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
    let org_val = org.map_or(core::ptr::null(), |v|v as *const RECT);
    let tit_val = tit.as_ptr();
    let bgpat_val = bgpat.map_or(core::ptr::null(), |v|v as *const PAT);
    let atr_val = atr.map_or(core::ptr::null(), |v|v as *const WDDISP);

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
    let wid_val = wid.unwrap_or(0);
    unsafe { ret = b_wreq_dsp(wid_val) }
    if ret >= 0 {
        Ok(ret)
    } else {
        Err(ret)
    }
}

pub fn wsta_dsp(wid: WID, r: Option<*mut RECT>, _: Option<&RLIST>) -> Result<W, ERR> {
    let ret;
    let r_val = r.unwrap_or(core::ptr::null_mut());
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
    let r_val = r.map_or(core::ptr::null(), |v| v as *const RECT);
    unsafe { ret = b_wera_wnd(wid, r_val) }
    if ret >= 0 {
        Ok(ret)
    } else {
        Err(ret)
    }
}

pub fn wmov_drg(evt: &WEVENT, newr: Option<*mut RECT>) -> Result<bool, ERR> {
    let ret;
    let newr_val = newr.unwrap_or(core::ptr::null_mut());
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
