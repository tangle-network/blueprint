use crate::{CloudProvider, InstanceStatus, ProvisionedInstance};
use serde::{Deserialize, Serialize};

#[derive(schemars::JsonSchema, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderErrorCode {
    InvalidRequest,
    AuthRejected,
    ProviderNotConfigured,
    NoStock,
    SkuUnknown,
    Unsupported,
    StillPresent,
    Preempted,
    Quota,
    RateLimited,
    NotFound,
    Conflict,
    Indeterminate,
    ProviderError,
}

impl core::fmt::Display for ProviderErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let value = serde_json::to_value(self).map_err(|_| core::fmt::Error)?;
        f.write_str(value.as_str().ok_or(core::fmt::Error)?)
    }
}

/// Identity advertised by the native seller, without guessed region aliases.
#[derive(schemars::JsonSchema, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnderlyingAllocation {
    pub cloud: String,
    pub sku: String,
    pub region: String,
}

/// An available whole-node offer, before the SDK applies its common ranking.
#[derive(schemars::JsonSchema, Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeOffer {
    pub provider: CloudProvider,
    pub native_id: String,
    pub instance_type: String,
    pub region: String,
    pub gpu_class: String,
    pub gpu_count: u32,
    pub hourly_cost_usd: f64,
    pub spot: bool,
    pub pricing_source: String,
    pub underlying: Option<UnderlyingAllocation>,
}

/// Opaque candidate is returned unchanged to `create_managed`.
#[derive(schemars::JsonSchema, Debug, Clone, Serialize, Deserialize)]
pub struct ProviderQuote {
    #[serde(flatten)]
    pub offer: NativeOffer,
    pub candidate: String,
    pub family: Option<String>,
    pub arm: Option<String>,
    pub output_tokens_per_second: Option<f64>,
    pub usd_per_million_output_tokens: Option<f64>,
    pub measurement: Option<blueprint_core::resources::ThroughputMeasurement>,
}

#[derive(schemars::JsonSchema, Debug, Serialize, Deserialize)]
pub struct ProviderFailure {
    pub provider: CloudProvider,
    pub code: ProviderErrorCode,
}

/// Failed providers remain visible beside the full ranked priced list.
#[derive(schemars::JsonSchema, Debug, Serialize, Deserialize)]
pub struct QuoteSet {
    pub quotes: Vec<ProviderQuote>,
    pub errors: Vec<ProviderFailure>,
}

#[derive(schemars::JsonSchema, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedResourceKind {
    Instance,
    Volume,
    Firewall,
}

#[derive(schemars::JsonSchema, Debug, Clone, Serialize, Deserialize)]
pub struct ManagedResource {
    pub instance: ProvisionedInstance,
    pub namespace: String,
    pub lease_id: String,
    pub request_fingerprint: Option<String>,
    pub kind: ManagedResourceKind,
    pub worker_url: Option<String>,
    pub endpoint_url: Option<String>,
    pub ssh_endpoint: Option<String>,
    /// Advertised by the native SKU/listing; device attestation is separate.
    pub gpu_class: Option<String>,
    pub gpu_count: Option<u32>,
    pub gpu_vram_gb: Option<u32>,
    pub hourly_cost_usd: Option<f64>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub preempted: bool,
}

impl ManagedResource {
    pub fn is_present(&self) -> bool {
        !matches!(self.instance.status, InstanceStatus::Terminated)
    }
}
