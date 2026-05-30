//! Architecture-aware binary manifest.
//!
//! An on-chain `BinaryVersion` carries a single `binaryUri` + single `sha256`.
//! That is fine for a single-arch raw tarball, but an operator fleet spans
//! `x86_64` and `aarch64` — a literal fetch hands an aarch64 box an x86_64
//! binary. To stay backward compatible while fixing that, the manager treats
//! `binaryUri` as one of two things:
//!
//!   * a **manifest** (this module): a small JSON document, itself pinned by
//!     the on-chain `sha256`, that lists one download per `(os, arch)`. The
//!     manager verifies the manifest bytes against the on-chain hash, selects
//!     the entry for the current platform, then verifies the downloaded asset
//!     against that entry's own `sha256` (+ optional `blake3`).
//!   * a **raw binary/tarball** (legacy): fetched literally and verified
//!     against the on-chain `sha256`, exactly as before.
//!
//! Platform matching reuses [`normalize_os`]/[`normalize_arch`] from the
//! initial-fetch path so the two resolvers can never select different
//! binaries for the same host.

use crate::blueprint::native::{normalize_arch, normalize_os};
use crate::error::Error;
use crate::sdk::utils::get_formatted_os_string;
use serde::Deserialize;

/// Schema discriminator embedded in every manifest. Bumping the version is a
/// wire-protocol change: old managers must reject documents they don't
/// understand rather than guess.
pub const MANIFEST_SCHEMA_V1: &str = "tangle-binary-manifest/v1";

/// Architecture-aware binary manifest, version 1.
///
/// Deserialized from the bytes referenced by an on-chain `binaryUri`. The
/// `schema` field is validated on parse; unknown values are rejected.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct BinaryManifest {
    /// Must equal [`MANIFEST_SCHEMA_V1`].
    pub schema: String,
    /// One entry per supported `(os, arch)`. Order is not significant.
    pub binaries: Vec<ManifestBinary>,
}

/// A single per-platform download within a [`BinaryManifest`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ManifestBinary {
    /// OS identifier (e.g. `linux`, `macos`). Compared via [`normalize_os`].
    pub os: String,
    /// Architecture identifier (e.g. `x86_64`, `aarch64`). Compared via
    /// [`normalize_arch`].
    pub arch: String,
    /// Fully-qualified download URL for this platform's artifact. May use the
    /// `ipfs://` scheme; the caller resolves it through the configured gateway.
    pub url: String,
    /// Hex-encoded (no `0x`) sha256 of the artifact at `url`. Required — the
    /// per-asset digest is the trust root once the manifest itself is pinned.
    pub sha256: String,
    /// Optional hex-encoded blake3 of the artifact, verified in addition to
    /// sha256 when present.
    #[serde(default)]
    pub blake3: Option<String>,
}

impl ManifestBinary {
    /// Decode `sha256` into a 32-byte array.
    ///
    /// # Errors
    ///
    /// `Error::Other` if the field is not 32 bytes of hex.
    pub fn sha256_bytes(&self) -> Result<[u8; 32], Error> {
        decode_hex32(&self.sha256, "sha256")
    }

    /// Decode the optional `blake3` field into a 32-byte array.
    ///
    /// # Errors
    ///
    /// `Error::Other` if the field is present but not 32 bytes of hex.
    pub fn blake3_bytes(&self) -> Result<Option<[u8; 32]>, Error> {
        match &self.blake3 {
            Some(hex) => Ok(Some(decode_hex32(hex, "blake3")?)),
            None => Ok(None),
        }
    }
}

/// Heuristic used to decide whether to treat a `binaryUri` as a manifest based
/// on its path alone (before any bytes are fetched). A `.json` suffix is the
/// publish convention; bytes that parse as a manifest are the authoritative
/// signal handled separately by [`parse_manifest`].
#[must_use]
pub fn uri_looks_like_manifest(uri: &str) -> bool {
    // Strip any query/fragment before checking the extension so
    // `…/manifest.json?token=…` still resolves as a manifest.
    let path = uri.split(['?', '#']).next().unwrap_or(uri);
    path.trim_end_matches('/')
        .rsplit('/')
        .next()
        .is_some_and(|name| name.to_ascii_lowercase().ends_with(".json"))
}

/// Parse and validate manifest bytes.
///
/// # Errors
///
/// * `Error::Serialization` if the bytes are not valid JSON of the expected
///   shape.
/// * `Error::Other` if the `schema` field is not [`MANIFEST_SCHEMA_V1`], or if
///   the manifest lists zero binaries.
pub fn parse_manifest(bytes: &[u8]) -> Result<BinaryManifest, Error> {
    let manifest: BinaryManifest = serde_json::from_slice(bytes)?;
    if manifest.schema != MANIFEST_SCHEMA_V1 {
        return Err(Error::Other(format!(
            "unsupported binary manifest schema `{}` (expected `{MANIFEST_SCHEMA_V1}`)",
            manifest.schema
        )));
    }
    if manifest.binaries.is_empty() {
        return Err(Error::Other(
            "binary manifest lists no binaries".to_string(),
        ));
    }
    Ok(manifest)
}

/// Best-effort detection: do these bytes parse as a valid v1 manifest?
///
/// Used to disambiguate the case where a `binaryUri` does not end in `.json`
/// but nonetheless points at a manifest (e.g. an IPFS CID). A raw tarball is
/// not valid UTF-8 JSON, so this is false for the legacy path.
#[must_use]
pub fn bytes_are_manifest(bytes: &[u8]) -> bool {
    parse_manifest(bytes).is_ok()
}

/// Select the manifest entry matching the host the manager is running on.
///
/// Returns `None` when no entry matches — the caller MUST abort (never run a
/// foreign-arch binary) rather than fall back to an arbitrary entry.
#[must_use]
pub fn select_for_current_platform(manifest: &BinaryManifest) -> Option<&ManifestBinary> {
    let host_os = normalize_os(&get_formatted_os_string());
    let host_arch = normalize_arch(std::env::consts::ARCH);
    manifest.binaries.iter().find(|binary| {
        normalize_os(&binary.os) == host_os && normalize_arch(&binary.arch) == host_arch
    })
}

fn decode_hex32(value: &str, field: &str) -> Result<[u8; 32], Error> {
    let trimmed = value.strip_prefix("0x").unwrap_or(value);
    let bytes = hex::decode(trimmed)
        .map_err(|err| Error::Other(format!("manifest `{field}` is not valid hex: {err}")))?;
    let array: [u8; 32] = bytes.try_into().map_err(|bytes: Vec<u8>| {
        Error::Other(format!(
            "manifest `{field}` must be 32 bytes, got {}",
            bytes.len()
        ))
    })?;
    Ok(array)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest_json() -> String {
        format!(
            r#"{{
              "schema": "{MANIFEST_SCHEMA_V1}",
              "binaries": [
                {{ "os": "linux", "arch": "x86_64",
                   "url": "https://x/a-x86_64-unknown-linux-gnu.tar.xz",
                   "sha256": "{}" }},
                {{ "os": "linux", "arch": "aarch64",
                   "url": "https://x/a-aarch64-unknown-linux-gnu.tar.xz",
                   "sha256": "{}", "blake3": "{}" }},
                {{ "os": "macos", "arch": "aarch64",
                   "url": "https://x/a-aarch64-apple-darwin.tar.xz",
                   "sha256": "{}" }}
              ]
            }}"#,
            hex::encode([0x11u8; 32]),
            hex::encode([0x22u8; 32]),
            hex::encode([0x33u8; 32]),
            hex::encode([0x44u8; 32]),
        )
    }

    #[test]
    fn parses_v1_and_decodes_digests() {
        let manifest = parse_manifest(manifest_json().as_bytes()).unwrap();
        assert_eq!(manifest.schema, MANIFEST_SCHEMA_V1);
        assert_eq!(manifest.binaries.len(), 3);
        assert_eq!(manifest.binaries[0].sha256_bytes().unwrap(), [0x11u8; 32]);
        assert_eq!(manifest.binaries[0].blake3_bytes().unwrap(), None);
        assert_eq!(manifest.binaries[1].sha256_bytes().unwrap(), [0x22u8; 32]);
        assert_eq!(
            manifest.binaries[1].blake3_bytes().unwrap(),
            Some([0x33u8; 32])
        );
    }

    #[test]
    fn rejects_unknown_schema() {
        // A future schema version must hard-fail on an old manager rather than
        // be silently misinterpreted as v1.
        let bad = r#"{"schema":"tangle-binary-manifest/v2","binaries":[]}"#;
        let err = parse_manifest(bad.as_bytes()).unwrap_err();
        assert!(matches!(err, Error::Other(msg) if msg.contains("unsupported")));
    }

    #[test]
    fn rejects_empty_binaries() {
        let bad = format!(r#"{{"schema":"{MANIFEST_SCHEMA_V1}","binaries":[]}}"#);
        let err = parse_manifest(bad.as_bytes()).unwrap_err();
        assert!(matches!(err, Error::Other(msg) if msg.contains("no binaries")));
    }

    #[test]
    fn raw_tarball_bytes_are_not_a_manifest() {
        // The legacy detection path: arbitrary binary bytes must never be
        // mistaken for a manifest.
        assert!(!bytes_are_manifest(&[0xde, 0xad, 0xbe, 0xef, 0x00, 0x1f]));
        assert!(!bytes_are_manifest(b"#!/bin/sh\necho hi\n"));
    }

    #[test]
    fn uri_suffix_detection() {
        assert!(uri_looks_like_manifest("https://x/manifest.json"));
        assert!(uri_looks_like_manifest("https://x/MANIFEST.JSON"));
        assert!(uri_looks_like_manifest("https://x/m.json?token=abc"));
        assert!(uri_looks_like_manifest("ipfs://Qm.../binary.json"));
        assert!(!uri_looks_like_manifest("https://x/binary.tar.xz"));
        assert!(!uri_looks_like_manifest("ipfs://Qmraw"));
    }

    #[test]
    fn invalid_hex_digest_is_rejected() {
        let bad = format!(
            r#"{{"schema":"{MANIFEST_SCHEMA_V1}","binaries":[
                {{"os":"linux","arch":"x86_64","url":"https://x/a","sha256":"zz"}}]}}"#
        );
        let manifest = parse_manifest(bad.as_bytes()).unwrap();
        assert!(manifest.binaries[0].sha256_bytes().is_err());
    }

    #[test]
    fn selects_entry_for_current_platform() {
        // Build a manifest that always contains an entry for whatever host the
        // test runs on, plus a decoy for a different arch. Selection must pick
        // the host entry, proving the resolver is arch-aware (the bug fix).
        let host_arch = std::env::consts::ARCH;
        let decoy_arch = if host_arch == "x86_64" {
            "aarch64"
        } else {
            "x86_64"
        };
        let host_os = normalize_os(&get_formatted_os_string());
        let manifest = BinaryManifest {
            schema: MANIFEST_SCHEMA_V1.to_string(),
            binaries: vec![
                ManifestBinary {
                    os: host_os.clone(),
                    arch: decoy_arch.to_string(),
                    url: "https://x/decoy".to_string(),
                    sha256: hex::encode([0x01u8; 32]),
                    blake3: None,
                },
                ManifestBinary {
                    os: host_os.clone(),
                    arch: host_arch.to_string(),
                    url: "https://x/host".to_string(),
                    sha256: hex::encode([0x02u8; 32]),
                    blake3: None,
                },
            ],
        };
        let selected = select_for_current_platform(&manifest).expect("host entry must match");
        assert_eq!(selected.url, "https://x/host");
        assert_eq!(normalize_arch(&selected.arch), normalize_arch(host_arch));
    }

    #[test]
    fn no_match_when_arch_absent() {
        // No entry for the host arch → None → caller aborts. This is the
        // "never run a foreign-arch binary" invariant.
        let decoy_arch = if std::env::consts::ARCH == "x86_64" {
            "aarch64"
        } else {
            "x86_64"
        };
        let manifest = BinaryManifest {
            schema: MANIFEST_SCHEMA_V1.to_string(),
            binaries: vec![ManifestBinary {
                os: normalize_os(&get_formatted_os_string()),
                arch: decoy_arch.to_string(),
                url: "https://x/decoy".to_string(),
                sha256: hex::encode([0x01u8; 32]),
                blake3: None,
            }],
        };
        assert!(select_for_current_platform(&manifest).is_none());
    }

    #[test]
    fn arch_aliases_match() {
        // amd64 / arm64 aliases must normalize to the Rust arch names so a
        // manifest published by foreign tooling still resolves.
        let host_os = normalize_os(&get_formatted_os_string());
        let (alias, _canonical) = match std::env::consts::ARCH {
            "x86_64" => ("amd64", "x86_64"),
            "aarch64" => ("arm64", "aarch64"),
            other => (other, other),
        };
        let manifest = BinaryManifest {
            schema: MANIFEST_SCHEMA_V1.to_string(),
            binaries: vec![ManifestBinary {
                os: host_os,
                arch: alias.to_string(),
                url: "https://x/aliased".to_string(),
                sha256: hex::encode([0x05u8; 32]),
                blake3: None,
            }],
        };
        assert!(select_for_current_platform(&manifest).is_some());
    }
}
