//! GPU allocations with per-lease access and persisted expiry cleanup.

use crate::deployment::tracker::{CleanupHandler, DeploymentRecord, DeploymentType};
use crate::infra::traits::CloudProviderAdapter;
use crate::infra::types::InstanceStatus;
use crate::providers::common::InstanceSelection;
use crate::{CloudProvider, CloudProvisioner, DeploymentTracker, Error, ResourceSpec, Result};
use std::{collections::HashMap, path::PathBuf, sync::Arc, time::Duration};
use zeroize::Zeroizing;

/// A provider's current total USD/hour price for the requested node.
#[derive(Debug, Clone)]
pub struct ProviderQuote {
    pub provider: CloudProvider,
    pub region: String,
    pub pricing_source: &'static str,
    pub selection: InstanceSelection,
}

/// Access is unique to the allocation; provider account credentials stay with the caller.
pub struct LeaseAccess {
    pub public_key: String,
    private_key: Option<Zeroizing<String>>,
}

impl LeaseAccess {
    /// Transfer this key only to the authorized lessee, over an authenticated channel.
    pub fn private_key(&self) -> Option<&str> {
        self.private_key.as_deref().map(String::as_str)
    }
}

impl std::fmt::Debug for LeaseAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LeaseAccess")
            .field("private_key", &"[REDACTED]")
            .finish()
    }
}

/// A rented node. Expiry is persisted before access is returned.
pub struct Lease {
    pub id: String,
    pub instance_id: String,
    pub provider: CloudProvider,
    pub endpoint: String,
    pub access: LeaseAccess,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub hourly_cost_usd: f64,
    tracker: Arc<DeploymentTracker>,
}

impl Lease {
    /// Whether the persisted lease remains within its allocation period.
    pub async fn is_active(&self) -> Result<bool> {
        Ok(self.tracker.get(&self.id).await?.is_some() && chrono::Utc::now() < self.expires_at)
    }

    /// Retire early; successful expiry or an earlier termination makes this idempotent.
    pub async fn terminate(&self) -> Result<()> {
        if self.tracker.get(&self.id).await?.is_some() {
            self.tracker.handle_termination(&self.id).await?;
        }
        Ok(())
    }
}

struct LeaseCleanup {
    providers: Arc<HashMap<CloudProvider, Box<dyn CloudProviderAdapter>>>,
}

#[async_trait::async_trait]
impl CleanupHandler for LeaseCleanup {
    async fn cleanup(&self, record: &DeploymentRecord) -> Result<()> {
        let provider = record
            .provider
            .as_ref()
            .ok_or_else(|| Error::Other("Lease missing provider".into()))?;
        let adapter = self
            .providers
            .get(provider)
            .ok_or_else(|| Error::ProviderNotConfigured(provider.clone()))?;
        let id = record
            .resource_ids
            .get("instance_id")
            .ok_or_else(|| Error::Other("Lease missing instance id".into()))?;
        terminate_checked(adapter.as_ref(), id).await
    }
}

impl CloudProvisioner {
    /// Select a durable directory before allocation; reuse it after restarting.
    pub fn with_lease_store(mut self, path: impl Into<PathBuf>) -> Self {
        self.lease_store = path.into();
        self.lease_tracker = Arc::new(tokio::sync::OnceCell::new());
        self
    }

    pub fn with_region(mut self, provider: CloudProvider, region: impl Into<String>) -> Self {
        self.regions.insert(provider, region.into());
        self
    }

    /// Restore cleanup deadlines using the configured provider clients.
    pub async fn resume_leases(&self) -> Result<()> {
        self.lease_tracker().await.map(|_| ())
    }

    async fn lease_tracker(&self) -> Result<&Arc<DeploymentTracker>> {
        self.lease_tracker
            .get_or_try_init(|| async {
                tokio::fs::create_dir_all(&self.lease_store).await?;
                let tracker = Arc::new(DeploymentTracker::new(&self.lease_store).await?);
                tracker
                    .set_cleanup_handler(
                        DeploymentType::SshRemote,
                        Box::new(LeaseCleanup {
                            providers: self.providers.clone(),
                        }),
                    )
                    .await;
                tracker.resume_ttls().await;
                Ok(tracker)
            })
            .await
    }

    /// Compare only valid live prices across configured providers.
    pub async fn quotes(&self, spec: &ResourceSpec) -> Result<Vec<ProviderQuote>> {
        spec.validate().map_err(Error::ConfigurationError)?;
        let results = futures::future::join_all(
            self.providers
                .iter()
                .filter(|(provider, _)| {
                    spec.provider
                        .as_ref()
                        .is_none_or(|required| required == *provider)
                        && (!spec.tee_required || provider.supports_tee())
                })
                .map(|(provider, adapter)| async move {
                    let region = spec
                        .region
                        .clone()
                        .or_else(|| self.regions.get(provider).cloned())
                        .unwrap_or_default();
                    let mut selection = match adapter.quote(spec, &region).await {
                        Ok(selection) => selection,
                        Err(Error::AuthRejected) => return Err(Error::AuthRejected),
                        Err(_) => return Ok(None),
                    };
                    if let Some(count) = spec.gpu_count {
                        if selection.gpu_class.is_none() {
                            selection.gpu_class = blueprint_core::gpu::catalog()
                                .into_iter()
                                .find(|class| {
                                    class.sku(provider.catalog_key(), count)
                                        == Some(selection.instance_type.as_str())
                                })
                                .map(|class| class.id);
                        }
                        let Some(class) = selection.gpu_class.as_ref() else {
                            return Ok(None);
                        };
                        if spec
                            .gpu_class
                            .as_ref()
                            .is_some_and(|required| required != class)
                        {
                            return Ok(None);
                        }
                        let mut resolved = spec.clone();
                        resolved.gpu_class = Some(class.clone());
                        if resolved.validate().is_err() {
                            return Ok(None);
                        }
                    }
                    let Some(price) = selection.estimated_hourly_cost else {
                        return Ok(None);
                    };
                    Ok((price.is_finite()
                        && price > 0.0
                        && spec.max_hourly_cost_usd.is_none_or(|cap| price <= cap)
                        && !selection.instance_type.is_empty())
                    .then(|| ProviderQuote {
                        provider: provider.clone(),
                        region,
                        pricing_source: adapter.quote_pricing_source(),
                        selection,
                    }))
                }),
        )
        .await;
        let auth_rejected = results
            .iter()
            .any(|result| matches!(result, Err(Error::AuthRejected)));
        let mut quotes: Vec<_> = results
            .into_iter()
            .filter_map(Result::ok)
            .flatten()
            .collect();
        if quotes.is_empty() && auth_rejected {
            return Err(Error::AuthRejected);
        }
        quotes.sort_by(|a, b| {
            a.selection
                .estimated_hourly_cost
                .unwrap()
                .total_cmp(&b.selection.estimated_hourly_cost.unwrap())
                .then_with(|| a.provider.catalog_key().cmp(b.provider.catalog_key()))
        });
        Ok(quotes)
    }

    /// Rent the cheapest in-stock node, trying the next provider on create failure.
    pub async fn allocate(&self, spec: &ResourceSpec, ttl: Duration) -> Result<Lease> {
        let owner = self.clone();
        let request = spec.clone();
        let (sender, receiver) = tokio::sync::oneshot::channel();
        tokio::spawn(async move {
            let result = owner.allocate_owned(&request, ttl).await;
            if let Err(Ok(lease)) = sender.send(result) {
                if let Err(error) = lease.terminate().await {
                    blueprint_core::error!(lease_id = %lease.id, %error, "Cancelled allocation retirement failed; expiry record retained");
                }
            }
        });
        receiver
            .await
            .map_err(|_| Error::Other("Allocation task stopped before returning a lease".into()))?
    }

    async fn allocate_owned(&self, spec: &ResourceSpec, ttl: Duration) -> Result<Lease> {
        let duration = chrono::Duration::from_std(ttl).map_err(|_| {
            Error::ConfigurationError("Lease TTL exceeds supported duration".into())
        })?;
        let expires_at = chrono::Utc::now()
            .checked_add_signed(duration)
            .filter(|_| ttl.as_secs() > 0)
            .ok_or_else(|| {
                Error::ConfigurationError("Lease TTL must have a valid future expiry".into())
            })?;
        let tracker = self.lease_tracker().await?.clone();
        let mut request = spec.clone();
        let access = allocation_access(spec.ssh_public_key.as_deref()).await?;
        request.ssh_public_key = Some(access.public_key.clone());
        request.lease_duration_secs = Some(ttl.as_secs());
        let quotes = self.quotes(&request).await?;
        for quote in quotes {
            let adapter = self.get_adapter(&quote.provider)?;
            let instance = match adapter
                .provision_resources(
                    &quote.selection.instance_type,
                    &quote.region,
                    &request,
                    request.tee_required,
                )
                .await
            {
                Ok(instance) => instance,
                Err(_) => continue,
            };
            let id = format!("gpu-lease-{}", uuid::Uuid::new_v4());
            let mut record =
                DeploymentRecord::new(id.clone(), DeploymentType::SshRemote, request.clone(), None);
            record.ttl_seconds = Some(ttl.as_secs());
            record.expires_at = Some(expires_at);
            record.set_cloud_info(quote.provider.clone(), quote.region);
            record.add_resource("instance_id".into(), instance.id.clone());
            if let Err(error) = tracker.register_deployment(id.clone(), record).await {
                self.terminate(quote.provider, &instance.id).await?;
                return Err(error);
            }
            let ready = async {
                adapter.prepare_access(&instance, &request).await?;
                let remaining = (expires_at - chrono::Utc::now())
                    .to_std()
                    .map_err(|_| Error::Other("Lease expired during provisioning".into()))?;
                tokio::time::timeout(remaining, async {
                    let ready = crate::providers::common::gpu_adapter::wait_for_instance(
                        adapter,
                        &instance.id,
                    )
                    .await?;
                    adapter.lease_endpoint(&ready).await
                })
                .await
                .map_err(|_| Error::Other("Lease expired during provisioning".into()))?
            }
            .await;
            let endpoint = match ready {
                Ok(endpoint) => endpoint,
                Err(_) => {
                    tracker.handle_termination(&id).await?;
                    continue;
                }
            };
            return Ok(Lease {
                id,
                instance_id: instance.id,
                provider: quote.provider,
                endpoint,
                access,
                expires_at,
                hourly_cost_usd: quote.selection.estimated_hourly_cost.unwrap(),
                tracker,
            });
        }
        Err(Error::Other(
            "No priced provider could allocate the requested GPU resources".into(),
        ))
    }
}

pub(crate) async fn allocation_access(public_key: Option<&str>) -> Result<LeaseAccess> {
    if let Some(key) = public_key {
        if key.contains(['\r', '\n'])
            || !(key.starts_with("ssh-ed25519 ") || key.starts_with("ssh-rsa "))
        {
            return Err(Error::ConfigurationError("Invalid SSH public key".into()));
        }
        return Ok(LeaseAccess {
            public_key: key.into(),
            private_key: None,
        });
    }
    let directory = tempfile::tempdir()?;
    let path = directory.path().join("lease-key");
    let output = tokio::process::Command::new("ssh-keygen")
        .args(["-q", "-t", "ed25519", "-N", "", "-f"])
        .arg(&path)
        .output()
        .await?;
    if !output.status.success() {
        return Err(Error::Other("Unable to generate lease SSH key".into()));
    }
    let private_key = Zeroizing::new(tokio::fs::read_to_string(&path).await?);
    let public_key = tokio::fs::read_to_string(path.with_extension("pub")).await?;
    Ok(LeaseAccess {
        public_key: public_key.trim().into(),
        private_key: Some(private_key),
    })
}

pub(crate) async fn terminate_checked(adapter: &dyn CloudProviderAdapter, id: &str) -> Result<()> {
    adapter.terminate_instance(id).await?;
    for _ in 0..10 {
        match adapter.get_instance_status(id).await {
            Ok(InstanceStatus::Terminated) => return Ok(()),
            Err(error) if crate::infra::provisioner::is_not_found_error(&error) => return Ok(()),
            _ => tokio::time::sleep(Duration::from_secs(5)).await,
        }
    }
    Err(Error::Other(
        "Provider termination verification timed out".into(),
    ))
}
