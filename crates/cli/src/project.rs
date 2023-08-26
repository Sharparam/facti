use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use facti_lib::ModInfo;
use tracing::{debug, error};

use crate::vcs::{self, Vcs};

#[derive(Debug)]
pub struct Project {
    /// Path to the root of the project, often the root of a VCS repo.
    ///
    /// Can be the same as [`mod_path`][Self::mod_path].
    pub path: PathBuf,

    /// Path to the actual mod files, where `info.json` is.
    ///
    /// Can be the same as [`path`][Self::path].
    pub mod_path: PathBuf,

    /// Path to the `info.json` file for the mod.
    pub mod_info_path: PathBuf,

    /// Contains information about the mod.
    ///
    /// This data is read from the `info.json` file inside [`mod_path`][Self::mod_path].
    pub mod_info: ModInfo,
}

impl Project {
    pub fn load(path: &Path) -> Result<Self> {
        debug!("Loading project from path {}", path.display());
        let (path, mod_path) = resolve_paths(path)?;
        let mod_info_path = mod_path.join("info.json");
        debug!("Loading mod info from {}", mod_info_path.display());
        let mod_info_file = File::open(&mod_info_path).context("Failed to open info.json file")?;
        let mod_info_reader = BufReader::new(mod_info_file);
        let mod_info = serde_json::from_reader(mod_info_reader)
            .context("Failed to deserialize ModInfo from info.json")?;

        Ok(Self {
            path,
            mod_path,
            mod_info_path,
            mod_info,
        })
    }

    pub fn dist_path(&self) -> PathBuf {
        self.path.join("dist")
    }
}

fn resolve_paths(path: &Path) -> Result<(PathBuf, PathBuf)> {
    debug!("Resolving mod directories from {}", path.display());

    if let Some(repo_root) = vcs::git::Git::check(path)? {
        debug!(
            "Path is in a VCS repo ({}), checking for info.json",
            repo_root.display()
        );
        let src_path = repo_root.join("src");
        if path_has_infojson(&src_path) {
            debug!("Found info.json in src sub-dir of repo root");
            return Ok((repo_root, src_path));
        } else if path_has_infojson(&repo_root) {
            debug!("Found info.json in repo root");
            return Ok((repo_root.to_owned(), repo_root));
        };
    }

    let src_dir = path.join("src");
    if path_has_infojson(&src_dir) {
        debug!("Found info.json in src sub-dir");
        return Ok((path.to_owned(), src_dir));
    }

    if path_has_infojson(path) {
        debug!("Found info.json at root");
        return Ok((path.to_owned(), path.to_owned()));
    }

    error!("Could not locate info.json at any supported path");

    bail!(
        "Unable to resolve a supported mod directory from {}",
        path.display()
    );
}

fn path_has_infojson(path: &Path) -> bool {
    path.join("info.json").exists()
}
