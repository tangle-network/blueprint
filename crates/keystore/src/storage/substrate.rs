use blueprint_std::sync::Arc;

use super::RawStorage;
use crate::error::Result;
use blueprint_crypto::KeyTypeId;
use sp_core::Pair;
use sp_keystore::Keystore;
/// A substrate-backed local key storage
///
/// This wrapper is used to provide a substrate-backed local key storage.
/// It implements the [`RawStorage`] trait, which allows for storing and retrieving keys in a
/// substrate-compatible format.
pub struct SubstrateStorage {
    inner: Arc<sc_keystore::LocalKeystore>,
}

impl core::fmt::Debug for SubstrateStorage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SubstrateStorage")
            .field("inner", &"sc_keystore::LocalKeystore")
            .finish()
    }
}

impl Clone for SubstrateStorage {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

mod role_ecdsa {
    use sp_core::crypto::KeyTypeId;
    use sp_core::ecdsa;

    pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"role");

    sp_application_crypto::app_crypto!(ecdsa, KEY_TYPE);
}

mod role_ed25519 {
    use sp_core::crypto::KeyTypeId;
    use sp_core::ed25519;

    pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"role");

    sp_application_crypto::app_crypto!(ed25519, KEY_TYPE);
}

mod acco_sr25519 {
    use sp_core::crypto::KeyTypeId;
    use sp_core::sr25519;

    pub const KEY_TYPE: KeyTypeId = sp_core::crypto::key_types::ACCOUNT;

    sp_application_crypto::app_crypto!(sr25519, KEY_TYPE);
}

impl Default for SubstrateStorage {
    fn default() -> Self {
        let keystore = sc_keystore::LocalKeystore::in_memory();
        Self {
            inner: Arc::new(keystore),
        }
    }
}

impl SubstrateStorage {
    /// Creates a new `SubstrateStorage` instance.
    ///
    /// This wrapper is used to provide a substrate-backed local key storage.
    #[must_use]
    pub fn new(inner: Arc<sc_keystore::LocalKeystore>) -> Self {
        Self { inner }
    }
}

impl RawStorage for SubstrateStorage {
    fn store_raw(
        &self,
        type_id: KeyTypeId,
        public_bytes: Vec<u8>,
        secret_bytes: Vec<u8>,
    ) -> Result<()> {
        let secret = format!("0x{}", hex::encode(secret_bytes));
        self.inner
            .insert(sp_keystore_key_type_id_of(type_id)?, &secret, &public_bytes)
            .map_err(|()| crate::Error::SpKeystoreError)?;
        Ok(())
    }

    fn load_secret_raw(
        &self,
        type_id: KeyTypeId,
        public_bytes: Vec<u8>,
    ) -> Result<Option<Box<[u8]>>> {
        match type_id {
            #[cfg(feature = "bn254")]
            KeyTypeId::Bn254 => Err(crate::Error::KeyTypeNotSupported),
            #[cfg(any(feature = "ecdsa", feature = "tangle"))]
            KeyTypeId::Ecdsa => {
                let public = role_ecdsa::Public::try_from(public_bytes.as_slice())
                    .map_err(|()| crate::Error::KeyTypeNotSupported)?;
                let Ok(pair) = self.inner.key_pair::<role_ecdsa::Pair>(&public) else {
                    return Ok(None);
                };
                let secret = pair.map(|pair| pair.into_inner().seed().to_vec());
                Ok(secret.map(|s| s.into_boxed_slice()))
            }
            #[cfg(any(feature = "sr25519-schnorrkel", feature = "tangle"))]
            KeyTypeId::Sr25519 => {
                let public = acco_sr25519::Public::try_from(public_bytes.as_slice())
                    .map_err(|()| crate::Error::KeyTypeNotSupported)?;
                let Ok(pair) = self.inner.key_pair::<acco_sr25519::Pair>(&public) else {
                    return Ok(None);
                };
                let secret = pair.map(|pair| pair.into_inner().to_raw_vec());
                Ok(secret.map(|s| s.into_boxed_slice()))
            }
            #[cfg(any(feature = "bls", feature = "tangle"))]
            KeyTypeId::Bls381 => Err(crate::Error::KeyTypeNotSupported),
            #[cfg(any(feature = "bls", feature = "tangle"))]
            KeyTypeId::Bls377 => Err(crate::Error::KeyTypeNotSupported),
            #[cfg(any(feature = "zebra", feature = "tangle"))]
            KeyTypeId::Ed25519 => {
                let public = role_ed25519::Public::try_from(public_bytes.as_slice())
                    .map_err(|()| crate::Error::KeyTypeNotSupported)?;
                let Ok(pair) = self.inner.key_pair::<role_ed25519::Pair>(&public) else {
                    return Ok(None);
                };
                let secret = pair.map(|pair| pair.into_inner().seed().to_vec());
                Ok(secret.map(|s| s.into_boxed_slice()))
            }
            #[cfg(all(
                not(feature = "bn254"),
                not(feature = "ecdsa"),
                not(feature = "sr25519-schnorrkel"),
                not(feature = "bls"),
                not(feature = "zebra"),
                not(feature = "tangle")
            ))]
            _ => unreachable!("All possible variants are feature-gated"),
        }
    }

    fn remove_raw(&self, _type_id: KeyTypeId, _public_bytes: Vec<u8>) -> Result<()> {
        Err(crate::Error::KeystoreOperationNotSupported)
    }

    fn contains_raw(&self, type_id: KeyTypeId, public_bytes: Vec<u8>) -> bool {
        let Ok(key_type) = sp_keystore_key_type_id_of(type_id) else {
            return false;
        };
        let keys = [(public_bytes, key_type)];
        self.inner.has_keys(&keys)
    }

    fn list_raw(&self, type_id: KeyTypeId) -> Box<dyn Iterator<Item = Box<[u8]>> + '_> {
        let Ok(key_type) = sp_keystore_key_type_id_of(type_id) else {
            return Box::new(std::iter::empty());
        };
        let Ok(keys) = self.inner.keys(key_type) else {
            return Box::new(std::iter::empty());
        };

        Box::new(keys.into_iter().map(Into::into))
    }
}

fn sp_keystore_key_type_id_of(key_type: KeyTypeId) -> Result<sp_core::crypto::KeyTypeId> {
    match key_type {
        #[cfg(feature = "bn254")]
        KeyTypeId::Bn254 => Err(crate::Error::KeyTypeNotSupported),
        #[cfg(any(feature = "ecdsa", feature = "tangle"))]
        KeyTypeId::Ecdsa => Ok(role_ecdsa::KEY_TYPE),
        #[cfg(any(feature = "sr25519-schnorrkel", feature = "tangle"))]
        KeyTypeId::Sr25519 => Ok(acco_sr25519::KEY_TYPE),
        #[cfg(any(feature = "bls", feature = "tangle"))]
        KeyTypeId::Bls381 => Err(crate::Error::KeyTypeNotSupported),
        #[cfg(any(feature = "bls", feature = "tangle"))]
        KeyTypeId::Bls377 => Err(crate::Error::KeyTypeNotSupported),
        #[cfg(any(feature = "zebra", feature = "tangle"))]
        KeyTypeId::Ed25519 => Ok(role_ed25519::KEY_TYPE),
        #[cfg(all(
            not(feature = "bn254"),
            not(feature = "ecdsa"),
            not(feature = "sr25519-schnorrkel"),
            not(feature = "bls"),
            not(feature = "zebra"),
            not(feature = "tangle")
        ))]
        _ => unreachable!("All possible variants are feature-gated"),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use blueprint_crypto::sp_core::{SpSr25519, SpSr25519Public};
    use blueprint_crypto::{IntoCryptoError, KeyType, k256::K256Ecdsa};

    use super::*;
    use crate::storage::TypedStorage;

    #[test]
    fn test_basic_operations() -> Result<()> {
        let tmpdir = tempfile::tempdir()?;
        let keystore = sc_keystore::LocalKeystore::open(tmpdir.path(), None).unwrap();
        let raw_storage = SubstrateStorage::new(Arc::new(keystore));
        let storage = TypedStorage::new(raw_storage);

        // Generate a key pair
        let secret =
            K256Ecdsa::generate_with_seed(None).map_err(IntoCryptoError::into_crypto_error)?;
        let public = K256Ecdsa::public_from_secret(&secret);

        // Test store and load
        storage.store::<K256Ecdsa>(&public, &secret)?;

        // Test contains
        assert!(storage.contains::<K256Ecdsa>(&public));

        let loaded = storage.load::<K256Ecdsa>(&public)?;
        assert_eq!(loaded.as_ref(), Some(&secret));

        // Test list
        let keys: Vec<_> = storage.list::<K256Ecdsa>().collect();
        assert_eq!(keys.len(), 1);
        assert_eq!(&keys[0], &public);

        Ok(())
    }

    #[test]
    fn test_multiple_key_types() -> Result<()> {
        let tmpdir = tempfile::tempdir()?;
        let keystore = sc_keystore::LocalKeystore::open(tmpdir.path(), None).unwrap();
        let raw_storage = SubstrateStorage::new(Arc::new(keystore));
        let storage = TypedStorage::new(raw_storage);

        // Create keys of different types
        let k256_secret =
            K256Ecdsa::generate_with_seed(None).map_err(IntoCryptoError::into_crypto_error)?;
        let k256_public = K256Ecdsa::public_from_secret(&k256_secret);

        // Store keys
        storage.store::<K256Ecdsa>(&k256_public, &k256_secret)?;

        // Verify isolation between types
        assert!(storage.contains::<K256Ecdsa>(&k256_public));

        // List should only show keys of the requested type
        assert_eq!(storage.list::<K256Ecdsa>().count(), 1);

        Ok(())
    }

    #[test]
    fn native_substrate_keystore_ops() -> Result<()> {
        let tmpdir = tempfile::tempdir()?;
        let keystore = sc_keystore::LocalKeystore::open(tmpdir.path(), None).unwrap();
        let public = keystore
            .sr25519_generate_new(acco_sr25519::KEY_TYPE, None)
            .unwrap();
        let raw_storage = SubstrateStorage::new(Arc::new(keystore));
        let storage = TypedStorage::new(raw_storage);

        let public = SpSr25519Public(public);
        // Test contains
        assert!(storage.contains::<SpSr25519>(&public));

        let loaded = storage.load::<SpSr25519>(&public)?;
        assert!(loaded.is_some());

        // Test list
        let keys: Vec<_> = storage.list::<SpSr25519>().collect();
        assert_eq!(keys.len(), 1);
        assert_eq!(&keys[0], &public);

        Ok(())
    }
}
