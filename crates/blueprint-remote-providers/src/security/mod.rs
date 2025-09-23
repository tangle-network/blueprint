//! Security modules for blueprint-remote-providers
//!
//! Provides secure credential storage, authentication, and security utilities

pub mod encrypted_credentials;
pub mod secure_http_client;

pub use encrypted_credentials::{
    EncryptedCloudCredentials, PlaintextCredentials, SecureCredentialManager,
};

pub use secure_http_client::{ApiAuthentication, SecureHttpClient};
