use axum::http::uri;
use prost::Message;

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
}

/// A service owner model stored in the database.
#[derive(prost::Message, Clone)]
pub struct ServiceOwnerModel {
    /// The Public key type.
    ///
    /// See [`KeyType`](crate::types::KeyType) for more details.
    #[prost(enumeration = "KeyType")]
    pub key_type: i32,
    /// The public key bytes.
    #[prost(bytes)]
    pub key_bytes: Vec<u8>,
}

impl ApiTokenModel {
    /// Find a token by its ID in the database.
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
    pub fn is(&self, plaintext: &str) -> bool {
        use tiny_keccak::Hasher;

        let mut hasher = tiny_keccak::Keccak::v256();
        hasher.update(plaintext.as_bytes());
        let mut output = [0u8; 32];
        hasher.finalize(&mut output);

        let token_hash = base64::Engine::encode(&CUSTOM_ENGINE, output);

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
}

impl From<&GeneratedApiToken> for ApiTokenModel {
    fn from(token: &GeneratedApiToken) -> Self {
        Self {
            id: 0,
            token: token.token.clone(),
            service_id: token.service_id.0,
            sub_service_id: token.service_id.1,
            expires_at: token.expires_at().unwrap_or(0),
            is_enabled: true,
        }
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
}

#[cfg(test)]
mod tests {
    use crate::{api_tokens::ApiTokenGenerator, types::ServiceId};

    use super::*;

    #[test]
    fn token_generator() {
        let mut rng = rand::thread_rng();
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
}
