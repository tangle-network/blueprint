use crate::{CloudProvider, InstanceStatus, ProvisionedInstance};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnderlyingAllocation {
    pub cloud: String,
    pub sku: String,
    pub region: String,
}

/// An available whole-node offer, before the SDK applies its common ranking.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderQuote {
    #[serde(flatten)]
    pub offer: NativeOffer,
    pub candidate: String,
    pub family: Option<String>,
    pub arm: Option<String>,
    pub output_tokens_per_second: Option<f64>,
    pub usd_per_million_output_tokens: Option<f64>,
    pub measurement: Option<MeasurementReceipt>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderFailure {
    pub provider: CloudProvider,
    pub code: ProviderErrorCode,
}

/// Public audit counters and commitments, deliberately not an execution recipe.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeasurementReceipt {
    pub weights_manifest_sha256: String,
    pub raw_result_sha256: String,
    pub measured_at: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub elapsed_ms: u64,
    pub concurrency: u32,
    pub tp: u32,
    pub replicas: u32,
}

impl From<&blueprint_core::resources::ThroughputMeasurement> for MeasurementReceipt {
    fn from(row: &blueprint_core::resources::ThroughputMeasurement) -> Self {
        Self {
            weights_manifest_sha256: row.weights_manifest_sha256.clone(),
            raw_result_sha256: row.raw_result_sha256.clone(),
            measured_at: row.measured_at.clone(),
            input_tokens: row.input_tokens,
            output_tokens: row.output_tokens,
            elapsed_ms: row.elapsed_ms,
            concurrency: row.concurrency,
            tp: row.tp,
            replicas: row.replicas,
        }
    }
}

/// Failed providers remain visible beside the full ranked priced list.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteSet {
    pub quotes: Vec<ProviderQuote>,
    pub errors: Vec<ProviderFailure>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedResourceKind {
    Instance,
    Template,
    Volume,
    Firewall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
