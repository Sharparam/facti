use std::{env, path::PathBuf};

use anyhow::{Context, Result};
use clap::{Args, ValueHint};
use tracing::info;

use crate::project::Project;

#[derive(Args, Debug)]
pub struct DistArgs {
    #[arg(long, env = "FACTI_DIST_PATH", value_hint = ValueHint::FilePath)]
    path: Option<PathBuf>,
}

impl DistArgs {
    pub fn run(&self) -> Result<()> {
        let path = self
            .path
            .to_owned()
            .unwrap_or(env::current_dir().context("Failed to get current directory")?);
        let project = Project::load(&path)?;

        info!(
            "Loaded project ({} v{} by {}) at {}",
            project.mod_info.name,
            project.mod_info.version,
            project.mod_info.author,
            project.path.display()
        );
        info!("Mod source files at {}", project.mod_path.display());

        Ok(())
    }
}
