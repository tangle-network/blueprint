//! Request and response types for the aggregation service API

use alloy_primitives::U256;
use serde::{Deserialize, Serialize};

/// Unique identifier for a job aggregation task
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId {
    pub service_id: u64,
    pub call_id: u64,
}

impl TaskId {
    pub fn new(service_id: u64, call_id: u64) -> Self {
        Self {
            service_id,
            call_id,
        }
    }
}

/// Description of how many signatures are required for a task
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ThresholdConfig {
    /// Require a fixed number of operators to sign.
    Count { required_signers: u32 },
    /// Require stake-weighted participation. Carries the per-index stakes.
    StakeWeighted {
        /// Basis points (0-10000) of total stake required.
        threshold_bps: u32,
        /// Stake weight for each operator index.
        operator_stakes: Vec<OperatorStake>,
    },
}

/// Stake weight for a specific operator index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorStake {
    /// Operator index within the service operator set.
    pub operator_index: u32,
    /// Relative stake or weight used for aggregation.
    pub stake: u64,
}

/// Request to submit a signature for aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitSignatureRequest {
    /// Service ID
    pub service_id: u64,
    /// Job call ID
    pub call_id: u64,
    /// Operator index in the service's operator list
    pub operator_index: u32,
    /// The job output being signed
    #[serde(with = "hex_bytes")]
    pub output: Vec<u8>,
    /// BLS signature (G1 point, 64 bytes compressed)
    #[serde(with = "hex_bytes")]
    pub signature: Vec<u8>,
    /// Operator's BLS public key (G2 point, 128 bytes compressed)
    #[serde(with = "hex_bytes")]
    pub public_key: Vec<u8>,
}

/// Response after submitting a signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitSignatureResponse {
    /// Whether the submission was accepted
    pub accepted: bool,
    /// Current number of signatures collected
    pub signatures_collected: usize,
    /// Required threshold
    pub threshold_required: usize,
    /// Whether threshold has been met
    pub threshold_met: bool,
    /// Optional error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Request to query aggregation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatusRequest {
    pub service_id: u64,
    pub call_id: u64,
}

/// Response with aggregation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatusResponse {
    /// Whether the task exists
    pub exists: bool,
    /// Current signatures collected
    pub signatures_collected: usize,
    /// Required threshold
    pub threshold_required: usize,
    /// Whether threshold is met
    pub threshold_met: bool,
    /// Signer bitmap (which operators have signed)
    pub signer_bitmap: U256,
    /// Signed stake in basis points (0-10000), for stake-weighted thresholds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_stake_bps: Option<u32>,
    /// Whether already submitted to chain
    pub submitted: bool,
    /// Whether the task has expired
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_expired: Option<bool>,
    /// Time remaining until expiry in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_remaining_secs: Option<u64>,
}

/// Request to initialize an aggregation task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitTaskRequest {
    pub service_id: u64,
    pub call_id: u64,
    /// Number of operators in the service
    pub operator_count: u32,
    /// Threshold definition (count or stake-weighted)
    pub threshold: ThresholdConfig,
    /// The output to be signed
    #[serde(with = "hex_bytes")]
    pub output: Vec<u8>,
}

/// Response after initializing a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitTaskResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Aggregated result ready for submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedResultResponse {
    pub service_id: u64,
    pub call_id: u64,
    #[serde(with = "hex_bytes")]
    pub output: Vec<u8>,
    pub signer_bitmap: U256,
    /// Indices of operators who did not sign (for potential slashing)
    pub non_signer_indices: Vec<u32>,
    /// Aggregated signature (G1 point)
    #[serde(with = "hex_bytes")]
    pub aggregated_signature: Vec<u8>,
    /// Aggregated public key (G2 point)
    #[serde(with = "hex_bytes")]
    pub aggregated_pubkey: Vec<u8>,
}

/// Hex encoding/decoding for byte arrays in JSON
mod hex_bytes {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{}", hex::encode(bytes)))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.strip_prefix("0x").unwrap_or(&s);
        hex::decode(s).map_err(serde::de::Error::custom)
    }
}
