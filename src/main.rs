#![allow(non_snake_case)]

use std::mem;
use structopt::StructOpt;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::UINT;
use winapi::um::errhandlingapi;
use winapi::um::winnt::PVOID;
use winapi::um::winuser;
use winapi::STRUCT;

const FKF_AVAILABLE: DWORD = 0x00000002;
const FKF_FILTERKEYSON: DWORD = 0x00000001;

STRUCT! {struct FILTERKEYS {
    cbSize: UINT,
    dwFlags: DWORD,
    iWaitMSec: DWORD,
    iDelayMSec: DWORD,
    iRepeatMSec: DWORD,
    iBounceMSec: DWORD,
}}

impl Default for FILTERKEYS {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short)]
    rate: f64,

    #[structopt(short)]
    delay: u32,
}

fn main() {
    let opt = Opt::from_args();

    let clicks_per_second = (1.0 / opt.rate) * 1000.0;

    let mut keys = FILTERKEYS {
        cbSize: mem::size_of::<FILTERKEYS>() as u32,
        dwFlags: FKF_AVAILABLE | FKF_FILTERKEYSON,
        iDelayMSec: opt.delay,
        iRepeatMSec: clicks_per_second.round() as DWORD,
        ..Default::default()
    };

    unsafe {
        if winuser::SystemParametersInfoW(
            winuser::SPI_SETFILTERKEYS,
            keys.cbSize,
            &mut keys as *mut _ as PVOID,
            0,
        ) == 0
        {
            println!("could not set keys: {}", errhandlingapi::GetLastError());
        }
    }
}
