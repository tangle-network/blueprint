//! Nonsecret recovery metadata stored in native provider tags, env or labels.

use super::{ManagedLeaseIdentity, ManagedResource, ManagedResourceKind, ProviderErrorCode};
use crate::{Error, ProvisionedInstance, ResourceSpec, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedMetadata {
    pub identity: ManagedLeaseIdentity,
    pub request_fingerprint: String,
    pub created_at: DateTime<Utc>,
    /// Requested retirement deadline; native expiry enforcement is provider-specific.
    pub expires_at: DateTime<Utc>,
    pub endpoint_port: Option<u16>,
}

impl ManagedMetadata {
    pub fn for_spec(
        spec: &ResourceSpec,
        fingerprint: &str,
        created_at: DateTime<Utc>,
    ) -> Result<Self> {
        let invalid = || Error::Provider(ProviderErrorCode::InvalidRequest);
        if spec
            .env
            .keys()
            .any(|key| key.to_ascii_uppercase().starts_with("BPG_"))
        {
            return Err(invalid());
        }
        let seconds = spec
            .lease_duration_secs
            .and_then(|v| i64::try_from(v).ok())
            .filter(|v| *v > 0)
            .ok_or_else(invalid)?;
        let metadata = Self {
            identity: spec.ownership.clone().ok_or_else(invalid)?,
            request_fingerprint: fingerprint.into(),
            created_at,
            expires_at: created_at
                .checked_add_signed(chrono::Duration::try_seconds(seconds).ok_or_else(invalid)?)
                .ok_or_else(invalid)?,
            endpoint_port: spec.endpoint_port,
        };
        metadata.validate().map_err(|_| invalid())?;
        Ok(metadata)
    }

    fn validate(&self) -> Result<()> {
        if self.identity.validate().is_err()
            || self.request_fingerprint.len() != 64
            || !self
                .request_fingerprint
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit())
            || self.expires_at <= self.created_at
            || self.endpoint_port == Some(0)
        {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        Ok(())
    }

    /// Bind submitted recovery metadata without inventing provider billing observations.
    pub fn bind(self, instance: ProvisionedInstance, kind: ManagedResourceKind) -> ManagedResource {
        ManagedResource {
            instance,
            namespace: self.identity.namespace,
            lease_id: self.identity.lease_id,
            request_fingerprint: Some(self.request_fingerprint),
            kind,
            worker_url: None,
            endpoint_url: None,
            ssh_endpoint: None,
            gpu_class: None,
            gpu_count: None,
            gpu_vram_gb: None,
            hourly_cost_usd: None,
            created_at: None,
            expires_at: Some(self.expires_at),
            preempted: false,
        }
    }

    /// Native request identity includes the tenant and lease, even for equal specs.
    pub fn idempotency_key(&self) -> String {
        let tuple = serde_json::to_vec(&(
            &self.identity.namespace,
            &self.identity.lease_id,
            &self.request_fingerprint,
        ))
        .expect("string tuple serializes");
        hex::encode(aws_lc_rs::digest::digest(
            &aws_lc_rs::digest::SHA256,
            &tuple,
        ))
    }

    /// A DNS-compatible locator. Full native metadata establishes ownership.
    pub fn name(&self) -> String {
        let identity = format!("{}\0{}", self.identity.namespace, self.identity.lease_id);
        let digest = hex::encode(aws_lc_rs::digest::digest(
            &aws_lc_rs::digest::SHA256,
            identity.as_bytes(),
        ));
        format!("bpg1-{}", &digest[..48])
    }

    pub fn tags(&self) -> BTreeMap<String, String> {
        let mut tags = BTreeMap::from([
            ("bpg-namespace".into(), self.identity.namespace.clone()),
            ("bpg-lease-id".into(), self.identity.lease_id.clone()),
            ("bpg-fingerprint".into(), self.request_fingerprint.clone()),
            ("bpg-created-at".into(), self.created_at.to_rfc3339()),
            ("bpg-expires-at".into(), self.expires_at.to_rfc3339()),
        ]);
        if let Some(port) = self.endpoint_port {
            tags.insert("bpg-endpoint-port".into(), port.to_string());
        }
        tags
    }

    pub fn from_tags(tags: &BTreeMap<String, String>) -> Result<Option<Self>> {
        if !tags.keys().any(|key| key.starts_with("bpg-")) {
            return Ok(None);
        }
        let conflict = || Error::Provider(ProviderErrorCode::Conflict);
        let get = |key: &str| tags.get(key).cloned().ok_or_else(conflict);
        let metadata = Self {
            identity: ManagedLeaseIdentity {
                namespace: get("bpg-namespace")?,
                lease_id: get("bpg-lease-id")?,
            },
            request_fingerprint: get("bpg-fingerprint")?,
            created_at: get("bpg-created-at")?.parse().map_err(|_| conflict())?,
            expires_at: get("bpg-expires-at")?.parse().map_err(|_| conflict())?,
            endpoint_port: tags
                .get("bpg-endpoint-port")
                .map(|value| value.parse())
                .transpose()
                .map_err(|_| conflict())?,
        };
        metadata.validate()?;
        let canonical = metadata.tags();
        if tags
            .iter()
            .filter(|(key, _)| key.starts_with("bpg-"))
            .any(|(key, value)| canonical.get(key) != Some(value))
        {
            return Err(conflict());
        }
        Ok(Some(metadata))
    }

    pub fn env(&self) -> BTreeMap<String, String> {
        self.tags()
            .into_iter()
            .map(|(key, value)| (key.to_ascii_uppercase().replace('-', "_"), value))
            .collect()
    }

    pub fn from_env(env: &BTreeMap<String, String>) -> Result<Option<Self>> {
        let tags = env
            .iter()
            .filter(|(key, _)| key.starts_with("BPG_"))
            .map(|(key, value)| (key.to_ascii_lowercase().replace('_', "-"), value.clone()))
            .collect();
        Self::from_tags(&tags)
    }

    pub fn label(&self) -> Result<String> {
        serde_json::to_string(self)
            .map(|json| format!("bpg1:{json}"))
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))
    }

    pub fn from_label(label: &str) -> Result<Option<Self>> {
        let Some(json) = label.strip_prefix("bpg1:") else {
            return Ok(None);
        };
        let metadata: Self =
            serde_json::from_str(json).map_err(|_| Error::Provider(ProviderErrorCode::Conflict))?;
        metadata.validate()?;
        Ok(Some(metadata))
    }
}
