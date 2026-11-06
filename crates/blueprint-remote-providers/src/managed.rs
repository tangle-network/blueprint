//! Provider operations for an external orchestrator that owns the lease lifecycle.

use crate::infra::traits::CloudProviderAdapter;
use crate::{
    CloudProvider, CloudProvisioner, Error, ManagedResource, ManagedResourceKind, ResourceSpec,
    Result,
};
use blueprint_core::resources::ManagedLeaseIdentity;
use serde::Deserialize;
use std::collections::HashMap;
use zeroize::Zeroize;

/// Provider identities and credential names share the catalog's source owner.
pub fn provider_catalog() -> serde_json::Value {
    let source: serde_json::Value = serde_json::from_str(blueprint_core::gpu::CATALOG_JSON)
        .expect("published provider catalog must be valid");
    serde_json::Value::Array(
        source["providers"]
            .as_array()
            .expect("provider descriptors")
            .iter()
            .map(|provider| {
                serde_json::json!({
                    "id": provider["id"], "aliases": provider["aliases"],
                    "credential_env_vars": provider["credentialEnvVars"],
                    "capabilities": provider["capabilities"],
                })
            })
            .collect(),
    )
}

/// Digest of the immutable allocation request, including secret environment values.
/// Only the digest is put in native tags; no environment values leave the request.
pub fn request_fingerprint(spec: &ResourceSpec) -> Result<String> {
    let mut value = serde_json::to_value(spec)
        .map_err(|_| Error::ConfigurationError("Invalid allocation request".into()))?;
    value["env"] = serde_json::to_value(&spec.env)
        .map_err(|_| Error::ConfigurationError("Invalid workload environment".into()))?;
    let bytes = serde_json::to_vec(&value)
        .map_err(|_| Error::ConfigurationError("Invalid allocation request".into()))?;
    let digest = aws_lc_rs::digest::digest(&aws_lc_rs::digest::SHA256, &bytes);
    Ok(digest
        .as_ref()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

async fn ownership_lock(
    provider: &CloudProvider,
    identity: &ManagedLeaseIdentity,
) -> Result<std::fs::File> {
    let directory = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(std::path::PathBuf::from)
        .ok_or_else(|| {
            Error::ConfigurationError("Managed operations require a user cache directory".into())
        })?
        .join(".cache/blueprint/gpu-operation-locks");
    let filename = format!(
        "{}-{}.lock",
        provider.catalog_key(),
        crate::providers::common::gpu_adapter::managed_resource_name(
            &identity.namespace,
            &identity.lease_id
        )?
    );
    tokio::task::spawn_blocking(move || -> Result<std::fs::File> {
        let mut directories = std::fs::DirBuilder::new();
        directories.recursive(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            directories.mode(0o700);
        }
        directories.create(&directory)?;
        let mut options = std::fs::OpenOptions::new();
        options.create(true).truncate(false).read(true).write(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let file = options.open(directory.join(filename))?;
        file.lock()?;
        Ok(file)
    })
    .await
    .map_err(|_| Error::Other("Managed operation lock failed".into()))?
}

/// Caller-owned API credentials. They are never serialized or returned as access.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderCredentials {
    pub api_key: String,
    #[serde(default)]
    pub api_secret: Option<String>,
    #[serde(default)]
    pub rate_keys: Option<crate::providers::modal::ModalRateKeys>,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
}

impl std::fmt::Debug for ProviderCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ProviderCredentials([REDACTED])")
    }
}

impl Drop for ProviderCredentials {
    fn drop(&mut self) {
        self.api_key.zeroize();
        self.api_secret.zeroize();
    }
}

impl CloudProvisioner {
    /// Construct provider clients without changing the process environment.
    pub fn from_credentials(
        credentials: HashMap<CloudProvider, ProviderCredentials>,
    ) -> Result<Self> {
        use crate::providers::{
            bittensor_lium::BittensorLiumAdapter, lambda_labs::LambdaLabsAdapter,
            modal::ModalAdapter, runpod::RunPodAdapter, vast_ai::VastAiAdapter,
        };
        let mut clients: HashMap<CloudProvider, Box<dyn CloudProviderAdapter>> = HashMap::new();
        let mut regions = HashMap::new();
        for (provider, credentials) in credentials {
            if credentials.api_key.trim().is_empty() {
                return Err(Error::ConfigurationError(
                    "Provider API key is empty".into(),
                ));
            }
            let key = credentials.api_key.clone();
            let endpoint = credentials.endpoint.as_deref();
            let client: Box<dyn CloudProviderAdapter> = match (&provider, endpoint) {
                (CloudProvider::RunPod, Some(url)) => {
                    Box::new(RunPodAdapter::with_endpoint(key, url)?)
                }
                (CloudProvider::RunPod, None) => Box::new(RunPodAdapter::with_credentials(key)?),
                (CloudProvider::VastAi, Some(url)) => {
                    Box::new(VastAiAdapter::with_endpoint(key, url)?)
                }
                (CloudProvider::VastAi, None) => Box::new(VastAiAdapter::with_credentials(key)?),
                (CloudProvider::LambdaLabs, Some(url)) => {
                    Box::new(LambdaLabsAdapter::with_endpoint(key, url)?)
                }
                (CloudProvider::LambdaLabs, None) => {
                    Box::new(LambdaLabsAdapter::with_credentials(key)?)
                }
                (CloudProvider::BittensorLium, Some(url)) => {
                    Box::new(BittensorLiumAdapter::with_endpoint(key, url)?)
                }
                (CloudProvider::BittensorLium, None) => {
                    Box::new(BittensorLiumAdapter::with_credentials(key)?)
                }
                (CloudProvider::Modal, endpoint) => {
                    let secret = credentials
                        .api_secret
                        .as_ref()
                        .filter(|secret| !secret.is_empty())
                        .ok_or_else(|| {
                            Error::ConfigurationError("Modal requires its API secret".into())
                        })?;
                    let mut adapter = match endpoint {
                        Some(url) => ModalAdapter::with_endpoint(key, secret.clone(), url)?,
                        None => ModalAdapter::with_credentials(key, secret.clone())?,
                    };
                    if let Some(keys) = &credentials.rate_keys {
                        adapter = adapter.with_rate_keys(keys.clone());
                    }
                    Box::new(adapter)
                }
                _ => {
                    return Err(Error::ConfigurationError(
                        "Use a caller-configured adapter for this provider".into(),
                    ));
                }
            };
            if let Some(region) = &credentials.region {
                regions.insert(provider.clone(), region.clone());
            }
            clients.insert(provider, client);
        }
        let mut provisioner = Self::with_providers(clients);
        provisioner.regions = regions;
        Ok(provisioner)
    }

    /// List recoverable resources from configured accounts, including orphan firewalls.
    /// An error from any account prevents a partial list being mistaken for absence.
    pub async fn list_managed(&self, namespace: &str) -> Result<Vec<ManagedResource>> {
        ManagedLeaseIdentity {
            namespace: namespace.into(),
            lease_id: "listing".into(),
        }
        .validate()
        .map_err(Error::ConfigurationError)?;
        let mut resources = Vec::new();
        for adapter in self.providers.values() {
            resources.extend(adapter.list_managed(namespace).await?);
        }
        Ok(resources)
    }

    /// Create a tagged resource and return immediately after the provider gives its ID.
    /// The external orchestrator owns readiness, renewal and retirement. Repeating an
    /// identity adopts its existing VM instead of creating another one.
    pub async fn create_managed(&self, spec: &ResourceSpec) -> Result<ManagedResource> {
        let mut resolved = spec.clone();
        if resolved.region.is_none() {
            resolved.region = resolved
                .provider
                .as_ref()
                .and_then(|provider| self.regions.get(provider).cloned());
        }
        let spec = &resolved;
        spec.validate().map_err(Error::ConfigurationError)?;
        let identity = spec.ownership.as_ref().ok_or_else(|| {
            Error::ConfigurationError("Managed creation requires an ownership identity".into())
        })?;
        let provider = spec.provider.as_ref().ok_or_else(|| {
            Error::ConfigurationError("Managed creation requires an explicit provider".into())
        })?;
        if spec.lease_duration_secs.is_none_or(|ttl| ttl == 0) {
            return Err(Error::ConfigurationError(
                "Managed creation requires a positive lifetime".into(),
            ));
        }
        let _guard = ownership_lock(provider, identity).await?;
        let adapter = self.get_adapter(provider)?;
        let existing: Vec<_> = adapter
            .list_managed(&identity.namespace)
            .await?
            .into_iter()
            .filter(|resource| {
                resource.lease_id == identity.lease_id
                    && resource.namespace == identity.namespace
                    && resource.kind == ManagedResourceKind::Instance
            })
            .collect();
        if existing.len() > 1 {
            return Err(Error::Other(
                "Multiple provider resources share this lease identity; reconcile before creating"
                    .into(),
            ));
        }
        let fingerprint = request_fingerprint(spec)?;
        if let Some(resource) = existing.into_iter().next() {
            if resource.request_fingerprint.as_deref() != Some(fingerprint.as_str()) {
                return Err(Error::ConfigurationError(
                    "Existing lease allocation does not match this request".into(),
                ));
            }
            return Ok(resource);
        }
        let quote = self.quotes(spec).await?.into_iter().next().ok_or_else(|| {
            Error::ConfigurationError("No in-stock allocation has a known price".into())
        })?;
        let instance = adapter
            .provision_resources(
                &quote.selection.instance_type,
                &quote.region,
                spec,
                spec.tee_required,
            )
            .await?;
        // Return the native ID before any readiness or metadata HTTP request.
        // The orchestrator persists it, then observes billing and endpoints via status.
        let tls_mode = adapter.application_tls_mode(spec);
        Ok(ManagedResource {
            instance,
            namespace: identity.namespace.clone(),
            lease_id: identity.lease_id.clone(),
            kind: ManagedResourceKind::Instance,
            endpoint: None,
            tls_mode,
            request_fingerprint: Some(fingerprint),
            hourly_cost_usd: None,
            created_at: None,
            expires_at: None,
            expiry_source: None,
        })
    }

    /// Observe only a resource owned by this namespace and lease identity.
    pub async fn status_managed(
        &self,
        provider: &CloudProvider,
        identity: &ManagedLeaseIdentity,
        provider_id: &str,
    ) -> Result<Option<ManagedResource>> {
        identity.validate().map_err(Error::ConfigurationError)?;
        Ok(self
            .get_adapter(provider)?
            .list_managed(&identity.namespace)
            .await?
            .into_iter()
            .find(|resource| {
                resource.namespace == identity.namespace
                    && resource.lease_id == identity.lease_id
                    && resource.instance.id == provider_id
            }))
    }

    /// Retire every resource owned by this lease, then prove the ownership list empty.
    /// Listing also finds a Lambda firewall after its VM has disappeared.
    pub async fn destroy_managed(
        &self,
        provider: &CloudProvider,
        identity: &ManagedLeaseIdentity,
    ) -> Result<()> {
        identity.validate().map_err(Error::ConfigurationError)?;
        let _guard = ownership_lock(provider, identity).await?;
        let adapter = self.get_adapter(provider)?;
        let mut owned: Vec<_> = adapter
            .list_managed(&identity.namespace)
            .await?
            .into_iter()
            .filter(|resource| {
                resource.namespace == identity.namespace && resource.lease_id == identity.lease_id
            })
            .collect();
        owned.sort_by_key(|resource| resource.kind != ManagedResourceKind::Instance);
        for resource in owned {
            crate::leases::terminate_checked(adapter, &resource.instance.id).await?;
        }
        if adapter
            .list_managed(&identity.namespace)
            .await?
            .iter()
            .any(|resource| {
                resource.namespace == identity.namespace && resource.lease_id == identity.lease_id
            })
        {
            return Err(Error::Other(
                "Provider still reports resources for this lease".into(),
            ));
        }
        Ok(())
    }
}
