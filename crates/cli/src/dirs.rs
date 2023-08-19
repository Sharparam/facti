use std::path::PathBuf;

use anyhow::{Context, Result};
use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};

const APP_TLD: &str = "com";
const APP_AUTHOR: &str = "Sharparam";
const APP_NAME: &str = "facti";

pub fn strategy() -> Result<impl AppStrategy> {
    choose_app_strategy(AppStrategyArgs {
        top_level_domain: APP_TLD.to_owned(),
        author: APP_AUTHOR.to_owned(),
        app_name: APP_NAME.to_owned(),
    })
    .context("Failed to create app strategy")
}

pub fn config() -> Result<PathBuf> {
    Ok(strategy()?.config_dir())
}

#[allow(dead_code)]
pub fn data() -> Result<PathBuf> {
    Ok(strategy()?.data_dir())
}

pub fn state() -> Result<PathBuf> {
    let strategy = strategy()?;
    match strategy.state_dir() {
        Some(path) => Ok(path),
        None => Ok(strategy.data_dir()),
    }
}
