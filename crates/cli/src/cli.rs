use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint};
use url::Url;

use crate::logging::LogLevelFilter;

use self::{completion::CompletionArgs, new::NewArgs, portal::PortalArgs, verbose::Verbosity};

pub mod completion;
pub mod new;
pub mod portal;
mod verbose;

const ENV_CONFIG_PATH: &str = "FACTI_CONFIG";
const ENV_LOG_LEVEL: &str = "FACTI_LOG_LEVEL";
const ENV_API_KEY: &str = "FACTI_API_KEY";
const ENV_API_KEY_FILE: &str = "FACTI_API_KEY_FILE";
const ENV_BASE_URL: &str = "FACTI_BASE_URL";

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: Verbosity,

    /// Specifies desired logging level.
    ///
    /// If this option is specified, the verbosity (-v) and quiet (-q) flags
    /// will be ignored.
    #[arg(
        short,
        long,
        env = ENV_LOG_LEVEL,
        alias = "log-level-filter",
        global = true
    )]
    pub log_level: Option<LogLevelFilter>,

    /// Override config file path.
    ///
    /// When specified, will use this config file instead of looking in the
    /// default locations.
    #[arg(short, long, env = ENV_CONFIG_PATH, value_hint = ValueHint::FilePath)]
    pub config: Option<PathBuf>,

    /// Set API key to use.
    #[arg(short = 'k', long, env = ENV_API_KEY, value_hint = ValueHint::Other)]
    pub api_key: Option<String>,

    /// Read API key from stdin.
    ///
    /// This can be useful as a way to avoid having your API key end up
    /// in your shell history.
    #[arg(long, conflicts_with_all = &["api_key", "api_key_file"])]
    pub api_key_stdin: bool,

    /// Read API key from file.
    ///
    /// The specified file should contain nothing but the API key.
    ///
    /// This can be useful as a way to avoid having your API key end up
    /// in your shell history.
    #[arg(long, env = ENV_API_KEY_FILE, value_hint = ValueHint::FilePath, conflicts_with_all = &["api_key", "api_key_stdin"])]
    pub api_key_file: Option<PathBuf>,

    /// Override the base URL to the Factorio REST API.
    ///
    /// The default base URL is https://mods.factorio.com/api/
    #[arg(long, env = ENV_BASE_URL, value_hint = ValueHint::Url)]
    pub base_url: Option<Url>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Portal(PortalArgs),

    #[command(visible_alias = "init")]
    New(Box<NewArgs>),

    Completion(CompletionArgs),

    /// Do nothing.
    ///
    /// Used for debugging.
    #[cfg(debug_assertions)]
    #[command(name = "noop", visible_alias = "nop")]
    NoOp,
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
