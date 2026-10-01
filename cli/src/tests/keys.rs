use crate::command::keys::{SupportedKey, export_key, generate_key, import_key, list_keys};
use crate::command::signer::{load_ecdsa_signing_key, load_keystore};
use blueprint_crypto::k256::K256Ecdsa;
use blueprint_crypto::{BytesEncoding, KeyType};
use blueprint_runner::config::Protocol;
use color_eyre::eyre::Result;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_cli_fs_key_generation() -> Result<()> {
    let temp_dir = tempdir()?;
    let output_path = temp_dir.path();

    for key_type in [SupportedKey::Ecdsa, SupportedKey::Bn254] {
        let (public, secret) = generate_key(key_type, Some(&output_path), None, true)?;
        assert!(!public.is_empty());
        assert!(secret.as_ref().is_some_and(|s| !s.is_empty()));
    }

    Ok(())
}

#[test]
fn test_cli_mem_key_generation() -> Result<()> {
    for key_type in [SupportedKey::Ecdsa, SupportedKey::Bn254] {
        let (public, secret) = generate_key(key_type, None::<&PathBuf>, None, true)?;
        assert!(!public.is_empty());
        assert!(secret.as_ref().is_some_and(|s| !s.is_empty()));
    }
    Ok(())
}

#[test]
fn test_generate_mnemonic() -> Result<()> {
    use crate::command::keys::generate_mnemonic;

    let mnemonic = generate_mnemonic(None)?;
    let words: Vec<&str> = mnemonic.split_whitespace().collect();
    assert_eq!(words.len(), 12);

    for count in [12, 15, 18, 21, 24] {
        let mnemonic = generate_mnemonic(Some(count))?;
        let words: Vec<&str> = mnemonic.split_whitespace().collect();
        assert_eq!(words.len(), count as usize);
    }

    Ok(())
}

#[test]
fn test_key_import_export() -> Result<()> {
    let temp_dir = tempdir()?;
    let keystore_path = temp_dir.path();

    for key_type in [SupportedKey::Ecdsa, SupportedKey::Bn254] {
        let (_public, secret) = generate_key(key_type, Some(&keystore_path), None, true)?;
        let secret = secret.expect("secret missing");

        let imported_public = import_key(Protocol::Tangle, key_type, &secret, keystore_path)?;
        assert!(!imported_public.is_empty());

        let exported_secret = export_key(key_type, &imported_public, keystore_path)?;
        assert_eq!(secret, exported_secret);
    }

    Ok(())
}

#[test]
fn test_list_keys() -> Result<()> {
    let temp_dir = tempdir()?;
    let keystore_path = temp_dir.path();

    let mut expected = Vec::new();
    for key_type in [SupportedKey::Ecdsa, SupportedKey::Bn254] {
        let (public, _) = generate_key(key_type, Some(&keystore_path), None, true)?;
        expected.push((key_type, public));
    }

    let listed_keys = list_keys(keystore_path)?;
    assert_eq!(listed_keys.len(), expected.len());

    for (kind, public) in expected {
        assert!(listed_keys.iter().any(|k| k.0 == kind && k.1 == public));
    }

    Ok(())
}

/// Control: one ECDSA key resolves to that key, no error.
#[test]
fn load_ecdsa_signing_key_resolves_single_key() -> Result<()> {
    let temp_dir = tempdir()?;
    let keystore_path = temp_dir.path();

    let (public, secret) = generate_key(SupportedKey::Ecdsa, Some(&keystore_path), None, true)?;
    let secret = secret.expect("secret missing");

    let keystore = load_keystore(keystore_path)?;
    let signing_key = load_ecdsa_signing_key(&keystore)?;

    assert_eq!(
        hex::encode(K256Ecdsa::public_from_secret(&signing_key).to_bytes()),
        public
    );
    assert!(!secret.is_empty());

    Ok(())
}

/// Reproducer: two ECDSA keys are a supported state (`import_key` inserts
/// without replacing), and registration must not silently pick one.
#[test]
fn load_ecdsa_signing_key_rejects_ambiguous_keystore() -> Result<()> {
    let temp_dir = tempdir()?;
    let keystore_path = temp_dir.path();

    let mut imported = Vec::new();
    for _ in 0..2 {
        let (_public, secret) =
            generate_key(SupportedKey::Ecdsa, Some(&keystore_path), None, true)?;
        let secret = secret.expect("secret missing");
        imported.push(import_key(
            Protocol::Tangle,
            SupportedKey::Ecdsa,
            &secret,
            keystore_path,
        )?);
    }
    imported.sort();
    imported.dedup();
    assert_eq!(
        imported.len(),
        2,
        "fixture must hold two distinct ECDSA keys"
    );

    let keystore = load_keystore(keystore_path)?;
    let err = load_ecdsa_signing_key(&keystore)
        .expect_err("ambiguous keystore must not resolve to a key")
        .to_string();

    assert!(err.contains("2 ECDSA keys"), "unexpected error: {err}");
    for public in &imported {
        assert!(
            err.contains(public),
            "error must name candidate {public}: {err}"
        );
    }

    Ok(())
}
