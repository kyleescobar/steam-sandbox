#![feature(pointer_byte_offsets)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]
#![allow(dead_code)]
#![feature(fn_traits, unboxed_closures)]
#![feature(strict_provenance)]
#![feature(tuple_trait)]

mod sdk;
mod sandbox;

pub use faithe;
pub use faithe::{global, interface};

use std::{panic, thread};
use simple_logger::SimpleLogger;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::um::consoleapi::AllocConsole;
use crate::sdk::SDK;

#[no_mangle]
pub extern "system" fn DllMain(
    module: HINSTANCE,
    reason: DWORD,
    _: LPVOID
) -> BOOL {
    match reason {
        1 => { thread::spawn(move || {
            unsafe { on_load(); }
        }); },
        _ => {}
    };
    TRUE
}

unsafe fn on_load() {
    let res = panic::catch_unwind(|| {
        AllocConsole();
        SimpleLogger::new().init().unwrap();
        lazy_static::initialize(&SDK);
    });

    match res {
        Err(e) => log::error!("Error: {:?}", e),
        _ => {}
    };
}