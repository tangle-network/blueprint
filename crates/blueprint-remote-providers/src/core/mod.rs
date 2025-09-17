//! Core types and utilities for blueprint remote providers

pub mod error;
pub mod remote;
pub mod resources;

#[cfg(test)]
pub mod test_utils;

// Re-export commonly used items
pub use error::{Error, Result};
pub use remote::{CloudProvider, RemoteClusterManager};
pub use resources::ResourceSpec;