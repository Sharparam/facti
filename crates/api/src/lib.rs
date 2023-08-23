#![doc = include_str!("../README.md")]
//! ## Features
//!
//! ### Default
//!
//! - **`async`:** Enables usage of the async client ([`ApiClient`], [`ApiClientBuilder`]).
//!
//! ### Optional
//!
//! - **`blocking`:** Enables the [`blocking`] module, which provides a blocking client.
//!

#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod client;

#[cfg(feature = "async")]
mod r#async;
#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub mod blocking;
pub mod data;
pub mod error;
mod reqwest;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub use r#async::client::{ApiClient, ApiClientBuilder};
use url::Url;

/// The default base URL for the Factorio mod portal (non-API resources).
pub const DEFAULT_PORTAL_BASE_URL: &str = "https://mods.factorio.com/";

/// The default base URL for the Factorio mod portal API.
pub const DEFAULT_PORTAL_API_BASE_URL: &str = "https://mods.factorio.com/api/";

/// The default base URL for the Factorio game API.
pub const DEFAULT_GAME_BASE_URL: &str = "https://factorio.com/api/";

pub struct FactorioUrls {
    pub portal_base_url: Url,
    pub portal_api_base_url: Url,
    pub game_base_url: Url,
}

impl FactorioUrls {
    pub fn new() -> Self {
        Self {
            portal_base_url: Url::parse(DEFAULT_PORTAL_BASE_URL).unwrap(),
            portal_api_base_url: Url::parse(DEFAULT_PORTAL_API_BASE_URL).unwrap(),
            game_base_url: Url::parse(DEFAULT_GAME_BASE_URL).unwrap(),
        }
    }

    pub fn portal(&self, path: &str) -> Result<Url, url::ParseError> {
        self.portal_base_url.join(path)
    }

    pub fn portal_api(&self, path: &str) -> Result<Url, url::ParseError> {
        self.portal_api_base_url.join(path)
    }

    pub fn game(&self, path: &str) -> Result<Url, url::ParseError> {
        self.game_base_url.join(path)
    }
}

impl Default for FactorioUrls {
    fn default() -> Self {
        Self::new()
    }
}
