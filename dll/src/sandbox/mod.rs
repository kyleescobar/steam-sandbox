use crate::{function, hook};
use crate::sdk::SDK;

fn print_hello() {
    println!("Hello World")
}

function! {
    pub TEST: fn() = "sandbox.dll"#0x1234567;
}

pub fn start() {
    log::info!("Starting sandbox.");

    /*
     * Start the main loop.
     */
    SDK.run_until_shutdown();
}