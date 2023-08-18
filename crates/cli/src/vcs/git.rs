use std::path::PathBuf;

use super::Vcs;
use anyhow::{bail, Context, Result};
use git2::Repository;
use tracing::{debug, info};

pub struct Git;

impl Vcs for Git {
    fn init(path: &std::path::Path) -> Result<()> {
        if let Some(repo_path) =
            Self::check(path).context("Check for existing Git repo failed inside init")?
        {
            bail!(
                "{} is already part of a Git repo ({})",
                path.display(),
                repo_path.display()
            );
        }

        Repository::init(path)?;

        info!("Initialized Git repository at {}", path.display());

        Ok(())
    }

    fn check(path: &std::path::Path) -> Result<Option<PathBuf>> {
        match Repository::discover(path) {
            Ok(repo) => {
                debug!(
                    "{} is part of a git repo ({})",
                    path.display(),
                    repo.path().display()
                );

                let git_dir = repo.path();
                let repo_dir = if let Some(dir) = git_dir.parent() {
                    dir
                } else {
                    git_dir
                };

                Ok(Some(repo_dir.to_owned()))
            }
            Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn user_name() -> Result<Option<String>> {
        let config =
            git2::Config::open_default().context("Failed opening Git config to read user.name")?;
        match config.get_string("user.name") {
            Ok(name) => Ok(Some(name)),
            Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn user_email() -> Result<Option<String>> {
        let config =
            git2::Config::open_default().context("Failed opening Git config to read user.email")?;
        match config.get_string("user.email") {
            Ok(email) => Ok(Some(email)),
            Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
