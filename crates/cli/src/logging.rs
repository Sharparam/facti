use std::{fmt::Display, io, str::FromStr};

use anyhow::Context;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use tracing_subscriber::FmtSubscriber;

#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize, ValueEnum)]
pub enum LogLevelFilter {
    Off,

    #[default]
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

pub fn init_logging<T: Into<LogLevelFilter>>(filter: T) -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_writer(io::stderr)
        .with_max_level(filter.into())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .context("failed to set global default subscriber")?;

    Ok(())
}

impl From<LogLevelFilter> for tracing::metadata::LevelFilter {
    fn from(level: LogLevelFilter) -> Self {
        use LogLevelFilter::*;
        match level {
            Off => Self::OFF,
            Error => Self::ERROR,
            Warn => Self::WARN,
            Info => Self::INFO,
            Debug => Self::DEBUG,
            Trace => Self::TRACE,
        }
    }
}

impl From<tracing::metadata::LevelFilter> for LogLevelFilter {
    fn from(level: tracing::metadata::LevelFilter) -> Self {
        use tracing::metadata::LevelFilter;
        use LogLevelFilter::*;
        match level {
            LevelFilter::OFF => Off,
            LevelFilter::ERROR => Error,
            LevelFilter::WARN => Warn,
            LevelFilter::INFO => Info,
            LevelFilter::DEBUG => Debug,
            LevelFilter::TRACE => Trace,
        }
    }
}

impl From<tracing::Level> for LogLevelFilter {
    fn from(level: tracing::Level) -> Self {
        use tracing::Level;
        use LogLevelFilter::*;
        match level {
            Level::ERROR => Error,
            Level::WARN => Warn,
            Level::INFO => Info,
            Level::DEBUG => Debug,
            Level::TRACE => Trace,
        }
    }
}

impl From<Option<tracing::Level>> for LogLevelFilter {
    fn from(level: Option<tracing::Level>) -> Self {
        level.map_or(LogLevelFilter::Off, LogLevelFilter::from)
    }
}

impl From<LogLevelFilter> for Option<tracing::Level> {
    fn from(level: LogLevelFilter) -> Self {
        use tracing::Level;
        use LogLevelFilter::*;
        match level {
            Off => None,
            Error => Some(Level::ERROR),
            Warn => Some(Level::WARN),
            Info => Some(Level::INFO),
            Debug => Some(Level::DEBUG),
            Trace => Some(Level::TRACE),
        }
    }
}

impl Display for LogLevelFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use LogLevelFilter::*;
        match self {
            Off => f.write_str("OFF"),
            Error => f.write_str("ERROR"),
            Warn => f.write_str("WARN"),
            Info => f.write_str("INFO"),
            Debug => f.write_str("DEBUG"),
            Trace => f.write_str("TRACE"),
        }
    }
}

impl FromStr for LogLevelFilter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use anyhow::anyhow;
        use LogLevelFilter::*;

        s.parse::<usize>()
            .ok()
            .and_then(|n| match n {
                0 => Some(Off),
                1 => Some(Error),
                2 => Some(Warn),
                3 => Some(Info),
                4 => Some(Debug),
                5 => Some(Trace),
                _ => None,
            })
            .or_else(|| match s {
                "" => Some(Default::default()),
                s if s.eq_ignore_ascii_case("e") => Some(Error),
                s if s.eq_ignore_ascii_case("err") => Some(Error),
                s if s.eq_ignore_ascii_case("error") => Some(Error),
                s if s.eq_ignore_ascii_case("w") => Some(Warn),
                s if s.eq_ignore_ascii_case("warn") => Some(Warn),
                s if s.eq_ignore_ascii_case("i") => Some(Info),
                s if s.eq_ignore_ascii_case("inf") => Some(Info),
                s if s.eq_ignore_ascii_case("info") => Some(Info),
                s if s.eq_ignore_ascii_case("d") => Some(Debug),
                s if s.eq_ignore_ascii_case("dbg") => Some(Debug),
                s if s.eq_ignore_ascii_case("debug") => Some(Debug),
                s if s.eq_ignore_ascii_case("t") => Some(Trace),
                s if s.eq_ignore_ascii_case("trace") => Some(Trace),
                s if s.eq_ignore_ascii_case("o") => Some(Off),
                s if s.eq_ignore_ascii_case("off") => Some(Off),
                s if s.eq_ignore_ascii_case("disable") => Some(Off),
                s if s.eq_ignore_ascii_case("disabled") => Some(Off),
                s if s.eq_ignore_ascii_case("none") => Some(Off),
                _ => None,
            })
            .ok_or(anyhow!("invalid log level: {}", s))
    }
}
