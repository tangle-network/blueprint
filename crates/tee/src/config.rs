//! TEE configuration types.
//!
//! Provides the core configuration model for TEE integration with the Blueprint runner.

use crate::errors::TeeError;
use serde::{Deserialize, Serialize};

/// The operational mode for TEE integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TeeMode {
    /// TEE is disabled; no TEE operations are performed.
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// Configuration for the key exchange subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeeKeyExchangeConfig {
    /// Maximum time-to-live for ephemeral session keys, in seconds.
    #[serde(default = "default_session_ttl_secs")]
    pub session_ttl_secs: u64,
    /// Maximum number of concurrent key exchange sessions.
    #[serde(default = "default_max_sessions")]
    pub max_sessions: usize,
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
}

fn default_max_attestation_age_secs() -> u64 {
    3600
}

impl Default for TeeConfig {
    fn default() -> Self {
        Self {
            requirement: TeeRequirement::default(),
            mode: TeeMode::default(),
            provider_selector: TeeProviderSelector::default(),
            key_exchange: TeeKeyExchangeConfig::default(),
            max_attestation_age_secs: default_max_attestation_age_secs(),
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
}

/// Builder for [`TeeConfig`].
#[derive(Debug, Default)]
pub struct TeeConfigBuilder {
    requirement: Option<TeeRequirement>,
    mode: Option<TeeMode>,
    provider_selector: Option<TeeProviderSelector>,
    key_exchange: Option<TeeKeyExchangeConfig>,
    max_attestation_age_secs: Option<u64>,
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

        Ok(TeeConfig {
            requirement,
            mode,
            provider_selector: self.provider_selector.unwrap_or_default(),
            key_exchange: self.key_exchange.unwrap_or_default(),
            max_attestation_age_secs: self
                .max_attestation_age_secs
                .unwrap_or_else(default_max_attestation_age_secs),
        })
    }
}
