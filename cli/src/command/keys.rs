use bip39::{Language, Mnemonic};
use blueprint_crypto::bn254::{ArkBlsBn254, ArkBlsBn254Public, ArkBlsBn254Secret};
use blueprint_crypto::k256::{K256Ecdsa, K256VerifyingKey};
use blueprint_crypto_core::{BytesEncoding, KeyType};
use blueprint_keystore::{Keystore, KeystoreConfig, backends::Backend};
use blueprint_runner::config::Protocol;
use blueprint_std::path::Path;
use clap::ValueEnum;
use color_eyre::eyre::{Result, eyre};
use dialoguer::{Input, Select};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unsupported key type")]
    UnsupportedKey,
    #[error("Keystore error: {0}")]
    KeystoreError(#[from] blueprint_keystore::error::Error),
    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),
    #[error("Invalid mnemonic word count: {0}. Must be 12, 15, 18, 21, or 24")]
    InvalidWordCount(u32),
}

/// Key algorithms supported by the CLI.
#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum SupportedKey {
    /// secp256k1/ECDSA key used for operator identities.
    Ecdsa,
    /// BLS12-377 key used for BN254 aggregation.
    Bn254,
}

impl SupportedKey {
    fn all() -> &'static [SupportedKey] {
        &[SupportedKey::Ecdsa, SupportedKey::Bn254]
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn prompt_for_keys(
    key_types: Vec<SupportedKey>,
) -> color_eyre::Result<Vec<(SupportedKey, String)>> {
    let mut collected_keys = Vec::new();

    if key_types.is_empty() {
        loop {
            let mut options = SupportedKey::all()
                .iter()
                .map(|kt| format!("Enter key for {kt:?}"))
                .collect::<Vec<_>>();
            options.push("Continue".to_string());

            let selection = Select::new()
                .with_prompt("Select key type to enter (or Continue when done)")
                .items(&options)
                .default(0)
                .interact()?;

            if selection == options.len() - 1 {
                break;
            }

            let key_type = SupportedKey::all()[selection];
            let key: String = Input::new()
                .with_prompt(format!("Enter private key for {key_type:?}"))
                .interact_text()?;

            collected_keys.push((key_type, key));
        }
    } else {
        for key_type in key_types {
            let key: String = Input::new()
                .with_prompt(format!("Enter private key for {key_type:?}"))
                .interact_text()?;
            collected_keys.push((key_type, key));
        }
    }

    Ok(collected_keys)
}

pub fn generate_key(
    key_type: SupportedKey,
    output: Option<&impl AsRef<Path>>,
    seed: Option<&[u8]>,
    show_secret: bool,
) -> Result<(String, Option<String>)> {
    let mut config = KeystoreConfig::new();
    if let Some(path) = output {
        if !path.as_ref().exists() {
            std::fs::create_dir_all(path.as_ref())?;
        }
        config = config.fs_root(path);
    }

    let keystore = Keystore::new(config)?;

    let (public_bytes, secret_bytes) = match key_type {
        SupportedKey::Ecdsa => {
            let public = keystore.generate::<K256Ecdsa>(seed)?;
            let secret = keystore.get_secret::<K256Ecdsa>(&public)?;
            keystore.insert::<K256Ecdsa>(&secret)?;
            (public.to_bytes(), secret.to_bytes())
        }
        SupportedKey::Bn254 => {
            let public = keystore.generate::<ArkBlsBn254>(seed)?;
            let secret = keystore.get_secret::<ArkBlsBn254>(&public)?;
            keystore.insert::<ArkBlsBn254>(&secret)?;
            (public.to_bytes(), secret.to_bytes())
        }
    };

    let (public, secret) = (hex::encode(public_bytes), hex::encode(secret_bytes));
    // Only hide secret if show_secret is false AND we're saving to a file
    // If output is None, we must show the secret (user has no other way to see it)
    let secret = if !show_secret && output.is_some() {
        None
    } else {
        Some(secret)
    };

    Ok((public, secret))
}

pub fn generate_mnemonic(word_count: Option<u32>) -> Result<String> {
    let count = match word_count {
        Some(count) if !(12..=24).contains(&count) || count % 3 != 0 => {
            return Err(Error::InvalidWordCount(count).into());
        }
        Some(count) => count,
        None => 12,
    };
    let mut rng = bip39::rand::thread_rng();
    let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, count as usize)
        .map_err(|e| eyre!(e.to_string()))?;
    Ok(mnemonic.to_string())
}

pub fn import_key(
    _protocol: Protocol,
    key_type: SupportedKey,
    secret: &str,
    keystore_path: &Path,
) -> Result<String> {
    let config = KeystoreConfig::new().fs_root(keystore_path);
    let keystore = Keystore::new(config)?;

    let public_bytes = match key_type {
        SupportedKey::Ecdsa => {
            let signing_key = K256Ecdsa::generate_with_string(secret.to_string())?;
            keystore.insert::<K256Ecdsa>(&signing_key)?;
            K256Ecdsa::public_from_secret(&signing_key).to_bytes()
        }
        SupportedKey::Bn254 => {
            let signing_key = parse_bn254_secret(secret)?;
            keystore.insert::<ArkBlsBn254>(&signing_key)?;
            ArkBlsBn254::public_from_secret(&signing_key).to_bytes()
        }
    };

    Ok(hex::encode(public_bytes))
}

pub fn export_key(key_type: SupportedKey, public: &str, keystore_path: &Path) -> Result<String> {
    let config = KeystoreConfig::new().fs_root(keystore_path);
    let keystore = Keystore::new(config)?;
    let public_bytes = hex::decode(public).map_err(|e| Error::InvalidKeyFormat(e.to_string()))?;

    let secret = match key_type {
        SupportedKey::Ecdsa => {
            let public = K256VerifyingKey::from_bytes(&public_bytes)
                .map_err(|e| Error::InvalidKeyFormat(e.to_string()))?;
            let secret = keystore
                .get_secret::<K256Ecdsa>(&public)
                .map_err(Error::KeystoreError)?;
            secret.to_bytes()
        }
        SupportedKey::Bn254 => {
            let public = ArkBlsBn254Public::from_bytes(&public_bytes)
                .map_err(|e| Error::InvalidKeyFormat(e.to_string()))?;
            let secret = keystore
                .get_secret::<ArkBlsBn254>(&public)
                .map_err(Error::KeystoreError)?;
            secret.to_bytes()
        }
    };

    Ok(hex::encode(secret))
}

pub fn list_keys(keystore_path: &Path) -> Result<Vec<(SupportedKey, String)>> {
    let config = KeystoreConfig::new().fs_root(keystore_path);
    let keystore = Keystore::new(config)?;

    let mut keys = Vec::new();
    if let Ok(list) = keystore.list_local::<K256Ecdsa>() {
        keys.extend(
            list.into_iter()
                .map(|key| (SupportedKey::Ecdsa, hex::encode(key.to_bytes()))),
        );
    }
    if let Ok(list) = keystore.list_local::<ArkBlsBn254>() {
        keys.extend(
            list.into_iter()
                .map(|key| (SupportedKey::Bn254, hex::encode(key.to_bytes()))),
        );
    }

    Ok(keys)
}

fn parse_bn254_secret(secret: &str) -> Result<ArkBlsBn254Secret> {
    let trimmed = secret.trim();
    let maybe_hex = trimmed.strip_prefix("0x").unwrap_or(trimmed);
    let looks_hex = trimmed.starts_with("0x") || maybe_hex.len() == 64;
    if looks_hex && maybe_hex.chars().all(|c| c.is_ascii_hexdigit()) {
        let bytes = hex::decode(maybe_hex).map_err(|e| Error::InvalidKeyFormat(e.to_string()))?;
        ArkBlsBn254Secret::from_bytes(&bytes)
            .map_err(|e| Error::InvalidKeyFormat(e.to_string()).into())
    } else {
        Ok(ArkBlsBn254::generate_with_string(trimmed.to_string())?)
    }
}
