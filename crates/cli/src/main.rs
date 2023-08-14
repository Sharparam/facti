#![doc = include_str!("../README.md")]

use std::{io, process::ExitCode};

use clap::Parser;

use crate::{cli::Cli, config::ConfigPath};

mod cli;
mod config;
mod logging;

fn main() -> ExitCode {
    if let Err(err) = try_main() {
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

fn try_main() -> anyhow::Result<()> {
    let cli = Cli::try_parse()?;

    let config = config::Config::load(match &cli.config {
        Some(path) => ConfigPath::Custom(path.to_path_buf()),
        None => ConfigPath::Default,
    })?;

    let level_filter = match cli.log_level_filter() {
        Some(f) => f,
        None => match config.log_level_filter {
            Some(f) => f,
            None => Default::default(),
        },
    };

    logging::init_logging(level_filter)?;

    let base_url = cli.base_url.or(config.factorio_api.base_url);
    let api_key = cli.api_key.or(config.factorio_api.api_key);
    let api_client = facti_api::ApiClient::builder()
        .base_url(base_url)
        .api_key(api_key)
        .build();

    match cli.command {
        cli::Commands::Portal(portal) => portal.run(&api_client),
        cli::Commands::Completion(completion) => completion.run(),
    }
}
