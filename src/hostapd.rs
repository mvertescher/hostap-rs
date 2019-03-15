//! hostapd management

use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

use crate::core;

use log::*;

/// Represent the hostapd configuration as an ordered set of key/value pairs
pub struct Config(Vec<(String, String)>);

macro_rules! insert {
    ($map:expr, $key:expr, $val:expr) => {{
        $map.push(($key.to_string(), $val.to_string()))
    }}
}

impl Default for Config {
    fn default() -> Config {
        let mut map: Vec<(String, String)> = Vec::new();
        insert!(map, "interface", "");
        insert!(map, "driver", "nl80211");
        insert!(map, "channel", "6");
        insert!(map, "hw_mode", "g");
        insert!(map, "country_code", "US");
        insert!(map, "ssid", "alfa");
        insert!(map, "auth_algs", "1");
        insert!(map, "wpa", "3");
        insert!(map, "wpa_passphrase", "funfunfun");
        insert!(map, "wpa_key_mgmt", "WPA-PSK");
        insert!(map, "wpa_pairwise", "TKIP");
        insert!(map, "rsn_pairwise", "CCMP");
        Config(map)
    }
}

impl Config {
    /// Convert the configuration to a string.
    fn serialize(self) -> String {
        let mut string = String::new();
        for (key, value) in self.0 {
            string.push_str(key.as_ref());
            string.push_str("=");
            string.push_str(value.as_ref());
            string.push_str("\n");
        }
        string
    }

    /// Write the configuration to a file.
    pub fn to_file(self, path: &PathBuf) {
        let mut file = File::create(path)
            .expect("failed to create file");
        file.write_all(self.serialize().as_ref()).unwrap();
    }
}

/// Bring up hostapd on the interface
pub fn up(interface: &str) {
    let mut config_path = PathBuf::new();
    config_path.push("hostapd.conf");

    let mut config = Config::default();
    config.0.remove(0);
    config.0.insert(0, ("interface".to_string(), interface.to_string()));
    config.to_file(&config_path);

    let output = Command::new("hostapd")
        .args(&["-B", "-t", "-K", "-f", "hostapd.log"])
        .arg(config_path.to_str().unwrap())
        .output()
        .expect("failed to execute process");
    info!("{}", String::from_utf8_lossy(&output.stdout));
    info!("{}", String::from_utf8_lossy(&output.stderr));
    fs::remove_file(config_path).unwrap();
}

/// Tear down hostapd
pub fn down() {
    core::pkill("hostapd");
}
