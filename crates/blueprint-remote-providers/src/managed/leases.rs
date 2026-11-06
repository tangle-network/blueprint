//! Rented GPU nodes with per-lease SSH access; provider tags carry each lease's deadline.

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
    /// Present only when the SDK generated the key; hand it to the lessee alone.
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
                pair.public_key().as_ref().try_into().map_err(|_| failed())?,
            ),
            private: Ed25519PrivateKey::from_bytes(&seed),
        };
        let key = ssh_key::PrivateKey::new(KeypairData::Ed25519(keypair), "")
            .map_err(|_| failed())?;
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
            .field("private_key", &self.private_key.as_ref().map(|_| "[REDACTED]"))
            .finish()
    }
}

/// A rented node. `resource.expires_at` is the deadline recorded with the provider.
#[derive(Debug)]
pub struct Lease {
    pub resource: ManagedResource,
    pub access: LeaseAccess,
}

impl CloudProvisioner {
    /// Rent the best-ranked candidate in stock for `ttl`, with per-lease SSH access.
    ///
    /// Only refusals proving nothing was rented move on to the next quote. Any other
    /// failure returns at once; `retire_expired` or `destroy_managed` then cleans up.
    pub async fn allocate(&self, spec: &ResourceSpec, ttl: Duration) -> Result<Lease> {
        if spec.ownership.is_none() || ttl.as_secs() == 0 {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        let access = match spec.ssh_public_key.as_deref() {
            Some(key) => LeaseAccess {
                public_key: crate::security::canonical_public_key(key)?,
                private_key: None,
            },
            None => LeaseAccess::generate()?,
        };
        let mut request = spec.clone();
        request.ssh_public_key = Some(access.public_key.clone());
        request.lease_duration_secs = Some(ttl.as_secs());
        let quotes = self.quotes(&request).await?;
        let mut refusal = quotes
            .errors
            .first()
            .map_or(ProviderErrorCode::NoStock, |failure| failure.code);
        for quote in quotes.quotes {
            match self.create_managed(&request, &quote.candidate).await {
                Ok(resource) => return Ok(Lease { resource, access }),
                Err(Error::Provider(
                    code @ (ProviderErrorCode::NoStock
                    | ProviderErrorCode::SkuUnknown
                    | ProviderErrorCode::Quota
                    | ProviderErrorCode::RateLimited
                    | ProviderErrorCode::Unsupported),
                )) => refusal = code,
                Err(error) => return Err(error),
            }
        }
        Err(Error::Provider(refusal))
    }

    /// Retire every lease in `namespace` whose provider-recorded deadline has passed.
    ///
    /// The provider listing is the only state read, so this is safe after a restart.
    /// Every expired lease is attempted; the first failure is returned afterwards.
    pub async fn retire_expired(&self, namespace: &str) -> Result<Vec<ManagedLeaseIdentity>> {
        let now = chrono::Utc::now();
        let expired: Vec<(CloudProvider, ManagedLeaseIdentity, String)> = self
            .list_managed(namespace)
            .await?
            .into_iter()
            .filter(|resource| {
                resource.kind == ManagedResourceKind::Instance
                    && resource.is_present()
                    && resource.expires_at.is_some_and(|deadline| deadline <= now)
            })
            .map(|resource| {
                let identity = ManagedLeaseIdentity {
                    namespace: resource.namespace,
                    lease_id: resource.lease_id,
                };
                (resource.instance.provider, identity, resource.instance.id)
            })
            .collect();
        let mut retired = Vec::new();
        let mut failure = None;
        for (provider, identity, id) in expired {
            match self.destroy_managed(&provider, &identity, Some(&id)).await {
                Ok(()) => retired.push(identity),
                Err(error) => {
                    failure.get_or_insert(error);
                }
            }
        }
        failure.map_or(Ok(retired), Err)
    }
}
