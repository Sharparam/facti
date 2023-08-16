//! Blocking client API.
//!
//! # Optional
//!
//! This requires the optional `blocking` feature to be enabled.

mod client;
mod error;
mod reqwest;

pub use client::{ApiClient, ApiClientBuilder};
