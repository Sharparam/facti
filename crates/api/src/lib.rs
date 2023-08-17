#![doc = include_str!("../README.md")]
//! ## Features
//!
//! - **`blocking`:** Enables the [`blocking`] module, which provides a blocking client.
//!

#[cfg(feature = "blocking")]
pub mod blocking;
pub mod data;
pub mod error;
mod reqwest;

/// The default base URL for the Factorio mod portal API.
pub const DEFAULT_BASE_URL: &str = "https://mods.factorio.com/api/";
