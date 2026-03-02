//! TEE runtime backend abstraction.
//!
//! Provides the [`TeeRuntimeBackend`] trait for TEE lifecycle management
//! and a registry for provider discovery.

pub mod backend;
pub mod direct;
pub mod registry;

#[cfg(feature = "aws-nitro")]
pub mod aws_nitro;

#[cfg(feature = "azure-snp")]
pub mod azure_skr;

#[cfg(feature = "gcp-confidential")]
pub mod gcp_confidential;

pub use backend::{
    TeeDeployRequest, TeeDeploymentHandle, TeeDeploymentStatus, TeePublicKey, TeeRuntimeBackend,
};
pub use registry::BackendRegistry;
