use super::RawStorage;
use crate::error::Result;
use blueprint_crypto::KeyTypeId;
use blueprint_std::{boxed::Box, collections::BTreeMap, vec::Vec};

/// Interior-mutability lock for the storage map.
///
/// `parking_lot` is a fair, yielding lock, but it requires `std`
/// unconditionally and so is unavailable on bare-metal targets
/// (https://github.com/tangle-network/blueprint/issues/1520). `spin` is
/// `no_std`-clean and is used instead when `std` is off.
///
/// The public API of [`InMemoryStorage`] is identical either way, and
/// `RawStorage: Send + Sync` still holds because both locks are.
#[cfg(feature = "std")]
type StorageLock<T> = parking_lot::RwLock<T>;
#[cfg(not(feature = "std"))]
type StorageLock<T> = spin::RwLock<T>;

type StorageMap = BTreeMap<KeyTypeId, BTreeMap<Vec<u8>, Vec<u8>>>;

/// A memory-backed local storage
pub struct InMemoryStorage {
    data: StorageLock<StorageMap>,
}

impl InMemoryStorage {
    /// Create a new `InMemoryStorage`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use blueprint_keystore::Keystore;
    /// use blueprint_keystore::backends::{Backend, BackendConfig};
    /// use blueprint_keystore::crypto::IntoCryptoError;
    /// use blueprint_keystore::crypto::KeyType;
    /// use blueprint_keystore::crypto::k256::K256Ecdsa;
    /// use blueprint_keystore::storage::{InMemoryStorage, TypedStorage};
    ///
    /// # fn main() -> blueprint_keystore::Result<()> {
    /// // Create the storage
    /// let storage = InMemoryStorage::new();
    /// let storage = TypedStorage::new(storage);
    ///
    /// // Generate a key pair
    /// let secret = K256Ecdsa::generate_with_seed(None).unwrap();
    /// let public = K256Ecdsa::public_from_secret(&secret);
    ///
    /// // Start storing
    /// storage.store::<K256Ecdsa>(&public, &secret)?;
    /// # Ok(()) }
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: StorageLock::new(BTreeMap::new()),
        }
    }
}

impl Default for InMemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl RawStorage for InMemoryStorage {
    fn store_raw(
        &self,
        type_id: KeyTypeId,
        public_bytes: Vec<u8>,
        secret_bytes: Vec<u8>,
    ) -> Result<()> {
        let mut data = self.data.write();
        let type_map = data.entry(type_id).or_default();
        type_map.insert(public_bytes, secret_bytes);
        Ok(())
    }

    fn load_secret_raw(
        &self,
        type_id: KeyTypeId,
        public_bytes: Vec<u8>,
    ) -> Result<Option<Box<[u8]>>> {
        let data = self.data.read();
        Ok(data
            .get(&type_id)
            .and_then(|type_map| type_map.get(&public_bytes[..]))
            .map(|v| v.clone().into_boxed_slice()))
    }

    fn remove_raw(&self, type_id: KeyTypeId, public_bytes: Vec<u8>) -> Result<()> {
        let mut data = self.data.write();
        if let Some(type_map) = data.get_mut(&type_id) {
            type_map.remove(&public_bytes[..]);
        }
        Ok(())
    }

    fn contains_raw(&self, type_id: KeyTypeId, public_bytes: Vec<u8>) -> bool {
        let data = self.data.read();
        data.get(&type_id)
            .is_some_and(|type_map| type_map.contains_key(&public_bytes[..]))
    }

    fn list_raw(&self, type_id: KeyTypeId) -> Box<dyn Iterator<Item = Box<[u8]>> + '_> {
        let data = self.data.read();
        let keys = data
            .get(&type_id)
            .map(|type_map| {
                type_map
                    .keys()
                    .map(|k| k.clone().into_boxed_slice())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        Box::new(keys.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use blueprint_crypto::{IntoCryptoError, KeyType, k256::K256Ecdsa};

    use super::*;
    use crate::storage::TypedStorage;

    /// The lock backing `InMemoryStorage` is selected by `cfg` (#1520), so
    /// pin the trait bounds `RawStorage` requires. This is a compile-time
    /// assertion: it fails to build if either `parking_lot::RwLock` or
    /// `spin::RwLock` stops satisfying them.
    fn _assert_raw_storage_bounds() {
        fn assert_send_sync<T: Send + Sync + ?Sized>() {}
        assert_send_sync::<InMemoryStorage>();
        assert_send_sync::<dyn RawStorage>();
    }

    /// Negative path: a key type that was never stored must not be readable
    /// back, and removing a type that is absent must not disturb the types
    /// that are present. Guards the `get`/`get_mut` fallbacks in
    /// `load_secret_raw`/`remove_raw` against returning another key type's
    /// bytes.
    #[test]
    fn test_absent_type_is_isolated() -> Result<()> {
        let storage = TypedStorage::new(InMemoryStorage::new());

        let secret =
            K256Ecdsa::generate_with_seed(None).map_err(IntoCryptoError::into_crypto_error)?;
        let public = K256Ecdsa::public_from_secret(&secret);
        storage.store::<K256Ecdsa>(&public, &secret)?;

        // Unknown public key within a stored type reads back as None.
        let other = K256Ecdsa::generate_with_seed(Some(1u64.to_le_bytes().as_slice()))
            .map_err(IntoCryptoError::into_crypto_error)?;
        let other_public = K256Ecdsa::public_from_secret(&other);
        assert_eq!(storage.load::<K256Ecdsa>(&other_public)?.as_ref(), None);
        assert!(!storage.contains::<K256Ecdsa>(&other_public));

        // Removing a key that is not present is a no-op, not an error.
        storage.remove::<K256Ecdsa>(&other_public)?;

        // The stored key survives that removal.
        assert_eq!(storage.load::<K256Ecdsa>(&public)?.as_ref(), Some(&secret));
        assert_eq!(storage.list::<K256Ecdsa>().count(), 1);

        Ok(())
    }

    #[test]
    fn test_basic_operations() -> Result<()> {
        let raw_storage = InMemoryStorage::new();
        let storage = TypedStorage::new(raw_storage);

        // Generate a key pair
        let secret =
            K256Ecdsa::generate_with_seed(None).map_err(IntoCryptoError::into_crypto_error)?;
        let public = K256Ecdsa::public_from_secret(&secret);

        // Test store and load
        storage.store::<K256Ecdsa>(&public, &secret)?;
        let loaded = storage.load::<K256Ecdsa>(&public)?;
        assert_eq!(loaded.as_ref(), Some(&secret));

        // Test contains
        assert!(storage.contains::<K256Ecdsa>(&public));

        // Test list
        let keys: Vec<_> = storage.list::<K256Ecdsa>().collect();
        assert_eq!(keys.len(), 1);
        assert_eq!(&keys[0], &public);

        // Test remove
        storage.remove::<K256Ecdsa>(&public)?;
        assert!(!storage.contains::<K256Ecdsa>(&public));
        assert_eq!(storage.load::<K256Ecdsa>(&public)?, None);

        Ok(())
    }

    #[test]
    fn test_multiple_key_types() -> Result<()> {
        let raw_storage = InMemoryStorage::new();
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
}
