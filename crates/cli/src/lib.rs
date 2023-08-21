//! # Features
//!
//! - **`ron`:** Enables RON support for changelog conversion.
//! - **`sexpr`:** Enables S-Expression and Emacs Lisp support for changelog conversion.
//! - **`yaml`:** Enables YAML support for changelog conversion.

#![cfg_attr(docsrs, feature(doc_cfg))]

use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use anyhow::{Context, Result};

use clap::Parser;

use self::{
    cli::Cli,
    config::{Config, ConfigPath},
    logging::LogState,
};

mod cli;
mod config;
mod dirs;
mod logging;
mod vcs;

/// xtask needs access to the structs for main and sub commands for the CLI,
/// but we don't want to expose them to users.
#[doc(hidden)]
pub mod __xtask {
    pub use super::cli::Cli;
}

/// Runs the CLI interface.
///
/// Not meant to be called by anything other than the `facti` binary.
/// It needs to be exposed in order for the `facti` crate to be able to be used
/// in the `xtask` crate to generate manpages.
pub fn run() -> Result<LogState> {
    let cli = Cli::try_parse()?;

    let level_filter = cli.log_level_filter();

    let log_guard = logging::init(level_filter.unwrap_or_default())?;

    let config = Config::load(match &cli.config {
        Some(path) => ConfigPath::Custom(path.to_path_buf()),
        None => ConfigPath::Default,
    })?;

    let level_filter = level_filter.unwrap_or_else(|| match config.log_level_filter {
        Some(f) => f,
        None => Default::default(),
    });

    log_guard.set_level_filter(level_filter)?;

    let portal_base_url = if let Some(url) = &cli.portal_base_url {
        Some(url.to_owned())
    } else {
        config.factorio_api.portal_base_url.to_owned()
    };

    let game_base_url = if let Some(url) = &cli.game_base_url {
        Some(url.to_owned())
    } else {
        config.factorio_api.game_base_url.to_owned()
    };

    let api_key = resolve_api_key(&cli, &config).context("Failed to resolve API key")?;

    let mut api_builder = facti_api::blocking::ApiClient::builder();

    if let Some(base_url) = portal_base_url {
        api_builder.portal_base_url(base_url);
    }

    if let Some(base_url) = game_base_url {
        api_builder.game_base_url(base_url);
    }

    if let Some(api_key) = api_key {
        api_builder.api_key(api_key);
    }

    let api_client = api_builder.build();

    match cli.command {
        cli::Commands::Portal(portal) => portal.run(&api_client),
        cli::Commands::New(new) => new.run(&config),
        cli::Commands::Changelog(changelog) => changelog.run(),
        cli::Commands::Completion(completion) => completion.run(),

        #[cfg(debug_assertions)]
        cli::Commands::NoOp => Ok(()),
        #[cfg(debug_assertions)]
        cli::Commands::LogTest => log_test(),
    }?;

    Ok(log_guard)
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

#[cfg(debug_assertions)]
fn log_test() -> Result<()> {
    let _error_span = tracing::error_span!("This is an error span").entered();
    tracing::trace!("This is a trace message");
    tracing::debug!("This is a debug message");
    tracing::info!("This is an info message");
    tracing::warn!("This is a warning");
    tracing::error!("This is an error");

    Ok(())
}
