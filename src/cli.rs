//! Command line argument parsing

use std::path::PathBuf;

use crate::iproute2;

use clap::{App, Arg};
use clap::{crate_authors, crate_description, crate_name, crate_version};

/// Arguments passed in on the command line
#[derive(Debug)]
pub struct Args {
    pub interface: String,
    pub gateway: String,
    pub config: Option<PathBuf>,
    pub command: Command,
}

/// Arguments passed in on the command line
#[derive(Debug, PartialEq)]
pub enum Command {
    Up,
    Down,
    Info,
}

impl Default for Args {
    fn default() -> Args {
        Args {
            interface: "wlan0".to_string(),  // TODO: search for default interface?
            gateway: iproute2::default_gateway(),
            config: None,
            command: Command::Info,
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
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity"))
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
        .arg(Arg::with_name("command")
             .help("Task to run")
             .index(1)
             .possible_values(&["up", "down", "info"])
             .required(true))
        .get_matches();

        if matches.is_present("v") {
            ::std::env::set_var("RUST_LOG", "info");
        }
        pretty_env_logger::init();

        if let Some(path) = matches.value_of("config") {
            args.config = Some(PathBuf::from(path.to_string()));
        }

        if let Some(interface) = matches.value_of("interface") {
            args.interface = interface.to_string();
        }

        match matches.value_of("command").unwrap() {
            "up" => args.command = Command::Up,
            "down" => args.command = Command::Down,
            "info" => args.command = Command::Info,
            _ => unreachable!(),
        }

        args
    }
}
