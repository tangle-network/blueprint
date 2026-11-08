//! Rented GPU nodes with per-lease SSH access and durable original authorizations.

use super::*;
use crate::{CloudProvider, CloudProvisioner, Error};
use std::time::Duration;
use zeroize::Zeroizing;

/// Per-lease SSH access; provider account credentials stay with the allocator.
pub struct LeaseAccess {
    pub public_key: String,
    private_key: Option<Zeroizing<String>>,
}

impl LeaseAccess {
    fn public(public_key: String) -> Self {
        Self {
            public_key,
            private_key: None,
        }
    }

    pub fn private_key(&self) -> Option<&str> {
        self.private_key.as_deref().map(String::as_str)
    }

    fn generate() -> Result<Self> {
        use aws_lc_rs::{rand::SecureRandom, signature::KeyPair};
        use ssh_key::private::{Ed25519Keypair, Ed25519PrivateKey, KeypairData};
        let failed = || Error::Other("Unable to generate lease SSH key".into());
        let mut seed = Zeroizing::new([0u8; 32]);
        aws_lc_rs::rand::SystemRandom::new()
            .fill(seed.as_mut())
            .map_err(|_| failed())?;
        let pair = aws_lc_rs::signature::Ed25519KeyPair::from_seed_unchecked(seed.as_ref())
            .map_err(|_| failed())?;
        let keypair = Ed25519Keypair {
            public: ssh_key::public::Ed25519PublicKey(
                pair.public_key()
                    .as_ref()
                    .try_into()
                    .map_err(|_| failed())?,
            ),
            private: Ed25519PrivateKey::from_bytes(&seed),
        };
        let key =
            ssh_key::PrivateKey::new(KeypairData::Ed25519(keypair), "").map_err(|_| failed())?;
        Ok(Self {
            public_key: key.public_key().to_openssh().map_err(|_| failed())?,
            private_key: Some(
                key.to_openssh(ssh_key::LineEnding::LF)
                    .map_err(|_| failed())?,
            ),
        })
    }
}

impl core::fmt::Debug for LeaseAccess {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LeaseAccess")
            .field("public_key", &self.public_key)
            .field(
                "private_key",
                &self.private_key.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

/// A rented node. `resource.expires_at` is the deadline recorded with the provider.
#[derive(Debug)]
pub struct Lease {
    pub resource: ManagedResource,
    pub access: LeaseAccess,
}

/// Leases retired by `retire_expired`, and the providers it could not finish.
#[derive(Debug, Default)]
pub struct RetireReport {
    pub retired: Vec<ManagedLeaseIdentity>,
    pub failures: Vec<ProviderFailure>,
}

pub(super) fn failure(provider: &CloudProvider, code: ProviderErrorCode) -> ProviderFailure {
    ProviderFailure {
        provider: provider.clone(),
        code,
    }
}

impl RetireReport {
    fn fail(&mut self, provider: &CloudProvider, code: ProviderErrorCode) {
        self.failures.push(failure(provider, code));
    }
}

impl CloudProvisioner {
    fn lease_request(&self, spec: &ResourceSpec, ttl: Duration) -> Result<ResourceSpec> {
        self.lease_state_dir()?;
        if ttl.as_secs() == 0 || spec.ownership.is_none() {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        let mut request = spec.clone();
        request.lease_duration_secs = Some(ttl.as_secs());
        request
            .validate()
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        Ok(request)
    }

    /// Recover only: inspect all original configured accounts, never request capacity.
    /// A stable caller-owned public key and the original private authorization store
    /// are required. `None` is a complete absence proof, never permission to re-rent.
    pub async fn recover_lease(&self, spec: &ResourceSpec, ttl: Duration) -> Result<Option<Lease>> {
        let request = self.lease_request(spec, ttl)?;
        let identity = request.ownership.as_ref().unwrap();
        let _guard =
            super::ownership::lock_at(self.lease_state_dir()?, "allocation", identity).await?;
        self.accounts()?;
        match self.authorization(identity)? {
            Some(row) => self
                .recover_authorized(&request, &row)
                .await
                .map(|resource| {
                    resource.map(|resource| Lease {
                        resource,
                        access: LeaseAccess::public(row.public_key),
                    })
                }),
            None => {
                self.require_absent(identity).await?;
                Ok(None)
            }
        }
    }

    /// Rent once with a durable original quote/deadline and an immutable attempt fence.
    /// Fresh calls may generate an SSH key; recovery cannot replace a lost private key.
    /// An acknowledged or uncertain attempt never rents again for the same identity.
    pub async fn allocate(&self, spec: &ResourceSpec, ttl: Duration) -> Result<Lease> {
        let mut request = self.lease_request(spec, ttl)?;
        let identity = request.ownership.as_ref().unwrap().clone();
        let _guard =
            super::ownership::lock_at(self.lease_state_dir()?, "allocation", &identity).await?;
        self.accounts()?;
        if let Some(row) = self.authorization(&identity)? {
            let resource = self
                .recover_authorized(&request, &row)
                .await?
                .ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
            return Ok(Lease {
                resource,
                access: LeaseAccess::public(row.public_key),
            });
        }
        self.require_absent(&identity).await?;
        let intent = request.clone();
        let access = match request.ssh_public_key.as_deref() {
            Some(key) => LeaseAccess::public(crate::security::canonical_public_key(key)?),
            None => LeaseAccess::generate()?,
        };
        request.ssh_public_key = Some(access.public_key.clone());
        let quotes = self.quotes(&request).await?;
        let mut refusal = quotes
            .errors
            .first()
            .map_or(ProviderErrorCode::NoStock, |failure| failure.code);
        for quote in quotes.quotes {
            let mut fenced = false;
            match self
                .create_prepared(&request, &quote.candidate, Some(&intent), &mut fenced)
                .await
            {
                Ok(resource) => return Ok(Lease { resource, access }),
                Err(Error::Provider(
                    code @ (ProviderErrorCode::NoStock
                    | ProviderErrorCode::SkuUnknown
                    | ProviderErrorCode::Quota
                    | ProviderErrorCode::RateLimited
                    | ProviderErrorCode::Unsupported),
                )) if !fenced => refusal = code,
                Err(error) => return Err(error),
            }
        }
        Err(Error::Provider(refusal))
    }

    /// Retire leases in `namespace` past their original authorized deadline.
    ///
    /// With a state context, original private deadlines govern; provider metadata cannot extend them.
    /// A failing account never stops cleanup of another correctly bound account.
    pub async fn retire_expired(&self, namespace: &str) -> RetireReport {
        let now = chrono::Utc::now();
        let authorizations = self
            .authorization_dir
            .as_deref()
            .map(super::authorization::load)
            .transpose();
        let mut report = RetireReport::default();
        for provider in self
            .init_errors
            .keys()
            .chain(self.account_regions.keys().filter(|provider| {
                !self.providers.contains_key(*provider) && !self.init_errors.contains_key(*provider)
            }))
        {
            report.fail(provider, ProviderErrorCode::ProviderNotConfigured);
        }
        let entries = match &authorizations {
            Ok(store) => store.as_ref().map(|store| &store.entries),
            Err(_) => {
                for (provider, adapter) in &self.providers {
                    if adapter.managed().is_some() {
                        report.fail(provider, ProviderErrorCode::Conflict);
                    }
                }
                return report;
            }
        };
        // Stored obligations exist even when a different account lists no objects.
        // Eligibility is per authorization: a changed historical account must not
        // block another lease explicitly authorized against the current account.
        let mut eligible =
            std::collections::HashMap::<CloudProvider, std::collections::HashSet<String>>::new();
        if let Some(entries) = entries {
            for row in entries
                .values()
                .filter(|row| row.metadata.identity.namespace == namespace)
            {
                let provider = super::quotes::Candidate::decode(&row.candidate)
                    .expect("loaded authorizations have valid candidates")
                    .offer
                    .provider;
                if self.managed_adapter(&provider).is_err() {
                    report.fail(&provider, ProviderErrorCode::ProviderNotConfigured);
                } else if !self
                    .account_binding(&provider)
                    .is_ok_and(|binding| row.accounts.get(provider.catalog_key()) == Some(&binding))
                {
                    report.fail(&provider, ProviderErrorCode::Conflict);
                } else {
                    eligible
                        .entry(provider)
                        .or_default()
                        .insert(row.metadata.identity.lease_id.clone());
                }
            }
        }
        let providers = if entries.is_some() {
            eligible.keys().cloned().collect::<Vec<_>>()
        } else {
            self.providers.keys().cloned().collect()
        };
        for provider in &providers {
            let Some(native) = self
                .providers
                .get(provider)
                .and_then(|adapter| adapter.managed())
            else {
                continue;
            };
            let listed = match native.list(namespace).await {
                Ok(listed) => listed,
                Err(error) => {
                    report.fail(provider, error.into());
                    continue;
                }
            };
            let mut expired: Vec<ManagedLeaseIdentity> = Vec::new();
            for resource in listed {
                let deadline = if let Some(entries) = entries {
                    let original = entries
                        .get(&resource.lease_id)
                        .filter(|row| row.metadata.identity.namespace == namespace);
                    match original {
                        Some(row)
                            if eligible
                                .get(provider)
                                .is_some_and(|leases| leases.contains(&resource.lease_id)) =>
                        {
                            Some(row.metadata.expires_at)
                        }
                        _ => {
                            report.fail(provider, ProviderErrorCode::Conflict);
                            continue;
                        }
                    }
                } else {
                    resource.expires_at
                };
                let due = &resource.instance.provider == provider
                    && resource.is_present()
                    && deadline.is_some_and(|deadline| deadline <= now);
                let identity = ManagedLeaseIdentity {
                    namespace: resource.namespace,
                    lease_id: resource.lease_id,
                };
                if due && identity.namespace == namespace && !expired.contains(&identity) {
                    expired.push(identity);
                }
            }
            for identity in expired {
                match self.destroy_managed(provider, &identity, None).await {
                    Ok(()) => report.retired.push(identity),
                    Err(error) => report.fail(provider, error.into()),
                }
            }
        }
        report
    }
}
