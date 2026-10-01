//! Tests for [`Backend::sole_local`], the fail-closed key selector for
//! operator identity paths.
//!
//! `first_local` resolves a multi-key keystore by silently picking the
//! lexicographically-first public key — an unpredictable choice that can bind
//! an operator to a key they never selected (see issue #1552 and the CLI-side
//! fix in #1549). `sole_local` must return the only key, and must refuse to
//! guess otherwise, naming the candidates.

#![cfg(all(feature = "ecdsa", feature = "std"))]

use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::K256Ecdsa;
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};

fn keystore_with_keys(seeds: &[&[u8]]) -> (Keystore, tempfile::TempDir) {
    let dir = tempfile::tempdir().expect("tempdir");
    let keystore = Keystore::new(KeystoreConfig::new().fs_root(dir.path())).expect("keystore");

    for (i, seed) in seeds.iter().enumerate() {
        let public = keystore
            .generate::<K256Ecdsa>(Some(seed))
            .unwrap_or_else(|e| panic!("seed {i} should generate: {e}"));
        assert_eq!(
            keystore
                .list_local::<K256Ecdsa>()
                .expect("list")
                .iter()
                .filter(|p| p.to_bytes() == public.to_bytes())
                .count(),
            1,
            "seed {i} should insert a distinct key"
        );
    }

    (keystore, dir)
}

/// Control: a keystore holding exactly one key resolves to that key.
#[test]
fn sole_local_resolves_single_key() {
    let (keystore, _dir) = keystore_with_keys(&[b"sole-local-control-seed"]);
    let expected = keystore
        .list_local::<K256Ecdsa>()
        .expect("list")
        .first()
        .copied()
        .expect("one key");

    let got = keystore.sole_local::<K256Ecdsa>().expect("sole key");

    assert_eq!(got.to_bytes(), expected.to_bytes());
}

/// An empty keystore is a missing key, not a guess.
#[test]
fn sole_local_errors_when_empty() {
    let (keystore, _dir) = keystore_with_keys(&[]);

    let err = keystore
        .sole_local::<K256Ecdsa>()
        .expect_err("empty keystore must not resolve");

    assert!(
        matches!(err, blueprint_keystore::error::Error::KeyNotFound),
        "expected KeyNotFound, got: {err}"
    );
}

/// The defect this closes: two keys of the same type are a supported state
/// (distinct seeds, no replacement), and identity selection must refuse to
/// pick one silently. The error must name every candidate so the operator can
/// act on it.
#[test]
fn sole_local_refuses_ambiguous_keystore_and_names_candidates() {
    let (keystore, _dir) = keystore_with_keys(&[b"sole-local-a", b"sole-local-b"]);

    let listed = keystore.list_local::<K256Ecdsa>().expect("list");
    assert_eq!(listed.len(), 2, "fixture must hold two distinct keys");

    let err = keystore
        .sole_local::<K256Ecdsa>()
        .expect_err("ambiguous keystore must not resolve")
        .to_string();

    assert!(err.contains("2 keys"), "error should count the keys: {err}");
    assert!(
        err.contains("refusing to guess"),
        "error should say why: {err}"
    );
    for public in &listed {
        let candidate = hex::encode(public.to_bytes());
        assert!(
            err.contains(&candidate),
            "error must name candidate {candidate}: {err}"
        );
    }
}

/// Contrast: `first_local` silently returns the lexicographically-first key on
/// the same fixture. This pins the behavior identity paths must not rely on;
/// if it ever starts erroring, this test and the doc note on `first_local`
/// are the tripwire.
#[test]
fn first_local_still_picks_lexicographically_first() {
    let (keystore, _dir) = keystore_with_keys(&[b"sole-local-a", b"sole-local-b"]);

    let mut sorted = keystore.list_local::<K256Ecdsa>().expect("list");
    sorted.sort_unstable();
    let first = keystore.first_local::<K256Ecdsa>().expect("first");

    assert_eq!(
        first.to_bytes(),
        sorted[0].to_bytes(),
        "first_local must remain the lexicographic minimum for compatibility"
    );
}
