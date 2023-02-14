use std::thread;
use std::time::Duration;
use crate::sdk::SDK;
use crate::sdk::client::*;
use crate::sdk::hooks::function_ref::FunctionRef;

pub fn start() {
    log::info!("Starting sandbox.");

    /*
     * Start the main loop.
     */
    thread::spawn( move || {
        loop {
            unsafe { println!("Addr: 0x{:x}", (functions::FN_DRAW_SCENE.get_ptr()) as usize) };
            thread::sleep(Duration::from_secs(1));
        }
    });
    SDK.run_until_shutdown();
}