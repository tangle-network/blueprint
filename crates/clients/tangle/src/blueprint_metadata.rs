//! Helpers for reading and writing blueprint deployment metadata.

use crate::contracts::ITangleTypes;
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use serde_json::{Map, Value, json};

/// Schema marker for structured blueprint metadata payloads.
pub const METADATA_SCHEMA_V1: &str = "tangle.blueprint.metadata.v1";

const TEE_REQUIRED_KEYS: [&str; 2] = ["tee_required", "teeRequired"];
const PROFILE_KEYS: [&str; 4] = [
    "runtime_profile",
    "runtimeProfile",
    "deployment_profile",
    "deploymentProfile",
];
const LEGACY_PROFILES_BLOB_KEYS: [&str; 3] = [
    "job_profiles_b64_gzip",
    "profiling_data_b64_gzip",
    "profilingDataB64Gzip",
];

/// Resolve explicit TEE requirement from on-chain blueprint metadata.
///
/// Returns:
/// - `Some(true)` when metadata requires TEE.
/// - `Some(false)` when metadata explicitly marks non-TEE runtime.
/// - `None` when runtime intent is unspecified.
#[must_use]
pub fn resolve_tee_required(metadata: &ITangleTypes::BlueprintMetadata) -> Option<bool> {
    resolve_tee_required_from_fields(
        metadata.profilingData.as_str(),
        metadata.description.as_str(),
        metadata.category.as_str(),
    )
}

/// Resolve explicit TEE requirement from metadata string fields.
///
/// Resolution order:
/// 1. `profiling_data` structured payload.
/// 2. Legacy `description`/`category` payloads for backward compatibility.
#[must_use]
pub fn resolve_tee_required_from_fields(
    profiling_data: &str,
    description: &str,
    category: &str,
) -> Option<bool> {
    if let Some(tee_required) = parse_tee_required_hint(profiling_data) {
        return Some(tee_required);
    }

    for raw in [description, category] {
        if let Some(tee_required) = parse_tee_required_hint(raw) {
            return Some(tee_required);
        }
    }

    None
}

/// Inject or update explicit `tee_required` intent inside `profiling_data`.
///
/// If `profiling_data` already contains JSON metadata, this updates it in-place.
/// If `profiling_data` is a legacy compressed/base64 blob, the blob is preserved
/// under `job_profiles_b64_gzip` inside a structured metadata object.
#[must_use]
pub fn inject_tee_required(profiling_data: &str, tee_required: bool) -> String {
    let trimmed = profiling_data.trim();
    if trimmed.is_empty() {
        return default_metadata_payload(tee_required).to_string();
    }

    if let Ok(mut value) = serde_json::from_str::<Value>(trimmed) {
        if let Some(root) = value.as_object_mut() {
            upsert_tee_required(root, tee_required);
            return value.to_string();
        }
    }

    json!({
        "schema": METADATA_SCHEMA_V1,
        "deployment_profile": {
            "tee_required": tee_required,
        },
        "job_profiles_b64_gzip": trimmed,
    })
    .to_string()
}

/// Extract compressed/base64 job profile payload, if present.
///
/// Supports:
/// - legacy plain payload (`profiling_data` as base64+gzip string),
/// - structured metadata payloads carrying `job_profiles_b64_gzip`.
#[must_use]
pub fn extract_job_profiles_blob(profiling_data: &str) -> Option<String> {
    let trimmed = profiling_data.trim();
    if trimmed.is_empty() {
        return None;
    }

    if !trimmed.starts_with('{') {
        return Some(trimmed.to_owned());
    }

    let value: Value = serde_json::from_str(trimmed).ok()?;
    let object = value.as_object()?;
    LEGACY_PROFILES_BLOB_KEYS.iter().find_map(|key| {
        object
            .get(*key)
            .and_then(Value::as_str)
            .map(ToOwned::to_owned)
    })
}

fn default_metadata_payload(tee_required: bool) -> Value {
    json!({
        "schema": METADATA_SCHEMA_V1,
        "deployment_profile": {
            "tee_required": tee_required,
        }
    })
}

fn upsert_tee_required(root: &mut Map<String, Value>, tee_required: bool) {
    root.entry("schema".to_string())
        .or_insert_with(|| Value::String(METADATA_SCHEMA_V1.to_string()));

    let profile_key = if root.contains_key("deployment_profile") {
        "deployment_profile"
    } else if root.contains_key("deploymentProfile") {
        "deploymentProfile"
    } else if root.contains_key("runtime_profile") {
        "runtime_profile"
    } else if root.contains_key("runtimeProfile") {
        "runtimeProfile"
    } else {
        "deployment_profile"
    };

    let existing = root.remove(profile_key);
    let mut profile = match existing {
        Some(Value::Object(map)) => map,
        Some(Value::String(mode)) => {
            let mut map = Map::new();
            map.insert("mode".to_string(), Value::String(mode));
            map
        }
        _ => Map::new(),
    };
    profile.insert("tee_required".to_string(), Value::Bool(tee_required));
    root.insert(profile_key.to_string(), Value::Object(profile));
}

fn parse_tee_required_hint(raw: &str) -> Option<bool> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }

    if trimmed.starts_with('{') {
        let value: Value = serde_json::from_str(trimmed).ok()?;
        return extract_tee_required_from_value(&value);
    }

    parse_runtime_profile_mode(trimmed)
}

fn extract_tee_required_from_value(value: &Value) -> Option<bool> {
    for key in TEE_REQUIRED_KEYS {
        if let Some(tee_required) = value.get(key).and_then(Value::as_bool) {
            return Some(tee_required);
        }
    }

    for key in PROFILE_KEYS {
        if let Some(profile) = value.get(key) {
            if let Some(tee_required) = extract_tee_required_from_value(profile) {
                return Some(tee_required);
            }
            if let Some(mode) = profile.as_str() {
                if let Some(tee_required) = parse_runtime_profile_mode(mode) {
                    return Some(tee_required);
                }
            }
        }
    }

    if let Some(mode) = value.get("mode").and_then(Value::as_str) {
        if let Some(tee_required) = parse_runtime_profile_mode(mode) {
            return Some(tee_required);
        }
    }
    if let Some(mode) = value.get("runtime").and_then(Value::as_str) {
        if let Some(tee_required) = parse_runtime_profile_mode(mode) {
            return Some(tee_required);
        }
    }

    None
}

fn parse_runtime_profile_mode(mode: &str) -> Option<bool> {
    let normalized = mode.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "tee"
        | "trusted_execution"
        | "trusted-execution"
        | "confidential"
        | "confidential_compute"
        | "confidential-compute" => Some(true),
        "native" | "container" | "wasm" | "none" | "disabled" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::ITangleTypes;

    #[test]
    fn resolves_from_structured_profiling_data() {
        let mut metadata: ITangleTypes::BlueprintMetadata = Default::default();
        metadata.profilingData = r#"{"deployment_profile":{"tee_required":true}}"#.into();
        assert_eq!(resolve_tee_required(&metadata), Some(true));
    }

    #[test]
    fn resolves_from_runtime_profile_mode() {
        let mut metadata: ITangleTypes::BlueprintMetadata = Default::default();
        metadata.profilingData = r#"{"runtimeProfile":"native"}"#.into();
        assert_eq!(resolve_tee_required(&metadata), Some(false));
    }

    #[test]
    fn preserves_legacy_description_hints() {
        let mut metadata: ITangleTypes::BlueprintMetadata = Default::default();
        metadata.description = "tee".into();
        assert_eq!(resolve_tee_required(&metadata), Some(true));
    }

    #[test]
    fn injects_into_empty_payload() {
        let payload = inject_tee_required("", true);
        let value: Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(
            value
                .get("deployment_profile")
                .and_then(|v| v.get("tee_required"))
                .and_then(Value::as_bool),
            Some(true)
        );
    }

    #[test]
    fn injects_into_existing_object_payload() {
        let payload = inject_tee_required(r#"{"runtime_profile":"native"}"#, true);
        let value: Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(
            value
                .get("runtime_profile")
                .and_then(|v| v.get("tee_required"))
                .and_then(Value::as_bool),
            Some(true)
        );
    }

    #[test]
    fn wraps_legacy_blob_payload() {
        let payload = inject_tee_required("H4sIAAAAA...", true);
        let value: Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(
            value
                .get("job_profiles_b64_gzip")
                .and_then(Value::as_str)
                .unwrap(),
            "H4sIAAAAA..."
        );
        assert_eq!(
            value
                .get("deployment_profile")
                .and_then(|v| v.get("tee_required"))
                .and_then(Value::as_bool),
            Some(true)
        );
    }

    #[test]
    fn extracts_profiles_blob_from_structured_payload() {
        let payload =
            r#"{"deployment_profile":{"tee_required":true},"job_profiles_b64_gzip":"abc"}"#;
        assert_eq!(extract_job_profiles_blob(payload), Some("abc".to_string()));
    }

    #[test]
    fn extract_profiles_blob_returns_none_for_tee_only_payload() {
        let payload = r#"{"deployment_profile":{"tee_required":true}}"#;
        assert_eq!(extract_job_profiles_blob(payload), None);
    }
}
