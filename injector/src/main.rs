use std::process::Command;
use std::thread;
use std::time::Duration;
use dll_syringe::process::{OwnedProcess, Process};
use dll_syringe::Syringe;

fn main() {
    let mut process = OwnedProcess::find_first_by_name("osclient.exe");
    if process.is_some() {
        println!("Killing currently running process.");
        process.unwrap().kill().unwrap();
        thread::sleep(Duration::from_secs(5));
    }

    println!("Launching Old School RuneScape client.");
    let proc = Command::new("injector/bin/osclient.exe").spawn().unwrap();

    loop {
        process = Some(OwnedProcess::from_pid(proc.id()).unwrap());
        if process.is_none() {
            thread::sleep(Duration::from_millis(100));
        } else {
            break;
        }
    }
    println!("Found osclient.exe process. PID: {:?}", proc.id());

    let injector = Syringe::for_process(process.unwrap());
    injector.inject("target/debug/sandbox.dll").unwrap();
    println!("Successfully injected sandbox.dll into process.");
}
