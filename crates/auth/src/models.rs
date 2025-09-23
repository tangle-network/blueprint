use axum::http::uri;
use base64::Engine;
use blueprint_core::debug;
use prost::Message;
use std::collections::BTreeMap;
use tracing::instrument;

use crate::{
    Error,
    api_tokens::{CUSTOM_ENGINE, GeneratedApiToken},
    db::{RocksDb, cf},
    types::{KeyType, ServiceId},
};

#[derive(prost::Message, Clone)]
pub struct ApiTokenModel {
    /// The token ID.
    #[prost(uint64)]
    id: u64,
    /// The token hash.
    #[prost(string)]
    token: String,
    /// The service ID this token is associated with.
    #[prost(uint64)]
    service_id: u64,
    /// The sub-service ID this token is associated with (zero means no sub-service).
    #[prost(uint64)]
    sub_service_id: u64,
    /// The token's expiration time in seconds since the epoch.
    ///
    /// Zero means no expiration.
    #[prost(uint64)]
    pub expires_at: u64,
    /// Whether the token is enabled.
    #[prost(bool)]
    pub is_enabled: bool,
    /// Additional headers to be forwarded to the upstream service.
    #[prost(bytes)]
    pub additional_headers: Vec<u8>,
}

/// TLS profile configuration for a service
#[derive(prost::Message, Clone, serde::Serialize, serde::Deserialize)]
pub struct TlsProfile {
    /// Whether TLS is enabled for this service
    #[prost(bool)]
    pub tls_enabled: bool,
    /// Whether client mTLS is required
    #[prost(bool)]
    pub require_client_mtls: bool,
    /// Encrypted server certificate PEM
    #[prost(bytes)]
    pub encrypted_server_cert: Vec<u8>,
    /// Encrypted server private key PEM
    #[prost(bytes)]
    pub encrypted_server_key: Vec<u8>,
    /// Encrypted client CA bundle PEM
    #[prost(bytes)]
    pub encrypted_client_ca_bundle: Vec<u8>,
    /// Encrypted upstream CA bundle PEM
    #[prost(bytes)]
    pub encrypted_upstream_ca_bundle: Vec<u8>,
    /// Encrypted upstream client certificate PEM
    #[prost(bytes)]
    pub encrypted_upstream_client_cert: Vec<u8>,
    /// Encrypted upstream client private key PEM
    #[prost(bytes)]
    pub encrypted_upstream_client_key: Vec<u8>,
    /// Maximum client certificate TTL in hours
    #[prost(uint32)]
    pub client_cert_ttl_hours: u32,
    /// Optional SNI hostname for this service
    #[prost(string, optional)]
    pub sni: Option<String>,
    /// Template to derive subjectAltNames for issued certificates
    #[prost(string, optional)]
    pub subject_alt_name_template: Option<String>,
    /// Allowed DNS names for issued certificates
    #[prost(string, repeated)]
    pub allowed_dns_names: Vec<String>,
}

/// Represents a service model stored in the database.
#[derive(prost::Message, Clone)]
pub struct ServiceModel {
    /// The service API Key prefix.
    #[prost(string)]
    pub api_key_prefix: String,
    /// A List of service owners.
    #[prost(message, repeated)]
    pub owners: Vec<ServiceOwnerModel>,
    /// The service upstream URL.
    ///
    /// This what the proxy will use to forward requests to the service.
    #[prost(string)]
    pub upstream_url: String,
    /// TLS profile configuration for this service
    #[prost(message, optional)]
    pub tls_profile: Option<TlsProfile>,
}

/// A service owner model stored in the database.
#[derive(prost::Message, Clone, PartialEq, Eq)]
pub struct ServiceOwnerModel {
    /// The Public key type.
    ///
    /// See [`KeyType`] for more details.
    #[prost(enumeration = "KeyType")]
    pub key_type: i32,
    /// The public key bytes.
    #[prost(bytes)]
    pub key_bytes: Vec<u8>,
}

/// TLS certificate metadata for issued certificates
#[derive(prost::Message, Clone, PartialEq, Eq)]
pub struct TlsCertMetadata {
    /// The service ID this certificate belongs to
    #[prost(uint64)]
    pub service_id: u64,
    /// The certificate ID (unique within service)
    #[prost(string)]
    pub cert_id: String,
    /// Certificate PEM encoded
    #[prost(string)]
    pub certificate_pem: String,
    /// Certificate serial number
    #[prost(string)]
    pub serial: String,
    /// Certificate expiration timestamp (seconds since epoch)
    #[prost(uint64)]
    pub expires_at: u64,
    /// Whether the certificate is revoked
    #[prost(bool)]
    pub is_revoked: bool,
    /// Certificate usage (client, server, both)
    #[prost(string)]
    pub usage: String,
    /// Subject common name
    #[prost(string)]
    pub common_name: String,
    /// Subject alternative names
    #[prost(string, repeated)]
    pub subject_alt_names: Vec<String>,
    /// Issuance timestamp (seconds since epoch)
    #[prost(uint64)]
    pub issued_at: u64,
    /// Issuing API key ID (for auditing)
    #[prost(uint64)]
    pub issued_by_api_key_id: u64,
    /// Tenant ID associated with this certificate
    #[prost(string, optional)]
    pub tenant_id: Option<String>,
}

impl ApiTokenModel {
    /// Find a token by its ID in the database.
    #[instrument(skip(db), err)]
    pub fn find_token_id(id: u64, db: &RocksDb) -> Result<Option<Self>, crate::Error> {
        let cf = db
            .cf_handle(cf::TOKENS_OPTS_CF)
            .ok_or(crate::Error::UnknownColumnFamily(cf::TOKENS_OPTS_CF))?;
        let token_opts_bytes = db.get_pinned_cf(&cf, id.to_be_bytes())?;

        token_opts_bytes
            .map(|bytes| ApiTokenModel::decode(bytes.as_ref()))
            .transpose()
            .map_err(Into::into)
    }

    /// Checks if the given plaintext matches the stored token hash.
    #[instrument(skip(self), ret)]
    pub fn is(&self, plaintext: &str) -> bool {
        use tiny_keccak::Hasher;

        let mut hasher = tiny_keccak::Keccak::v256();
        hasher.update(plaintext.as_bytes());
        let mut output = [0u8; 32];
        hasher.finalize(&mut output);

        let token_hash = CUSTOM_ENGINE.encode(output);

        debug!(
            %plaintext,
            %self.token,
            %token_hash,
            token_match = self.token == token_hash,
            "Checking token match",
        );

        self.token == token_hash
    }

    /// Saves the token to the database and returns the ID.
    pub fn save(&mut self, db: &RocksDb) -> Result<u64, crate::Error> {
        let tokens_cf = db
            .cf_handle(cf::TOKENS_OPTS_CF)
            .ok_or(crate::Error::UnknownColumnFamily(cf::TOKENS_OPTS_CF))?;
        if self.id != 0 {
            // update the existing token
            let token_bytes = self.encode_to_vec();
            db.put_cf(&tokens_cf, self.id.to_be_bytes(), token_bytes)?;
            Ok(self.id)
        } else {
            self.create(db)
        }
    }

    fn create(&mut self, db: &RocksDb) -> Result<u64, crate::Error> {
        let tokens_cf = db
            .cf_handle(cf::TOKENS_OPTS_CF)
            .ok_or(crate::Error::UnknownColumnFamily(cf::TOKENS_OPTS_CF))?;

        let seq_cf = db
            .cf_handle(cf::SEQ_CF)
            .ok_or(crate::Error::UnknownColumnFamily(cf::SEQ_CF))?;

        let txn = db.transaction();
        // Increment the sequence number
        // internally, the adder merge operator will increment the sequence number
        let mut retry_count = 0;
        let max_retries = 10;
        loop {
            let result = txn.merge_cf(&seq_cf, b"tokens", 1u64.to_be_bytes());
            match result {
                Ok(()) => break,
                Err(e) if e.kind() == rocksdb::ErrorKind::Busy => {
                    retry_count += 1;
                    if retry_count >= max_retries {
                        return Err(crate::Error::RocksDB(e));
                    }
                }
                Err(e) if e.kind() == rocksdb::ErrorKind::TryAgain => {
                    retry_count += 1;
                    if retry_count >= max_retries {
                        return Err(crate::Error::RocksDB(e));
                    }
                }
                Err(e) => return Err(crate::Error::RocksDB(e)),
            }
        }

        let next_id = txn
            .get_cf(&seq_cf, b"tokens")?
            .map(|v| {
                let mut id = [0u8; 8];
                id.copy_from_slice(&v);
                u64::from_be_bytes(id)
            })
            .unwrap_or(0u64);
        self.id = next_id;
        let tokens_bytes = self.encode_to_vec();
        txn.put_cf(&tokens_cf, next_id.to_be_bytes(), tokens_bytes)?;
        // commit the transaction
        txn.commit()?;

        Ok(next_id)
    }

    /// Returns the token expiration time in milliseconds since the epoch.
    /// Zero means no expiration.
    pub fn expires_at(&self) -> Option<u64> {
        if self.expires_at == 0 {
            None
        } else {
            Some(self.expires_at)
        }
    }

    /// Checks if the token is expired.
    #[instrument(skip(self), ret)]
    pub fn is_expired(&self) -> bool {
        if self.expires_at == 0 {
            return false;
        }
        let now = std::time::SystemTime::now();
        let since_epoch = now
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();

        let now = since_epoch.as_secs();
        self.expires_at < now
    }

    /// Return the service ID associated with this token.
    pub fn service_id(&self) -> ServiceId {
        ServiceId::new(self.service_id).with_subservice(self.sub_service_id)
    }

    /// Get the additional headers as a BTreeMap
    pub fn get_additional_headers(&self) -> BTreeMap<String, String> {
        if self.additional_headers.is_empty() {
            BTreeMap::new()
        } else {
            serde_json::from_slice(&self.additional_headers).unwrap_or_default()
        }
    }

    /// Set the additional headers from a BTreeMap
    pub fn set_additional_headers(&mut self, headers: &BTreeMap<String, String>) {
        self.additional_headers = serde_json::to_vec(headers).unwrap_or_default();
    }
}

impl From<&GeneratedApiToken> for ApiTokenModel {
    fn from(token: &GeneratedApiToken) -> Self {
        let mut model = Self {
            id: 0,
            token: token.token.clone(),
            service_id: token.service_id.0,
            sub_service_id: token.service_id.1,
            expires_at: token.expires_at().unwrap_or(0),
            is_enabled: true,
            additional_headers: Vec::new(),
        };
        model.set_additional_headers(token.additional_headers());
        model
    }
}

impl ServiceModel {
    /// Find a service by its ID in the database.
    pub fn find_by_id(id: ServiceId, db: &RocksDb) -> Result<Option<Self>, crate::Error> {
        let cf = db
            .cf_handle(cf::SERVICES_USER_KEYS_CF)
            .ok_or(crate::Error::UnknownColumnFamily(cf::SERVICES_USER_KEYS_CF))?;
        let service_bytes = db.get_pinned_cf(&cf, id.to_be_bytes())?;

        service_bytes
            .map(|bytes| ServiceModel::decode(bytes.as_ref()))
            .transpose()
            .map_err(Into::into)
    }

    /// Saves the service to the database at the given ID.
    pub fn save(&self, id: ServiceId, db: &RocksDb) -> Result<(), crate::Error> {
        let cf = db
            .cf_handle(cf::SERVICES_USER_KEYS_CF)
            .ok_or(crate::Error::UnknownColumnFamily(cf::SERVICES_USER_KEYS_CF))?;
        let service_bytes = self.encode_to_vec();
        db.put_cf(&cf, id.to_be_bytes(), service_bytes)?;
        Ok(())
    }

    /// Deletes the service from the database.
    pub fn delete(id: ServiceId, db: &RocksDb) -> Result<(), crate::Error> {
        let cf = db
            .cf_handle(cf::SERVICES_USER_KEYS_CF)
            .ok_or(crate::Error::UnknownColumnFamily(cf::SERVICES_USER_KEYS_CF))?;
        db.delete_cf(&cf, id.to_be_bytes())?;
        Ok(())
    }

    pub fn api_key_prefix(&self) -> &str {
        &self.api_key_prefix
    }

    /// Checks if the service has a specific owner.
    pub fn is_owner(&self, key_type: KeyType, key_bytes: &[u8]) -> bool {
        self.owners
            .iter()
            .any(|owner| owner.key_type == key_type as i32 && owner.key_bytes == key_bytes)
    }

    /// Adds a new owner to the service.
    pub fn add_owner(&mut self, key_type: KeyType, key_bytes: Vec<u8>) {
        let owner = ServiceOwnerModel {
            key_type: key_type as i32,
            key_bytes,
        };
        self.owners.push(owner);
    }

    /// Removes an owner from the service.
    pub fn remove_owner(&mut self, key_type: KeyType, key_bytes: &[u8]) {
        self.owners
            .retain(|owner| !(owner.key_type == key_type as i32 && owner.key_bytes == key_bytes));
    }

    /// Returns the list of owners.
    pub fn owners(&self) -> &[ServiceOwnerModel] {
        &self.owners
    }

    /// Returns the upstream URL.
    pub fn upstream_url(&self) -> Result<uri::Uri, Error> {
        self.upstream_url.parse::<uri::Uri>().map_err(Into::into)
    }

    /// Get the TLS profile for this service
    pub fn tls_profile(&self) -> Option<&TlsProfile> {
        self.tls_profile.as_ref()
    }

    /// Set the TLS profile for this service
    pub fn set_tls_profile(&mut self, tls_profile: TlsProfile) {
        self.tls_profile = Some(tls_profile);
    }

    /// Check if TLS is enabled for this service
    pub fn is_tls_enabled(&self) -> bool {
        self.tls_profile
            .as_ref()
            .map(|p| p.tls_enabled)
            .unwrap_or(false)
    }

    /// Check if client mTLS is required for this service
    pub fn requires_client_mtls(&self) -> bool {
        self.tls_profile
            .as_ref()
            .map(|p| p.require_client_mtls)
            .unwrap_or(false)
    }
}

/// Configuration for creating a new TlsCertMetadata
pub struct TlsCertMetadataConfig {
    pub service_id: u64,
    pub cert_id: String,
    pub certificate_pem: String,
    pub serial: String,
    pub expires_at: u64,
    pub usage: String,
    pub common_name: String,
    pub issued_by_api_key_id: u64,
}

impl TlsCertMetadata {
    /// Create a new certificate metadata entry
    pub fn new(config: TlsCertMetadataConfig) -> Self {
        Self {
            service_id: config.service_id,
            cert_id: config.cert_id,
            certificate_pem: config.certificate_pem,
            serial: config.serial,
            expires_at: config.expires_at,
            is_revoked: false,
            usage: config.usage,
            common_name: config.common_name,
            subject_alt_names: Vec::new(),
            issued_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            issued_by_api_key_id: config.issued_by_api_key_id,
            tenant_id: None,
        }
    }

    /// Generate the database key for this certificate metadata
    pub fn db_key(&self) -> Vec<u8> {
        let mut key = Vec::with_capacity(16 + self.cert_id.len());
        key.extend_from_slice(&self.service_id.to_be_bytes());
        key.extend_from_slice(self.cert_id.as_bytes());
        key
    }

    /// Save the certificate metadata to the database
    pub fn save(&self, db: &RocksDb) -> Result<(), crate::Error> {
        let cf = db.cf_handle(crate::db::cf::TLS_CERT_METADATA_CF).ok_or(
            crate::Error::UnknownColumnFamily(crate::db::cf::TLS_CERT_METADATA_CF),
        )?;

        let key = self.db_key();
        let metadata_bytes = self.encode_to_vec();
        db.put_cf(&cf, key, metadata_bytes)?;
        Ok(())
    }

    /// Find certificate metadata by service ID and certificate ID
    pub fn find_by_service_and_cert_id(
        service_id: u64,
        cert_id: &str,
        db: &RocksDb,
    ) -> Result<Option<Self>, crate::Error> {
        let cf = db.cf_handle(crate::db::cf::TLS_CERT_METADATA_CF).ok_or(
            crate::Error::UnknownColumnFamily(crate::db::cf::TLS_CERT_METADATA_CF),
        )?;

        let mut key = Vec::with_capacity(16 + cert_id.len());
        key.extend_from_slice(&service_id.to_be_bytes());
        key.extend_from_slice(cert_id.as_bytes());

        let metadata_bytes = db.get_pinned_cf(&cf, key)?;

        metadata_bytes
            .map(|bytes| TlsCertMetadata::decode(bytes.as_ref()))
            .transpose()
            .map_err(Into::into)
    }

    /// Check if the certificate is expired
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.expires_at < now
    }

    /// Revoke the certificate
    pub fn revoke(&mut self) {
        self.is_revoked = true;
    }

    /// Add a subject alternative name
    pub fn add_subject_alt_name(&mut self, san: String) {
        self.subject_alt_names.push(san);
    }

    /// Set the tenant ID
    pub fn set_tenant_id(&mut self, tenant_id: String) {
        self.tenant_id = Some(tenant_id);
    }
}

/// Helper functions for TLS asset management
pub mod tls_assets {
    use super::*;
    use crate::db::cf;

    /// Store encrypted TLS asset in the database
    pub fn store_tls_asset(
        db: &RocksDb,
        service_id: u64,
        asset_type: &str,
        encrypted_data: &[u8],
    ) -> Result<(), crate::Error> {
        let cf = db
            .cf_handle(cf::TLS_ASSETS_CF)
            .ok_or(crate::Error::UnknownColumnFamily(cf::TLS_ASSETS_CF))?;

        let key = format!("{service_id}:{asset_type}");
        db.put_cf(&cf, key.as_bytes(), encrypted_data)?;
        Ok(())
    }

    /// Retrieve encrypted TLS asset from the database
    pub fn get_tls_asset(
        db: &RocksDb,
        service_id: u64,
        asset_type: &str,
    ) -> Result<Option<Vec<u8>>, crate::Error> {
        let cf = db
            .cf_handle(cf::TLS_ASSETS_CF)
            .ok_or(crate::Error::UnknownColumnFamily(cf::TLS_ASSETS_CF))?;

        let key = format!("{service_id}:{asset_type}");
        let asset_bytes = db.get_pinned_cf(&cf, key.as_bytes())?;
        Ok(asset_bytes.map(|bytes| bytes.to_vec()))
    }

    /// Delete TLS asset from the database
    pub fn delete_tls_asset(
        db: &RocksDb,
        service_id: u64,
        asset_type: &str,
    ) -> Result<(), crate::Error> {
        let cf = db
            .cf_handle(cf::TLS_ASSETS_CF)
            .ok_or(crate::Error::UnknownColumnFamily(cf::TLS_ASSETS_CF))?;

        let key = format!("{service_id}:{asset_type}");
        db.delete_cf(&cf, key.as_bytes())?;
        Ok(())
    }

    /// Log certificate issuance for auditing
    pub fn log_certificate_issuance(
        db: &RocksDb,
        service_id: u64,
        cert_id: &str,
        api_key_id: u64,
        tenant_id: Option<&str>,
    ) -> Result<(), crate::Error> {
        let cf = db
            .cf_handle(cf::TLS_ISSUANCE_LOG_CF)
            .ok_or(crate::Error::UnknownColumnFamily(cf::TLS_ISSUANCE_LOG_CF))?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let mut log_entry = Vec::new();
        log_entry.extend_from_slice(&timestamp.to_be_bytes());
        log_entry.extend_from_slice(&service_id.to_be_bytes());
        log_entry.extend_from_slice(cert_id.as_bytes());
        log_entry.push(0u8); // separator
        log_entry.extend_from_slice(&api_key_id.to_be_bytes());
        if let Some(tenant_id) = tenant_id {
            log_entry.extend_from_slice(tenant_id.as_bytes());
        }

        let log_key = format!("{timestamp}:{cert_id}");
        db.put_cf(&cf, log_key.as_bytes(), log_entry)?;
        Ok(())
    }

    /// Get certificate issuance log for a service
    pub fn get_certificate_issuance_log(
        db: &RocksDb,
        service_id: u64,
    ) -> Result<Vec<TlsCertMetadata>, crate::Error> {
        let cf = db
            .cf_handle(cf::TLS_CERT_METADATA_CF)
            .ok_or(crate::Error::UnknownColumnFamily(cf::TLS_CERT_METADATA_CF))?;

        let mut certificates = Vec::new();
        let prefix = service_id.to_be_bytes();

        // Iterate through all certificates for this service
        let iter = db.prefix_iterator_cf(&cf, prefix);
        for item in iter {
            let (_key, value) = item?;
            let metadata = TlsCertMetadata::decode(&*value)?;
            if metadata.service_id == service_id {
                certificates.push(metadata);
            }
        }

        Ok(certificates)
    }
}

#[cfg(test)]
mod tests {
    use crate::{api_tokens::ApiTokenGenerator, types::ServiceId};

    use super::*;

    #[test]
    fn token_generator() {
        let mut rng = blueprint_std::BlueprintRng::new();
        let tmp_dir = tempfile::tempdir().unwrap();
        let db = RocksDb::open(tmp_dir.path(), &Default::default()).unwrap();
        let service_id = ServiceId::new(1);
        let generator = ApiTokenGenerator::new();
        let token = generator.generate_token(service_id, &mut rng);
        let mut token = ApiTokenModel::from(&token);

        // Save the token to the database
        let id = token.save(&db).unwrap();
        assert_eq!(id, 1);

        // Find the token by ID
        let found_token = ApiTokenModel::find_token_id(id, &db).unwrap();
        assert!(found_token.is_some());
        let found_token = found_token.unwrap();
        assert_eq!(found_token.id, id);
        assert_eq!(found_token.token, token.token);
        assert_eq!(found_token.expires_at, token.expires_at);
        assert_eq!(found_token.is_enabled, token.is_enabled);
    }

    #[test]
    fn token_with_headers() {
        use std::collections::BTreeMap;

        let mut rng = blueprint_std::BlueprintRng::new();
        let tmp_dir = tempfile::tempdir().unwrap();
        let db = RocksDb::open(tmp_dir.path(), &Default::default()).unwrap();
        let service_id = ServiceId::new(1);
        let generator = ApiTokenGenerator::new();

        // Create headers
        let mut headers = BTreeMap::new();
        headers.insert("X-Tenant-Id".to_string(), "tenant123".to_string());
        headers.insert("X-User-Type".to_string(), "premium".to_string());

        // Generate token with headers
        let token = generator.generate_token_with_expiration_and_headers(
            service_id,
            0,
            headers.clone(),
            &mut rng,
        );
        let mut token_model = ApiTokenModel::from(&token);

        // Save the token to the database
        let id = token_model.save(&db).unwrap();

        // Find the token by ID
        let found_token = ApiTokenModel::find_token_id(id, &db).unwrap().unwrap();

        // Verify headers are preserved
        let found_headers = found_token.get_additional_headers();
        assert_eq!(found_headers, headers);
    }

    #[test]
    fn test_additional_headers_methods() {
        use std::collections::BTreeMap;

        let mut token_model = ApiTokenModel {
            id: 0,
            token: "test".to_string(),
            service_id: 1,
            sub_service_id: 0,
            expires_at: 0,
            is_enabled: true,
            additional_headers: Vec::new(),
        };

        // Test empty headers
        assert!(token_model.get_additional_headers().is_empty());

        // Set headers
        let mut headers = BTreeMap::new();
        headers.insert("X-Test".to_string(), "value".to_string());
        token_model.set_additional_headers(&headers);

        // Get headers back
        let retrieved = token_model.get_additional_headers();
        assert_eq!(retrieved, headers);
    }
}
