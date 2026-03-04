//! Helpers for reading and writing blueprint deployment metadata.

use crate::contracts::ITangleTypes;
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

/// Schema marker for structured blueprint metadata payloads.
pub const METADATA_SCHEMA_V1: &str = "tangle.blueprint.metadata.v1";

const DEPLOYMENT_PROFILE_KEY: &str = "deployment_profile";
const JOB_PROFILES_BLOB_KEY: &str = "job_profiles_b64_gzip";

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

/// Resolve TEE deployment profile from on-chain metadata.
pub fn resolve_tee_deployment_profile(
    metadata: &ITangleTypes::BlueprintMetadata,
) -> Result<Option<TeeDeploymentProfile>, String> {
    try_resolve_tee_deployment_profile_from_profiling_data(metadata.profilingData.as_str())
}

/// Resolve TEE deployment profile from `profiling_data` payload.
///
/// Clean path only: expects structured JSON metadata in `profiling_data`.
#[must_use]
pub fn resolve_tee_deployment_profile_from_profiling_data(
    profiling_data: &str,
) -> Option<TeeDeploymentProfile> {
    try_resolve_tee_deployment_profile_from_profiling_data(profiling_data)
        .ok()
        .flatten()
}

/// Resolve TEE deployment profile from `profiling_data` payload with strict errors.
///
/// Returns an error when metadata is present but malformed.
pub fn try_resolve_tee_deployment_profile_from_profiling_data(
    profiling_data: &str,
) -> Result<Option<TeeDeploymentProfile>, String> {
    let trimmed = profiling_data.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let root: Value = serde_json::from_str(trimmed)
        .map_err(|e| format!("profiling_data must be valid JSON: {e}"))?;
    let Some(raw_profile) = root.get(DEPLOYMENT_PROFILE_KEY) else {
        return Ok(None);
    };
    let profile = raw_profile
        .as_object()
        .ok_or_else(|| "deployment_profile must be an object".to_string())?;

    let tee_required = parse_optional_bool(profile, "tee_required")?.unwrap_or(false);
    let supports_tee = parse_optional_bool(profile, "supports_tee")?.unwrap_or(false);

    if !tee_required && !supports_tee {
        return Ok(None);
    }

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
) -> Result<Option<bool>, String> {
    Ok(resolve_tee_deployment_profile(metadata)?.map(|profile| profile.tee_required))
}

/// Resolve explicit `supports_tee` capability from metadata.
pub fn resolve_tee_support(
    metadata: &ITangleTypes::BlueprintMetadata,
) -> Result<Option<bool>, String> {
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
    root.entry("schema".to_string())
        .or_insert_with(|| Value::String(METADATA_SCHEMA_V1.to_string()));
    root.insert(
        DEPLOYMENT_PROFILE_KEY.to_string(),
        profile_to_value(profile),
    );
}

fn parse_optional_bool(profile: &Map<String, Value>, field: &str) -> Result<Option<bool>, String> {
    match profile.get(field) {
        Some(value) => value
            .as_bool()
            .map(Some)
            .ok_or_else(|| format!("deployment_profile.{field} must be a boolean")),
        None => Ok(None),
    }
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
        assert!(resolve_tee_deployment_profile(&metadata).is_err());
    }

    #[test]
    fn strict_parser_errors_on_non_json_payloads() {
        let err = try_resolve_tee_deployment_profile_from_profiling_data("tee")
            .expect_err("expected parse error");
        assert!(err.contains("valid JSON"));
    }

    #[test]
    fn strict_parser_errors_on_non_boolean_fields() {
        let err = try_resolve_tee_deployment_profile_from_profiling_data(
            r#"{"deployment_profile":{"tee_required":"true"}}"#,
        )
        .expect_err("expected type error");
        assert!(err.contains("tee_required"));
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
