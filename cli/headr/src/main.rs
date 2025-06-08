use std::process::exit;
use clap::Error;
use clap::error::ErrorKind;
use headr::{get_args, run};

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        exit(1)
    }
}
