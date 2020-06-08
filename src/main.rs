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

const LONG_ABOUT: &str = "
This program uses Filter Keys to pseudo modify keyboard settings. If no\n\
arguments are present, the program disables Filter Keys. Otherwise, the\n\
program enables and initializes Filter Keys with the given or\n\
defaulted parameters.
";

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
#[structopt(about, long_about = LONG_ABOUT)]
struct Opt {
    #[structopt(short, help = "Clicks per second (31)")]
    rate: Option<f64>,

    #[structopt(short, help = "Delay time (250ms)")]
    delay: Option<u32>,
}

fn main() {
    let opt = Opt::from_args();

    let dwFlag = match (&opt.rate, &opt.delay) {
        (None, None) => 0,
        _ => FKF_AVAILABLE | FKF_FILTERKEYSON,
    };

    let delay_period = opt.delay.unwrap_or(250);
    let clicks_per_second = opt.rate.unwrap_or(31.0);
    let repeat_period = 1000.0 / clicks_per_second;

    let keys = FILTERKEYS {
        cbSize: mem::size_of::<FILTERKEYS>() as UINT,
        dwFlags: dwFlag,
        iDelayMSec: delay_period,
        iRepeatMSec: repeat_period.round() as DWORD,
        ..Default::default()
    };

    unsafe {
        if winuser::SystemParametersInfoW(
            winuser::SPI_SETFILTERKEYS,
            keys.cbSize,
            &keys as *const _ as PVOID,
            0,
        ) == 0
        {
            eprintln!("error: Could not set keys: error code {}", errhandlingapi::GetLastError());
        }
    }
}
