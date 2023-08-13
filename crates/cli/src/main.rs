#![doc = include_str!("../README.md")]

use std::{io, process::ExitCode};

use tracing::Level;

use crate::config::ConfigPath;

mod config;
mod logging;

fn main() -> ExitCode {
    if let Err(err) = try_main() {
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

fn try_main() -> anyhow::Result<()> {
    logging::init_logging(Level::DEBUG)?;

    let config = config::Config::load(ConfigPath::Default)?;

    dbg!(config);

    Ok(())
}
