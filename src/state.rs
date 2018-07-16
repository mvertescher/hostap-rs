//! Stack state management module

// #[macro_use] extern crate log;
// extern crate pretty_env_logger;

use dhcpd;
use hostapd;
use iproute2;
use iptables;

/// Bring up hostap for the interface and gateway
pub fn up(interface: &str, gateway: &str) {
    info!("Bringing up hostap!");
    iproute2::interface_up(interface);
    hostapd::up(interface);
    dhcpd::up(interface);
    iptables::up(interface, gateway);
}

/// Bring down hostap for the interface and gateway
pub fn down(interface: &str, gateway: &str) {
    info!("Bringing down hostap!");
    iptables::down(interface, gateway);
    dhcpd::down();
    hostapd::down();
    iproute2::interface_down(interface);
}

/// Print stack state information
pub fn info(interface: &str) {
    println!("Printing some info about hostap running on {}", interface);
}
