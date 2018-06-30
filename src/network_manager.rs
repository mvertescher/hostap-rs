//! Network manager interface

use ini::Ini;
use nm;

/// Restart NetworkManager
///
/// Warning: This with cycle your host's connection
pub fn restart() {
    nm::NetworkManager::stop_service(10).unwrap();
    nm::NetworkManager::start_service(10).unwrap();
}

/// Tell NetworkManager to ignore a specific interface.
///
/// Warning: The following is very invasive currently.
/// See "man 5 NetworkManager.conf" for more details.
///
/// # Panics
///
/// Panics if not executed with root permissions.
pub fn ignore_interface(interface: &str) {
    let network_manager_config = "/etc/NetworkManager/NetworkManager.conf";
    let mut ini = Ini::load_from_file(network_manager_config)
        .expect("Failed to load ini file");

    let expected_value = {
        let mut value = "interface-name:".to_string();
        value.push_str(interface);
        value
    };

    // Check if the interface is already ignored
    match ini.section(Some("keyfile".to_owned())) {
        Some(section) => {
            match section.get("unmanaged-devices") {
                Some(value) => {
                    if value == &expected_value {
                        return;
                    }
                }
                None => (),
            }
        },
        None => (),
    }

    // Write a key/value to ignore the interface
    ini.with_section(Some("keyfile".to_owned()))
        .set("unmanaged-devices", expected_value.as_ref());

    ini.write_to_file(network_manager_config)
        .expect("Failed to write ini file");
}
