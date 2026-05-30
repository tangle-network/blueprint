//! Atomic binary swap pipeline.
//!
//! Used by the watcher when an `AUTO` policy or a confirmed `APPROVE` ack
//! indicates the manager should move a service onto a new binary. The
//! sequence is identical for both entrypoints — that uniformity is the only
//! way to keep the safety invariants honest:
//!
//!   1. Resolve `effectiveBinaryVersion(serviceId)` — that is the trust root.
//!   2. Download bytes from the published URI into a temp file under the
//!      service's existing cache dir.
//!   3. Compute sha256 (and blake3 if present) using the **same** helper the
//!      initial-fetch path uses. Mismatch -> abort, never run.
//!   4. Verify attestation if `attestationHash != 0`. Failure -> caller
//!      downgrades to APPROVE-style notification.
//!   5. Graceful-shutdown the existing service (drain).
//!   6. Re-spawn the service from the new binary.
//!   7. Update the `UpgradeState.running` slot to the new (versionId, sha256).
//!
//! Step (5) MUST come before step (6): blueprints expose a graceful-shutdown
//! API and killing mid-job loses work.

use super::error::{Result, UpgradeError};
use super::types::BinaryVersionInfo;
use crate::error::Error as ManagerError;
use crate::sdk::utils::make_executable;
use crate::sources::manifest::{
    bytes_are_manifest, parse_manifest, select_for_current_platform, uri_looks_like_manifest,
};
use crate::sources::remote::verify_binary_digest;
use blueprint_core::{info, warn};
use reqwest::Client;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use url::Url;

const DOWNLOAD_RETRIES: usize = 3;
const RETRY_BACKOFF_MS: u64 = 500;
const MAX_BINARY_BYTES_ENV: &str = "MAX_ARCHIVE_BYTES";
const DEFAULT_MAX_BINARY_BYTES: u64 = 512 * 1024 * 1024; // 512 MiB

const IPFS_GATEWAY_ENV: &str = "IPFS_GATEWAY_URL";

/// Downloads the binary referenced by `version.binary_uri` into the service's
/// cache directory and verifies it against `version.sha256`.
///
/// On success returns the path to an **executable** file gated by both
/// digests. Caller MUST treat any error here as "do not run this binary."
pub async fn download_and_verify(
    service_id: u64,
    cache_dir: &Path,
    version: &BinaryVersionInfo,
) -> Result<PathBuf> {
    fs::create_dir_all(cache_dir).await?;

    // Cache key includes the host arch so an `x86_64` artifact and an
    // `aarch64` artifact published under the same on-chain (versionId, sha256)
    // never collide on a shared cache dir. Legacy raw-URI swaps inherit the
    // same suffix harmlessly.
    let dest = cache_dir.join(format!(
        "binary-v{}-{}-{}",
        version.version_id,
        std::env::consts::ARCH,
        hex::encode(&version.sha256.as_slice()[..8])
    ));

    let max_bytes = std::env::var(MAX_BINARY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(DEFAULT_MAX_BINARY_BYTES);

    // Idempotency: a previously cached + verified artifact for this platform is
    // reused after a recheck. Manifest mode caches the per-asset bytes, so the
    // recheck digest differs by mode: raw mode rechecks against the on-chain
    // sha256, manifest mode cannot know the asset digest without re-reading the
    // manifest, so it always re-resolves (cheap) before trusting the cache.
    if fs::try_exists(&dest).await.unwrap_or(false) {
        let manifest_mode = uri_looks_like_manifest(&version.binary_uri);
        if !manifest_mode {
            let sha = version.sha256.0;
            if verify_binary_digest(&dest, &sha, None).is_ok() {
                info!(
                    target: "upgrade",
                    service_id,
                    version_id = version.version_id,
                    "binary cache hit; reusing verified copy"
                );
                return Ok(make_executable(&dest)?);
            }
        }
        // Stale cache (sha drift, partial write, or manifest-mode re-resolve) —
        // drop it and go through the full download+verify path again.
        let _ = fs::remove_file(&dest).await;
    }

    let url = resolve_url(&version.binary_uri)?;
    let temp_path = dest.with_extension("part");
    download(&url, &temp_path, max_bytes).await?;

    // Decide path: the URI suffix is a hint, the parsed bytes are authoritative.
    let downloaded = fs::read(&temp_path).await?;
    let is_manifest =
        uri_looks_like_manifest(&version.binary_uri) || bytes_are_manifest(&downloaded);

    if is_manifest {
        let result = swap_via_manifest(
            service_id,
            cache_dir,
            version,
            &temp_path,
            &downloaded,
            &dest,
            max_bytes,
        )
        .await;
        // The manifest itself was downloaded to `temp_path`; the resolved asset
        // lands at `dest`. Always purge the manifest temp file.
        let _ = fs::remove_file(&temp_path).await;
        return result;
    }

    // Legacy/raw mode: verify the literal download against the on-chain sha256.
    verify_against_onchain(service_id, version, &temp_path).await?;
    fs::rename(&temp_path, &dest).await?;
    Ok(make_executable(&dest)?)
}

/// Manifest-mode resolution: pin the manifest against the on-chain sha256,
/// select this host's entry, download + digest-verify that entry's artifact.
///
/// `manifest_path` holds the already-downloaded manifest bytes (also passed as
/// `manifest_bytes` to avoid a re-read); `dest` is the final cache path for the
/// resolved per-platform artifact.
async fn swap_via_manifest(
    service_id: u64,
    cache_dir: &Path,
    version: &BinaryVersionInfo,
    manifest_path: &Path,
    manifest_bytes: &[u8],
    dest: &Path,
    max_bytes: u64,
) -> Result<PathBuf> {
    // (1) Manifest integrity: the manifest bytes MUST hash to the on-chain
    // sha256. This is the trust root — a tampered manifest could redirect to a
    // malicious asset, so it is gated exactly like a raw binary.
    verify_against_onchain(service_id, version, manifest_path).await?;

    // (2) Parse + select this platform's entry. Parse failure or no arch match
    // aborts; we never run a foreign-arch (or unparseable) artifact.
    let manifest = parse_manifest(manifest_bytes).map_err(UpgradeError::Manager)?;
    let entry = select_for_current_platform(&manifest).ok_or_else(|| {
        UpgradeError::Manager(ManagerError::Other(format!(
            "binary manifest for service {service_id} version {} has no entry for {}/{}",
            version.version_id,
            std::env::consts::OS,
            std::env::consts::ARCH,
        )))
    })?;

    // (3) Download the selected artifact and verify against the manifest
    // entry's own sha256 (+ blake3 if present).
    let asset_url = resolve_url(&entry.url)?;
    let asset_tmp = cache_dir.join(format!(
        "binary-v{}-{}-asset.part",
        version.version_id,
        std::env::consts::ARCH
    ));
    download(&asset_url, &asset_tmp, max_bytes).await?;

    let expected_sha = entry.sha256_bytes().map_err(UpgradeError::Manager)?;
    let expected_blake3 = entry.blake3_bytes().map_err(UpgradeError::Manager)?;
    if let Err(err) = verify_binary_digest(&asset_tmp, &expected_sha, expected_blake3.as_ref()) {
        let _ = fs::remove_file(&asset_tmp).await;
        return Err(map_digest_error(service_id, version.version_id, err));
    }

    fs::rename(&asset_tmp, dest).await?;
    info!(
        target: "upgrade",
        service_id,
        version_id = version.version_id,
        arch = std::env::consts::ARCH,
        "resolved binary via manifest; per-asset digest verified"
    );
    Ok(make_executable(dest)?)
}

/// Verify a downloaded file against the on-chain `version.sha256`. Used for the
/// legacy raw binary and for manifest integrity. On mismatch the temp file is
/// purged and a `Sha256Mismatch` is returned; never leaves an unverified file.
async fn verify_against_onchain(
    service_id: u64,
    version: &BinaryVersionInfo,
    path: &Path,
) -> Result<()> {
    let sha = version.sha256.0;
    if let Err(err) = verify_binary_digest(path, &sha, None) {
        let _ = fs::remove_file(path).await;
        return Err(map_digest_error(service_id, version.version_id, err));
    }
    Ok(())
}

/// Map a digest-layer `HashMismatch` onto the upgrade-specific
/// `Sha256Mismatch` variant; pass everything else through unchanged.
fn map_digest_error(service_id: u64, version_id: u64, err: ManagerError) -> UpgradeError {
    match err {
        ManagerError::HashMismatch { expected, actual } => UpgradeError::Sha256Mismatch {
            service_id,
            version_id,
            expected,
            actual,
        },
        other => UpgradeError::Manager(other),
    }
}

fn resolve_url(raw: &str) -> Result<Url> {
    if let Some(rest) = raw.strip_prefix("ipfs://") {
        let gateway = std::env::var(IPFS_GATEWAY_ENV).map_err(|_| {
            UpgradeError::Manager(ManagerError::MissingIpfsGateway {
                url: raw.to_string(),
            })
        })?;
        let suffix = rest.trim_start_matches('/');
        let full = format!(
            "{}/{}",
            gateway.trim_end_matches('/'),
            suffix.trim_start_matches('/')
        );
        Url::parse(&full).map_err(|err| {
            UpgradeError::Manager(ManagerError::DownloadFailed {
                url: raw.to_string(),
                reason: format!("failed to build gateway URL: {err}"),
            })
        })
    } else {
        Url::parse(raw).map_err(|err| {
            UpgradeError::Manager(ManagerError::DownloadFailed {
                url: raw.to_string(),
                reason: err.to_string(),
            })
        })
    }
}

async fn download(url: &Url, dest: &Path, max_bytes: u64) -> Result<()> {
    let client = Client::builder()
        .build()
        .map_err(|e| UpgradeError::ChainRead(format!("http client: {e}")))?;

    let mut last_error = String::new();
    for attempt in 0..=DOWNLOAD_RETRIES {
        match client.get(url.clone()).send().await {
            Ok(resp) if resp.status().is_success() => {
                let total_len = resp.content_length().unwrap_or(0);
                if total_len > max_bytes && total_len > 0 {
                    return Err(UpgradeError::Manager(ManagerError::ArchiveTooLarge {
                        url: url.to_string(),
                        size: total_len,
                        max: max_bytes,
                    }));
                }
                let mut file = fs::File::create(dest).await?;
                let mut downloaded: u64 = 0;
                let mut resp = resp;
                while let Some(chunk) = resp.chunk().await.map_err(|err| {
                    UpgradeError::Manager(ManagerError::DownloadFailed {
                        url: url.to_string(),
                        reason: err.to_string(),
                    })
                })? {
                    downloaded += chunk.len() as u64;
                    if downloaded > max_bytes {
                        let _ = fs::remove_file(dest).await;
                        return Err(UpgradeError::Manager(ManagerError::ArchiveTooLarge {
                            url: url.to_string(),
                            size: downloaded,
                            max: max_bytes,
                        }));
                    }
                    file.write_all(&chunk).await?;
                }
                file.flush().await?;
                return Ok(());
            }
            Ok(resp) => {
                last_error = format!("HTTP {}", resp.status());
            }
            Err(err) => {
                last_error = err.to_string();
            }
        }
        if attempt < DOWNLOAD_RETRIES {
            let delay = RETRY_BACKOFF_MS * (attempt as u64 + 1);
            warn!(
                target: "upgrade",
                attempt = attempt + 1,
                url = %url,
                "binary download failed ({last_error}); retrying in {delay}ms"
            );
            tokio::time::sleep(Duration::from_millis(delay)).await;
        }
    }

    Err(UpgradeError::Manager(ManagerError::DownloadFailed {
        url: url.to_string(),
        reason: last_error,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::B256;
    use sha2::{Digest, Sha256};
    use tempfile::TempDir;

    fn version_for(bytes: &[u8]) -> BinaryVersionInfo {
        let sha: [u8; 32] = Sha256::digest(bytes).into();
        BinaryVersionInfo {
            version_id: 1,
            sha256: B256::from(sha),
            binary_uri: "http://invalid.local/never".into(),
            attestation_hash: B256::ZERO,
            published_at: 0,
            deprecated: false,
        }
    }

    #[tokio::test]
    async fn cache_hit_skips_download_when_sha256_matches() {
        // Regression guard for the swap pipeline: a previously verified
        // binary on disk MUST be reused (and rechecked) rather than
        // re-downloaded. Otherwise a network outage would block legitimate
        // already-verified upgrades.
        let dir = TempDir::new().unwrap();
        let payload = b"hello-world-blueprint-bytes";
        let version = version_for(payload);
        let cached = dir.path().join(format!(
            "binary-v{}-{}-{}",
            version.version_id,
            std::env::consts::ARCH,
            hex::encode(&version.sha256.as_slice()[..8])
        ));
        tokio::fs::write(&cached, payload).await.unwrap();

        let resolved = download_and_verify(1, dir.path(), &version).await.unwrap();
        assert_eq!(resolved, cached);
    }

    #[tokio::test]
    async fn corrupted_cache_is_purged_then_redownloaded() {
        // If the on-disk copy has been tampered with we MUST purge it. The
        // re-download will fail because the URL is unreachable, but the
        // important assertion is that we do not silently return the
        // mismatched cached file.
        let dir = TempDir::new().unwrap();
        let payload = b"hello-world-blueprint-bytes";
        let version = version_for(payload);
        let cached = dir.path().join(format!(
            "binary-v{}-{}-{}",
            version.version_id,
            std::env::consts::ARCH,
            hex::encode(&version.sha256.as_slice()[..8])
        ));
        tokio::fs::write(&cached, b"different-bytes-attacker-controlled")
            .await
            .unwrap();

        let err = download_and_verify(1, dir.path(), &version)
            .await
            .expect_err("expected download to fail for unreachable URL");
        // The contract violation we're catching is "silent return of stale
        // bytes"; any error variant other than HashMismatch / Sha256Mismatch
        // shows we correctly progressed past the cache check.
        assert!(
            !matches!(err, UpgradeError::Sha256Mismatch { .. }),
            "stale cache should have been purged before sha gate ran"
        );
        assert!(
            !tokio::fs::try_exists(&cached).await.unwrap(),
            "stale cached file should have been removed"
        );
    }

    #[tokio::test]
    // The `LOCK` MutexGuard intentionally crosses await: it serializes
    // process-global env mutation across concurrent tests in the same
    // process. An async Mutex would defeat that — env access is not
    // async-safe. The await it spans (`download_and_verify`) cannot
    // contend on this lock from any other code path.
    #[allow(clippy::await_holding_lock)]
    async fn rejects_ipfs_uri_without_gateway() {
        // Trust invariant: we will not silently swap from a URI scheme we
        // can't dereference. Without IPFS_GATEWAY_URL the gateway path must
        // fail loud — never default to "skip download, hope for the best."
        // SAFETY: test isolates env mutation behind a lock so concurrent
        // tests in the same process don't see a partially-modified env.
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = LOCK.lock().unwrap();

        // Stash any pre-existing gateway value so we don't poison the
        // process for the rest of the suite.
        let prior = std::env::var(IPFS_GATEWAY_ENV).ok();
        // SAFETY: env mutation guarded by LOCK above; only this test reads
        // IPFS_GATEWAY_URL within the upgrade module.
        unsafe { std::env::remove_var(IPFS_GATEWAY_ENV) };

        let dir = TempDir::new().unwrap();
        let mut version = version_for(b"unused");
        version.binary_uri = "ipfs://Qm-fake-cid".into();
        let err = download_and_verify(1, dir.path(), &version)
            .await
            .expect_err("ipfs:// URI without gateway must error");
        assert!(
            matches!(
                err,
                UpgradeError::Manager(ManagerError::MissingIpfsGateway { .. })
            ),
            "expected MissingIpfsGateway, got {err:?}"
        );

        if let Some(value) = prior {
            // SAFETY: same lock guards the restore.
            unsafe { std::env::set_var(IPFS_GATEWAY_ENV, value) };
        }
    }

    // ---------------------------------------------------------------------
    // Manifest-mode swap tests.
    //
    // These exercise the arch-aware resolution path end to end against a real
    // local HTTP server (mirrors the harness in
    // `tests/blueprint_sources_e2e.rs`). The regression they defend: an
    // aarch64 operator fetching the on-chain `binaryUri` literally would
    // receive an x86_64 binary. Manifest mode must instead select THIS host's
    // entry and verify its own digest.
    // ---------------------------------------------------------------------

    use crate::sources::manifest::MANIFEST_SCHEMA_V1;
    use std::net::SocketAddr;

    fn sha256_hex(bytes: &[u8]) -> String {
        hex::encode::<[u8; 32]>(Sha256::digest(bytes).into())
    }

    fn version_with_uri(uri: &str, manifest_bytes: &[u8]) -> BinaryVersionInfo {
        // The on-chain sha256 pins the MANIFEST bytes in manifest mode.
        let sha: [u8; 32] = Sha256::digest(manifest_bytes).into();
        BinaryVersionInfo {
            version_id: 7,
            sha256: B256::from(sha),
            binary_uri: uri.to_string(),
            attestation_hash: B256::ZERO,
            published_at: 0,
            deprecated: false,
        }
    }

    /// Bind an ephemeral port, hand the caller its address to build routes
    /// against, then serve those routes. This two-phase shape lets a manifest
    /// embed the asset URL of the very server that will host it without any
    /// fragile string rewriting.
    async fn serve_with<F>(build: F) -> (SocketAddr, tokio::task::JoinHandle<()>)
    where
        F: FnOnce(SocketAddr) -> Vec<(String, Vec<u8>)>,
    {
        use axum::Router;
        use axum::routing::get;

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let mut app = Router::new();
        for (path, data) in build(addr) {
            app = app.route(
                &path,
                get(move || {
                    let d = data.clone();
                    async move { d }
                }),
            );
        }

        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.ok();
        });
        (addr, handle)
    }

    /// Build a v1 manifest whose ONLY linux/macos entry for the host arch
    /// points at `asset_url`, plus a decoy entry for a different arch that
    /// points at a bogus URL. If the resolver were arch-blind it would pick
    /// the wrong (or first) entry.
    fn host_manifest(asset_url: &str, asset_sha: &str, decoy_url: &str) -> String {
        let host_os =
            crate::blueprint::native::normalize_os(&crate::sdk::utils::get_formatted_os_string());
        let host_arch = std::env::consts::ARCH;
        let decoy_arch = if host_arch == "x86_64" {
            "aarch64"
        } else {
            "x86_64"
        };
        format!(
            r#"{{
              "schema": "{MANIFEST_SCHEMA_V1}",
              "binaries": [
                {{ "os": "{host_os}", "arch": "{decoy_arch}",
                   "url": "{decoy_url}", "sha256": "{}" }},
                {{ "os": "{host_os}", "arch": "{host_arch}",
                   "url": "{asset_url}", "sha256": "{asset_sha}" }}
              ]
            }}"#,
            hex::encode([0xab_u8; 32]),
        )
    }

    #[tokio::test]
    async fn manifest_mode_selects_host_entry_and_verifies_digest() {
        // The core fix: with a multi-arch manifest the swap path must pick the
        // current host's artifact and gate it on that entry's own sha256.
        let dir = TempDir::new().unwrap();
        let asset = b"#!/bin/sh\necho host-binary\n".to_vec();
        let asset_sha = sha256_hex(&asset);

        // Capture the exact manifest bytes the server returns so we can pin the
        // on-chain sha256 to them.
        let captured: std::sync::Arc<std::sync::Mutex<Vec<u8>>> = Default::default();
        let cap = captured.clone();
        let asset2 = asset.clone();
        let (addr, server) = serve_with(move |addr| {
            let asset_url = format!("http://{addr}/host-asset.tar.xz");
            let decoy_url = format!("http://{addr}/decoy");
            let manifest = host_manifest(&asset_url, &asset_sha, &decoy_url).into_bytes();
            *cap.lock().unwrap() = manifest.clone();
            vec![
                ("/manifest.json".to_string(), manifest),
                ("/host-asset.tar.xz".to_string(), asset2),
            ]
        })
        .await;

        let manifest_bytes = captured.lock().unwrap().clone();
        let version = version_with_uri(&format!("http://{addr}/manifest.json"), &manifest_bytes);
        let resolved = download_and_verify(7, dir.path(), &version).await;
        server.abort();

        let path = resolved.expect("manifest-mode swap should resolve host artifact");
        assert_eq!(
            std::fs::read(&path).unwrap(),
            asset,
            "resolved binary must be the host artifact, not the decoy"
        );
    }

    #[tokio::test]
    async fn manifest_integrity_failure_aborts() {
        // The manifest bytes must hash to the on-chain sha256. A manifest that
        // doesn't match is a tampered trust root -> Sha256Mismatch, never run.
        let dir = TempDir::new().unwrap();
        let (addr, server) = serve_with(|addr| {
            let manifest = host_manifest(
                &format!("http://{addr}/asset"),
                &sha256_hex(b"x"),
                &format!("http://{addr}/decoy"),
            )
            .into_bytes();
            vec![("/manifest.json".to_string(), manifest)]
        })
        .await;

        // Pin the on-chain sha to DIFFERENT bytes so the integrity check fails.
        let version = version_with_uri(&format!("http://{addr}/manifest.json"), b"different");
        let err = download_and_verify(7, dir.path(), &version).await;
        server.abort();
        let err = err.expect_err("manifest integrity mismatch must abort");
        assert!(
            matches!(err, UpgradeError::Sha256Mismatch { .. }),
            "expected Sha256Mismatch on manifest integrity failure, got {err:?}"
        );
    }

    #[tokio::test]
    async fn manifest_no_arch_match_aborts() {
        // A manifest with no entry for the host arch must abort — never run a
        // foreign-arch binary.
        let dir = TempDir::new().unwrap();
        let host_os =
            crate::blueprint::native::normalize_os(&crate::sdk::utils::get_formatted_os_string());
        let foreign_arch = if std::env::consts::ARCH == "x86_64" {
            "aarch64"
        } else {
            "x86_64"
        };
        let manifest_bytes = format!(
            r#"{{"schema":"{MANIFEST_SCHEMA_V1}","binaries":[
                {{"os":"{host_os}","arch":"{foreign_arch}",
                  "url":"http://127.0.0.1:1/foreign","sha256":"{}"}}]}}"#,
            hex::encode([0x01_u8; 32]),
        )
        .into_bytes();

        let mb = manifest_bytes.clone();
        let (addr, server) = serve_with(move |_| vec![("/manifest.json".to_string(), mb)]).await;

        let version = version_with_uri(&format!("http://{addr}/manifest.json"), &manifest_bytes);
        let err = download_and_verify(7, dir.path(), &version).await;
        server.abort();
        let err = err.expect_err("no host-arch entry must abort");
        assert!(
            matches!(err, UpgradeError::Manager(ManagerError::Other(ref m)) if m.contains("no entry")),
            "expected no-arch-match abort, got {err:?}"
        );
    }

    #[tokio::test]
    async fn manifest_per_asset_sha_mismatch_aborts() {
        // Manifest integrity passes, host entry selected, but the downloaded
        // asset's bytes don't match the entry's sha256 -> Sha256Mismatch.
        let dir = TempDir::new().unwrap();
        let served_asset = b"actual-bytes-on-server".to_vec();
        // Manifest claims a sha that the asset does NOT have.
        let lying_sha = sha256_hex(b"what-the-manifest-claims");

        let captured: std::sync::Arc<std::sync::Mutex<Vec<u8>>> = Default::default();
        let cap = captured.clone();
        let asset2 = served_asset.clone();
        let (addr, server) = serve_with(move |addr| {
            let manifest = host_manifest(
                &format!("http://{addr}/asset.tar.xz"),
                &lying_sha,
                &format!("http://{addr}/decoy"),
            )
            .into_bytes();
            *cap.lock().unwrap() = manifest.clone();
            vec![
                ("/manifest.json".to_string(), manifest),
                ("/asset.tar.xz".to_string(), asset2),
            ]
        })
        .await;

        let manifest_bytes = captured.lock().unwrap().clone();
        let version = version_with_uri(&format!("http://{addr}/manifest.json"), &manifest_bytes);
        let err = download_and_verify(7, dir.path(), &version).await;
        server.abort();
        let err = err.expect_err("per-asset sha mismatch must abort");
        assert!(
            matches!(err, UpgradeError::Sha256Mismatch { .. }),
            "expected Sha256Mismatch on per-asset digest failure, got {err:?}"
        );
    }

    #[tokio::test]
    async fn legacy_raw_uri_path_still_works() {
        // Regression: a non-manifest (raw tarball) URI must behave exactly as
        // before — fetched literally, verified against the on-chain sha256.
        let dir = TempDir::new().unwrap();
        let raw = b"raw-genesis-v0-tarball-bytes".to_vec();

        let raw2 = raw.clone();
        let (addr, server) = serve_with(move |_| vec![("/binary.tar.xz".to_string(), raw2)]).await;
        let mut version = version_for(&raw); // on-chain sha == sha256(raw)
        version.binary_uri = format!("http://{addr}/binary.tar.xz");

        let resolved = download_and_verify(1, dir.path(), &version).await;
        server.abort();
        let path = resolved.expect("legacy raw-uri swap should succeed");
        assert_eq!(std::fs::read(&path).unwrap(), raw);
    }

    #[tokio::test]
    async fn legacy_raw_uri_sha_mismatch_aborts() {
        // Regression: raw mode still gates on the on-chain sha256.
        let dir = TempDir::new().unwrap();
        let (addr, server) =
            serve_with(|_| vec![("/binary.tar.xz".to_string(), b"served-bytes".to_vec())]).await;
        // on-chain sha pins DIFFERENT bytes.
        let mut version = version_for(b"expected-bytes");
        version.binary_uri = format!("http://{addr}/binary.tar.xz");

        let err = download_and_verify(1, dir.path(), &version).await;
        server.abort();
        let err = err.expect_err("raw sha mismatch must abort");
        assert!(
            matches!(err, UpgradeError::Sha256Mismatch { .. }),
            "expected Sha256Mismatch on raw digest failure, got {err:?}"
        );
    }
}
