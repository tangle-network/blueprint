//! Helpers for reading and writing blueprint deployment metadata.

use crate::contracts::ITangleTypes;
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use thiserror::Error;

/// Schema marker for structured blueprint metadata payloads.
pub const METADATA_SCHEMA_V1: &str = "tangle.blueprint.metadata.v1";

const DEPLOYMENT_PROFILE_KEY: &str = "deployment_profile";
const JOB_PROFILES_BLOB_KEY: &str = "job_profiles_b64_gzip";

/// Errors produced while parsing deployment profile metadata.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TeeDeploymentProfileError {
    /// `profiling_data` is not valid JSON.
    #[error("profiling_data must be valid JSON: {message}")]
    InvalidJson {
        /// Human-readable parser failure detail.
        message: String,
    },
    /// `profiling_data` JSON root is not an object.
    #[error("profiling_data must be a JSON object")]
    MetadataNotObject,
    /// `deployment_profile` exists but is not an object.
    #[error("deployment_profile must be an object")]
    DeploymentProfileNotObject,
    /// `deployment_profile` exists but does not match expected schema.
    #[error("deployment_profile is invalid: {message}")]
    InvalidDeploymentProfile {
        /// Human-readable schema validation detail.
        message: String,
    },
    /// `tee_required=true` cannot be paired with `supports_tee=false`.
    #[error("deployment_profile is invalid: tee_required=true requires supports_tee=true")]
    ContradictoryPolicy,
}

/// Blueprint deployment policy related to TEE placement.
///
/// - `tee_required=true` means manager must fail-closed when TEE is unavailable.
/// - `supports_tee=true` means the blueprint can run in TEE mode.
///
/// A dual-mode blueprint (TEE and non-TEE) should set:
/// `tee_required=false`, `supports_tee=true`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeeDeploymentProfile {
    /// Whether TEE placement is mandatory (fail-closed).
    pub tee_required: bool,
    /// Whether this blueprint can run on TEE-capable infrastructure.
    pub supports_tee: bool,
}

impl TeeDeploymentProfile {
    /// Normalize profile invariants.
    #[must_use]
    pub fn normalized(self) -> Self {
        if self.tee_required && !self.supports_tee {
            return Self {
                tee_required: true,
                supports_tee: true,
            };
        }
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawDeploymentProfile {
    tee_required: Option<bool>,
    supports_tee: Option<bool>,
}

/// Resolve TEE deployment profile from on-chain metadata.
pub fn resolve_tee_deployment_profile(
    metadata: &ITangleTypes::BlueprintMetadata,
) -> Result<Option<TeeDeploymentProfile>, TeeDeploymentProfileError> {
    resolve_tee_deployment_profile_from_profiling_data(metadata.profilingData.as_str())
}

/// Resolve TEE deployment profile from `profiling_data` payload.
///
/// Tolerant path:
/// - Returns `Ok(None)` for empty or legacy non-JSON/non-object payloads.
/// - Returns `Err` when structured JSON metadata is present but malformed.
pub fn resolve_tee_deployment_profile_from_profiling_data(
    profiling_data: &str,
) -> Result<Option<TeeDeploymentProfile>, TeeDeploymentProfileError> {
    let trimmed = profiling_data.trim();
    if trimmed.is_empty() || !looks_like_structured_metadata(trimmed) {
        return Ok(None);
    }

    try_resolve_tee_deployment_profile_from_profiling_data(trimmed)
}

/// Resolve TEE deployment profile from `profiling_data` payload with strict errors.
///
/// Strict path:
/// - Returns `Err` for any non-empty payload that is not valid structured metadata.
pub fn try_resolve_tee_deployment_profile_from_profiling_data(
    profiling_data: &str,
) -> Result<Option<TeeDeploymentProfile>, TeeDeploymentProfileError> {
    let trimmed = profiling_data.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let root: Value =
        serde_json::from_str(trimmed).map_err(|e| TeeDeploymentProfileError::InvalidJson {
            message: e.to_string(),
        })?;
    let Some(root_object) = root.as_object() else {
        return Err(TeeDeploymentProfileError::MetadataNotObject);
    };

    let Some(raw_profile_value) = root_object.get(DEPLOYMENT_PROFILE_KEY) else {
        return Ok(None);
    };
    let Some(raw_profile_object) = raw_profile_value.as_object() else {
        return Err(TeeDeploymentProfileError::DeploymentProfileNotObject);
    };

    let raw_profile: RawDeploymentProfile =
        serde_json::from_value(Value::Object(raw_profile_object.clone())).map_err(|e| {
            TeeDeploymentProfileError::InvalidDeploymentProfile {
                message: e.to_string(),
            }
        })?;
    let tee_required = raw_profile.tee_required.unwrap_or(false);
    if tee_required && matches!(raw_profile.supports_tee, Some(false)) {
        return Err(TeeDeploymentProfileError::ContradictoryPolicy);
    }
    let supports_tee = raw_profile.supports_tee.unwrap_or(false);

    Ok(Some(
        TeeDeploymentProfile {
            tee_required,
            supports_tee,
        }
        .normalized(),
    ))
}

/// Resolve explicit `tee_required` deployment enforcement from metadata.
pub fn resolve_tee_required(
    metadata: &ITangleTypes::BlueprintMetadata,
) -> Result<Option<bool>, TeeDeploymentProfileError> {
    Ok(resolve_tee_deployment_profile(metadata)?.map(|profile| profile.tee_required))
}

/// Resolve explicit `supports_tee` capability from metadata.
pub fn resolve_tee_support(
    metadata: &ITangleTypes::BlueprintMetadata,
) -> Result<Option<bool>, TeeDeploymentProfileError> {
    Ok(resolve_tee_deployment_profile(metadata)?.map(|profile| profile.supports_tee))
}

/// Inject or update structured TEE deployment profile inside `profiling_data`.
///
/// If `profiling_data` currently contains a legacy plain blob, it is preserved
/// under `job_profiles_b64_gzip`.
#[must_use]
pub fn inject_tee_deployment_profile(
    profiling_data: &str,
    profile: TeeDeploymentProfile,
) -> String {
    let profile = profile.normalized();
    let trimmed = profiling_data.trim();
    if trimmed.is_empty() {
        return default_metadata_payload(profile).to_string();
    }

    if let Ok(mut value) = serde_json::from_str::<Value>(trimmed) {
        if let Some(root) = value.as_object_mut() {
            upsert_deployment_profile(root, profile);
            return value.to_string();
        }
    }

    json!({
        "schema": METADATA_SCHEMA_V1,
        DEPLOYMENT_PROFILE_KEY: profile_to_value(profile),
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

fn looks_like_structured_metadata(payload: &str) -> bool {
    payload.starts_with('{')
}

fn default_metadata_payload(profile: TeeDeploymentProfile) -> Value {
    json!({
        "schema": METADATA_SCHEMA_V1,
        DEPLOYMENT_PROFILE_KEY: profile_to_value(profile),
    })
}

fn profile_to_value(profile: TeeDeploymentProfile) -> Value {
    json!({
        "tee_required": profile.tee_required,
        "supports_tee": profile.supports_tee,
    })
}

fn upsert_deployment_profile(root: &mut Map<String, Value>, profile: TeeDeploymentProfile) {
    root.insert(
        "schema".to_string(),
        Value::String(METADATA_SCHEMA_V1.to_string()),
    );
    root.insert(
        DEPLOYMENT_PROFILE_KEY.to_string(),
        profile_to_value(profile),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::ITangleTypes;

    #[test]
    fn resolves_required_profile() {
        let mut metadata: ITangleTypes::BlueprintMetadata = Default::default();
        metadata.profilingData =
            r#"{"deployment_profile":{"tee_required":true,"supports_tee":true}}"#.into();
        assert_eq!(
            resolve_tee_deployment_profile(&metadata).unwrap(),
            Some(TeeDeploymentProfile {
                tee_required: true,
                supports_tee: true,
            })
        );
    }

    #[test]
    fn resolves_optional_profile() {
        let mut metadata: ITangleTypes::BlueprintMetadata = Default::default();
        metadata.profilingData =
            r#"{"deployment_profile":{"tee_required":false,"supports_tee":true}}"#.into();
        assert_eq!(
            resolve_tee_deployment_profile(&metadata).unwrap(),
            Some(TeeDeploymentProfile {
                tee_required: false,
                supports_tee: true,
            })
        );
    }

    #[test]
    fn ignores_non_structured_payloads() {
        let mut metadata: ITangleTypes::BlueprintMetadata = Default::default();
        metadata.profilingData = "tee".into();
        assert_eq!(resolve_tee_deployment_profile(&metadata).unwrap(), None);
    }

    #[test]
    fn strict_parser_errors_on_non_json_payloads() {
        let err = try_resolve_tee_deployment_profile_from_profiling_data("tee")
            .expect_err("expected parse error");
        assert!(matches!(err, TeeDeploymentProfileError::InvalidJson { .. }));
    }

    #[test]
    fn tolerant_parser_ignores_non_json_payloads() {
        assert_eq!(
            resolve_tee_deployment_profile_from_profiling_data("tee").unwrap(),
            None
        );
    }

    #[test]
    fn tolerant_parser_errors_on_malformed_json_object_payloads() {
        let err = resolve_tee_deployment_profile_from_profiling_data("{")
            .expect_err("expected parse error");
        assert!(matches!(err, TeeDeploymentProfileError::InvalidJson { .. }));
    }

    #[test]
    fn strict_parser_errors_on_non_boolean_fields() {
        let err = try_resolve_tee_deployment_profile_from_profiling_data(
            r#"{"deployment_profile":{"tee_required":"true"}}"#,
        )
        .expect_err("expected type error");
        assert!(matches!(
            err,
            TeeDeploymentProfileError::InvalidDeploymentProfile { .. }
        ));
    }

    #[test]
    fn strict_parser_errors_on_contradictory_policy_flags() {
        let err = try_resolve_tee_deployment_profile_from_profiling_data(
            r#"{"deployment_profile":{"tee_required":true,"supports_tee":false}}"#,
        )
        .expect_err("expected contradiction error");
        assert!(matches!(
            err,
            TeeDeploymentProfileError::ContradictoryPolicy
        ));
    }

    #[test]
    fn strict_parser_errors_on_unknown_fields() {
        let err = try_resolve_tee_deployment_profile_from_profiling_data(
            r#"{"deployment_profile":{"tee_required":true,"tee_requred":true}}"#,
        )
        .expect_err("expected schema error");
        assert!(matches!(
            err,
            TeeDeploymentProfileError::InvalidDeploymentProfile { .. }
        ));
    }

    #[test]
    fn explicit_non_tee_profile_is_preserved() {
        let parsed = try_resolve_tee_deployment_profile_from_profiling_data(
            r#"{"deployment_profile":{"tee_required":false,"supports_tee":false}}"#,
        )
        .unwrap();
        assert_eq!(
            parsed,
            Some(TeeDeploymentProfile {
                tee_required: false,
                supports_tee: false,
            })
        );
    }

    #[test]
    fn required_implies_supports() {
        let profile = TeeDeploymentProfile {
            tee_required: true,
            supports_tee: false,
        }
        .normalized();
        assert_eq!(
            profile,
            TeeDeploymentProfile {
                tee_required: true,
                supports_tee: true,
            }
        );
    }

    #[test]
    fn injects_into_empty_payload() {
        let payload = inject_tee_deployment_profile(
            "",
            TeeDeploymentProfile {
                tee_required: false,
                supports_tee: true,
            },
        );
        let value: Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(
            value
                .get(DEPLOYMENT_PROFILE_KEY)
                .and_then(|v| v.get("supports_tee"))
                .and_then(Value::as_bool),
            Some(true)
        );
    }

    #[test]
    fn injects_into_existing_object_payload() {
        let payload = inject_tee_deployment_profile(
            r#"{"job_profiles_b64_gzip":"abc"}"#,
            TeeDeploymentProfile {
                tee_required: true,
                supports_tee: true,
            },
        );
        let value: Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(
            value
                .get(DEPLOYMENT_PROFILE_KEY)
                .and_then(|v| v.get("tee_required"))
                .and_then(Value::as_bool),
            Some(true)
        );
    }

    #[test]
    fn inject_overwrites_stale_schema_marker() {
        let payload = inject_tee_deployment_profile(
            r#"{"schema":"legacy.v0"}"#,
            TeeDeploymentProfile {
                tee_required: false,
                supports_tee: true,
            },
        );
        let value: Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(
            value.get("schema").and_then(Value::as_str),
            Some(METADATA_SCHEMA_V1)
        );
    }

    #[test]
    fn wraps_legacy_blob_payload() {
        let payload = inject_tee_deployment_profile(
            "H4sIAAAAA...",
            TeeDeploymentProfile {
                tee_required: true,
                supports_tee: true,
            },
        );
        let value: Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(
            value.get(JOB_PROFILES_BLOB_KEY).and_then(Value::as_str),
            Some("H4sIAAAAA...")
        );
    }

    #[test]
    fn extracts_profiles_blob_from_structured_payload() {
        let payload = r#"{"deployment_profile":{"tee_required":true,"supports_tee":true},"job_profiles_b64_gzip":"abc"}"#;
        assert_eq!(extract_job_profiles_blob(payload), Some("abc".to_string()));
    }

    #[test]
    fn extract_profiles_blob_requires_structured_payload() {
        assert_eq!(extract_job_profiles_blob("H4sIAAAAA..."), None);
    }
}
