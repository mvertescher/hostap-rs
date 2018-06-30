//! Core softap interface management

use std::process::Command;

pub fn pkill(process: &str) {
    Command::new("pkill")
        .arg("-9")
        .arg(process)
        .output()
        .expect("failed to execute process");
}
