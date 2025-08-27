use crate::db::{RocksDb, cf};
use crate::types::ServiceId;
use serde::{Deserialize, Serialize};

pub mod token;

pub fn rate_limit_check(
    db: &RocksDb,
    headers: &axum::http::HeaderMap,
    service_id: ServiceId,
    window_secs: u64,
    max_requests: u64,
) -> Result<(), String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let window = now / window_secs;
    let ip = headers
        .get(axum::http::header::FORWARDED)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");
    let key = format!("rl:{}:{}", service_id.to_string(), ip);

    let cf = db
        .cf_handle(cf::OAUTH_RL_CF)
        .ok_or_else(|| "rl store unavailable".to_string())?;

    let bucket_key = format!("{}:{}", key, window);

    let current = db
        .get_cf(&cf, bucket_key.as_bytes())
        .map_err(|_| "rl read error".to_string())?
        .and_then(|b| {
            String::from_utf8(b)
                .ok()
                .and_then(|s| s.parse::<u64>().ok())
        })
        .unwrap_or(0);

    if current >= max_requests {
        return Err("rate limit exceeded".into());
    }

    let new_val = (current + 1).to_string();
    db.put_cf(&cf, bucket_key.as_bytes(), new_val.as_bytes())
        .map_err(|_| "rl write error".to_string())?;
    Ok(())
}

/// Per-service OAuth policy
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceOAuthPolicy {
    /// Allowed assertion issuers (iss)
    pub allowed_issuers: Vec<String>,
    /// Required audience (aud); if empty, defaults to proxy origin policy
    pub required_audiences: Vec<String>,
    /// Allowed public keys in PEM (RSA/EC) used to verify assertions for this service
    /// At least one key must validate the signature
    pub public_keys_pem: Vec<String>,
    /// Allowed scopes; if None, scopes are not permitted and will be dropped
    pub allowed_scopes: Option<Vec<String>>,
    /// Whether DPoP is required for access token use at the proxy
    pub require_dpop: bool,
    /// Maximum access token TTL in seconds
    pub max_access_token_ttl_secs: u64,
    /// Maximum assertion age in seconds (exp - iat)
    pub max_assertion_ttl_secs: u64,
}

impl ServiceOAuthPolicy {
    pub fn load(service_id: ServiceId, db: &RocksDb) -> Result<Option<Self>, crate::Error> {
        let cf =
            db.cf_handle(cf::SERVICES_OAUTH_POLICY_CF)
                .ok_or(crate::Error::UnknownColumnFamily(
                    cf::SERVICES_OAUTH_POLICY_CF,
                ))?;
        if let Some(bytes) = db.get_cf(&cf, service_id.to_be_bytes())? {
            let policy: Self =
                serde_json::from_slice(&bytes).map_err(|_| crate::Error::UnknownKeyType)?;
            Ok(Some(policy))
        } else {
            Ok(None)
        }
    }

    pub fn save(&self, service_id: ServiceId, db: &RocksDb) -> Result<(), crate::Error> {
        let cf =
            db.cf_handle(cf::SERVICES_OAUTH_POLICY_CF)
                .ok_or(crate::Error::UnknownColumnFamily(
                    cf::SERVICES_OAUTH_POLICY_CF,
                ))?;
        let bytes = serde_json::to_vec(self).map_err(|_| crate::Error::UnknownKeyType)?;
        db.put_cf(&cf, service_id.to_be_bytes(), bytes)?;
        Ok(())
    }
}

/// OAuth assertion claims (subset)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssertionClaims {
    pub iss: String,
    pub sub: String,
    pub aud: Option<String>,
    pub iat: Option<u64>,
    pub exp: Option<u64>,
    pub jti: Option<String>,
    #[serde(default)]
    pub scope: Option<String>,
}

/// Result of assertion verification
#[derive(Debug, Clone)]
pub struct VerifiedAssertion {
    pub issuer: String,
    pub subject: String,
    pub audience: Option<String>,
    pub scopes: Option<Vec<String>>,
}

/// Assertion verifier
pub struct AssertionVerifier<'a> {
    db: &'a RocksDb,
}

impl<'a> AssertionVerifier<'a> {
    pub fn new(db: &'a RocksDb) -> Self {
        Self { db }
    }

    /// Verify JWT assertion against policy and return verified data
    pub fn verify(
        &self,
        jwt: &str,
        policy: &ServiceOAuthPolicy,
    ) -> Result<VerifiedAssertion, VerificationError> {
        if policy.allowed_issuers.is_empty() {
            return Err(VerificationError::PolicyViolation(
                "no issuers configured".into(),
            ));
        }

        // Decode header to inspect alg/kid
        let header = jsonwebtoken::decode_header(jwt)
            .map_err(|e| VerificationError::InvalidRequest(format!("invalid jwt header: {}", e)))?;
        let alg = header.alg;
        if !matches!(
            alg,
            jsonwebtoken::Algorithm::RS256 | jsonwebtoken::Algorithm::ES256
        ) {
            return Err(VerificationError::InvalidRequest("unsupported alg".into()));
        }

        // Build validation rules
        let mut validation = jsonwebtoken::Validation::new(alg);
        validation.set_required_spec_claims(&["iss", "sub", "exp", "iat"]);
        if !policy.required_audiences.is_empty() {
            validation.set_audience(&policy.required_audiences);
        } else {
            validation.validate_aud = false;
        }

        if policy.public_keys_pem.is_empty() {
            return Err(VerificationError::NotConfigured);
        }

        // Try each provided public key until signature validates
        let mut last_err: Option<jsonwebtoken::errors::Error> = None;
        let mut decoded: Option<jsonwebtoken::TokenData<AssertionClaims>> = None;
        for pem in &policy.public_keys_pem {
            // Accept PEM-formatted public keys. Convert to DER for jsonwebtoken.
            let pem_str = pem.trim();
            let key_res = if alg == jsonwebtoken::Algorithm::RS256 {
                pem::parse(pem_str)
                    .ok()
                    .filter(|p| p.tag == "PUBLIC KEY")
                    .map(|p| jsonwebtoken::DecodingKey::from_rsa_der(&p.contents))
            } else {
                pem::parse(pem_str)
                    .ok()
                    .filter(|p| p.tag == "PUBLIC KEY")
                    .map(|p| jsonwebtoken::DecodingKey::from_ec_der(&p.contents))
            };

            if let Some(k) = key_res {
                match jsonwebtoken::decode::<AssertionClaims>(jwt, &k, &validation) {
                    Ok(t) => {
                        decoded = Some(t);
                        break;
                    }
                    Err(e) => {
                        last_err = Some(e);
                    }
                }
            }
        }

        let token = decoded.ok_or_else(|| {
            VerificationError::InvalidGrant(format!(
                "signature verification failed: {}",
                last_err
                    .map(|e| e.to_string())
                    .unwrap_or_else(|| "unknown".into())
            ))
        })?;

        let claims = token.claims;

        // iss
        if !policy.allowed_issuers.iter().any(|i| i == &claims.iss) {
            return Err(VerificationError::InvalidGrant("issuer not allowed".into()));
        }

        // Validate iat/exp and assertion TTL
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let skew: u64 = 60;
        let iat = claims.iat.unwrap_or(0);
        let exp = claims.exp.unwrap_or(0);
        if iat > now + skew {
            return Err(VerificationError::InvalidGrant(
                "token used before issued (iat)".into(),
            ));
        }
        if exp + skew < now {
            return Err(VerificationError::InvalidGrant("token expired".into()));
        }
        if exp > iat && (exp - iat) > policy.max_assertion_ttl_secs {
            return Err(VerificationError::InvalidGrant(
                "assertion TTL exceeds policy".into(),
            ));
        }

        // Replay protection
        let jti = claims
            .jti
            .clone()
            .ok_or_else(|| VerificationError::InvalidRequest("missing jti".into()))?;
        if self.is_replayed(&jti, exp)? {
            return Err(VerificationError::InvalidGrant("assertion replayed".into()));
        }
        self.remember_jti(&jti, exp)?;

        // Scopes
        let scopes = claims
            .scope
            .as_ref()
            .map(|s| {
                s.split(' ')
                    .filter(|t| !t.is_empty())
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
            })
            .filter(|v| !v.is_empty());

        Ok(VerifiedAssertion {
            issuer: claims.iss,
            subject: claims.sub,
            audience: claims.aud.clone(),
            scopes,
        })
    }

    fn is_replayed(&self, jti: &str, now_or_exp: u64) -> Result<bool, VerificationError> {
        let cf = self
            .db
            .cf_handle(cf::OAUTH_JTI_CF)
            .ok_or_else(|| VerificationError::InvalidRequest("replay store unavailable".into()))?;
        if let Some(bytes) = self
            .db
            .get_cf(&cf, jti.as_bytes())
            .map_err(|_| VerificationError::InvalidRequest("replay read error".into()))?
        {
            if let Some(stored) = std::str::from_utf8(&bytes)
                .ok()
                .and_then(|s| s.parse::<u64>().ok())
            {
                return Ok(stored >= now_or_exp);
            }
        }
        Ok(false)
    }

    fn remember_jti(&self, jti: &str, exp: u64) -> Result<(), VerificationError> {
        let cf = self
            .db
            .cf_handle(cf::OAUTH_JTI_CF)
            .ok_or_else(|| VerificationError::InvalidRequest("replay store unavailable".into()))?;
        let val = exp.to_string();
        self.db
            .put_cf(&cf, jti.as_bytes(), val.as_bytes())
            .map_err(|_| VerificationError::InvalidRequest("replay write error".into()))?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VerificationError {
    #[error("invalid_request: {0}")]
    InvalidRequest(String),
    #[error("invalid_grant: {0}")]
    InvalidGrant(String),
    #[error("policy_violation: {0}")]
    PolicyViolation(String),
    #[error("not_configured")]
    NotConfigured,
}
