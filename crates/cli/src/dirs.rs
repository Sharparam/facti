use std::path::PathBuf;

use anyhow::{Context, Result};
use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};

const APP_QUALIFIER: &str = "com";
const APP_ORG: &str = "Sharparam";
const APP_NAME: &str = "facti";

macro_rules! strategy {
    () => {
        choose_app_strategy(AppStrategyArgs {
            top_level_domain: APP_QUALIFIER.to_owned(),
            author: APP_ORG.to_owned(),
            app_name: APP_NAME.to_owned(),
        })
        .context("Failed to create app strategy")
    };
}

pub fn config() -> Result<PathBuf> {
    Ok(strategy!()?.config_dir())
}

pub fn data() -> Result<PathBuf> {
    Ok(strategy!()?.data_dir())
}

pub fn state() -> Result<PathBuf> {
    let strategy = strategy!()?;
    match strategy.state_dir() {
        Some(path) => Ok(path),
        None => Ok(strategy.data_dir()),
    }
}
