//! Core softap interface management

use std::process::{Command, ExitStatus};

pub fn pkill(process: &str) {
    Command::new("pkill")
        .arg("-9")
        .arg(process)
        .output()
        .expect("failed to execute process");
}

pub fn interface_exists(interface: &str) -> bool {
    let output = Command::new("ip")
        .args(&["addr", "show", "dev"])
        .arg(interface)
        .output()
        .expect("failed to execute process");
    output.status.success()
}

pub fn default_gateway() -> String {
    let output = Command::new("ip")
        .args(&["route", "show", "default"])
        .output()
        .expect("failed to execute process");
    let line = String::from_utf8(output.stdout)
        .expect("failed convert stdout to string");
    let split: Vec<&str> = line.split(' ').collect();
    split[4].to_string()
}
