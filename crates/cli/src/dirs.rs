use std::path::PathBuf;

use anyhow::{Context, Result};
use directories::ProjectDirs;

const APP_QUALIFIER: &str = "com";
const APP_ORG: &str = "Sharparam";
const APP_NAME: &str = "facti";

pub fn config() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from(APP_QUALIFIER, APP_ORG, APP_NAME)
        .context("Failed to resolve project dirs")?;
    let path = proj_dirs.config_dir();
    Ok(path.to_owned())
}

pub fn data_local() -> Result<PathBuf> {
    Ok(ProjectDirs::from(APP_QUALIFIER, APP_ORG, APP_NAME)
        .context("Failed to resolve project dirs")?
        .data_local_dir()
        .to_owned())
}
