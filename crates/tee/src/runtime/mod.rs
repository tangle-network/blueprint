//! TEE runtime backend abstraction.
//!
//! Provides the [`TeeRuntimeBackend`] trait for TEE lifecycle management
//! and a registry for provider discovery.

pub mod backend;
pub mod direct;
pub mod registry;

pub use backend::{
    TeeDeployRequest, TeeDeploymentHandle, TeeDeploymentStatus, TeePublicKey, TeeRuntimeBackend,
};
pub use registry::BackendRegistry;
