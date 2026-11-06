//! Helpers for reading and writing blueprint execution metadata.

use crate::contracts::ITangleTypes;
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use thiserror::Error;

/// Schema marker for structured blueprint metadata payloads.
pub const METADATA_SCHEMA_V1: &str = "tangle.blueprint.metadata.v1";

const EXECUTION_PROFILE_KEY: &str = "execution_profile";
const JOB_PROFILES_BLOB_KEY: &str = "job_profiles_b64_gzip";

/// Errors produced while parsing execution profile metadata.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExecutionProfileError {
    /// `profiling_data` is not valid JSON.
    #[error("profiling_data must be valid JSON: {message}")]
    InvalidJson {
        /// Human-readable parser failure detail.
        message: String,
    },
    /// `profiling_data` JSON root is not an object.
    #[error("profiling_data must be a JSON object")]
    MetadataNotObject,
    /// `execution_profile` exists but is not an object.
    #[error("execution_profile must be an object")]
    ExecutionProfileNotObject,
    /// `execution_profile` exists but does not match expected schema.
    #[error("execution_profile is invalid: {message}")]
    InvalidExecutionProfile {
        /// Human-readable schema validation detail.
        message: String,
    },
}

/// Confidentiality policy for execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidentialityPolicy {
    /// Standard and TEE placement are both valid.
    Any,
    /// TEE execution is mandatory (fail closed).
    TeeRequired,
    /// Non-TEE execution is mandatory.
    StandardRequired,
    /// Prefer TEE, but allow non-TEE fallback.
    TeePreferred,
}

impl Default for ConfidentialityPolicy {
    fn default() -> Self {
        Self::Any
    }
}

/// GPU requirement policy for execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GpuPolicy {
    /// No GPU required (default).
    None,
    /// GPU execution is mandatory (fail closed).
    Required,
    /// Prefer GPU, but allow CPU fallback.
    Preferred,
}

impl Default for GpuPolicy {
    fn default() -> Self {
        Self::None
    }
}

/// GPU resource requirements for a blueprint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct GpuRequirements {
    /// GPU availability policy.
    #[serde(default)]
    pub policy: GpuPolicy,
    /// Optional canonical class id from the SDK GPU catalog.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    /// Minimum number of GPU devices required.
    #[serde(default)]
    pub min_count: u32,
    /// Minimum decimal GB of VRAM per device, matching the GPU catalog.
    ///
    /// Note: Kubernetes does not natively expose per-device VRAM as a
    /// schedulable resource. When `policy` is `Required`, the BPM adds a
    /// numeric node-affinity label (`gpu.tangle.tools/vram-gb`) so operators can
    /// label their nodes accordingly. When `policy` is `Preferred`, this
    /// field is advisory only.
    #[serde(default)]
    pub min_vram_gb: u32,
}

impl GpuRequirements {
    /// Normalize requirements: when policy is `Required` or `Preferred`,
    /// ensure `min_count` is at least 1.
    #[must_use]
    pub fn normalized(mut self) -> Self {
        if !matches!(self.policy, GpuPolicy::None) && self.min_count == 0 {
            self.min_count = 1;
        }
        self
    }
}

/// Blueprint deployment policy for execution environments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ExecutionProfile {
    /// Confidentiality policy for runtime placement.
    #[serde(default)]
    pub confidentiality: ConfidentialityPolicy,
    /// GPU resource requirements.
    #[serde(default)]
    pub gpu: GpuRequirements,
}

impl ExecutionProfile {
    /// Whether execution must run in TEE.
    #[must_use]
    pub fn tee_required(&self) -> bool {
        matches!(self.confidentiality, ConfidentialityPolicy::TeeRequired)
    }

    /// Whether execution may run in TEE.
    #[must_use]
    pub fn tee_supported(&self) -> bool {
        matches!(
            self.confidentiality,
            ConfidentialityPolicy::Any
                | ConfidentialityPolicy::TeeRequired
                | ConfidentialityPolicy::TeePreferred
        )
    }

    /// Whether non-TEE placement is required.
    #[must_use]
    pub fn standard_required(&self) -> bool {
        matches!(
            self.confidentiality,
            ConfidentialityPolicy::StandardRequired
        )
    }

    /// Whether GPU execution is mandatory.
    #[must_use]
    pub fn gpu_required(&self) -> bool {
        matches!(self.gpu.policy, GpuPolicy::Required)
    }

    /// Whether GPU execution is preferred but not mandatory.
    #[must_use]
    pub fn gpu_preferred(&self) -> bool {
        matches!(self.gpu.policy, GpuPolicy::Preferred)
    }

    /// Whether the blueprint has any GPU requirements.
    #[must_use]
    pub fn needs_gpu(&self) -> bool {
        !matches!(self.gpu.policy, GpuPolicy::None)
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawExecutionProfile {
    confidentiality: Option<ConfidentialityPolicy>,
    #[serde(default)]
    gpu: Option<GpuRequirements>,
}

/// Resolve execution profile from on-chain metadata.
pub fn resolve_execution_profile(
    metadata: &ITangleTypes::BlueprintMetadata,
) -> Result<Option<ExecutionProfile>, ExecutionProfileError> {
    resolve_execution_profile_from_profiling_data(metadata.profilingData.as_str())
}

/// Resolve execution profile from `profiling_data` payload.
///
/// This parser is strict by design:
/// - empty payload => `Ok(None)`
/// - non-empty payload must be structured JSON metadata
pub fn resolve_execution_profile_from_profiling_data(
    profiling_data: &str,
) -> Result<Option<ExecutionProfile>, ExecutionProfileError> {
    let trimmed = profiling_data.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let root: Value =
        serde_json::from_str(trimmed).map_err(|e| ExecutionProfileError::InvalidJson {
            message: e.to_string(),
        })?;
    let Some(root_object) = root.as_object() else {
        return Err(ExecutionProfileError::MetadataNotObject);
    };

    let Some(raw_profile_value) = root_object.get(EXECUTION_PROFILE_KEY) else {
        return Ok(None);
    };
    let Some(raw_profile_object) = raw_profile_value.as_object() else {
        return Err(ExecutionProfileError::ExecutionProfileNotObject);
    };

    let raw_profile: RawExecutionProfile =
        serde_json::from_value(Value::Object(raw_profile_object.clone())).map_err(|e| {
            ExecutionProfileError::InvalidExecutionProfile {
                message: e.to_string(),
            }
        })?;

    let gpu = raw_profile.gpu.unwrap_or_default().normalized();
    if !matches!(gpu.policy, GpuPolicy::None) || gpu.class.is_some() {
        blueprint_core::resources::ResourceSpec {
            gpu_count: Some(gpu.min_count.max(1)),
            gpu_class: gpu.class.clone(),
            gpu_vram_gb: (gpu.min_vram_gb > 0).then_some(gpu.min_vram_gb),
            ..Default::default()
        }
        .validate()
        .map_err(|message| ExecutionProfileError::InvalidExecutionProfile { message })?;
    }
    Ok(Some(ExecutionProfile {
        confidentiality: raw_profile.confidentiality.unwrap_or_default(),
        gpu,
    }))
}

/// Resolve GPU requirements from metadata.
pub fn resolve_gpu_requirements(
    metadata: &ITangleTypes::BlueprintMetadata,
) -> Result<Option<GpuRequirements>, ExecutionProfileError> {
    Ok(resolve_execution_profile(metadata)?.map(|profile| profile.gpu))
}

/// Resolve explicit confidentiality policy from metadata.
pub fn resolve_confidentiality_policy(
    metadata: &ITangleTypes::BlueprintMetadata,
) -> Result<Option<ConfidentialityPolicy>, ExecutionProfileError> {
    Ok(resolve_execution_profile(metadata)?.map(|profile| profile.confidentiality))
}

/// Inject or update structured execution profile inside `profiling_data`.
///
/// If `profiling_data` currently contains a non-JSON blob, it is preserved
/// under `job_profiles_b64_gzip`.
#[must_use]
pub fn inject_execution_profile(profiling_data: &str, profile: ExecutionProfile) -> String {
    let trimmed = profiling_data.trim();
    if trimmed.is_empty() {
        return default_metadata_payload(profile).to_string();
    }

    if let Ok(mut value) = serde_json::from_str::<Value>(trimmed)
        && let Some(root) = value.as_object_mut()
    {
        if let Some(schema) = root.get("schema").and_then(Value::as_str)
            && schema != METADATA_SCHEMA_V1
        {
            return json!({
                "schema": METADATA_SCHEMA_V1,
                EXECUTION_PROFILE_KEY: profile_to_value(profile),
                JOB_PROFILES_BLOB_KEY: trimmed,
            })
            .to_string();
        }
        upsert_execution_profile(root, profile);
        return value.to_string();
    }

    json!({
        "schema": METADATA_SCHEMA_V1,
        EXECUTION_PROFILE_KEY: profile_to_value(profile),
        JOB_PROFILES_BLOB_KEY: trimmed,
    })
    .to_string()
}

/// Extract compressed/base64 job profile payload from structured metadata.
#[must_use]
pub fn extract_job_profiles_blob(profiling_data: &str) -> Option<String> {
    let trimmed = profiling_data.trim();
    if trimmed.is_empty() {
        return None;
    }

    let value: Value = serde_json::from_str(trimmed).ok()?;
    value
        .as_object()?
        .get(JOB_PROFILES_BLOB_KEY)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

fn default_metadata_payload(profile: ExecutionProfile) -> Value {
    json!({
        "schema": METADATA_SCHEMA_V1,
        EXECUTION_PROFILE_KEY: profile_to_value(profile),
    })
}

fn profile_to_value(profile: ExecutionProfile) -> Value {
    let mut obj = Map::new();
    obj.insert(
        "confidentiality".to_string(),
        serde_json::to_value(profile.confidentiality).unwrap_or_default(),
    );
    if !matches!(profile.gpu.policy, GpuPolicy::None) {
        obj.insert(
            "gpu".to_string(),
            serde_json::to_value(profile.gpu).unwrap_or_default(),
        );
    }
    Value::Object(obj)
}

fn upsert_execution_profile(root: &mut Map<String, Value>, profile: ExecutionProfile) {
    root.insert(
        "schema".to_string(),
        Value::String(METADATA_SCHEMA_V1.to_string()),
    );
    root.insert(EXECUTION_PROFILE_KEY.to_string(), profile_to_value(profile));
}
