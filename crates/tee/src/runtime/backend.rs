//! TEE runtime backend trait.
//!
//! Defines the core lifecycle contract for TEE deployments:
//! deploy, get attestation, cached attestation, public key derivation, stop, and destroy.

use crate::attestation::report::AttestationReport;
use crate::config::{RuntimeLifecyclePolicy, TeeProvider};
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
    /// Additional ports the workload needs exposed beyond the default service port.
    ///
    /// Port mapping behavior varies by backend:
    /// - **Direct**: Docker port publish (host:container mapping)
    /// - **AWS Nitro**: Security group rules on the EC2 instance
    /// - **GCP**: Firewall rules on the Confidential Space VM
    /// - **Azure**: NSG rules on the CVM's NIC
    /// - **Phala**: Compose service port declarations
    ///
    /// Backends that don't fully support port mapping will log a warning
    /// and return an empty `port_mapping` in the handle.
    #[serde(default)]
    pub extra_ports: Vec<u16>,
}

impl TeeDeployRequest {
    /// Create a new deploy request for an image.
    pub fn new(image: impl Into<String>) -> Self {
        Self {
            image: image.into(),
            env: BTreeMap::new(),
            resources: BTreeMap::new(),
            preferred_provider: None,
            extra_ports: Vec::new(),
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

    /// Add extra ports to expose.
    pub fn with_extra_ports(mut self, ports: impl IntoIterator<Item = u16>) -> Self {
        self.extra_ports.extend(ports);
        self
    }
}

/// A TEE-bound public key, used for encrypting sealed secrets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeePublicKey {
    /// Raw public key bytes.
    pub key: Vec<u8>,
    /// Key type (e.g., "x25519", "secp256k1").
    pub key_type: String,
    /// Hex-encoded fingerprint of the key (for display/matching).
    pub fingerprint: String,
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
    /// Cached attestation from provision time, used for idempotent re-submission.
    ///
    /// When a provision is re-submitted (idempotent retry), this cached attestation
    /// is returned instead of an empty report. Without this, re-submission with
    /// `teeRequired=true` would cause on-chain reverts (`MissingTeeAttestation`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cached_attestation: Option<AttestationReport>,
    /// Mapping of requested extra ports to their host-side bindings.
    ///
    /// Keys are the container/workload ports, values are the host-side ports.
    /// Empty if the backend doesn't support port mapping.
    #[serde(default)]
    pub port_mapping: BTreeMap<u16, u16>,
    /// The lifecycle policy for this deployment.
    ///
    /// TEE deployments use `CloudManaged` — the GC/reaper must skip all
    /// Docker-level operations (commit, Hot/Warm/Cold transitions).
    #[serde(default = "default_lifecycle_policy")]
    pub lifecycle_policy: RuntimeLifecyclePolicy,
}

fn default_lifecycle_policy() -> RuntimeLifecyclePolicy {
    RuntimeLifecyclePolicy::CloudManaged
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
///
/// # Idempotent provision
///
/// When a provision is re-submitted, [`TeeRuntimeBackend::cached_attestation`]
/// returns the attestation captured at first provision. This prevents on-chain
/// reverts when the contract requires non-empty attestation.
///
/// # Secret injection
///
/// TEE deployments use sealed secrets exclusively. Container recreation
/// (env-var re-injection) is forbidden because it invalidates attestation.
/// Use [`TeeRuntimeBackend::derive_public_key`] to get the key for encrypting
/// sealed secrets.
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

    /// Return the cached attestation from provision time, if available.
    ///
    /// This is used for idempotent provision re-submission. When a provision
    /// has already completed, calling this returns the original attestation
    /// rather than generating a new one or returning empty.
    fn cached_attestation(
        &self,
        handle: &TeeDeploymentHandle,
    ) -> impl core::future::Future<Output = Result<Option<AttestationReport>, TeeError>> + Send;

    /// Derive the TEE-bound public key for a deployment.
    ///
    /// The returned key is used by clients to encrypt sealed secrets. Whether
    /// failure is fatal depends on [`TeePublicKeyPolicy`](crate::config::TeePublicKeyPolicy)
    /// in the config.
    fn derive_public_key(
        &self,
        handle: &TeeDeploymentHandle,
    ) -> impl core::future::Future<Output = Result<TeePublicKey, TeeError>> + Send;

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
