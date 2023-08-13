use std::io;

use anyhow::Context;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn init_logging(level: Level) -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_writer(io::stderr)
        .with_max_level(level)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .context("failed to set global default subscriber")?;

    Ok(())
}
