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

/// Blueprint deployment policy for execution environments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ExecutionProfile {
    /// Confidentiality policy for runtime placement.
    #[serde(default)]
    pub confidentiality: ConfidentialityPolicy,
}

impl ExecutionProfile {
    /// Whether execution must run in TEE.
    #[must_use]
    pub fn tee_required(self) -> bool {
        matches!(self.confidentiality, ConfidentialityPolicy::TeeRequired)
    }

    /// Whether execution may run in TEE.
    #[must_use]
    pub fn tee_supported(self) -> bool {
        matches!(
            self.confidentiality,
            ConfidentialityPolicy::Any
                | ConfidentialityPolicy::TeeRequired
                | ConfidentialityPolicy::TeePreferred
        )
    }

    /// Whether non-TEE placement is required.
    #[must_use]
    pub fn standard_required(self) -> bool {
        matches!(
            self.confidentiality,
            ConfidentialityPolicy::StandardRequired
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawExecutionProfile {
    confidentiality: Option<ConfidentialityPolicy>,
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

    Ok(Some(ExecutionProfile {
        confidentiality: raw_profile.confidentiality.unwrap_or_default(),
    }))
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

    if let Ok(mut value) = serde_json::from_str::<Value>(trimmed) {
        if let Some(root) = value.as_object_mut() {
            if let Some(schema) = root.get("schema").and_then(Value::as_str) {
                if schema != METADATA_SCHEMA_V1 {
                    return json!({
                        "schema": METADATA_SCHEMA_V1,
                        EXECUTION_PROFILE_KEY: profile_to_value(profile),
                        JOB_PROFILES_BLOB_KEY: trimmed,
                    })
                    .to_string();
                }
            }
            upsert_execution_profile(root, profile);
            return value.to_string();
        }
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
    json!({
        "confidentiality": profile.confidentiality,
    })
}

fn upsert_execution_profile(root: &mut Map<String, Value>, profile: ExecutionProfile) {
    root.insert(
        "schema".to_string(),
        Value::String(METADATA_SCHEMA_V1.to_string()),
    );
    root.insert(EXECUTION_PROFILE_KEY.to_string(), profile_to_value(profile));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::ITangleTypes;

    #[test]
    fn resolves_required_profile() {
        let mut metadata: ITangleTypes::BlueprintMetadata = Default::default();
        metadata.profilingData =
            r#"{"execution_profile":{"confidentiality":"tee_required"}}"#.into();
        assert_eq!(
            resolve_execution_profile(&metadata).unwrap(),
            Some(ExecutionProfile {
                confidentiality: ConfidentialityPolicy::TeeRequired,
            })
        );
    }

    #[test]
    fn resolves_optional_profile() {
        let mut metadata: ITangleTypes::BlueprintMetadata = Default::default();
        metadata.profilingData =
            r#"{"execution_profile":{"confidentiality":"tee_preferred"}}"#.into();
        assert_eq!(
            resolve_execution_profile(&metadata).unwrap(),
            Some(ExecutionProfile {
                confidentiality: ConfidentialityPolicy::TeePreferred,
            })
        );
    }

    #[test]
    fn strict_parser_errors_on_non_json_payloads() {
        let err =
            resolve_execution_profile_from_profiling_data("tee").expect_err("expected parse error");
        assert!(matches!(err, ExecutionProfileError::InvalidJson { .. }));
    }

    #[test]
    fn strict_parser_errors_on_non_object_json_payloads() {
        let err =
            resolve_execution_profile_from_profiling_data("[]").expect_err("expected type error");
        assert!(matches!(err, ExecutionProfileError::MetadataNotObject));
    }

    #[test]
    fn strict_parser_errors_on_non_string_confidentiality() {
        let err = resolve_execution_profile_from_profiling_data(
            r#"{"execution_profile":{"confidentiality":true}}"#,
        )
        .expect_err("expected type error");
        assert!(matches!(
            err,
            ExecutionProfileError::InvalidExecutionProfile { .. }
        ));
    }

    #[test]
    fn strict_parser_errors_on_unknown_fields() {
        let err = resolve_execution_profile_from_profiling_data(
            r#"{"execution_profile":{"confidentiality":"tee_required","bad":true}}"#,
        )
        .expect_err("expected schema error");
        assert!(matches!(
            err,
            ExecutionProfileError::InvalidExecutionProfile { .. }
        ));
    }

    #[test]
    fn injects_into_empty_payload() {
        let payload = inject_execution_profile(
            "",
            ExecutionProfile {
                confidentiality: ConfidentialityPolicy::Any,
            },
        );
        let value: Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(
            value
                .get(EXECUTION_PROFILE_KEY)
                .and_then(|v| v.get("confidentiality"))
                .and_then(Value::as_str),
            Some("any")
        );
    }

    #[test]
    fn updates_existing_object_payload() {
        let payload = inject_execution_profile(
            r#"{"job_profiles_b64_gzip":"abc"}"#,
            ExecutionProfile {
                confidentiality: ConfidentialityPolicy::TeeRequired,
            },
        );
        let value: Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(
            value
                .get(EXECUTION_PROFILE_KEY)
                .and_then(|v| v.get("confidentiality"))
                .and_then(Value::as_str),
            Some("tee_required")
        );
        assert_eq!(
            value.get(JOB_PROFILES_BLOB_KEY).and_then(Value::as_str),
            Some("abc")
        );
    }

    #[test]
    fn wraps_non_json_payload_as_job_profiles_blob() {
        let payload = inject_execution_profile(
            "H4sIAAAAAAAA/2NgYGBgBGIOAwA6rY+4BQAAAA==",
            ExecutionProfile {
                confidentiality: ConfidentialityPolicy::TeeRequired,
            },
        );
        let value: Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(
            value.get(JOB_PROFILES_BLOB_KEY).and_then(Value::as_str),
            Some("H4sIAAAAAAAA/2NgYGBgBGIOAwA6rY+4BQAAAA==")
        );
    }

    #[test]
    fn extracts_profiles_blob_from_structured_payload() {
        let payload = r#"{"execution_profile":{"confidentiality":"tee_required"},"job_profiles_b64_gzip":"abc"}"#;
        assert_eq!(extract_job_profiles_blob(payload), Some("abc".to_string()));
    }

    #[test]
    fn extract_profiles_blob_requires_structured_payload() {
        assert_eq!(extract_job_profiles_blob("H4sIAAAAA..."), None);
    }
}
