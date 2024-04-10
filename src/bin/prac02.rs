#![no_std]
#![no_main]
#![feature(c_size_t)]
#![feature(panic_info_message)]

extern crate alloc;
extern crate rust_bvgsdk_experiment;

use core::panic::PanicInfo;

use alloc::string::ToString;
use rust_bvgsdk_experiment::btron::allocator::Allocator;
use rust_bvgsdk_experiment::btron::print::*;
use rust_bvgsdk_experiment::btron::rustify::*;
use rust_bvgsdk_experiment::btron::types::*;
use rust_bvgsdk_experiment::{print, println};

#[global_allocator]
static ALLOCATOR: Allocator = Allocator {};

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
const TK_T: TC = TK_A + 19;

const TK_a: TC = 0x2361;
const TK_e: TC = TK_a + 4;
const TK_s: TC = TK_a + 18;
const TK_t: TC = TK_a + 19;

#[repr(C)]
pub struct MESSAGE {
    msg_type: i32,
    msg_size: i32,
}

fn killme(wid: WID) {
    let _ = gset_ptr(Some(StdPointerShape::PS_BUSY), None, None, None);
    let _ = wcls_wnd(wid, WClsWndOpt::CLR);
    ext_prc(0);
}

fn redisp(wid: WID) {
    loop {
        let result = wsta_dsp(wid, None, None).unwrap();
        if result == 0 {
            break;
        }
        let _ = wera_wnd(wid, None);

        let result = wend_dsp(wid).unwrap();
        match result {
            true => continue,
            false => break,
        }
    }
}

fn butdn(wid: WID, wev: &mut WEVENT) {
    match wev.cmd as W {
        W_PICT => {
            let result = wchk_dck(wev.s_time()).unwrap();
            match result {
                PD::W_DCLICK => {
                    killme(wid);
                }
                PD::W_PRESS => {
                    let result = wmov_drg(wev, None).unwrap();
                    if result {
                        redisp(wid)
                    }
                }
                _ => {}
            }
        }
        W_FRAM => {
            let result = wmov_drg(wev, None).unwrap();
            if result {
                redisp(wid)
            }
        }
        W_TITL => {
            let result = wmov_drg(wev, None).unwrap();
            if result {
                redisp(wid)
            }
        }
        _ => {}
    }
}

fn initialize(rect: &mut RECT, title: &[TC]) -> Result<WID, ERR> {
    dopn_dat(None)?;

    let wid = wopn_wnd(0, 0, rect, None, 2, title, None, None)?;

    Ok(wid)
}

fn event_loop(wid: WID) {
    let mut wev = WEVENT {
        r#type: 0,
        data: [0, 0, 0, 0],
        cmd: 0,
        wid: 0,
        src: 0,
    };

    let _ = wreq_dsp(Some(wid));

    loop {
        let _ = wget_evt(&mut wev, WAIT);
        match wev.r#type {
            EV_NULL => {
                if wev.wid as WID != wid {
                    continue;
                }
                if wev.cmd as W != W_WORK {
                    continue;
                }
                if wev.s_stat() & ES_CMD != 0 {
                    continue;
                }
                let _ = gset_ptr(Some(StdPointerShape::PS_SELECT), None, None, None);
            }
            EV_REQUEST => match wev.cmd as W {
                W_REDISP => redisp(wid),
                W_PASTE => {
                    let _ = wrsp_evt(&mut wev, true);
                }
                W_DELETE => {
                    let _ = wrsp_evt(&mut wev, false);
                    killme(wid);
                }
                W_FINISH => {
                    let _ = wrsp_evt(&mut wev, false);
                    killme(wid);
                }
                _ => {}
            },
            EV_RSWITCH => {
                redisp(wid);
            }
            EV_SWITCH => {
                butdn(wid, &mut wev);
            }
            EV_BUTDWN => {
                butdn(wid, &mut wev);
            }
            EV_INACT => {
                // do nothing
            }
            EV_DEVICE => {
                //oprc_dev(&wev0.e, NULL, 0);
            }
            EV_MSG => {
                let _ = clr_msg(MM_ALL, MM_ALL);
            }
            _ => {}
        }
    }
}

#[no_mangle]
pub extern "C" fn MAIN(_: *mut MESSAGE) -> i32 {
    let mut r0 = RECT {
        left: 100,
        top: 100,
        right: 300 + 7,
        bottom: 200 + 30,
    };
    let tit0: [TC; 5] = [TK_T, TK_e, TK_s, TK_t, TNULL];

    let result = initialize(&mut r0, &tit0);
    match result {
        Ok(wid) => event_loop(wid),
        Err(_) => {
            ext_prc(0);
        }
    }

    return 0;
}
