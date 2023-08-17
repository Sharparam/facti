//! [Async][async-book] client API.
//!
//! # Optional but default
//!
//! This requires the `async` feature to be enabled,
//! which is part of the default enabled features.
//!
//! [async-book]: https://rust-lang.github.io/async-book/

pub(crate) mod client;
mod error;
mod reqwest;
