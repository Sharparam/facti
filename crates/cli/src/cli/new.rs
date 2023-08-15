use std::{
    env,
    fs::{self, File},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Context, Result};
use clap::Args;
use facti_lib::{version::Version, FactorioVersion, ModInfo};
use tracing::{debug, info, warn};
use url::Url;

use crate::{
    config::Config,
    vcs::{self, Vcs},
};

/// Create a new mod.
#[derive(Args, Debug)]
pub struct NewArgs {
    /// The path to create the new mod at.
    ///
    /// Defaults to current directory if not given.
    #[arg(value_name = "MOD_PATH")]
    pub path: Option<PathBuf>,

    /// Override the mod name that will be set in info.json.
    ///
    /// Default is to take the basename of the directory where you are creating
    /// the mod.
    ///
    /// E.g. if you run `facti new foobar`, the name will be "foobar",
    /// and if you run `facti new my/awesome/thing`, it will be "thing".
    #[arg(long)]
    pub name: Option<String>,

    /// Version to set in info.json.
    #[arg(long, default_value_t = Version::new(0, 1, 0))]
    pub mod_version: Version,

    /// Title to set in info.json.
    ///
    /// Defaults to the internal mod name.
    #[arg(long)]
    pub title: Option<String>,

    /// Value to use for the author field in info.json.
    ///
    /// Defaults to resolving a relevant name from your system,
    /// for example by inspecting your VCS configuration.
    #[arg(long)]
    pub author: Option<String>,

    /// Value to use for the contact field in info.json.
    ///
    /// Defaults to resolving a relevant value from your system,
    /// for example by inspecting your VCS configuration.
    #[arg(long)]
    pub contact: Option<String>,

    /// Homepage to set in info.json.
    #[arg(long)]
    pub homepage: Option<Url>,

    /// Description to set in info.json.
    #[arg(long)]
    pub description: Option<String>,

    /// The Factorio version your mod supports.
    ///
    /// If not specified, it will check your config file for a default value,
    /// and if none exists there, it will fall back to version 0.12.
    #[arg(long)]
    pub factorio_version: Option<FactorioVersion>,
}

impl NewArgs {
    pub fn run(&self, config: &Config) -> Result<()> {
        let path = self.path.to_owned().unwrap_or(env::current_dir()?);
        let name = path
            .file_name()
            .context("Failed to get file_name (basename) of mod path")?
            .to_str()
            .ok_or(anyhow!("Mod folder name is not valid unicode"))?;

        init_dir(&path)?;

        info!("Creating new mod {} at {}", name, path.display());
        vcs::git::Git::init(&path)?;

        let src_path = path.join("src");
        fs::create_dir(&src_path).context("Failed to create src dir inside mod")?;

        let modinfo = create_modinfo(name, self, config).context("Failed to construct modinfo")?;
        let modinfo_path = src_path.join("info.json");
        let modinfo_file = File::create(modinfo_path)
            .context("Failed to create info.json for writing mod info")?;
        serde_json::to_writer_pretty(modinfo_file, &modinfo)
            .context("Failed to write mod info to info.json")?;

        println!("Created new mod at {}", path.display());

        Ok(())
    }
}

fn init_dir(path: &Path) -> Result<()> {
    let exists = path.exists();

    if exists && path.is_file() {
        warn!("Attempted to initialize mod in file {}", path.display());
        bail!("{} is a file", path.display());
    }

    if exists && !is_dir_empty(path)? {
        warn!(
            "Attempted to initialize mod in non-empty directory {}",
            path.display()
        );
        bail!("Directory {} is not empty", path.display());
    }

    if exists {
        debug!(
            "Directory {} already exists and is empty, no need to init",
            path.display()
        );
    } else {
        info!("Creating directory {}", path.display());
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory {}", path.display()))?;
    }

    Ok(())
}

fn is_dir_empty(path: &Path) -> Result<bool> {
    Ok(path
        .read_dir()
        .with_context(|| format!("Failed to read dir {} to check if empty", path.display()))?
        .next()
        .is_none())
}

fn create_modinfo(name: &str, args: &NewArgs, config: &Config) -> Result<ModInfo> {
    let mut builder = ModInfo::builder(
        resolve_name(args, name),
        args.mod_version,
        resolve_title(args, name),
        resolve_author(args, config)?,
    );

    if let Some(c) = resolve_contact(args, config)? {
        builder.contact(c);
    }

    if let Some(homepage) = args.homepage.to_owned() {
        builder.homepage(homepage);
    }

    if let Some(description) = args.description.to_owned() {
        builder.description(description);
    }

    builder.factorio_version(resolve_factorio_version(args, config));

    Ok(builder.build())
}

fn resolve_name<T: Into<String>>(args: &NewArgs, fallback: T) -> String {
    args.name.to_owned().unwrap_or_else(|| fallback.into())
}

fn resolve_title<T: Into<String>>(args: &NewArgs, fallback: T) -> String {
    args.title.to_owned().unwrap_or_else(|| fallback.into())
}

fn resolve_author(args: &NewArgs, config: &Config) -> Result<String> {
    if let Some(name) = &args.author {
        debug!("Author name resolved from CLI args");
        Ok(name.to_owned())
    } else if let Some(name) = &config.mod_defaults.author {
        debug!("Author name resolved from config file");
        Ok(name.to_owned())
    } else if let Some(name) =
        vcs::git::Git::user_name().context("Failed to resolve user name from VCS config")?
    {
        debug!("Author name resolved from VCS config");
        Ok(name)
    } else {
        debug!("Unable to resolve author name, using 'Unknown' as placeholder");
        Ok("Unknown".to_owned())
    }
}

fn resolve_contact(args: &NewArgs, config: &Config) -> Result<Option<String>> {
    if let Some(contact) = &args.contact {
        Ok(Some(contact.to_owned()))
    } else if let Some(contact) = &config.mod_defaults.contact {
        Ok(Some(contact.to_owned()))
    } else if let Some(email) =
        vcs::git::Git::user_email().context("Failed to resolve user email from VCS config")?
    {
        Ok(Some(email.to_owned()))
    } else {
        Ok(None)
    }
}

fn resolve_factorio_version(args: &NewArgs, config: &Config) -> FactorioVersion {
    args.factorio_version
        .or(config.mod_defaults.factorio_version)
        .unwrap_or_default()
}
