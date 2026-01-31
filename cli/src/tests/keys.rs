use crate::command::keys::{SupportedKey, export_key, generate_key, import_key, list_keys};
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
