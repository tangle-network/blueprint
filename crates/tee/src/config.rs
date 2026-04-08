//! TEE configuration types.
//!
//! Provides the core configuration model for TEE integration with the Blueprint runner.

use crate::errors::TeeError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The operational mode for TEE integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TeeMode {
    /// Probe for TEE hardware at startup.
    /// If detected, activate TEE (equivalent to Direct). Otherwise, run without TEE.
    #[default]
    Auto,
    /// TEE is disabled; no TEE operations are performed.
    Disabled,
    /// The runner itself is executing inside a TEE.
    /// Device passthrough, hardened defaults, native attestation.
    Direct,
    /// The runner provisions workloads in remote cloud TEE instances.
    Remote,
    /// Selected jobs/services run in TEE; others run normally.
    Hybrid,
}

/// Whether TEE is required or merely preferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TeeRequirement {
    /// TEE is preferred but the system degrades gracefully without it.
    #[default]
    Preferred,
    /// TEE is mandatory; fail closed if unavailable.
    Required,
}

/// Supported TEE hardware/cloud providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TeeProvider {
    /// AWS Nitro Enclaves.
    AwsNitro,
    /// Azure SEV-SNP / Confidential VMs with SKR.
    AzureSnp,
    /// Google Cloud Confidential Space.
    GcpConfidential,
    /// Intel Trust Domain Extensions (TDX).
    IntelTdx,
    /// AMD Secure Encrypted Virtualization - Secure Nested Paging.
    AmdSevSnp,
}

impl core::fmt::Display for TeeProvider {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AwsNitro => write!(f, "aws_nitro"),
            Self::AzureSnp => write!(f, "azure_snp"),
            Self::GcpConfidential => write!(f, "gcp_confidential"),
            Self::IntelTdx => write!(f, "intel_tdx"),
            Self::AmdSevSnp => write!(f, "amd_sev_snp"),
        }
    }
}

/// Selector for which TEE providers are acceptable.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TeeProviderSelector {
    /// Accept any available provider.
    Any,
    /// Accept only providers from this allowlist.
    AllowList(Vec<TeeProvider>),
}

impl Default for TeeProviderSelector {
    fn default() -> Self {
        Self::Any
    }
}

impl TeeProviderSelector {
    /// Check whether a provider is accepted by this selector.
    pub fn accepts(&self, provider: TeeProvider) -> bool {
        match self {
            Self::Any => true,
            Self::AllowList(providers) => providers.contains(&provider),
        }
    }
}

/// Declares that a blueprint requires or prefers TEE execution.
///
/// This struct is embedded in blueprint metadata/manifests and is read by the
/// blueprint manager at deploy time to route the workload to an appropriate
/// TEE-capable host. Without this, the manager has no way to know whether a
/// blueprint binary needs TEE before starting the runner.
///
/// # Examples
///
/// ```rust
/// use blueprint_tee::config::{TeeProviderSelector, TeeRequirement, TeeRequirements};
///
/// let requirements = TeeRequirements {
///     requirement: TeeRequirement::Required,
///     providers: TeeProviderSelector::Any,
///     min_attestation_age_secs: Some(3600),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeeRequirements {
    /// Whether TEE is required (fail if unavailable) or preferred (degrade gracefully).
    pub requirement: TeeRequirement,
    /// Acceptable providers. If `Any`, the manager chooses the best available.
    #[serde(default)]
    pub providers: TeeProviderSelector,
    /// Maximum acceptable age of attestation reports, in seconds.
    /// If `None`, the manager uses its own default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_attestation_age_secs: Option<u64>,
}

impl Default for TeeRequirements {
    fn default() -> Self {
        Self {
            requirement: TeeRequirement::Preferred,
            providers: TeeProviderSelector::Any,
            min_attestation_age_secs: None,
        }
    }
}

impl TeeRequirements {
    /// Create requirements that mandate TEE execution.
    pub fn required() -> Self {
        Self {
            requirement: TeeRequirement::Required,
            ..Self::default()
        }
    }

    /// Create requirements that prefer but don't mandate TEE execution.
    pub fn preferred() -> Self {
        Self::default()
    }

    /// Returns true if TEE is mandatory.
    pub fn is_required(&self) -> bool {
        self.requirement == TeeRequirement::Required
    }
}

/// Lifecycle policy for deployments, used by the manager's GC/reaper.
///
/// TEE deployments have a fundamentally different lifecycle than Docker containers:
/// - No Docker commit (there is no container to commit)
/// - No Hot/Warm/Cold tier transitions (cloud-managed VMs, not containers)
/// - Cleanup means cloud resource teardown (VM deletion, billing stop), not container removal
///
/// The manager's GC/reaper must consult this policy before attempting any
/// container-level operations on a deployment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeLifecyclePolicy {
    /// Standard container lifecycle: Docker commit, Hot/Warm/Cold transitions, GC.
    Container,
    /// Cloud-managed TEE lifecycle: no container ops, teardown via provider API.
    /// The GC/reaper must skip all Docker-level operations for these deployments.
    CloudManaged,
}

/// How secrets may be injected into a deployment.
///
/// For TEE deployments, env-var injection via container recreation is forbidden
/// because it invalidates attestation, breaks sealed secrets, and loses the
/// on-chain deployment ID. Sealed secrets via the key-exchange flow are the
/// only supported path.
///
/// This is enforced at the type level: a `TeeConfig` with any enabled TEE mode
/// always uses `SealedOnly`, preventing accidental use of the container
/// recreation path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretInjectionPolicy {
    /// Secrets may be injected via env vars (container recreation) or sealed secrets.
    /// Only valid for non-TEE (container) deployments.
    EnvOrSealed,
    /// Secrets may only be injected via the sealed-secret key-exchange flow.
    /// Container recreation is forbidden. This is mandatory for all TEE deployments.
    SealedOnly,
}

/// Policy for TEE public key derivation failure.
///
/// When `derive_public_key` fails on a backend, this controls whether the
/// deployment should be considered failed or can proceed in degraded mode
/// (without sealed-secret support).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TeePublicKeyPolicy {
    /// Public key derivation is required; failure is fatal.
    #[default]
    Required,
    /// Public key derivation is optional; failure logs a warning and proceeds.
    Optional,
}

/// Attestation freshness policy.
///
/// Currently only `ProvisionTimeOnly` is supported: attestation is captured once
/// at provision and its hash is stored on-chain. If the enclave reboots, the
/// on-chain hash becomes stale but is not automatically updated. Suitable for
/// long-running enclaves with stable workloads.
///
/// # Future work
///
/// Periodic refresh (re-attest on a timer and update the on-chain hash) is
/// planned but requires provider SDK integration for live re-attestation and
/// on-chain transaction submission. Track progress in the TEE roadmap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AttestationFreshnessPolicy {
    /// Attestation is captured once at provision time. The on-chain hash is
    /// never updated. This is the default and matches the sandbox blueprint's
    /// current behavior.
    #[default]
    ProvisionTimeOnly,
}

/// Where hybrid mode reads its TEE routing decisions from.
///
/// Currently only `PolicyFile` is supported: routing decisions come from a
/// local file that maps job IDs or service names to TEE/non-TEE execution.
///
/// # Future work
///
/// Contract-driven routing (reading `teeRequired` from the on-chain contract)
/// is planned but requires contract integration and ABI bindings. This would
/// be the recommended production approach to avoid config drift. Track progress
/// in the TEE roadmap.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HybridRoutingSource {
    /// Read routing policy from a local file. Maps job IDs or service names
    /// to TEE/non-TEE execution targets.
    PolicyFile(PathBuf),
}

impl Default for HybridRoutingSource {
    fn default() -> Self {
        Self::PolicyFile(PathBuf::from("/etc/tee/routing.json"))
    }
}

/// Configuration for the key exchange subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeeKeyExchangeConfig {
    /// Maximum time-to-live for ephemeral session keys, in seconds.
    #[serde(default = "default_session_ttl_secs")]
    pub session_ttl_secs: u64,
    /// Maximum number of concurrent key exchange sessions.
    #[serde(default = "default_max_sessions")]
    pub max_sessions: usize,
    /// Whether to verify attestation against the on-chain hash during key exchange.
    ///
    /// When enabled, the key exchange flow performs dual verification:
    /// 1. Local evidence check: is this a real TEE with the right measurement?
    /// 2. On-chain hash comparison: does this attestation match the hash submitted
    ///    at provision time (`keccak256(attestationJsonBytes)` stored in the contract)?
    ///
    /// This prevents a compromised operator from substituting a different TEE's
    /// attestation during key exchange.
    #[serde(default)]
    pub on_chain_verification: bool,
}

fn default_session_ttl_secs() -> u64 {
    300
}

fn default_max_sessions() -> usize {
    64
}

impl Default for TeeKeyExchangeConfig {
    fn default() -> Self {
        Self {
            session_ttl_secs: default_session_ttl_secs(),
            max_sessions: default_max_sessions(),
            on_chain_verification: false,
        }
    }
}

/// Top-level TEE configuration.
///
/// Use [`TeeConfig::builder()`] to construct a configuration.
///
/// # Examples
///
/// ```rust
/// use blueprint_tee::{TeeConfig, TeeMode, TeeRequirement};
///
/// let config = TeeConfig::builder()
///     .requirement(TeeRequirement::Required)
///     .mode(TeeMode::Direct)
///     .build()
///     .expect("valid config");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "TeeConfigRaw")]
pub struct TeeConfig {
    /// Whether TEE is required or preferred.
    pub requirement: TeeRequirement,
    /// The operational mode.
    pub mode: TeeMode,
    /// Which providers are acceptable.
    pub provider_selector: TeeProviderSelector,
    /// Key exchange configuration.
    pub key_exchange: TeeKeyExchangeConfig,
    /// Maximum age of attestation reports in seconds before they are considered stale.
    #[serde(default = "default_max_attestation_age_secs")]
    pub max_attestation_age_secs: u64,
    /// How secrets are injected into TEE deployments.
    ///
    /// Automatically set to `SealedOnly` when TEE is enabled. Container
    /// recreation (env-var injection) is forbidden for TEE deployments because
    /// it invalidates attestation and breaks sealed secrets.
    #[serde(default)]
    pub secret_injection: SecretInjectionPolicy,
    /// Attestation freshness policy.
    #[serde(default)]
    pub attestation_freshness: AttestationFreshnessPolicy,
    /// Policy for public key derivation failure.
    #[serde(default)]
    pub public_key_policy: TeePublicKeyPolicy,
    /// Source for hybrid routing decisions (only used when `mode` is `Hybrid`).
    #[serde(default)]
    pub hybrid_routing_source: HybridRoutingSource,
}

fn default_max_attestation_age_secs() -> u64 {
    3600
}

impl Default for SecretInjectionPolicy {
    fn default() -> Self {
        Self::EnvOrSealed
    }
}

impl Default for TeeConfig {
    fn default() -> Self {
        Self {
            requirement: TeeRequirement::default(),
            mode: TeeMode::default(),
            provider_selector: TeeProviderSelector::default(),
            key_exchange: TeeKeyExchangeConfig::default(),
            max_attestation_age_secs: default_max_attestation_age_secs(),
            secret_injection: SecretInjectionPolicy::default(),
            attestation_freshness: AttestationFreshnessPolicy::default(),
            public_key_policy: TeePublicKeyPolicy::default(),
            hybrid_routing_source: HybridRoutingSource::default(),
        }
    }
}

impl TeeConfig {
    /// Create a new builder for `TeeConfig`.
    pub fn builder() -> TeeConfigBuilder {
        TeeConfigBuilder::default()
    }

    /// Returns true if TEE is enabled (mode is not `Disabled`).
    pub fn is_enabled(&self) -> bool {
        self.mode != TeeMode::Disabled
    }

    /// Returns the lifecycle policy for deployments under this config.
    ///
    /// TEE deployments use `CloudManaged` — the GC/reaper must skip all
    /// container-level operations (Docker commit, Hot/Warm/Cold transitions).
    pub fn lifecycle_policy(&self) -> RuntimeLifecyclePolicy {
        if self.is_enabled() {
            RuntimeLifecyclePolicy::CloudManaged
        } else {
            RuntimeLifecyclePolicy::Container
        }
    }

    /// Validate invariants that the builder enforces.
    ///
    /// Called automatically during deserialization via `#[serde(try_from)]`
    /// to ensure configs loaded from JSON/TOML satisfy the same invariants
    /// as configs produced by [`TeeConfigBuilder::build`].
    pub fn validate(&self) -> Result<(), TeeError> {
        if self.requirement == TeeRequirement::Required && self.mode == TeeMode::Disabled {
            return Err(TeeError::Config(
                "TEE requirement is Required but mode is Disabled".to_string(),
            ));
        }

        // TEE-enabled configs must use SealedOnly (Auto is exempt — resolved at runtime)
        if !matches!(self.mode, TeeMode::Disabled | TeeMode::Auto)
            && self.secret_injection != SecretInjectionPolicy::SealedOnly
        {
            return Err(TeeError::Config(
                "TEE-enabled configs must use SealedOnly secret injection".to_string(),
            ));
        }

        Ok(())
    }
}

/// Raw deserialization target for [`TeeConfig`], used with `#[serde(try_from)]`
/// to enforce builder invariants on deserialized configs.
#[derive(Deserialize)]
struct TeeConfigRaw {
    requirement: TeeRequirement,
    mode: TeeMode,
    provider_selector: TeeProviderSelector,
    key_exchange: TeeKeyExchangeConfig,
    #[serde(default = "default_max_attestation_age_secs")]
    max_attestation_age_secs: u64,
    #[serde(default)]
    secret_injection: SecretInjectionPolicy,
    #[serde(default)]
    attestation_freshness: AttestationFreshnessPolicy,
    #[serde(default)]
    public_key_policy: TeePublicKeyPolicy,
    #[serde(default)]
    hybrid_routing_source: HybridRoutingSource,
}

impl TryFrom<TeeConfigRaw> for TeeConfig {
    type Error = TeeError;

    fn try_from(raw: TeeConfigRaw) -> Result<Self, Self::Error> {
        let config = TeeConfig {
            requirement: raw.requirement,
            mode: raw.mode,
            provider_selector: raw.provider_selector,
            key_exchange: raw.key_exchange,
            max_attestation_age_secs: raw.max_attestation_age_secs,
            secret_injection: raw.secret_injection,
            attestation_freshness: raw.attestation_freshness,
            public_key_policy: raw.public_key_policy,
            hybrid_routing_source: raw.hybrid_routing_source,
        };
        config.validate()?;
        Ok(config)
    }
}

/// Builder for [`TeeConfig`].
///
/// Use [`TeeConfig::builder()`] to create a new builder instance, then chain
/// setter methods and call [`build()`](Self::build) to produce a validated config.
///
/// The builder enforces two invariants:
/// - `TeeRequirement::Required` + `TeeMode::Disabled` is rejected.
/// - Any enabled TEE mode forces `SecretInjectionPolicy::SealedOnly`.
#[derive(Debug, Default)]
pub struct TeeConfigBuilder {
    requirement: Option<TeeRequirement>,
    mode: Option<TeeMode>,
    provider_selector: Option<TeeProviderSelector>,
    key_exchange: Option<TeeKeyExchangeConfig>,
    max_attestation_age_secs: Option<u64>,
    attestation_freshness: Option<AttestationFreshnessPolicy>,
    public_key_policy: Option<TeePublicKeyPolicy>,
    hybrid_routing_source: Option<HybridRoutingSource>,
}

impl TeeConfigBuilder {
    /// Set the TEE requirement level.
    pub fn requirement(mut self, requirement: TeeRequirement) -> Self {
        self.requirement = Some(requirement);
        self
    }

    /// Set the TEE operational mode.
    pub fn mode(mut self, mode: TeeMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Set the provider selector.
    pub fn provider_selector(mut self, selector: TeeProviderSelector) -> Self {
        self.provider_selector = Some(selector);
        self
    }

    /// Set an allowlist of accepted providers.
    pub fn allow_providers(mut self, providers: impl IntoIterator<Item = TeeProvider>) -> Self {
        self.provider_selector = Some(TeeProviderSelector::AllowList(
            providers.into_iter().collect(),
        ));
        self
    }

    /// Set the key exchange configuration.
    pub fn key_exchange(mut self, config: TeeKeyExchangeConfig) -> Self {
        self.key_exchange = Some(config);
        self
    }

    /// Set the maximum attestation age in seconds.
    pub fn max_attestation_age_secs(mut self, secs: u64) -> Self {
        self.max_attestation_age_secs = Some(secs);
        self
    }

    /// Set the attestation freshness policy.
    pub fn attestation_freshness(mut self, policy: AttestationFreshnessPolicy) -> Self {
        self.attestation_freshness = Some(policy);
        self
    }

    /// Set the public key derivation failure policy.
    pub fn public_key_policy(mut self, policy: TeePublicKeyPolicy) -> Self {
        self.public_key_policy = Some(policy);
        self
    }

    /// Set the hybrid routing source.
    pub fn hybrid_routing_source(mut self, source: HybridRoutingSource) -> Self {
        self.hybrid_routing_source = Some(source);
        self
    }

    /// Build the [`TeeConfig`], validating all fields.
    pub fn build(self) -> Result<TeeConfig, TeeError> {
        let mode = self.mode.unwrap_or_default();
        let requirement = self.requirement.unwrap_or_default();

        // If requirement is Required, mode must not be Disabled
        if requirement == TeeRequirement::Required && mode == TeeMode::Disabled {
            return Err(TeeError::Config(
                "TEE requirement is Required but mode is Disabled".to_string(),
            ));
        }

        // TEE-enabled deployments must use SealedOnly secret injection.
        // Container recreation (env-var re-injection) invalidates attestation,
        // breaks sealed secrets, and loses the on-chain deployment ID.
        // Auto mode uses EnvOrSealed until resolved at runtime.
        let secret_injection = if !matches!(mode, TeeMode::Disabled | TeeMode::Auto) {
            SecretInjectionPolicy::SealedOnly
        } else {
            SecretInjectionPolicy::EnvOrSealed
        };

        let attestation_freshness = self.attestation_freshness.unwrap_or_default();
        let hybrid_routing_source = self.hybrid_routing_source.unwrap_or_default();

        Ok(TeeConfig {
            requirement,
            mode,
            provider_selector: self.provider_selector.unwrap_or_default(),
            key_exchange: self.key_exchange.unwrap_or_default(),
            max_attestation_age_secs: self
                .max_attestation_age_secs
                .unwrap_or_else(default_max_attestation_age_secs),
            secret_injection,
            attestation_freshness,
            public_key_policy: self.public_key_policy.unwrap_or_default(),
            hybrid_routing_source,
        })
    }
}
