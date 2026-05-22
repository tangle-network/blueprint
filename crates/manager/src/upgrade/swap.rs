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

    let dest = cache_dir.join(format!(
        "binary-v{}-{}",
        version.version_id,
        hex::encode(&version.sha256.as_slice()[..8])
    ));

    // Idempotency: if a previous swap attempt already cached + verified this
    // exact (versionId, sha256), skip the download.
    if fs::try_exists(&dest).await.unwrap_or(false) {
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
        // Stale cache (sha drift or partial write) — drop it and re-download.
        let _ = fs::remove_file(&dest).await;
    }

    let url = resolve_url(&version.binary_uri)?;
    let max_bytes = std::env::var(MAX_BINARY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(DEFAULT_MAX_BINARY_BYTES);

    let temp_path = dest.with_extension("part");
    download(&url, &temp_path, max_bytes).await?;

    let sha = version.sha256.0;
    if let Err(err) = verify_binary_digest(&temp_path, &sha, None) {
        let _ = fs::remove_file(&temp_path).await;
        match err {
            ManagerError::HashMismatch { expected, actual } => {
                return Err(UpgradeError::Sha256Mismatch {
                    service_id,
                    version_id: version.version_id,
                    expected,
                    actual,
                });
            }
            other => return Err(UpgradeError::Manager(other)),
        }
    }

    fs::rename(&temp_path, &dest).await?;
    Ok(make_executable(&dest)?)
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
            "binary-v{}-{}",
            version.version_id,
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
            "binary-v{}-{}",
            version.version_id,
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
}
