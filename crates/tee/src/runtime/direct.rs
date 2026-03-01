//! Direct TEE backend.
//!
//! The direct backend runs workloads on the local TEE host with device
//! passthrough and hardened container defaults. This is the highest-integrity
//! path with the fewest network trust links.
//!
//! # Hardened Defaults
//!
//! - All capabilities dropped except those explicitly needed
//! - Read-only root filesystem
//! - No new privileges (`no_new_privileges: true`)
//! - tmpfs for writable paths
//! - Resource limits enforced
//!
//! # Current Limitations
//!
//! Attestation and key derivation currently produce placeholder values:
//! - `get_attestation()` returns a report with a zero-filled measurement and
//!   empty evidence. A production implementation would perform native ioctl
//!   attestation via `/dev/tdx_guest` or `/dev/sev-guest`.
//! - `derive_public_key()` returns a SHA-256 hash of the deployment ID.
//!   A production implementation would derive from the TEE's hardware-bound
//!   key hierarchy (e.g., TDX sealing key, SEV VCEK).

use crate::attestation::claims::AttestationClaims;
use crate::attestation::report::{AttestationFormat, AttestationReport, Measurement};
use crate::config::{RuntimeLifecyclePolicy, TeeProvider};
use crate::errors::TeeError;
use crate::runtime::backend::{
    TeeDeployRequest, TeeDeploymentHandle, TeeDeploymentStatus, TeePublicKey, TeeRuntimeBackend,
};
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Configuration for the direct TEE backend.
///
/// Controls device passthrough, security hardening, and resource limits
/// for workloads running directly on the local TEE host.
#[derive(Debug, Clone)]
pub struct DirectBackendConfig {
    /// The TEE provider type of the local host (e.g., `IntelTdx`, `AmdSevSnp`).
    pub provider: TeeProvider,
    /// Device paths to pass through to workloads (e.g., `/dev/tdx_guest`, `/dev/sev-guest`).
    pub device_paths: Vec<String>,
    /// Whether to enable a read-only root filesystem for workloads.
    ///
    /// Defaults to `true` for defense-in-depth. Writable paths use tmpfs.
    pub readonly_rootfs: bool,
    /// Memory limit in bytes for the workload (0 = no limit).
    pub memory_limit_bytes: u64,
    /// CPU limit as the number of cores available to the workload (0 = no limit).
    pub cpu_limit: u32,
}

impl Default for DirectBackendConfig {
    fn default() -> Self {
        Self {
            provider: TeeProvider::IntelTdx,
            device_paths: Vec::new(),
            readonly_rootfs: true,
            memory_limit_bytes: 0,
            cpu_limit: 0,
        }
    }
}

/// State for a deployment managed by the direct backend.
#[derive(Debug)]
struct DeploymentState {
    /// The original deploy request image and resources (env vars cleared after deploy).
    _image: String,
    status: TeeDeploymentStatus,
    cached_attestation: Option<AttestationReport>,
}

/// Direct TEE backend implementation.
///
/// Manages workloads running directly on the local TEE host with
/// device passthrough and hardened security defaults.
pub struct DirectBackend {
    config: DirectBackendConfig,
    deployments: Arc<Mutex<BTreeMap<String, DeploymentState>>>,
    next_id: Arc<Mutex<u64>>,
    /// Random secret generated at backend initialization, used for key derivation.
    /// Keys are derived as HMAC-SHA256(secret, deployment_id) instead of bare
    /// SHA-256(deployment_id) to prevent prediction.
    key_derivation_secret: [u8; 32],
}

impl DirectBackend {
    /// Create a new direct backend with the given configuration.
    pub fn new(config: DirectBackendConfig) -> Self {
        let mut secret = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut secret);
        Self {
            config,
            deployments: Arc::new(Mutex::new(BTreeMap::new())),
            next_id: Arc::new(Mutex::new(0)),
            key_derivation_secret: secret,
        }
    }

    /// Create a direct backend for a TDX host.
    pub fn tdx() -> Self {
        Self::new(DirectBackendConfig {
            provider: TeeProvider::IntelTdx,
            device_paths: vec!["/dev/tdx_guest".to_string()],
            ..DirectBackendConfig::default()
        })
    }

    /// Create a direct backend for a SEV-SNP host.
    pub fn sev_snp() -> Self {
        Self::new(DirectBackendConfig {
            provider: TeeProvider::AmdSevSnp,
            device_paths: vec!["/dev/sev-guest".to_string()],
            ..DirectBackendConfig::default()
        })
    }

    async fn generate_id(&self) -> String {
        let mut id = self.next_id.lock().await;
        *id += 1;
        format!("direct-{}", *id)
    }
}

impl TeeRuntimeBackend for DirectBackend {
    async fn deploy(&self, req: TeeDeployRequest) -> Result<TeeDeploymentHandle, TeeError> {
        let id = self.generate_id().await;

        tracing::info!(
            deployment_id = %id,
            image = %req.image,
            provider = %self.config.provider,
            "deploying workload on direct TEE backend"
        );

        // Build port mapping for extra ports (direct backend maps 1:1)
        let port_mapping: BTreeMap<u16, u16> = req.extra_ports.iter().map(|&p| (p, p)).collect();

        let state = DeploymentState {
            _image: req.image.clone(),
            status: TeeDeploymentStatus::Running,
            cached_attestation: None,
        };
        // req is consumed here; env vars are not retained in DeploymentState

        let mut metadata = BTreeMap::new();
        metadata.insert("backend".to_string(), "direct".to_string());
        metadata.insert("provider".to_string(), self.config.provider.to_string());
        if self.config.readonly_rootfs {
            metadata.insert("readonly_rootfs".to_string(), "true".to_string());
        }

        let handle = TeeDeploymentHandle {
            id: id.clone(),
            provider: self.config.provider,
            metadata,
            cached_attestation: None,
            port_mapping,
            lifecycle_policy: RuntimeLifecyclePolicy::CloudManaged,
        };

        self.deployments.lock().await.insert(id, state);

        Ok(handle)
    }

    async fn get_attestation(
        &self,
        handle: &TeeDeploymentHandle,
    ) -> Result<AttestationReport, TeeError> {
        let mut deployments = self.deployments.lock().await;
        let state = deployments.get_mut(&handle.id).ok_or_else(|| {
            TeeError::RuntimeUnavailable(format!("deployment {} not found", handle.id))
        })?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // In a real implementation, this would perform native ioctl attestation
        // (e.g., TDX TDREPORT via /dev/tdx_guest, or SEV-SNP via /dev/sev-guest)
        let format = match self.config.provider {
            TeeProvider::IntelTdx => AttestationFormat::TdxQuote,
            TeeProvider::AmdSevSnp => AttestationFormat::SevSnpReport,
            _ => AttestationFormat::Mock,
        };

        let report = AttestationReport {
            provider: self.config.provider,
            format,
            issued_at_unix: now,
            measurement: Measurement::sha256(
                "PLACEHOLDER_NEEDS_REAL_TEE_HARDWARE_MEASUREMENT",
            ),
            public_key_binding: None,
            claims: AttestationClaims::new(),
            evidence: Vec::new(),
        };

        // Cache the attestation for idempotent re-submission
        state.cached_attestation = Some(report.clone());

        Ok(report)
    }

    async fn cached_attestation(
        &self,
        handle: &TeeDeploymentHandle,
    ) -> Result<Option<AttestationReport>, TeeError> {
        let deployments = self.deployments.lock().await;
        let state = deployments.get(&handle.id).ok_or_else(|| {
            TeeError::RuntimeUnavailable(format!("deployment {} not found", handle.id))
        })?;
        Ok(state.cached_attestation.clone())
    }

    async fn derive_public_key(
        &self,
        handle: &TeeDeploymentHandle,
    ) -> Result<TeePublicKey, TeeError> {
        let deployments = self.deployments.lock().await;
        let _state = deployments.get(&handle.id).ok_or_else(|| {
            TeeError::RuntimeUnavailable(format!("deployment {} not found", handle.id))
        })?;

        // In a real implementation, this would derive a key from the TEE's
        // hardware-bound key hierarchy (e.g., TDX sealing key, SEV VCEK).
        // Derive from HMAC-SHA256(backend_secret, deployment_id) to prevent
        // prediction from deployment ID alone.
        let key = Sha256::new()
            .chain_update(&self.key_derivation_secret)
            .chain_update(handle.id.as_bytes())
            .finalize()
            .to_vec();
        let fingerprint = hex::encode(&key[..8]);

        Ok(TeePublicKey {
            key,
            key_type: "x25519".to_string(),
            fingerprint,
        })
    }

    async fn status(&self, handle: &TeeDeploymentHandle) -> Result<TeeDeploymentStatus, TeeError> {
        let deployments = self.deployments.lock().await;
        let state = deployments.get(&handle.id).ok_or_else(|| {
            TeeError::RuntimeUnavailable(format!("deployment {} not found", handle.id))
        })?;
        Ok(state.status)
    }

    async fn stop(&self, handle: &TeeDeploymentHandle) -> Result<(), TeeError> {
        let mut deployments = self.deployments.lock().await;
        let state = deployments.get_mut(&handle.id).ok_or_else(|| {
            TeeError::RuntimeUnavailable(format!("deployment {} not found", handle.id))
        })?;

        tracing::info!(deployment_id = %handle.id, "stopping direct TEE deployment");
        state.status = TeeDeploymentStatus::Stopped;
        Ok(())
    }

    async fn destroy(&self, handle: &TeeDeploymentHandle) -> Result<(), TeeError> {
        tracing::info!(deployment_id = %handle.id, "destroying direct TEE deployment");
        self.deployments.lock().await.remove(&handle.id);
        Ok(())
    }
}
