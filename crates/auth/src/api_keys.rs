//! Long-lived API key management
//!
//! API keys are the primary authentication mechanism for services.
//! They follow the format `ak_<key_id>.<secret>` where:
//!
//! - `ak_` is a fixed prefix for identification
//! - `key_id` is a unique identifier (5 chars, base64url)
//! - `secret` is the secret part (32 bytes, base64url encoded)
//!
//! # Storage
//!
//! Keys are stored in RocksDB with:
//! - The full key is never stored, only a hash
//! - Key metadata includes service ID, creation time, etc.
//! - Supports expiration and enabled/disabled states
//!
//! # Usage
//!
//! API keys are exchanged for short-lived access tokens:
//!
//! ```text
//! // Client sends API key
//! Authorization: Bearer ak_2n4f8.w9x7y6z5a4b3c2d1
//!
//! // Server validates and returns access token
//! {
//!   "access_token": "v4.local.xxxxx",
//!   "expires_in": 900
//! }
//! ```

use base64::Engine;
use blueprint_std::rand::{CryptoRng, RngCore};
use prost::Message;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    Error,
    api_tokens::CUSTOM_ENGINE,
    db::{RocksDb, cf},
    types::ServiceId,
};

/// Long-lived API Key model stored in database
#[derive(prost::Message, Clone)]
pub struct ApiKeyModel {
    /// Unique database ID
    #[prost(uint64)]
    pub id: u64,
    /// API key identifier (e.g., "ak_2n4f8...")
    #[prost(string)]
    pub key_id: String,
    /// Hashed secret part of the API key
    #[prost(string)]
    pub key_hash: String,
    /// Service ID this key belongs to
    #[prost(uint64)]
    pub service_id: u64,
    /// Sub-service ID (zero means no sub-service)
    #[prost(uint64)]
    pub sub_service_id: u64,
    /// When this key was created (seconds since epoch)
    #[prost(uint64)]
    pub created_at: u64,
    /// When this key was last used (seconds since epoch)
    #[prost(uint64)]
    pub last_used: u64,
    /// When this key expires (seconds since epoch)
    #[prost(uint64)]
    pub expires_at: u64,
    /// Whether this key is enabled
    #[prost(bool)]
    pub is_enabled: bool,
    /// Default headers to include in access tokens (JSON-encoded)
    #[prost(bytes)]
    pub default_headers: Vec<u8>,
    /// Human-readable description
    #[prost(string)]
    pub description: String,
}

/// Generated API Key containing both public and secret parts
#[derive(Debug, Clone)]
pub struct GeneratedApiKey {
    /// Public key identifier
    pub key_id: String,
    /// Full API key (key_id + secret)
    pub full_key: String,
    /// Service ID this key is for
    pub service_id: ServiceId,
    /// Expiration timestamp
    pub expires_at: u64,
    /// Default headers
    pub default_headers: BTreeMap<String, String>,
}

/// API Key generator
pub struct ApiKeyGenerator {
    prefix: String,
}

impl Default for ApiKeyGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiKeyGenerator {
    /// Create new generator with default prefix "ak_"
    pub fn new() -> Self {
        Self {
            prefix: "ak_".to_string(),
        }
    }

    /// Create generator with custom prefix
    pub fn with_prefix(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }

    /// Generate a new API key
    pub fn generate_key<R: RngCore + CryptoRng>(
        &self,
        service_id: ServiceId,
        expires_at: u64,
        default_headers: BTreeMap<String, String>,
        rng: &mut R,
    ) -> GeneratedApiKey {
        // Generate 32 bytes of randomness
        let mut secret_bytes = vec![0u8; 32];
        rng.fill_bytes(&mut secret_bytes);

        // Create key identifier (first 8 bytes, base64 encoded)
        let key_id = format!(
            "{}{}",
            self.prefix,
            CUSTOM_ENGINE.encode(&secret_bytes[..8])
        );

        // Full key is key_id + "." + remaining 24 bytes base64 encoded
        let secret_part = CUSTOM_ENGINE.encode(&secret_bytes[8..]);
        let full_key = format!("{key_id}.{secret_part}");

        GeneratedApiKey {
            key_id,
            full_key,
            service_id,
            expires_at,
            default_headers,
        }
    }
}

impl GeneratedApiKey {
    /// Get the public key identifier
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    /// Get the full API key (to be shared with client)
    pub fn full_key(&self) -> &str {
        &self.full_key
    }

    /// Get expiration time
    pub fn expires_at(&self) -> u64 {
        self.expires_at
    }

    /// Get default headers
    pub fn default_headers(&self) -> &BTreeMap<String, String> {
        &self.default_headers
    }
}

impl ApiKeyModel {
    /// Find API key by key_id
    pub fn find_by_key_id(key_id: &str, db: &RocksDb) -> Result<Option<Self>, Error> {
        let cf = db
            .cf_handle(cf::API_KEYS_CF)
            .ok_or(Error::UnknownColumnFamily(cf::API_KEYS_CF))?;

        if let Some(key_bytes) = db.get_pinned_cf(&cf, key_id.as_bytes())? {
            let model = Self::decode(key_bytes.as_ref())?;
            Ok(Some(model))
        } else {
            Ok(None)
        }
    }

    /// Find API key by database ID
    pub fn find_by_id(id: u64, db: &RocksDb) -> Result<Option<Self>, Error> {
        let cf = db
            .cf_handle(cf::API_KEYS_BY_ID_CF)
            .ok_or(Error::UnknownColumnFamily(cf::API_KEYS_BY_ID_CF))?;

        if let Some(key_id_bytes) = db.get_pinned_cf(&cf, id.to_be_bytes())? {
            let key_id =
                String::from_utf8(key_id_bytes.to_vec()).map_err(|_| Error::UnknownKeyType)?; // Reusing error type
            Self::find_by_key_id(&key_id, db)
        } else {
            Ok(None)
        }
    }

    /// Validate if the given full key matches this stored key
    pub fn validates_key(&self, full_key: &str) -> bool {
        // Parse full key: "ak_xxxx.yyyy"
        if let Some((key_id_part, _)) = full_key.split_once('.') {
            if key_id_part != self.key_id {
                return false;
            }

            // Hash the full key and compare
            use tiny_keccak::Hasher;
            let mut hasher = tiny_keccak::Keccak::v256();
            hasher.update(full_key.as_bytes());
            let mut output = [0u8; 32];
            hasher.finalize(&mut output);
            let computed_hash = CUSTOM_ENGINE.encode(output);

            self.key_hash == computed_hash
        } else {
            false
        }
    }

    /// Check if key is expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now >= self.expires_at
    }

    /// Update last used timestamp
    pub fn update_last_used(&mut self, db: &RocksDb) -> Result<(), Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.last_used = now;
        self.save(db)
    }

    /// Get default headers as BTreeMap
    pub fn get_default_headers(&self) -> BTreeMap<String, String> {
        if self.default_headers.is_empty() {
            BTreeMap::new()
        } else {
            serde_json::from_slice(&self.default_headers).unwrap_or_default()
        }
    }

    /// Set default headers from BTreeMap
    pub fn set_default_headers(&mut self, headers: &BTreeMap<String, String>) {
        self.default_headers = serde_json::to_vec(headers).unwrap_or_default();
    }

    /// Get service ID
    pub fn service_id(&self) -> ServiceId {
        ServiceId::new(self.service_id).with_subservice(self.sub_service_id)
    }

    /// Save to database
    pub fn save(&mut self, db: &RocksDb) -> Result<(), Error> {
        let keys_cf = db
            .cf_handle(cf::API_KEYS_CF)
            .ok_or(Error::UnknownColumnFamily(cf::API_KEYS_CF))?;
        let ids_cf = db
            .cf_handle(cf::API_KEYS_BY_ID_CF)
            .ok_or(Error::UnknownColumnFamily(cf::API_KEYS_BY_ID_CF))?;

        if self.id == 0 {
            self.create(db)
        } else {
            // Update existing
            let key_bytes = self.encode_to_vec();
            db.put_cf(&keys_cf, self.key_id.as_bytes(), key_bytes)?;
            db.put_cf(&ids_cf, self.id.to_be_bytes(), self.key_id.as_bytes())?;
            Ok(())
        }
    }

    fn create(&mut self, db: &RocksDb) -> Result<(), Error> {
        let keys_cf = db
            .cf_handle(cf::API_KEYS_CF)
            .ok_or(Error::UnknownColumnFamily(cf::API_KEYS_CF))?;
        let ids_cf = db
            .cf_handle(cf::API_KEYS_BY_ID_CF)
            .ok_or(Error::UnknownColumnFamily(cf::API_KEYS_BY_ID_CF))?;
        let seq_cf = db
            .cf_handle(cf::SEQ_CF)
            .ok_or(Error::UnknownColumnFamily(cf::SEQ_CF))?;

        let txn = db.transaction();

        // Increment sequence
        let mut retry_count = 0;
        let max_retries = 10;
        loop {
            let result = txn.merge_cf(&seq_cf, b"api_keys", 1u64.to_be_bytes());
            match result {
                Ok(()) => break,
                Err(e)
                    if matches!(
                        e.kind(),
                        rocksdb::ErrorKind::Busy | rocksdb::ErrorKind::TryAgain
                    ) =>
                {
                    retry_count += 1;
                    if retry_count >= max_retries {
                        return Err(Error::RocksDB(e));
                    }
                }
                Err(e) => return Err(Error::RocksDB(e)),
            }
        }

        let next_id = txn
            .get_cf(&seq_cf, b"api_keys")?
            .map(|v| {
                let mut id = [0u8; 8];
                id.copy_from_slice(&v);
                u64::from_be_bytes(id)
            })
            .unwrap_or(1u64);

        self.id = next_id;
        let key_bytes = self.encode_to_vec();
        txn.put_cf(&keys_cf, self.key_id.as_bytes(), key_bytes)?;
        txn.put_cf(&ids_cf, next_id.to_be_bytes(), self.key_id.as_bytes())?;

        txn.commit()?;
        Ok(())
    }

    /// Delete from database
    pub fn delete(&self, db: &RocksDb) -> Result<(), Error> {
        let keys_cf = db
            .cf_handle(cf::API_KEYS_CF)
            .ok_or(Error::UnknownColumnFamily(cf::API_KEYS_CF))?;
        let ids_cf = db
            .cf_handle(cf::API_KEYS_BY_ID_CF)
            .ok_or(Error::UnknownColumnFamily(cf::API_KEYS_BY_ID_CF))?;

        let txn = db.transaction();
        txn.delete_cf(&keys_cf, self.key_id.as_bytes())?;
        txn.delete_cf(&ids_cf, self.id.to_be_bytes())?;
        txn.commit()?;
        Ok(())
    }
}

impl From<&GeneratedApiKey> for ApiKeyModel {
    fn from(key: &GeneratedApiKey) -> Self {
        use tiny_keccak::Hasher;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Hash the full key
        let mut hasher = tiny_keccak::Keccak::v256();
        hasher.update(key.full_key.as_bytes());
        let mut output = [0u8; 32];
        hasher.finalize(&mut output);
        let key_hash = CUSTOM_ENGINE.encode(output);

        let mut model = Self {
            id: 0,
            key_id: key.key_id.clone(),
            key_hash,
            service_id: key.service_id.0,
            sub_service_id: key.service_id.1,
            created_at: now,
            last_used: 0,
            expires_at: key.expires_at,
            is_enabled: true,
            default_headers: Vec::new(),
            description: String::new(),
        };

        model.set_default_headers(&key.default_headers);
        model
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ServiceId;
    use tempfile::tempdir;

    #[test]
    fn test_api_key_generation() {
        let mut rng = blueprint_std::BlueprintRng::new();
        let generator = ApiKeyGenerator::new();
        let service_id = ServiceId::new(1);
        let expires_at = 1234567890;
        let mut headers = BTreeMap::new();
        headers.insert("X-Tenant-Id".to_string(), "tenant123".to_string());

        let key = generator.generate_key(service_id, expires_at, headers.clone(), &mut rng);

        assert!(key.key_id().starts_with("ak_"));
        assert!(key.full_key().contains('.'));
        assert_eq!(key.expires_at(), expires_at);
        assert_eq!(key.default_headers(), &headers);

        // Should have format: ak_xxxx.yyyy
        let parts: Vec<&str> = key.full_key().split('.').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], key.key_id());
    }

    #[test]
    fn test_api_key_validation() {
        let mut rng = blueprint_std::BlueprintRng::new();
        let generator = ApiKeyGenerator::new();

        let key = generator.generate_key(ServiceId::new(1), 1234567890, BTreeMap::new(), &mut rng);

        let model = ApiKeyModel::from(&key);

        // Should validate correct key
        assert!(model.validates_key(&key.full_key()));

        // Should not validate incorrect key
        let wrong_key = key.full_key().replace('a', "b");
        assert!(!model.validates_key(&wrong_key));

        // Should not validate malformed key
        assert!(!model.validates_key("invalid"));
        assert!(!model.validates_key("ak_test"));
    }

    #[test]
    fn test_api_key_database_operations() {
        let tmp_dir = tempdir().unwrap();
        let db_config = crate::db::RocksDbConfig::default();
        let db = RocksDb::open(tmp_dir.path(), &db_config).unwrap();
        let mut rng = blueprint_std::BlueprintRng::new();
        let generator = ApiKeyGenerator::new();

        let key = generator.generate_key(ServiceId::new(1), 1234567890, BTreeMap::new(), &mut rng);

        let mut model = ApiKeyModel::from(&key);

        // Save should assign ID
        model.save(&db).unwrap();
        assert_ne!(model.id, 0);

        // Should be able to find by key_id
        let found = ApiKeyModel::find_by_key_id(&key.key_id(), &db)
            .unwrap()
            .unwrap();
        assert_eq!(found.key_id, model.key_id);
        assert_eq!(found.id, model.id);

        // Should be able to find by ID
        let found_by_id = ApiKeyModel::find_by_id(model.id, &db).unwrap().unwrap();
        assert_eq!(found_by_id.key_id, model.key_id);

        // Should validate the key
        assert!(found.validates_key(&key.full_key()));

        // Delete should work
        model.delete(&db).unwrap();
        assert!(
            ApiKeyModel::find_by_key_id(&key.key_id(), &db)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn test_expiration() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut model = ApiKeyModel {
            id: 1,
            key_id: "ak_test".to_string(),
            key_hash: "hash".to_string(),
            service_id: 1,
            sub_service_id: 0,
            created_at: now,
            last_used: 0,
            expires_at: now - 1, // Expired
            is_enabled: true,
            default_headers: Vec::new(),
            description: "Test".to_string(),
        };

        assert!(model.is_expired());

        model.expires_at = now + 3600; // Not expired
        assert!(!model.is_expired());
    }

    #[test]
    fn test_headers_serialization() {
        let mut model = ApiKeyModel {
            id: 1,
            key_id: "ak_test".to_string(),
            key_hash: "hash".to_string(),
            service_id: 1,
            sub_service_id: 0,
            created_at: 0,
            last_used: 0,
            expires_at: 0,
            is_enabled: true,
            default_headers: Vec::new(),
            description: "Test".to_string(),
        };

        let mut headers = BTreeMap::new();
        headers.insert("X-Test".to_string(), "value".to_string());

        model.set_default_headers(&headers);
        let retrieved = model.get_default_headers();

        assert_eq!(retrieved, headers);
    }
}
