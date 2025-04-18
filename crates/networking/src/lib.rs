#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod behaviours;
pub mod blueprint_protocol;
pub mod discovery;
pub mod error;
pub mod service;
pub mod service_handle;
pub mod types;

#[cfg(test)]
mod tests;

// Make test_utils available for both internal tests and external crates' tests
#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;

pub use blueprint_crypto::KeyType;
pub use service::{NetworkConfig, NetworkEvent, NetworkService};
