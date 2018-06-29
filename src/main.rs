//! Host 802.11 access point creation utility

extern crate hostap;

fn main() {
    let args = hostap::cli::Args::parse();
    println!("Arguments: {:#?}", args);
}
