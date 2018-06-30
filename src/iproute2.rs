//! Basic iproute2 API

use std::process::{Command, ExitStatus};

/// Check if an interface exists by name
pub fn interface_exists(interface: &str) -> bool {
    let output = Command::new("ip")
        .args(&["addr", "show", "dev"])
        .arg(interface)
        .output()
        .expect("failed to execute process");
    output.status.success()
}

pub fn interface_up(interface: &str) {
    let output = Command::new("ip")
        .args(&["link", "set", "dev", interface, "up"])
        .output()
        .expect("failed to execute process");
    assert!(output.status.success())
}

pub fn interface_down(interface: &str) {
    let output = Command::new("ip")
        .args(&["link", "set", "dev", interface, "down"])
        .output()
        .expect("failed to execute process");
    assert!(output.status.success())
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
