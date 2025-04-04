use crate::backends::Backend;
use crate::error::{Error, Result};
use crate::keystore::Keystore;
use blueprint_crypto::sp_core::{
    SpEcdsa, SpEcdsaPair, SpEd25519, SpEd25519Pair, SpSr25519, SpSr25519Pair,
};
use blueprint_crypto::tangle_pair_signer::TanglePairSigner;
use blueprint_crypto::{BytesEncoding, KeyTypeId};
use sp_core::Pair;
use sp_core::{ecdsa, ed25519, sr25519};

pub trait TangleBackend: Send + Sync {
    // String-based Key Generation
    /// Generate an ECDSA key pair from a string seed
    ///
    /// # Errors
    ///
    /// Depending on the backend, this may error when attempting to store the key into the keystore.
    fn ecdsa_generate_from_string(&self, string: &str) -> Result<ecdsa::Public>;

    /// Generate an ed25519 key pair from a string seed
    ///
    /// # Errors
    ///
    /// Depending on the backend, this may error when attempting to store the key into the keystore.
    fn ed25519_generate_from_string(&self, string: &str) -> Result<ed25519::Public>;

    /// Generate an sr25519 key pair from a string seed
    ///
    /// # Errors
    ///
    /// Depending on the backend, this may error when attempting to store the key into the keystore.
    fn sr25519_generate_from_string(&self, string: &str) -> Result<sr25519::Public>;

    /// Insert an existing sr25519 key pair
    ///
    /// # Errors
    ///
    /// Depending on the backend, this may error when attempting to store the key into the keystore.
    fn create_sr25519_from_pair<T: Into<sr25519::Pair>>(
        &self,
        pair: T,
    ) -> Result<TanglePairSigner<sr25519::Pair>>;

    /// Insert an existing ed25519 key pair
    ///
    /// # Errors
    ///
    /// Depending on the backend, this may error when attempting to store the key into the keystore.
    fn create_ed25519_from_pair<T: Into<ed25519::Pair>>(
        &self,
        pair: T,
    ) -> Result<TanglePairSigner<ed25519::Pair>>;

    /// Insert an existing ECDSA key pair
    ///
    /// # Errors
    ///
    /// Depending on the backend, this may error when attempting to store the key into the keystore.
    fn create_ecdsa_from_pair<T: Into<ecdsa::Pair>>(
        &self,
        pair: T,
    ) -> Result<TanglePairSigner<ecdsa::Pair>>;
}

impl TangleBackend for Keystore {
    fn ecdsa_generate_from_string(&self, string: &str) -> Result<ecdsa::Public> {
        const KEY_TYPE_ID: KeyTypeId = KeyTypeId::Ecdsa;

        let pair = SpEcdsaPair(
            ecdsa::Pair::from_string(string, None).map_err(|e| Error::Other(e.to_string()))?,
        );
        let public = pair.public();

        // Store in all available storage backends
        let public_bytes = public.to_bytes();
        let secret_bytes = pair.to_bytes();

        if let Some(storages) = self.storages.get(&KEY_TYPE_ID) {
            for entry in storages {
                entry
                    .storage
                    .store_raw(KEY_TYPE_ID, public_bytes.clone(), secret_bytes.clone())?;
            }
        }

        Ok(public.0)
    }

    fn ed25519_generate_from_string(&self, string: &str) -> Result<ed25519::Public> {
        const KEY_TYPE_ID: KeyTypeId = KeyTypeId::Ed25519;

        let pair = SpEd25519Pair(
            ed25519::Pair::from_string(string, None).map_err(|e| Error::Other(e.to_string()))?,
        );
        let public = pair.public();

        // Store in all available storage backends
        let public_bytes = public.to_bytes();
        let secret_bytes = pair.to_bytes();

        if let Some(storages) = self.storages.get(&KEY_TYPE_ID) {
            for entry in storages {
                entry
                    .storage
                    .store_raw(KEY_TYPE_ID, public_bytes.clone(), secret_bytes.clone())?;
            }
        }

        Ok(public.0)
    }

    fn sr25519_generate_from_string(&self, string: &str) -> Result<sr25519::Public> {
        const KEY_TYPE_ID: KeyTypeId = KeyTypeId::Sr25519;

        let pair = SpSr25519Pair(
            sr25519::Pair::from_string(string, None).map_err(|e| Error::Other(e.to_string()))?,
        );
        let public = pair.public();

        // Store in all available storage backends
        let public_bytes = public.to_bytes();
        let secret_bytes = pair.to_bytes();

        if let Some(storages) = self.storages.get(&KEY_TYPE_ID) {
            for entry in storages {
                entry
                    .storage
                    .store_raw(KEY_TYPE_ID, public_bytes.clone(), secret_bytes.clone())?;
            }
        }

        Ok(public.0)
    }

    fn create_sr25519_from_pair<T: Into<sr25519::Pair>>(
        &self,
        pair: T,
    ) -> Result<TanglePairSigner<sr25519::Pair>> {
        let pair = pair.into();
        let seed = pair.as_ref().secret.to_bytes();
        let _ = self.generate::<SpSr25519>(Some(&seed))?;
        Ok(TanglePairSigner::new(sr25519::Pair::from_seed_slice(
            &seed,
        )?))
    }

    fn create_ed25519_from_pair<T: Into<ed25519::Pair>>(
        &self,
        pair: T,
    ) -> Result<TanglePairSigner<ed25519::Pair>> {
        let pair = pair.into();
        let seed = pair.seed();
        let _ = self.generate::<SpEd25519>(Some(&seed))?;
        Ok(TanglePairSigner::new(ed25519::Pair::from_seed_slice(
            &seed,
        )?))
    }

    fn create_ecdsa_from_pair<T: Into<ecdsa::Pair>>(
        &self,
        pair: T,
    ) -> Result<TanglePairSigner<ecdsa::Pair>> {
        let pair = pair.into();
        let seed = pair.seed();
        let _ = self.generate::<SpEcdsa>(Some(&seed))?;
        Ok(TanglePairSigner::new(ecdsa::Pair::from_seed_slice(&seed)?))
    }
}

#[cfg(feature = "tangle-bls")]
pub mod bls {
    use crate::error::{Error, Result};
    use crate::keystore::Keystore;
    use crate::keystore::backends::tangle::TangleBackend;
    use blueprint_crypto::sp_core::{SpBls377Pair, SpBls381Pair};
    use blueprint_crypto::{BytesEncoding, KeyTypeId};
    use sp_core::Pair;

    pub trait TangleBlsBackend: TangleBackend {
        // BLS Key Generation Methods
        /// Generate a BLS377 key pair from a string seed
        ///
        /// # Errors
        ///
        /// Depending on the backend, this may error when attempting to store the key into the keystore.
        fn bls377_generate_from_string(&self, string: &str) -> Result<sp_core::bls377::Public>;
        /// Generate a BLS381 key pair from a string seed
        ///
        /// # Errors
        ///
        /// Depending on the backend, this may error when attempting to store the key into the keystore.
        fn bls381_generate_from_string(&self, string: &str) -> Result<sp_core::bls381::Public>;
    }

    impl TangleBlsBackend for Keystore {
        fn bls377_generate_from_string(&self, string: &str) -> Result<sp_core::bls377::Public> {
            const KEY_TYPE_ID: KeyTypeId = KeyTypeId::Bls377;

            let (_, seed) = sp_core::bls377::Pair::from_string_with_seed(string, None)
                .map_err(|e| Error::Other(e.to_string()))?;

            let Some(seed) = seed else {
                return Err(Error::Other(String::from("Unable to determine seed")));
            };

            let pair = SpBls377Pair::from_bytes(seed.as_slice())?;
            let public = pair.public();

            // Store in all available storage backends
            let public_bytes = public.to_bytes();
            let secret_bytes = pair.to_bytes();

            if let Some(storages) = self.storages.get(&KEY_TYPE_ID) {
                for entry in storages {
                    entry.storage.store_raw(
                        KEY_TYPE_ID,
                        public_bytes.clone(),
                        secret_bytes.clone(),
                    )?;
                }
            }

            Ok(public.0)
        }

        fn bls381_generate_from_string(&self, string: &str) -> Result<sp_core::bls381::Public> {
            const KEY_TYPE_ID: KeyTypeId = KeyTypeId::Bls381;

            let (_, seed) = sp_core::bls381::Pair::from_string_with_seed(string, None)
                .map_err(|e| Error::Other(e.to_string()))?;

            let Some(seed) = seed else {
                return Err(Error::Other(String::from("Unable to determine seed")));
            };

            let pair = SpBls381Pair::from_bytes(seed.as_slice())?;
            let public = pair.public();

            // Store in all available storage backends
            let public_bytes = public.to_bytes();
            let secret_bytes = pair.to_bytes();

            if let Some(storages) = self.storages.get(&KEY_TYPE_ID) {
                for entry in storages {
                    entry.storage.store_raw(
                        KEY_TYPE_ID,
                        public_bytes.clone(),
                        secret_bytes.clone(),
                    )?;
                }
            }

            Ok(public.0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::KeystoreConfig;
        use crate::backends::Backend;
        use blueprint_crypto::KeyType;
        use blueprint_crypto::sp_core::{SpBls377, SpBls377Public, SpBls381, SpBls381Public};
        use sp_core::ByteArray;

        #[test]
        fn test_bls381_generation_from_string() -> Result<()> {
            const PUBLIC: &[u8] = b"88ff6c3a32542bc85f2adf1c490a929b7fcee50faeb95af9a036349390e9b3ea7326247c4fc4ebf88050688fd6265de0806284eec09ba0949f5df05dc93a787a14509749f36e4a0981bb748d953435483740907bb5c2fe8ffd97e8509e1a038b05fb08488db628ea0638b8d48c3ddf62ed437edd8b23d5989d6c65820fc70f80fb39b486a3766813e021124aec29a566";

            let keystore = Keystore::new(KeystoreConfig::new())?;

            // Generate key
            let public = keystore.bls381_generate_from_string(
                "0x9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
            )?;

            assert_eq!(public.as_slice(), hex::decode(PUBLIC).unwrap());

            let signature = keystore.sign_with_local::<SpBls381>(&SpBls381Public(public), b"")?;
            assert!(SpBls381::verify(&SpBls381Public(public), b"", &signature));

            Ok(())
        }

        #[test]
        fn test_bls377_generation_from_string() -> Result<()> {
            const PUBLIC: &[u8] = b"7a84ca8ce4c37c93c95ecee6a3c0c9a7b9c225093cf2f12dc4f69cbfb847ef9424a18f5755d5a742247d386ff2aabb806bcf160eff31293ea9616976628f77266c8a8cc1d8753be04197bd6cdd8c5c87a148f782c4c1568d599b48833fd539001e580cff64bbc71850605433fcd051f3afc3b74819786f815ffb5272030a8d03e5df61e6183f8fd8ea85f26defa83400";

            let keystore = Keystore::new(KeystoreConfig::new())?;

            // Generate key
            let public = keystore.bls377_generate_from_string(
                "0x9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
            )?;

            assert_eq!(public.as_slice(), hex::decode(PUBLIC).unwrap());
            let signature = keystore.sign_with_local::<SpBls377>(&SpBls377Public(public), b"")?;
            assert!(SpBls377::verify(&SpBls377Public(public), b"", &signature));

            Ok(())
        }
    }
}
