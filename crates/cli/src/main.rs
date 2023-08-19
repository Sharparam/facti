#![doc = include_str!("../README.md")]

use std::{io, process::ExitCode};

use facti::run;
use human_panic::setup_panic;

fn main() -> ExitCode {
    setup_panic!();
    if let Err(err) = run() {
        if let Some(clap_err) = err.root_cause().downcast_ref::<clap::Error>() {
            clap_err.print().unwrap();
            return match clap_err.kind() {
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion => {
                    ExitCode::SUCCESS
                }
                _ => ExitCode::from(64),
            };
        }

        eprintln!("Error: {:?}", err);

        for cause in err.chain() {
            if cause.downcast_ref::<io::Error>().is_some() {
                return ExitCode::from(66);
            }
        }

        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
