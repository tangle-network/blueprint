use super::quotes::{Candidate, resolve, valid_offer};
use super::*;
use crate::{CloudProvider, CloudProvisioner, Error};

fn owned(
    resource: &ManagedResource,
    provider: &CloudProvider,
    identity: &ManagedLeaseIdentity,
) -> Result<()> {
    if &resource.instance.provider != provider
        || resource.namespace != identity.namespace
        || resource.lease_id != identity.lease_id
    {
        return Err(Error::Provider(ProviderErrorCode::Conflict));
    }
    Ok(())
}

impl CloudProvisioner {
    /// Rent the selected candidate once; uncertain outcomes are never failed over.
    pub async fn create_managed(
        &self,
        spec: &ResourceSpec,
        candidate: &str,
    ) -> Result<ManagedResource> {
        spec.validate()
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        let identity = spec
            .ownership
            .as_ref()
            .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))?;
        if spec.lease_duration_secs.is_none_or(|seconds| seconds == 0) {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        let candidate = Candidate::decode(candidate)?;
        if candidate.request_fingerprint != request_fingerprint(spec)? {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        let arm_matches = match &candidate.arm {
            None => spec.arms.is_empty(),
            Some(selected) => spec
                .arms
                .iter()
                .any(|arm| serde_json::to_value(arm).ok() == serde_json::to_value(selected).ok()),
        };
        let resolved = resolve(spec, candidate.arm.as_ref());
        if !arm_matches || !valid_offer(&resolved, &candidate.offer) {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        let provider = &candidate.offer.provider;
        if spec
            .region
            .as_ref()
            .or_else(|| self.regions.get(provider))
            .is_some_and(|region| region != &candidate.offer.region)
        {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        let _guard = super::ownership::lock(provider, identity).await?;
        let native = self.managed_adapter(provider)?;
        let fingerprint = candidate.allocation_fingerprint()?;
        let existing = native.list(&identity.namespace).await?;
        let existing: Vec<_> = existing
            .into_iter()
            .filter(|resource| {
                resource.namespace == identity.namespace
                    && resource.lease_id == identity.lease_id
                    && resource.kind == ManagedResourceKind::Instance
                    && resource.is_present()
            })
            .collect();
        if existing.len() > 1 {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        if let Some(resource) = existing.into_iter().next() {
            owned(&resource, provider, identity)?;
            if resource.request_fingerprint.as_deref() != Some(fingerprint.as_str()) {
                return Err(Error::Provider(ProviderErrorCode::Conflict));
            }
            return Ok(resource);
        }
        let fresh = native
            .offers(&resolved, &candidate.offer.region)
            .await?
            .into_iter()
            .find(|offer| {
                offer.provider == candidate.offer.provider
                    && offer.native_id == candidate.offer.native_id
                    && offer.instance_type == candidate.offer.instance_type
                    && offer.region == candidate.offer.region
                    && offer.gpu_class == candidate.offer.gpu_class
                    && offer.gpu_count == candidate.offer.gpu_count
                    && offer.spot == candidate.offer.spot
                    && offer.hourly_cost_usd <= candidate.offer.hourly_cost_usd
                    && valid_offer(&resolved, offer)
            })
            .ok_or(Error::Provider(ProviderErrorCode::NoStock))?;
        let resource = native.create(&resolved, &fresh, &fingerprint).await?;
        owned(&resource, provider, identity)?;
        if resource.request_fingerprint.as_deref() != Some(fingerprint.as_str()) {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        Ok(resource)
    }

    pub async fn status_managed(
        &self,
        provider: &CloudProvider,
        identity: &ManagedLeaseIdentity,
        provider_id: &str,
    ) -> Result<Option<ManagedResource>> {
        identity
            .validate()
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        if provider_id.is_empty() {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        let result = self
            .managed_adapter(provider)?
            .status(identity, provider_id)
            .await?;
        if let Some(resource) = &result {
            owned(resource, provider, identity)?;
            if resource.instance.id != provider_id {
                return Err(Error::Provider(ProviderErrorCode::Conflict));
            }
        }
        Ok(result)
    }

    /// A failed account never becomes a partial absence proof.
    pub async fn list_managed(&self, namespace: &str) -> Result<Vec<ManagedResource>> {
        ManagedLeaseIdentity {
            namespace: namespace.into(),
            lease_id: "listing".into(),
        }
        .validate()
        .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        let mut resources = Vec::new();
        if self.providers.is_empty() {
            return Err(Error::Provider(ProviderErrorCode::ProviderNotConfigured));
        }
        for (provider, adapter) in &self.providers {
            let native = adapter
                .managed()
                .ok_or(Error::Provider(ProviderErrorCode::Unsupported))?;
            let listed = native.list(namespace).await?;
            if listed.iter().any(|resource| {
                &resource.instance.provider != provider || resource.namespace != namespace
            }) {
                return Err(Error::Provider(ProviderErrorCode::Conflict));
            }
            resources.extend(listed);
        }
        Ok(resources)
    }

    /// Unknown native IDs are retired by exact caller identity, never by account.
    pub async fn destroy_managed(
        &self,
        provider: &CloudProvider,
        identity: &ManagedLeaseIdentity,
        provider_id: Option<&str>,
    ) -> Result<()> {
        identity
            .validate()
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        let _guard = super::ownership::lock(provider, identity).await?;
        let native = self.managed_adapter(provider)?;
        let resources = if let Some(id) = provider_id {
            self.status_managed(provider, identity, id)
                .await?
                .into_iter()
                .collect()
        } else {
            native
                .list(&identity.namespace)
                .await?
                .into_iter()
                .filter(|resource| resource.lease_id == identity.lease_id)
                .collect::<Vec<_>>()
        };
        for resource in resources {
            owned(&resource, provider, identity)?;
            native.destroy(identity, &resource).await?;
        }
        if native
            .list(&identity.namespace)
            .await?
            .iter()
            .any(|resource| {
                resource.namespace == identity.namespace
                    && resource.lease_id == identity.lease_id
                    && resource.is_present()
            })
        {
            return Err(Error::Provider(ProviderErrorCode::StillPresent));
        }
        Ok(())
    }
}
