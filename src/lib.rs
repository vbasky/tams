//! Rust implementation of the Time-Addressable Media Store (TAMS) API.
//!
//! See <https://tams.org/> and <https://github.com/bbc/tams> for the specification.

pub mod models;
pub mod client;
pub mod error;

pub use error::Error;
pub use client::Client;

/// Result type used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;
