//! TEE runtime backend trait.
//!
//! Defines the core lifecycle contract for TEE deployments:
//! deploy, get attestation, stop, and destroy.

use crate::attestation::report::AttestationReport;
use crate::config::TeeProvider;
use crate::errors::TeeError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A request to deploy a workload in a TEE.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeeDeployRequest {
    /// Container image or binary to deploy.
    pub image: String,
    /// Environment variables to pass to the workload.
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    /// Resource requirements (provider-specific).
    #[serde(default)]
    pub resources: BTreeMap<String, String>,
    /// The preferred TEE provider, if any.
    pub preferred_provider: Option<TeeProvider>,
}

impl TeeDeployRequest {
    /// Create a new deploy request for an image.
    pub fn new(image: impl Into<String>) -> Self {
        Self {
            image: image.into(),
            env: BTreeMap::new(),
            resources: BTreeMap::new(),
            preferred_provider: None,
        }
    }

    /// Add an environment variable.
    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.insert(key.into(), value.into());
        self
    }

    /// Set the preferred provider.
    pub fn with_provider(mut self, provider: TeeProvider) -> Self {
        self.preferred_provider = Some(provider);
        self
    }
}

/// A handle to a deployed TEE workload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeeDeploymentHandle {
    /// Unique deployment identifier.
    pub id: String,
    /// The provider this deployment is running on.
    pub provider: TeeProvider,
    /// Provider-specific metadata needed for lifecycle operations.
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

/// The status of a TEE deployment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TeeDeploymentStatus {
    /// Deployment is being provisioned.
    Provisioning,
    /// Deployment is running and healthy.
    Running,
    /// Deployment is being stopped.
    Stopping,
    /// Deployment has been stopped.
    Stopped,
    /// Deployment has failed.
    Failed,
}

/// Trait for managing TEE runtime lifecycle.
///
/// Implementations handle provider-specific deployment, attestation retrieval,
/// and teardown. This is the core SPI that backend providers implement.
///
/// # Lifecycle
///
/// ```text
/// deploy() → Running → get_attestation() → ... → stop() → destroy()
/// ```
pub trait TeeRuntimeBackend: Send + Sync {
    /// Deploy a workload into a TEE environment.
    fn deploy(
        &self,
        req: TeeDeployRequest,
    ) -> impl core::future::Future<Output = Result<TeeDeploymentHandle, TeeError>> + Send;

    /// Retrieve a fresh attestation report from a running deployment.
    fn get_attestation(
        &self,
        handle: &TeeDeploymentHandle,
    ) -> impl core::future::Future<Output = Result<AttestationReport, TeeError>> + Send;

    /// Get the current status of a deployment.
    fn status(
        &self,
        handle: &TeeDeploymentHandle,
    ) -> impl core::future::Future<Output = Result<TeeDeploymentStatus, TeeError>> + Send;

    /// Gracefully stop a running deployment.
    fn stop(
        &self,
        handle: &TeeDeploymentHandle,
    ) -> impl core::future::Future<Output = Result<(), TeeError>> + Send;

    /// Destroy a deployment and release all resources.
    fn destroy(
        &self,
        handle: &TeeDeploymentHandle,
    ) -> impl core::future::Future<Output = Result<(), TeeError>> + Send;
}
