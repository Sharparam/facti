use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint};
use url::Url;

use crate::logging::LogLevelFilter;

use self::{completion::CompletionArgs, portal::PortalArgs, verbose::Verbosity};

pub mod completion;
pub mod portal;
mod verbose;

// const LOG_LEVEL_VALS: &[&str] = &[
//     "error", "err", "e", "warning", "warn", "w", "info", "inf", "i", "debug", "dbg", "d", "trace",
//     "t", "0", "1", "2", "3", "4", "5",
// ];

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: Verbosity,

    /// Specifies desired logging level.
    ///
    /// If this option is specified, the verbosity (-v) and quiet (-q) flags
    /// will be ignored.
    #[arg(short, long, global = true)]
    pub log_level: Option<LogLevelFilter>,

    /// Override config file path.
    ///
    /// When specified, will use this config file instead of looking in the
    /// default locations.
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub config: Option<PathBuf>,

    /// Set API key to use.
    #[arg(short = 'k', long, global = true, value_hint = ValueHint::Other)]
    pub api_key: Option<String>,

    /// Override the base URL to the Factorio REST API.
    ///
    /// The default base URL is !!!TODO!!!
    #[arg(long, value_hint = ValueHint::Url)]
    pub base_url: Option<Url>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Portal(PortalArgs),
    Completion(CompletionArgs),
}

impl Cli {
    pub fn log_level_filter(&self) -> Option<LogLevelFilter> {
        if let Some(ll) = self.log_level {
            return Some(ll);
        }
        match self.verbose.is_given() {
            true => Some(self.verbose.log_level_filter().into()),
            false => None,
        }
    }
}
