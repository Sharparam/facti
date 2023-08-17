#![doc = include_str!("../README.md")]

pub mod changelog;
pub mod dependency;
pub mod error;
pub mod modinfo;
mod semver;
mod serde;
pub mod version;

pub use modinfo::{ModInfo, ModInfoBuilder};
pub use version::FactorioVersion;
