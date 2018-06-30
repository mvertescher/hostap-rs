//! Command line argument parsing

use clap::{App, Arg};
use iproute2;
use std::path::PathBuf;

/// Arguments passed in on the command line
#[derive(Debug)]
pub struct Args {
    pub interface: String,
    pub gateway: String,
    pub config: Option<PathBuf>,
}

impl Default for Args {
    fn default() -> Args {
        Args {
            interface: "wlan0".to_string(),  // TODO: search for default interface?
            gateway: iproute2::default_gateway(),
            config: None,
        }
    }
}

impl Args {
    pub fn parse() -> Args {
        let mut args = Args::default();

        let matches = App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("CONFIG")
             .help("Configuration file path")
             .takes_value(true))
        .arg(Arg::with_name("interface")
             .short("i")
             .long("interface")
             .value_name("INTERFACE")
             .help("Name of the virtual interface to use")
             .takes_value(true))
        .get_matches();

        if let Some(path) = matches.value_of("config") {
            args.config = Some(PathBuf::from(path.to_string()));
        }

        if let Some(interface) = matches.value_of("interface") {
            args.interface = interface.to_string();
        }

        args
    }
}
