pub type B = i8;
pub type H = i16;
pub type W = i32;
pub type UB = u8;
pub type UH = u16;
pub type UW = u32;

pub type TC = UH;

pub type ERR = W;

pub type COLOR = UW;

pub const TNULL: TC = 0;

#[repr(C)]
pub struct SIZE {
    pub h: H,
    pub v: H,
}

#[repr(C)]
pub struct PNT {
    pub x: H,
    pub y: H,
}

#[repr(C)]
pub struct RECT {
    pub left: H,
    pub top: H,
    pub right: H,
    pub bottom: H,
}

const L_FSNM: usize = 20;

#[repr(C)]
pub struct LINK {
    pub fs_name: [TC; L_FSNM],
    pub f_id: UH,
    pub atr1: UH,
    pub atr2: UH,
    pub atr3: UH,
    pub atr4: UH,
    pub atr5: UH,
}

pub const EV_NULL: W = 0;
pub const EV_BUTDWN: W = 1;
pub const EV_BUTUP: W = 2;
pub const EV_KEYDWN: W = 3;
pub const EV_KEYUP: W = 4;
pub const EV_AUTKEY: W = 5;
pub const EV_DEVICE: W = 6;

pub const MM_ALL: W = 0x7fffffff;
pub const MM_NULL: W = 0;

#[repr(C)]
pub struct RLIST {
    pub next: *mut RLIST,
    pub comp: RECT,
}

#[repr(C)]
pub struct PAT {
    pub kind: W, // TODO
}

const MAX_PTR_SIZE: usize = 10;
const P_SIZE: usize = (MAX_PTR_SIZE * MAX_PTR_SIZE) / 8;

#[repr(C)]
pub struct PTRIMG {
    pub hotpt: PNT,
    pub size: SIZE,
    pub data: [UB; P_SIZE],
    pub mask: [UB; P_SIZE],
}

pub type PTRSTL = W;

pub type WID = W;

pub const EV_SWITCH: W = 8;
pub const EV_REQUEST: W = 9;
pub const EV_MENU: W = 10;
pub const EV_RSWITCH: W = 16;
pub const EV_INACT: W = 17;
pub const EV_MSG: W = 18;
pub const EV_NOMSG: W = 19;

pub const ES_CMD: UW = 0x00000080;

pub const W_WORK: W = 0;
pub const W_FRAM: W = 1;
pub const W_PICT: W = 2;
pub const W_TITL: W = 3;
pub const W_LTHD: W = 4;
pub const W_RTHD: W = 5;
pub const W_LBHD: W = 6;
pub const W_RBHD: W = 7;
pub const W_RBAR: W = 8;
pub const W_BBAR: W = 9;
pub const W_LBAR: W = 10;
pub const W_SWITCH: W = 128;
pub const W_CLOSED: W = 129;

pub const W_REDISP: W = 0;
pub const W_PASTE: W = 1;
pub const W_DELETE: W = 2;
pub const W_FINISH: W = 3;
pub const W_OPENED: W = 5;

pub const WAIT: W = 0;
pub const NOWAIT: W = 1;
pub const NOMSG: W = 0x0010;
pub const RAWEVT: W = 0x0020;
pub const DRGEVT: W = 0x0030;

#[repr(C)]
pub struct WEVENT {
    pub r#type: W,
    pub data: [H; 4],
    pub cmd: H,
    pub wid: H,
    pub src: W,
}

#[repr(C)]
pub struct WDDISP {
    pub frame: UW,
    pub tlbg: UW,
    pub barpat: UW,
    pub barbg: UW,
    pub tlcol: COLOR,
}
