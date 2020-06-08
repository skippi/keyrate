#![allow(non_snake_case)]

use std::mem;
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

fn main() {
    let mut keys = FILTERKEYS {
        cbSize: mem::size_of::<FILTERKEYS>() as u32,
        dwFlags: FKF_AVAILABLE | FKF_FILTERKEYSON,
        iDelayMSec: 175,
        iRepeatMSec: 6,
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
