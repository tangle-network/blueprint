//! TEE key exchange and sealed secret handoff.
//!
//! Provides the [`TeeAuthService`] background service that manages
//! ephemeral session keys and sealed secret injection.

pub mod protocol;
pub mod service;

pub use service::TeeAuthService;
