#![doc = include_str!("../README.md")]

use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
    process::ExitCode,
};

use anyhow::{Context, Result};

use clap::Parser;

use crate::{cli::Cli, config::ConfigPath};

use self::config::Config;

mod cli;
mod config;
mod logging;
mod vcs;

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

fn try_main() -> Result<()> {
    let cli = Cli::try_parse()?;

    let config = Config::load(match &cli.config {
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

    let base_url = if let Some(url) = &cli.base_url {
        Some(url.to_owned())
    } else {
        config.factorio_api.base_url.to_owned()
    };

    let api_key = resolve_api_key(&cli, &config).context("Failed to resolve API key")?;

    let api_client = facti_api::ApiClient::builder()
        .base_url(base_url)
        .api_key(api_key)
        .build();

    match cli.command {
        cli::Commands::Portal(portal) => portal.run(&api_client),
        cli::Commands::New(new) => new.run(&config),
        cli::Commands::Completion(completion) => completion.run(),

        #[cfg(debug_assertions)]
        cli::Commands::NoOp => Ok(()),
    }
}

fn resolve_api_key(cli: &Cli, config: &Config) -> Result<Option<String>> {
    if let Some(api_key) = resolve_cli_api_key(cli)? {
        Ok(Some(api_key))
    } else {
        config.factorio_api.api_key()
    }
}

fn resolve_cli_api_key(cli: &Cli) -> Result<Option<String>> {
    if cli.api_key.is_some() {
        return Ok(cli.api_key.to_owned());
    }

    if cli.api_key_stdin {
        eprintln!("Enter API key: ");
        eprint!("> ");
        io::stderr()
            .flush()
            .context("Failed to flush STDERR when showing API key prompt")?;
        let api_key = rpassword::read_password().context("Failed to read API key from STDIN")?;
        return Ok(Some(api_key.trim().to_owned()));
    }

    if let Some(path) = &cli.api_key_file {
        api_key_from_file(path).map(Some)
    } else {
        Ok(None)
    }
}

fn api_key_from_file(path: &Path) -> Result<String> {
    let file = File::open(path).context("Failed to open API key file")?;
    let mut reader = io::BufReader::new(file);
    let mut api_key = String::new();
    reader
        .read_to_string(&mut api_key)
        .context("Failed to read API key from file")?;
    Ok(api_key.trim().to_owned())
}
