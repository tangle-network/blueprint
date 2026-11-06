//! Security modules for blueprint-remote-providers
//!
//! Provides secure credential storage, authentication, and security utilities

pub mod auth;
pub mod encrypted_credentials;
pub mod secure_http_client;
mod ssh_keys;
pub(crate) use ssh_keys::{canonical_public_key, parse_public_key};

pub use encrypted_credentials::{
    EncryptedCloudCredentials, PlaintextCredentials, SecureCredentialManager,
};

pub use secure_http_client::{ApiAuthentication, SecureHttpClient};
