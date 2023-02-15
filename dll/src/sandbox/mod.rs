use std::thread;
use std::time::Duration;
use crate::sdk::SDK;
use crate::sdk::client::*;

pub fn start() {
    log::info!("Starting sandbox.");

    /*
     * Start the main loop.
     */
    thread::spawn( move || {
        loop {
        }
    });
    SDK.run_until_shutdown();
}