use std::path::{Path, PathBuf};

use alloy_primitives::{Address, B256, Bytes};
use alloy_signer_local::PrivateKeySigner;
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::{K256Ecdsa, K256SigningKey};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use color_eyre::eyre::{Context, Result, eyre};

/// Environment variable pointing at the default keystore directory.
pub const KEYSTORE_PATH_ENV: &str = "BLUEPRINT_KEYSTORE_URI";

/// Minimal metadata required to sign Tangle EVM transactions.
#[derive(Debug, Clone)]
pub struct EvmSigner {
    /// Derived operator address.
    pub operator_address: Address,
    /// Local signer usable with Alloy providers.
    pub signer: PrivateKeySigner,
    /// Uncompressed 65-byte ECDSA public key.
    pub public_key: Bytes,
}

impl EvmSigner {
    fn from_signing_key(key: &K256SigningKey) -> Result<Self> {
        let secret = key.to_bytes();
        let signer = PrivateKeySigner::from_bytes(&B256::from_slice(&secret))
            .map_err(|e| eyre!("failed to create signer: {e}"))?;
        let operator_address = signer.address();
        let public_key = Bytes::copy_from_slice(&key.public().to_bytes());

        Ok(Self {
            operator_address,
            signer,
            public_key,
        })
    }
}

/// Artifacts required for `registerOperator`.
#[derive(Debug, Clone)]
pub struct RegistrationPayload {
    /// Signing metadata (address + public key).
    pub signer: EvmSigner,
    /// RPC endpoint advertised during registration.
    pub rpc_endpoint: String,
    /// Optional opaque registration inputs passed through to the contract.
    pub registration_inputs: Bytes,
}

/// Resolve the keystore path from `BLUEPRINT_KEYSTORE_URI`.
pub fn keystore_path_from_env() -> Result<PathBuf> {
    std::env::var(KEYSTORE_PATH_ENV)
        .wrap_err_with(|| format!("{KEYSTORE_PATH_ENV} is not set – point it at your keystore"))
        .map(PathBuf::from)
}

/// Load the on-disk keystore.
pub fn load_keystore(path: impl AsRef<Path>) -> Result<Keystore> {
    let config = KeystoreConfig::new().fs_root(path);
    Keystore::new(config).context("failed to open keystore")
}

/// Fetch the first local ECDSA signing key from the keystore.
pub fn load_ecdsa_signing_key(keystore: &Keystore) -> Result<K256SigningKey> {
    let public = keystore
        .first_local::<K256Ecdsa>()
        .context("keystore does not contain an ECDSA key")?;

    keystore
        .get_secret::<K256Ecdsa>(&public)
        .context("failed to load ECDSA secret")
}

/// Load an EVM signer (address + alloy-compatible signer) from a keystore path.
pub fn load_evm_signer(keystore_path: impl AsRef<Path>) -> Result<EvmSigner> {
    let keystore = load_keystore(keystore_path)?;
    let signing_key = load_ecdsa_signing_key(&keystore)?;
    EvmSigner::from_signing_key(&signing_key)
}

/// Construct the payload needed when calling `registerOperator`.
pub fn build_registration_payload(
    keystore_path: impl AsRef<Path>,
    rpc_endpoint: impl Into<String>,
    registration_inputs: Option<Vec<u8>>,
) -> Result<RegistrationPayload> {
    let keystore = load_keystore(keystore_path)?;
    let signing_key = load_ecdsa_signing_key(&keystore)?;
    let signer = EvmSigner::from_signing_key(&signing_key)?;

    Ok(RegistrationPayload {
        signer,
        rpc_endpoint: rpc_endpoint.into(),
        registration_inputs: Bytes::copy_from_slice(&registration_inputs.unwrap_or_default()),
    })
}
