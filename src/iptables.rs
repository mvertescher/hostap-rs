//! iptables management

use crate::ipt;
use std::process::Command;

pub fn up(interface: &str, gateway: &str) {
    let output = Command::new("sysctl")
        .arg("net.ipv4.ip_forward=1")
        .output()
        .expect("failed to execute process");
    assert!(output.status.success());

    // TODO: Comment below
    let ipt = ipt::new(false).unwrap();
    assert_eq!(ipt.append("filter", "FORWARD",
                          &format!("-i {} -o {} -j ACCEPT", interface, gateway)).unwrap(), true);
    assert_eq!(ipt.append("filter", "FORWARD",
                          &format!("-i {} -o {} -m state --state ESTABLISHED,RELATED -j ACCEPT",
                                  gateway, interface)).unwrap(), true);
    assert_eq!(ipt.append("nat", "POSTROUTING",
                          &format!("-o {} -j MASQUERADE", gateway)).unwrap(), true);
}

pub fn down(interface: &str, gateway: &str) {
    let output = Command::new("sysctl")
        .arg("net.ipv4.ip_forward=0")
        .output()
        .expect("failed to execute process");
    assert!(output.status.success());

    let ipt = ipt::new(false).unwrap();
    ipt.delete("filter", "FORWARD",
               &format!("-i {} -o {} -j ACCEPT", interface, gateway)).unwrap();
    ipt.delete("filter", "FORWARD",
               &format!("-i {} -o {} -m state --state ESTABLISHED,RELATED -j ACCEPT",
                        gateway, interface)).unwrap();
    ipt.delete("nat", "POSTROUTING",
               &format!("-o {} -j MASQUERADE", gateway)).unwrap();
}
