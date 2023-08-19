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

/// The default base URL for the Factorio mod portal API.
pub const DEFAULT_PORTAL_BASE_URL: &str = "https://mods.factorio.com/api/";

/// The default base URL for the Factorio game API.
pub const DEFAULT_GAME_BASE_URL: &str = "https://factorio.com/api/";
