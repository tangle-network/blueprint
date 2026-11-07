use super::quotes::{Candidate, resolve, selector, valid_offer};
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
    /// Rent an explicit original quote. With a lease state context, record its
    /// authorization before creating; without one this is the legacy low-level path.
    pub async fn create_managed(
        &self,
        spec: &ResourceSpec,
        candidate: &str,
    ) -> Result<ManagedResource> {
        let identity = spec
            .ownership
            .as_ref()
            .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))?;
        let mut fenced = false;
        if let Some(root) = self.authorization_dir.as_deref() {
            let _guard = super::ownership::lock_at(root, "allocation", identity).await?;
            if let Some(row) = self.authorization(identity)? {
                return self
                    .recover_authorized(spec, &row)
                    .await?
                    .ok_or(Error::Provider(ProviderErrorCode::Conflict));
            }
            self.require_absent(identity).await?;
            return self
                .create_prepared(spec, candidate, Some(spec), &mut fenced)
                .await;
        }
        self.create_prepared(spec, candidate, None, &mut fenced)
            .await
    }

    pub(super) fn validate_candidate(
        &self,
        spec: &ResourceSpec,
        candidate: &Candidate,
    ) -> Result<ResourceSpec> {
        spec.validate()
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        if spec.lease_duration_secs.is_none_or(|seconds| seconds == 0) {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        if candidate.request_fingerprint != request_fingerprint(spec)? {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        let canonical_arm = match &candidate.arm {
            None if spec.arms.is_empty() => None,
            None => return Err(Error::Provider(ProviderErrorCode::InvalidRequest)),
            Some(selected) => Some(
                spec.arms
                    .iter()
                    .find_map(|arm| {
                        // Legacy full-arm candidates retain exact equality. New selectors
                        // compare against the original arm with only measurements cleared.
                        let expected = if selected.measurements.is_empty() {
                            selector(arm)
                        } else {
                            arm.clone()
                        };
                        serde_json::to_value(expected)
                            .is_ok_and(|expected| {
                                serde_json::to_value(selected)
                                    .is_ok_and(|selected| expected == selected)
                            })
                            .then_some(arm)
                    })
                    .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))?,
            ),
        };
        let resolved = resolve(spec, canonical_arm);
        if !valid_offer(&resolved, &candidate.offer) {
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
        Ok(resolved)
    }

    pub(super) async fn finish_assignment(
        &self,
        spec: &ResourceSpec,
        candidate: &Candidate,
        resource: ManagedResource,
        metadata: Option<&metadata::ManagedMetadata>,
        observed: bool,
    ) -> Result<ManagedResource> {
        let resolved = self.validate_candidate(spec, candidate)?;
        let identity = spec
            .ownership
            .as_ref()
            .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))?;
        let provider = &candidate.offer.provider;
        let native = self.managed_adapter(provider)?;
        let fingerprint = candidate.allocation_fingerprint()?;
        let validate = |resource: &ManagedResource| {
            owned(resource, provider, identity)?;
            if resource.instance.id.is_empty()
                || resource.kind != ManagedResourceKind::Instance
                || resource.request_fingerprint.as_deref() != Some(fingerprint.as_str())
            {
                return Err(Error::Provider(ProviderErrorCode::Conflict));
            }
            Ok(())
        };
        let assigned = |resource: &ManagedResource, id: &str| {
            validate(resource)?;
            let (Some(class), Some(count), Some(vram), Some(price)) = (
                resource.gpu_class.as_deref(),
                resource.gpu_count,
                resource.gpu_vram_gb,
                resource.hourly_cost_usd,
            ) else {
                return Err(Error::Provider(ProviderErrorCode::Indeterminate));
            };
            if resource.instance.region.is_empty() || !price.is_finite() || price <= 0.0 {
                return Err(Error::Provider(ProviderErrorCode::Indeterminate));
            }
            if metadata.is_some_and(|metadata| resource.expires_at != Some(metadata.expires_at))
                || resource.instance.id != id
                || class != candidate.offer.gpu_class
                || count != candidate.offer.gpu_count
                || resource.instance.region != candidate.offer.region
                || vram < resolved.gpu_vram_gb.unwrap_or(0)
                || price
                    > resolved
                        .max_hourly_cost_usd
                        .unwrap_or(candidate.offer.hourly_cost_usd)
                        .min(candidate.offer.hourly_cost_usd)
            {
                return Err(Error::Provider(ProviderErrorCode::Conflict));
            }
            Ok(())
        };
        if let Some(metadata) = metadata {
            metadata.check_deadline()?;
        }
        if observed {
            assigned(&resource, &resource.instance.id)?;
        } else {
            validate(&resource)?;
        }
        let id = resource.instance.id.clone();
        let resource = native
            .status(identity, &id)
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::Indeterminate))?;
        assigned(&resource, &id)?;
        let resource = native.complete(&resolved, resource).await?;
        assigned(&resource, &id)?;
        if let Some(metadata) = metadata {
            metadata.check_deadline()?;
        }
        Ok(resource)
    }

    pub(super) async fn create_prepared(
        &self,
        spec: &ResourceSpec,
        encoded: &str,
        intent: Option<&ResourceSpec>,
        fenced: &mut bool,
    ) -> Result<ManagedResource> {
        let candidate = Candidate::decode(encoded)?;
        let resolved = self.validate_candidate(spec, &candidate)?;
        let provider = &candidate.offer.provider;
        if self
            .selection
            .as_ref()
            .is_some_and(|selected| !selected.contains(provider))
        {
            return Err(Error::Provider(ProviderErrorCode::ProviderNotConfigured));
        }
        let identity = spec
            .ownership
            .as_ref()
            .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))?;
        let _guard = super::ownership::lock(provider, identity).await?;
        let native = self.managed_adapter(provider)?;
        let existing = native.list(&identity.namespace).await?;
        let owned = |resource: &ManagedResource| {
            resource.namespace == identity.namespace
                && resource.lease_id == identity.lease_id
                && resource.is_present()
        };
        let template_expiry = if intent.is_none() {
            existing
                .iter()
                .find(|resource| owned(resource) && resource.kind == ManagedResourceKind::Template)
                .and_then(|resource| resource.expires_at)
        } else {
            None
        };
        let mut existing = existing
            .into_iter()
            .filter(|resource| owned(resource) && resource.kind == ManagedResourceKind::Instance);
        if let Some(resource) = existing.next() {
            if intent.is_some() || existing.next().is_some() {
                return Err(Error::Provider(ProviderErrorCode::Conflict));
            }
            return self
                .finish_assignment(spec, &candidate, resource, None, true)
                .await;
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
        let mut metadata = metadata::ManagedMetadata::for_spec(
            spec,
            &candidate.allocation_fingerprint()?,
            chrono::Utc::now(),
        )?;
        if let Some(expires_at) = template_expiry {
            // Legacy quoted retries keep the orphan template's deadline. The
            // adapter still requires exact original metadata before any POST.
            metadata.created_at = expires_at
                .checked_sub_signed(metadata.expires_at - metadata.created_at)
                .filter(|created| *created <= chrono::Utc::now())
                .ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
            metadata.expires_at = expires_at;
        }
        if let Some(intent) = intent {
            let row = authorization::Authorization {
                intent: authorization::intent(intent)?,
                public_key: spec
                    .ssh_public_key
                    .clone()
                    .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))?,
                candidate: encoded.into(),
                metadata: metadata.clone(),
                accounts: self.accounts()?,
            };
            // Even a failed durable write is an uncertain fence, never a failover signal.
            *fenced = true;
            self.reserve(row).await?;
        }
        metadata.check_deadline()?;
        let resource = native.create(&resolved, &fresh, &metadata).await?;
        self.finish_assignment(spec, &candidate, resource, Some(&metadata), false)
            .await
    }

    pub(super) async fn require_absent(&self, identity: &ManagedLeaseIdentity) -> Result<()> {
        if self
            .list_managed(&identity.namespace)
            .await?
            .iter()
            .any(|resource| resource.lease_id == identity.lease_id && resource.is_present())
        {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        Ok(())
    }

    pub(super) async fn recover_authorized(
        &self,
        spec: &ResourceSpec,
        row: &authorization::Authorization,
    ) -> Result<Option<ManagedResource>> {
        if row.intent != authorization::intent(spec)? || row.accounts != self.accounts()? {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        let key = spec
            .ssh_public_key
            .as_deref()
            .ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
        if crate::security::canonical_public_key(key)?
            != crate::security::canonical_public_key(&row.public_key)?
        {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        let mut effective = spec.clone();
        effective.ssh_public_key = Some(row.public_key.clone());
        let candidate = Candidate::decode(&row.candidate)?;
        self.validate_candidate(&effective, &candidate)?;
        let identity = &row.metadata.identity;
        let resources = self.list_managed(&identity.namespace).await?;
        let mut matching = resources.into_iter().filter(|resource| {
            resource.lease_id == identity.lease_id
                && resource.is_present()
                && resource.kind == ManagedResourceKind::Instance
        });
        let Some(resource) = matching.next() else {
            return Ok(None);
        };
        if matching.next().is_some() {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        self.finish_assignment(&effective, &candidate, resource, Some(&row.metadata), true)
            .await
            .map(Some)
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
        if let Some(provider) = self.init_errors.keys().next() {
            return Err(Error::ProviderNotConfigured(provider.clone()));
        }
        if self.providers.is_empty() {
            return Err(Error::Provider(ProviderErrorCode::ProviderNotConfigured));
        }
        for (provider, adapter) in &self.providers {
            let Some(native) = adapter.managed() else {
                continue;
            };
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
        if self.authorization_dir.is_some()
            && let Some(row) = self.authorization(identity)?
            && row.accounts.get(provider.catalog_key()) != Some(&self.account_binding(provider)?)
        {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        let _guard = super::ownership::lock(provider, identity).await?;
        let native = self.managed_adapter(provider)?;
        let primary = match provider_id {
            Some(id) => self.status_managed(provider, identity, id).await?,
            None => None,
        };
        let mut resources = native.list(&identity.namespace).await?;
        resources.retain(|resource| resource.lease_id == identity.lease_id);
        if let Some(primary) = primary {
            if !resources.iter().any(|resource| {
                resource.instance.id == primary.instance.id && resource.kind == primary.kind
            }) {
                resources.push(primary);
            }
        }
        for resource in &resources {
            owned(resource, provider, identity)?;
        }
        resources.sort_by_key(|resource| resource.kind != ManagedResourceKind::Instance);
        for resource in resources {
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
