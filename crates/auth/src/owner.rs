//! Resource-scoped EIP-191 sessions with live, application-owned authorization.
use crate::{
    db::RocksDb,
    models::ServiceModel,
    paseto_tokens::{AccessTokenClaims, PasetoTokenManager},
    types::{KeyType, ServiceId},
};
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    future::Future,
    time::{SystemTime, UNIX_EPOCH},
};
use tiny_keccak::{Hasher, Keccak};
use tokio::sync::Mutex;

/// Server-selected signing/token domain. Supply chain, operator and contract context when applicable.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerDomain {
    pub audience: String,
    pub chain_id: Option<u64>,
    pub operator: Option<[u8; 20]>,
    pub contract: Option<[u8; 20]>,
}

/// A resource within an existing service; the application chooses its canonical identifier.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceScope {
    pub service_id: ServiceId,
    pub resource: String,
}

/// Authoritative policy decision, checked again at challenge, issuance and access.
#[derive(Clone, Debug)]
pub struct OwnerGrant {
    pub expires_at: u64,
    pub scopes: Vec<String>,
}

/// The only application-specific part of scoped authentication.
pub trait OwnerPredicate: Send + Sync {
    fn authorize(
        &self,
        resource: &ResourceScope,
        owner: &[u8; 20],
    ) -> impl Future<Output = Result<OwnerGrant, OwnerAuthError>> + Send;
}

#[derive(Debug, thiserror::Error)]
pub enum OwnerAuthError {
    #[error("invalid authentication request")]
    InvalidRequest,
    #[error("unknown, expired, or consumed challenge")]
    InvalidChallenge,
    #[error("invalid EIP-191 signature")]
    InvalidSignature,
    #[error("resource access is not authorized")]
    Unauthorized,
    #[error("too many pending authentication challenges")]
    Capacity,
    #[error("authentication policy unavailable")]
    Unavailable,
}

/// Existing registered service-owner semantics, read from the authoritative database each time.
pub struct RegisteredServiceOwners {
    db: RocksDb,
    lifetime: u64,
}

impl OwnerPredicate for RegisteredServiceOwners {
    async fn authorize(
        &self,
        scope: &ResourceScope,
        owner: &[u8; 20],
    ) -> Result<OwnerGrant, OwnerAuthError> {
        let service = ServiceModel::find_by_id(scope.service_id, &self.db)
            .map_err(|_| OwnerAuthError::Unavailable)?
            .ok_or(OwnerAuthError::Unauthorized)?;
        if !service.owners.iter().any(|entry| {
            entry.key_type == KeyType::Ecdsa as i32
                && VerifyingKey::from_sec1_bytes(&entry.key_bytes)
                    .is_ok_and(|key| address(&key) == *owner)
        }) {
            return Err(OwnerAuthError::Unauthorized);
        }
        Ok(OwnerGrant {
            expires_at: now()?.saturating_add(self.lifetime),
            scopes: vec!["service:access".into()],
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OwnerChallenge {
    pub nonce: [u8; 32],
    /// Exact UTF-8 message for EIP-191 personal_sign.
    pub message: String,
    pub expires_at: u64,
}

#[derive(Serialize)]
pub struct OwnerSession {
    pub token: String,
    pub resource: ResourceScope,
    pub scopes: Vec<String>,
    pub expires_at: u64,
}

#[derive(Clone)]
struct Pending {
    challenge: OwnerChallenge,
    resource: ResourceScope,
    owner: [u8; 20],
    grant: OwnerGrant,
}

/// Handles signatures, one-use challenges and bounded tokens; the predicate only decides ownership.
pub struct ScopedOwnerAuth<P = RegisteredServiceOwners> {
    tokens: PasetoTokenManager,
    domain: OwnerDomain,
    owner: P,
    pending: Mutex<BTreeMap<[u8; 32], Pending>>,
}

impl ScopedOwnerAuth {
    /// Uses the existing registered service owners unless a custom predicate is supplied.
    pub fn new(
        tokens: PasetoTokenManager,
        domain: OwnerDomain,
        db: RocksDb,
    ) -> Result<Self, OwnerAuthError> {
        let lifetime = tokens.default_ttl().as_secs();
        Self::with_owner(tokens, domain, RegisteredServiceOwners { db, lifetime })
    }
}

impl<P: OwnerPredicate> ScopedOwnerAuth<P> {
    pub fn with_owner(
        tokens: PasetoTokenManager,
        domain: OwnerDomain,
        owner: P,
    ) -> Result<Self, OwnerAuthError> {
        if domain.audience.is_empty()
            || domain.audience.len() > 1024
            || domain.chain_id == Some(0)
            || domain.operator == Some([0; 20])
            || domain.contract == Some([0; 20])
            || tokens.default_ttl().as_secs() == 0
        {
            return Err(OwnerAuthError::InvalidRequest);
        }
        Ok(Self {
            tokens,
            domain,
            owner,
            pending: Mutex::new(BTreeMap::new()),
        })
    }

    async fn grant(
        &self,
        resource: &ResourceScope,
        owner: &[u8; 20],
    ) -> Result<OwnerGrant, OwnerAuthError> {
        if resource.resource.is_empty() || resource.resource.len() > 1024 {
            return Err(OwnerAuthError::InvalidRequest);
        }
        let grant = self.owner.authorize(resource, owner).await?;
        if grant.expires_at <= now()?
            || grant.scopes.is_empty()
            || grant.scopes.iter().any(String::is_empty)
        {
            return Err(OwnerAuthError::Unauthorized);
        }
        Ok(grant)
    }

    fn binding(&self, resource: &ResourceScope) -> Result<String, OwnerAuthError> {
        let bytes = serde_json::to_vec(&(&self.domain, resource))
            .map_err(|_| OwnerAuthError::InvalidRequest)?;
        Ok(format!("owner:{}", hex::encode(keccak(&bytes))))
    }

    pub async fn challenge(
        &self,
        resource: ResourceScope,
        owner: [u8; 20],
    ) -> Result<OwnerChallenge, OwnerAuthError> {
        let grant = self.grant(&resource, &owner).await?;
        let time = now()?;
        let expires_at = grant
            .expires_at
            .min(time.saturating_add(self.tokens.default_ttl().as_secs()));
        let nonce = crate::generate_challenge(&mut blueprint_std::BlueprintRng::new());
        let payload = serde_json::to_string(&(
            &self.domain,
            &resource,
            hex::encode(owner),
            hex::encode(nonce),
            expires_at,
            &grant.scopes,
        ))
        .map_err(|_| OwnerAuthError::InvalidRequest)?;
        let challenge = OwnerChallenge {
            nonce,
            message: format!("Blueprint resource access\n{payload}"),
            expires_at,
        };
        let mut pending = self.pending.lock().await;
        pending.retain(|_, request| request.challenge.expires_at > time);
        if pending.len() >= 4096 {
            return Err(OwnerAuthError::Capacity);
        }
        pending.insert(
            nonce,
            Pending {
                challenge: challenge.clone(),
                resource,
                owner,
                grant,
            },
        );
        Ok(challenge)
    }

    pub async fn verify(
        &self,
        nonce: &[u8; 32],
        signature: &[u8],
    ) -> Result<OwnerSession, OwnerAuthError> {
        let pending = self
            .pending
            .lock()
            .await
            .get(nonce)
            .cloned()
            .ok_or(OwnerAuthError::InvalidChallenge)?;
        if pending.challenge.expires_at <= now()? {
            return Err(OwnerAuthError::InvalidChallenge);
        }
        if recover_eip191(&pending.challenge.message, signature)? != pending.owner {
            return Err(OwnerAuthError::InvalidSignature);
        }
        let live = self.grant(&pending.resource, &pending.owner).await?;
        let time = now()?;
        let expires_at = live
            .expires_at
            .min(pending.grant.expires_at)
            .min(time.saturating_add(self.tokens.default_ttl().as_secs()));
        let scopes: Vec<_> = pending
            .grant
            .scopes
            .into_iter()
            .filter(|scope| live.scopes.contains(scope))
            .collect();
        if pending.challenge.expires_at <= time || expires_at <= time || scopes.is_empty() {
            return Err(OwnerAuthError::Unauthorized);
        }
        if self.pending.lock().await.remove(nonce).is_none() {
            return Err(OwnerAuthError::InvalidChallenge);
        }
        let mut claims = AccessTokenClaims::new(
            pending.resource.service_id,
            self.binding(&pending.resource)?,
            std::time::Duration::ZERO,
            Some(hex::encode(pending.owner)),
            BTreeMap::new(),
            Some(scopes.clone()),
        );
        claims.expires_at = expires_at;
        let token = self
            .tokens
            .generate_token_with_claims(&claims)
            .map_err(|_| OwnerAuthError::Unavailable)?;
        Ok(OwnerSession {
            token,
            resource: pending.resource,
            scopes,
            expires_at,
        })
    }

    /// Validate the token's exact domain/resource and recheck live ownership and requested scope.
    pub async fn authorize(
        &self,
        token: &str,
        resource: &ResourceScope,
        required_scope: &str,
    ) -> Result<AccessTokenClaims, OwnerAuthError> {
        let claims = self
            .tokens
            .validate_token(token)
            .map_err(|_| OwnerAuthError::Unauthorized)?;
        if claims.service_id != resource.service_id
            || claims.key_id != self.binding(resource)?
            || !claims
                .scopes
                .as_ref()
                .is_some_and(|scopes| scopes.iter().any(|scope| scope == required_scope))
        {
            return Err(OwnerAuthError::Unauthorized);
        }
        let owner: [u8; 20] = claims
            .tenant_id
            .as_ref()
            .and_then(|value| hex::decode(value).ok())
            .and_then(|bytes| bytes.try_into().ok())
            .ok_or(OwnerAuthError::Unauthorized)?;
        let live = self.grant(resource, &owner).await?;
        if claims.expires_at > live.expires_at
            || !live.scopes.iter().any(|scope| scope == required_scope)
            || claims.is_expired()
        {
            return Err(OwnerAuthError::Unauthorized);
        }
        Ok(claims)
    }
}

fn now() -> Result<u64, OwnerAuthError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|time| time.as_secs())
        .map_err(|_| OwnerAuthError::Unavailable)
}

fn keccak(bytes: &[u8]) -> [u8; 32] {
    let mut digest = [0; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes);
    hasher.finalize(&mut digest);
    digest
}

fn address(key: &VerifyingKey) -> [u8; 20] {
    keccak(&key.to_encoded_point(false).as_bytes()[1..])[12..]
        .try_into()
        .expect("address length")
}

/// Recover an Ethereum address from a 65-byte RSV personal_sign signature (v = 0/1 or 27/28).
pub fn recover_eip191(message: &str, signature: &[u8]) -> Result<[u8; 20], OwnerAuthError> {
    if signature.len() != 65 {
        return Err(OwnerAuthError::InvalidSignature);
    }
    let signature_value =
        Signature::from_slice(&signature[..64]).map_err(|_| OwnerAuthError::InvalidSignature)?;
    if signature_value.normalize_s().is_some() {
        return Err(OwnerAuthError::InvalidSignature);
    }
    let recovery = match signature[64] {
        value @ 0..=1 => value,
        value @ 27..=28 => value - 27,
        _ => return Err(OwnerAuthError::InvalidSignature),
    };
    let prefix = format!("\x19Ethereum Signed Message:\n{}", message.len());
    let digest = keccak(&[prefix.as_bytes(), message.as_bytes()].concat());
    VerifyingKey::recover_from_prehash(
        &digest,
        &signature_value,
        RecoveryId::from_byte(recovery).ok_or(OwnerAuthError::InvalidSignature)?,
    )
    .map(|key| address(&key))
    .map_err(|_| OwnerAuthError::InvalidSignature)
}
