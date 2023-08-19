use std::{
    fs::{self, File},
    io::{BufReader, Read, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use facti_lib::FactorioVersion;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use url::Url;

use crate::{dirs, logging::LogLevelFilter};

const CONFIG_FILENAME: &str = "config.toml";

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(alias = "log_level_filter", skip_serializing_if = "Option::is_none")]
    pub log_level_filter: Option<LogLevelFilter>,

    #[serde(default, alias = "factorio_api")]
    pub factorio_api: FactorioApiConfig,

    #[serde(default, alias = "mod_defaults")]
    pub mod_defaults: ModDefaultsConfig,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FactorioApiConfig {
    #[serde(alias = "portal_base_url", skip_serializing_if = "Option::is_none")]
    pub portal_base_url: Option<Url>,

    #[serde(alias = "game_base_url", skip_serializing_if = "Option::is_none")]
    pub game_base_url: Option<Url>,

    #[serde(alias = "api_key", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,

    #[serde(alias = "api_key_file", skip_serializing_if = "Option::is_none")]
    pub api_key_file: Option<PathBuf>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ModDefaultsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,

    #[serde(alias = "factorio_version", skip_serializing_if = "Option::is_none")]
    pub factorio_version: Option<FactorioVersion>,
}

#[derive(Default, Debug)]
pub enum ConfigPath {
    #[default]
    Default,
    Custom(PathBuf),
}

impl Config {
    pub fn default_path() -> Result<PathBuf> {
        let config_dir = dirs::config()?;
        let config_path = config_dir.join(CONFIG_FILENAME);

        debug!("Resolved default config path as {}", config_path.display());

        Ok(config_path)
    }

    pub fn load(path: ConfigPath) -> Result<Config> {
        let config_path = if let ConfigPath::Custom(p) = path {
            debug!("Loading config from specific path");
            p
        } else {
            debug!("Loading config from default path");
            Self::default_path()?
        };
        info!("Loading config from {}", config_path.display());
        let config: Config = if config_path.exists() {
            let file = File::open(config_path).context("Failed to open config file")?;
            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            reader
                .read_to_string(&mut contents)
                .context("Failed to read config file")?;
            toml::from_str(&contents).context("Failed to parse config file")?
        } else {
            Default::default()
        };

        Ok(config)
    }

    #[allow(dead_code)]
    pub fn save(&self, path: ConfigPath) -> Result<()> {
        let config_path = match path {
            ConfigPath::Default => Self::default_path()?,
            ConfigPath::Custom(p) => p,
        };
        let dir = config_path
            .parent()
            .context("Failed to get parent directory of config path")?;
        fs::create_dir_all(dir).context("Failed to create config directory (and parents)")?;
        let contents = toml::to_string_pretty(self).context("Failed to serialize config")?;
        let mut file = File::create(config_path).context("Failed to create config file")?;
        file.write_all(contents.as_bytes())
            .context("Failed to write config to file")
    }
}

impl FactorioApiConfig {
    /// Resolves the API key to use.
    ///
    /// If [`FactorioApiConfig::api_key`] is set, then that value will be
    /// returned.
    ///
    /// If [`FactorioApiConfig::api_key_file`] is set, then the API key will be
    /// read from that file and returned.
    ///
    /// If neither are set, then this function will return [`None`].
    ///
    /// # Errors
    ///
    /// This function can error if there is an issue reading the file specified
    /// in [`FactorioApiConfig::api_key_file`].
    ///
    /// If the file is not set and it's instead reading the API key from
    /// [`FactorioApiConfig::api_key`], then no error can occur.
    pub fn api_key(&self) -> Result<Option<String>> {
        if let Some(api_key) = &self.api_key {
            return Ok(Some(api_key.to_string()));
        }

        if self.api_key_file.is_none() {
            return Ok(None);
        }

        let path = self.api_key_file.as_ref().unwrap();
        let file = File::open(path).context("Failed to open API key file")?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader
            .read_to_string(&mut contents)
            .context("Failed to read API key file")?;
        let api_key = contents.trim();
        Ok(Some(api_key.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parse() {
        let config: Config = toml::from_str(
            r#"
            [factorio-api]
            api-key = "foobar"
            "#,
        )
        .unwrap();

        assert_eq!(config.factorio_api.api_key.unwrap(), "foobar");
    }

    #[test]
    fn test_config_snake_case_parse() {
        let config: Config = toml::from_str(
            r#"
            [factorio_api]
            api_key = "foobar"
            "#,
        )
        .unwrap();

        assert_eq!(config.factorio_api.api_key.unwrap(), "foobar");
    }

    #[test]
    fn test_config_defaults_to_empty() {
        let config: Config = toml::from_str("").unwrap();
        assert!(config.factorio_api.api_key.is_none());
    }
}
