use std::path::{Path, PathBuf};

use anyhow::Result;

pub mod git;

pub trait Vcs {
    /// Initializes a new VCS repository at the given path.
    fn init(path: &Path) -> Result<()>;

    /// Checks if the given path is part of a VCS repository.
    fn check(path: &Path) -> Result<Option<PathBuf>>;

    /// Resolves the configured user name, if set.
    fn user_name() -> Result<Option<String>>;

    /// Resolves the configured user email, if set.
    fn user_email() -> Result<Option<String>>;
}
