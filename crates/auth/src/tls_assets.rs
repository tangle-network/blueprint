//! TLS assets management module
//! Provides functionality to manage TLS certificates, keys, and related assets

use std::collections::HashMap;

use crate::db::{RocksDb, cf};
use crate::models::{TlsCertMetadata, TlsProfile};
use crate::tls_envelope::TlsEnvelope;
use blueprint_std::Rng;
use prost::Message;
use tracing::error;

use x509_parser::prelude::X509Certificate;

/// TLS asset manager for handling certificate and key operations
#[derive(Clone, Debug)]
pub struct TlsAssetManager {
    db: RocksDb,
    tls_envelope: TlsEnvelope,
}

impl TlsAssetManager {
    /// Create a new TLS asset manager
    pub fn new(db: RocksDb, tls_envelope: TlsEnvelope) -> Self {
        Self { db, tls_envelope }
    }

    /// Store a TLS profile for a service
    pub fn store_tls_profile(
        &self,
        service_id: u64,
        profile: TlsProfile,
    ) -> Result<(), crate::Error> {
        let key = format!("tls_profile:{service_id}");
        let value = profile.encode_to_vec();
        let cf_handle = self.db.cf_handle(cf::TLS_ASSETS_CF).unwrap();

        self.db.put_cf(&cf_handle, key.as_bytes(), &value)?;
        Ok(())
    }

    /// Retrieve a TLS profile for a service
    pub fn get_tls_profile(&self, service_id: u64) -> Result<Option<TlsProfile>, crate::Error> {
        let key = format!("tls_profile:{service_id}");
        let cf_handle = self.db.cf_handle(cf::TLS_ASSETS_CF).unwrap();

        match self.db.get_cf(&cf_handle, key.as_bytes())? {
            Some(value) => {
                let profile = TlsProfile::decode(&value[..])?;
                Ok(Some(profile))
            }
            None => Ok(None),
        }
    }

    /// Delete a TLS profile for a service
    pub fn delete_tls_profile(&self, service_id: u64) -> Result<(), crate::Error> {
        let key = format!("tls_profile:{service_id}");
        let cf_handle = self.db.cf_handle(cf::TLS_ASSETS_CF).unwrap();
        self.db.delete_cf(&cf_handle, key.as_bytes())?;
        Ok(())
    }

    /// Store certificate metadata
    pub fn store_cert_metadata(
        &self,
        cert_id: &str,
        metadata: TlsCertMetadata,
    ) -> Result<(), crate::Error> {
        let key = format!("cert_metadata:{cert_id}");
        let value = metadata.encode_to_vec();
        let cf_handle = self.db.cf_handle(cf::TLS_CERT_METADATA_CF).unwrap();

        self.db.put_cf(&cf_handle, key.as_bytes(), &value)?;
        Ok(())
    }

    /// Retrieve certificate metadata
    pub fn get_cert_metadata(
        &self,
        cert_id: &str,
    ) -> Result<Option<TlsCertMetadata>, crate::Error> {
        let key = format!("cert_metadata:{cert_id}");
        let cf_handle = self.db.cf_handle(cf::TLS_CERT_METADATA_CF).unwrap();

        match self.db.get_cf(&cf_handle, key.as_bytes())? {
            Some(value) => {
                let metadata = TlsCertMetadata::decode(&value[..])?;
                Ok(Some(metadata))
            }
            None => Ok(None),
        }
    }

    /// List all certificate metadata for a service
    pub fn list_cert_metadata(
        &self,
        service_id: u64,
    ) -> Result<Vec<TlsCertMetadata>, crate::Error> {
        let prefix = format!("cert_metadata:service:{service_id}:");
        let mut result = Vec::new();
        let cf_handle = self.db.cf_handle(cf::TLS_CERT_METADATA_CF).unwrap();

        for item in self.db.prefix_iterator_cf(&cf_handle, prefix.as_bytes()) {
            let (_key, value) = item?;
            let metadata = TlsCertMetadata::decode(&value[..])?;
            result.push(metadata);
        }

        Ok(result)
    }

    /// Store encrypted certificate data
    pub fn store_encrypted_cert(
        &self,
        cert_id: &str,
        cert_data: &[u8],
    ) -> Result<(), crate::Error> {
        let key = format!("cert_data:{cert_id}");
        let encrypted_data = self.tls_envelope.encrypt(cert_data)?;
        let cf_handle = self.db.cf_handle(cf::TLS_ASSETS_CF).unwrap();

        self.db
            .put_cf(&cf_handle, key.as_bytes(), &encrypted_data)?;
        Ok(())
    }

    /// Retrieve and decrypt certificate data
    pub fn get_encrypted_cert(&self, cert_id: &str) -> Result<Option<Vec<u8>>, crate::Error> {
        let key = format!("cert_data:{cert_id}");
        let cf_handle = self.db.cf_handle(cf::TLS_ASSETS_CF).unwrap();

        match self.db.get_cf(&cf_handle, key.as_bytes())? {
            Some(encrypted_data) => {
                let cert_data = self.tls_envelope.decrypt(&encrypted_data)?;
                Ok(Some(cert_data))
            }
            None => Ok(None),
        }
    }

    /// Store encrypted private key data
    pub fn store_encrypted_private_key(
        &self,
        key_id: &str,
        key_data: &[u8],
    ) -> Result<(), crate::Error> {
        let key = format!("private_key_data:{key_id}");
        let encrypted_data = self.tls_envelope.encrypt(key_data)?;
        let cf_handle = self.db.cf_handle(cf::TLS_ASSETS_CF).unwrap();

        self.db
            .put_cf(&cf_handle, key.as_bytes(), &encrypted_data)?;
        Ok(())
    }

    /// Retrieve and decrypt private key data
    pub fn get_encrypted_private_key(&self, key_id: &str) -> Result<Option<Vec<u8>>, crate::Error> {
        let key = format!("private_key_data:{key_id}");
        let cf_handle = self.db.cf_handle(cf::TLS_ASSETS_CF).unwrap();

        match self.db.get_cf(&cf_handle, key.as_bytes())? {
            Some(encrypted_data) => {
                let key_data = self.tls_envelope.decrypt(&encrypted_data)?;
                Ok(Some(key_data))
            }
            None => Ok(None),
        }
    }

    /// Store encrypted CA bundle
    pub fn store_encrypted_ca_bundle(
        &self,
        bundle_id: &str,
        bundle_data: &[u8],
    ) -> Result<(), crate::Error> {
        let key = format!("ca_bundle_data:{bundle_id}");
        let encrypted_data = self.tls_envelope.encrypt(bundle_data)?;
        let cf_handle = self.db.cf_handle(cf::TLS_ASSETS_CF).unwrap();

        self.db
            .put_cf(&cf_handle, key.as_bytes(), &encrypted_data)?;
        Ok(())
    }

    /// Retrieve and decrypt CA bundle
    pub fn get_encrypted_ca_bundle(
        &self,
        bundle_id: &str,
    ) -> Result<Option<Vec<u8>>, crate::Error> {
        let key = format!("ca_bundle_data:{bundle_id}");
        let cf_handle = self.db.cf_handle(cf::TLS_ASSETS_CF).unwrap();

        match self.db.get_cf(&cf_handle, key.as_bytes())? {
            Some(encrypted_data) => {
                let bundle_data = self.tls_envelope.decrypt(&encrypted_data)?;
                Ok(Some(bundle_data))
            }
            None => Ok(None),
        }
    }

    /// Log certificate issuance
    pub fn log_cert_issuance(&self, log_entry: &str) -> Result<(), crate::Error> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let key = format!("issuance_log:{timestamp}");
        let value = log_entry.as_bytes();
        let cf_handle = self.db.cf_handle(cf::TLS_ISSUANCE_LOG_CF).unwrap();

        self.db.put_cf(&cf_handle, key.as_bytes(), value)?;
        Ok(())
    }

    /// Get certificate issuance logs
    pub fn get_issuance_logs(
        &self,
        start_time: u64,
        end_time: u64,
    ) -> Result<Vec<String>, crate::Error> {
        let mut result = Vec::new();
        let cf_handle = self.db.cf_handle(cf::TLS_ISSUANCE_LOG_CF).unwrap();

        for item in self.db.prefix_iterator_cf(&cf_handle, b"issuance_log:") {
            let (key, value) = item?;
            // Extract timestamp from key
            let timestamp_str = String::from_utf8_lossy(&key[13..]); // Skip "issuance_log:"
            if let Ok(timestamp) = timestamp_str.parse::<u64>() {
                if timestamp >= start_time && timestamp <= end_time {
                    result.push(String::from_utf8_lossy(&value).to_string());
                }
            }
        }

        Ok(result)
    }

    /// Generate a unique certificate ID
    pub fn generate_cert_id(&self, service_id: u64) -> String {
        use blueprint_std::BlueprintRng;
        let mut rng = BlueprintRng::new();
        let random_bytes: [u8; 16] = rng.r#gen();
        format!("cert_{}_{}", service_id, hex::encode(random_bytes))
    }

    /// Generate a unique private key ID
    pub fn generate_key_id(&self, service_id: u64) -> String {
        use blueprint_std::BlueprintRng;
        let mut rng = BlueprintRng::new();
        let random_bytes: [u8; 16] = rng.r#gen();
        format!("key_{}_{}", service_id, hex::encode(random_bytes))
    }

    /// Validate certificate chain
    pub fn validate_certificate_chain(
        &self,
        cert_data: &[u8],
        ca_bundle_data: &[u8],
    ) -> Result<bool, crate::Error> {
        if cert_data.is_empty() || ca_bundle_data.is_empty() {
            return Ok(false);
        }

        let leaf_cert_der = extract_certificates(cert_data).map_err(|e| {
            crate::Error::Io(std::io::Error::other(format!(
                "Failed to parse certificate: {e}"
            )))
        })?;
        let leaf_der = leaf_cert_der
            .first()
            .ok_or_else(|| crate::Error::Io(std::io::Error::other("Certificate data missing")))?;
        let (_, leaf_cert) = X509Certificate::from_der(leaf_der).map_err(|e| {
            crate::Error::Io(std::io::Error::other(format!(
                "Invalid certificate DER: {e}"
            )))
        })?;

        let ca_ders = extract_certificates(ca_bundle_data).map_err(|e| {
            crate::Error::Io(std::io::Error::other(format!(
                "Failed to parse CA bundle: {e}"
            )))
        })?;

        for ca_der in ca_ders {
            match X509Certificate::from_der(&ca_der) {
                Ok((_, ca_cert)) => {
                    if leaf_cert
                        .verify_signature(Some(&ca_cert.public_key()))
                        .is_ok()
                    {
                        return Ok(true);
                    }
                }
                Err(err) => {
                    error!("Failed to parse CA certificate for validation: {err}");
                }
            }
        }

        Ok(false)
    }

    /// Check if certificate is expired
    pub fn is_certificate_expired(&self, not_after: u64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        now > not_after
    }

    /// Check if certificate is not yet valid
    pub fn is_certificate_not_yet_valid(&self, not_before: u64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        now < not_before
    }

    /// Get all TLS profiles for a service
    pub fn get_all_tls_profiles(&self) -> Result<HashMap<u64, TlsProfile>, crate::Error> {
        let mut result = HashMap::new();
        let cf_handle = self.db.cf_handle(cf::TLS_ASSETS_CF).unwrap();

        for item in self.db.prefix_iterator_cf(&cf_handle, b"tls_profile:") {
            let (key, value) = item?;
            // Extract service_id from key
            let service_id_str = String::from_utf8_lossy(&key[11..]); // Skip "tls_profile:"
            if let Ok(service_id) = service_id_str.parse::<u64>() {
                let profile = TlsProfile::decode(&value[..])?;
                result.insert(service_id, profile);
            }
        }

        Ok(result)
    }

    fn extract_certificates(input: &[u8]) -> Result<Vec<Vec<u8>>, String> {
        if input.is_empty() {
            return Ok(Vec::new());
        }

        if let Ok(pem_str) = std::str::from_utf8(input) {
            if let Ok(blocks) = pem::parse_many(pem_str) {
                let mut ders = Vec::new();
                for block in blocks {
                    if block.tag == "CERTIFICATE" {
                        ders.push(block.contents);
                    }
                }
                if !ders.is_empty() {
                    return Ok(ders);
                }
            }
        }

        Ok(vec![input.to_vec()])
    }

    /// Cleanup expired certificates
    pub fn cleanup_expired_certificates(&self) -> Result<u32, crate::Error> {
        let mut cleaned_count = 0;
        let metadata_cf_handle = self.db.cf_handle(cf::TLS_CERT_METADATA_CF).unwrap();
        let assets_cf_handle = self.db.cf_handle(cf::TLS_ASSETS_CF).unwrap();

        // Get all certificate metadata
        let mut errors = Vec::new();

        for item in self
            .db
            .prefix_iterator_cf(&metadata_cf_handle, b"cert_metadata:")
        {
            let (key, value) = item?;
            if let Ok(metadata) = TlsCertMetadata::decode(&value[..]) {
                if self.is_certificate_expired(metadata.expires_at) {
                    // Delete the metadata
                    self.db.delete_cf(&metadata_cf_handle, &key)?;

                    // Delete the encrypted certificate data
                    let cert_key = format!("cert_data:{}", metadata.cert_id);
                    if let Err(err) = self.db.delete_cf(&assets_cf_handle, cert_key.as_bytes()) {
                        error!(
                            certificate_id = %metadata.cert_id,
                            error = %err,
                            "failed to delete encrypted certificate data during cleanup"
                        );
                        errors.push(format!("cert_data:{}: {}", metadata.cert_id, err));
                    }

                    // Delete the encrypted private key data
                    let key_key = format!("private_key_data:{}", metadata.cert_id);
                    if let Err(err) = self.db.delete_cf(&assets_cf_handle, key_key.as_bytes()) {
                        error!(
                            certificate_id = %metadata.cert_id,
                            error = %err,
                            "failed to delete encrypted private key during cleanup"
                        );
                        errors.push(format!("private_key_data:{}: {}", metadata.cert_id, err));
                    }

                    cleaned_count += 1;
                }
            }
        }

        if errors.is_empty() {
            Ok(cleaned_count)
        } else {
            Err(crate::Error::Tls(format!(
                "cleaned {cleaned_count} expired certificates but encountered errors: {}",
                errors.join("; ")
            )))
        }
    }
}
