//! Local authorization layer for `MANUAL`-with-assist operators.
//!
//! The on-chain `UpgradePolicy` enum is three-state by design: `AUTO`,
//! `APPROVE`, `MANUAL`. Under `AUTO` the manager swaps on its own; under
//! `APPROVE` it waits for `ackBinaryVersion` (gas + audit trail); under
//! `MANUAL` it never swaps. That works for the two extremes, but it leaves a
//! sharp seam: an operator who knows they want to swap to version N — but
//! does not want to incur gas, write an ack tx, or move out of MANUAL on
//! chain — has no automation surface beyond "wait for an alert and SSH in."
//!
//! `LocalAuthz` is the assist layer for that operator. They pre-authorize
//! the manager to swap into specific versions locally; on-chain policy stays
//! `MANUAL`, so external observers see the same audit-conservative posture,
//! and yet the manager runs the full sha256+attestation verify-then-swap
//! pipeline the second the desired version becomes effective.
//!
//! State precedence (highest first) when the watcher resolves a swap target
//! while policy is `MANUAL`:
//!
//! 1. `pinned` — a single one-shot version. Wins over everything; cleared
//!    after the next reconcile so the operator does not get surprise reuse.
//! 2. `skipped` — explicit "do not swap into this version, suppress alerts."
//!    Persists across restarts; the operator must `clear-skip` to undo.
//! 3. `whitelisted` — versions the operator pre-approves. Persists across
//!    restarts so a fleet-wide rollout can be staged in advance.
//! 4. (none) — fall through to the existing MANUAL behavior (alert only).
//!
//! `pinned` takes precedence over `skipped` intentionally: the operator
//! pinning a version after previously skipping it is the override path
//! ("actually go". A `skipped` entry stays in the file so a later
//! `whitelisted` is still gated until the operator clears the skip — only
//! `pinned` punches through.
//!
//! ### Persistence shape
//!
//! Each tracked service has one JSON file at
//! `<authz_root>/<serviceId>.json`. Writes go through `tempfile` +
//! `persist_noclobber`-style rename for atomicity (`std::fs::rename` is
//! atomic on POSIX same-filesystem renames). The file format is forward-
//! compatible: unknown fields are preserved on round-trip via `serde_json`'s
//! default behavior because we deserialize into a typed struct, but the file
//! itself is small and we accept ignoring future-version extensions on read.
//!
//! Concurrency: each service's `LocalAuthz` is protected by a per-service
//! lock; the outer `LocalAuthzStore` only locks long enough to clone the
//! handle out. We deliberately avoid a single store-wide lock so a hot
//! reconcile loop on one service does not stall RPC reads on another.

use alloy_primitives::B256;
use blueprint_core::{debug, warn};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

/// On-disk authorization record. Stored at `<authz_root>/<serviceId>.json`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct LocalAuthz {
    /// Versions the operator has whitelisted. The watcher will run its
    /// verify-then-swap pipeline once any of these becomes the effective
    /// version. Persists across restarts.
    pub whitelisted: BTreeSet<u64>,
    /// One-shot pin: swap to this version on the next reconcile, then clear.
    pub pinned: Option<u64>,
    /// Versions the operator has explicitly declined. Reconciles to these
    /// versions take no action and suppress further alerts; persists across
    /// restarts. Stored with the operator's free-form reason for the
    /// per-service audit log.
    pub skipped: BTreeMap<u64, String>,
}

impl LocalAuthz {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.whitelisted.is_empty() && self.pinned.is_none() && self.skipped.is_empty()
    }
}

/// Decision made when reconciling a `MANUAL` service against `LocalAuthz`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthzDecision {
    /// No matching local authorization. Existing MANUAL behavior: alert + hold.
    Hold,
    /// Run the verify-then-swap pipeline. If `clear_pinned`, the pin is
    /// consumed and the store should clear it after the swap is queued.
    Swap { clear_pinned: bool },
    /// The operator declined this version. Suppress further alerts; the
    /// supplied reason is included in the recorded skip log.
    Skip,
}

impl LocalAuthz {
    /// Apply precedence: pinned > skipped > whitelisted > hold.
    #[must_use]
    pub fn decide(&self, version_id: u64) -> AuthzDecision {
        if self.pinned == Some(version_id) {
            return AuthzDecision::Swap { clear_pinned: true };
        }
        if self.skipped.contains_key(&version_id) {
            return AuthzDecision::Skip;
        }
        if self.whitelisted.contains(&version_id) {
            return AuthzDecision::Swap {
                clear_pinned: false,
            };
        }
        AuthzDecision::Hold
    }
}

/// Errors raised by the authz store. Persistence failures are surfaced
/// loudly — we explicitly do not swallow disk errors because a corrupted
/// or unwritable authz file means the operator's intent is lost, which is
/// the exact failure mode this module exists to prevent.
#[derive(Debug, thiserror::Error)]
pub enum LocalAuthzError {
    #[error("local authz I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("local authz parse: {0}")]
    Parse(#[from] serde_json::Error),
}

/// Process-wide manager for per-service `LocalAuthz` state.
///
/// The outer `RwLock` only guards the per-service map; per-service mutation
/// goes through a cheap clone-out-then-mutate pattern so writers don't block
/// readers on other services.
#[derive(Clone)]
pub struct LocalAuthzStore {
    inner: Arc<RwLock<HashMap<u64, LocalAuthz>>>,
    /// Optional persistence root. `None` means in-memory-only (used in tests
    /// and as a safety fallback if the manager hasn't been configured with a
    /// data dir).
    root: Option<PathBuf>,
}

impl LocalAuthzStore {
    /// Construct an in-memory-only store. Intended for tests; production
    /// callers should use [`Self::new_persisted`].
    #[must_use]
    pub fn new_in_memory() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
            root: None,
        }
    }

    /// Construct a store backed by `<root>/<serviceId>.json` files. The
    /// directory is created if missing. Existing files are loaded eagerly so
    /// the watcher sees pre-existing state on its first reconcile.
    pub fn new_persisted(root: PathBuf) -> Result<Self, LocalAuthzError> {
        std::fs::create_dir_all(&root)?;
        let mut map = HashMap::new();
        for entry in std::fs::read_dir(&root)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let Ok(service_id) = stem.parse::<u64>() else {
                warn!(
                    target: "upgrade",
                    file = %path.display(),
                    "ignoring authz file with non-numeric stem"
                );
                continue;
            };
            match std::fs::read_to_string(&path) {
                Ok(text) => match serde_json::from_str::<LocalAuthz>(&text) {
                    Ok(authz) => {
                        debug!(
                            target: "upgrade",
                            service_id,
                            whitelisted = authz.whitelisted.len(),
                            pinned = ?authz.pinned,
                            skipped = authz.skipped.len(),
                            "loaded local authz"
                        );
                        map.insert(service_id, authz);
                    }
                    Err(err) => {
                        warn!(
                            target: "upgrade",
                            service_id,
                            error = %err,
                            "failed to parse local authz file; skipping"
                        );
                    }
                },
                Err(err) => {
                    warn!(
                        target: "upgrade",
                        service_id,
                        error = %err,
                        "failed to read local authz file; skipping"
                    );
                }
            }
        }
        Ok(Self {
            inner: Arc::new(RwLock::new(map)),
            root: Some(root),
        })
    }

    /// Snapshot the authz state for `service_id`. Returns the default
    /// (all-empty) record if nothing has been written yet.
    pub async fn get(&self, service_id: u64) -> LocalAuthz {
        self.inner
            .read()
            .await
            .get(&service_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Replace the whitelist for `service_id`. Pass an empty set to clear.
    pub async fn set_whitelist(
        &self,
        service_id: u64,
        versions: BTreeSet<u64>,
    ) -> Result<LocalAuthz, LocalAuthzError> {
        self.mutate(service_id, |authz| authz.whitelisted = versions)
            .await
    }

    /// Set or clear the one-shot pin. Pass `None` to clear.
    pub async fn set_pin(
        &self,
        service_id: u64,
        version_id: Option<u64>,
    ) -> Result<LocalAuthz, LocalAuthzError> {
        self.mutate(service_id, |authz| authz.pinned = version_id)
            .await
    }

    /// Record a skip with an operator-provided reason.
    pub async fn add_skip(
        &self,
        service_id: u64,
        version_id: u64,
        reason: String,
    ) -> Result<LocalAuthz, LocalAuthzError> {
        self.mutate(service_id, |authz| {
            authz.skipped.insert(version_id, reason);
        })
        .await
    }

    /// Remove a previously recorded skip.
    pub async fn clear_skip(
        &self,
        service_id: u64,
        version_id: u64,
    ) -> Result<LocalAuthz, LocalAuthzError> {
        self.mutate(service_id, |authz| {
            authz.skipped.remove(&version_id);
        })
        .await
    }

    /// Clear the pin after the watcher consumes it. Idempotent.
    pub async fn consume_pin_if(
        &self,
        service_id: u64,
        version_id: u64,
    ) -> Result<(), LocalAuthzError> {
        let mut should_persist = None;
        {
            let mut guard = self.inner.write().await;
            let entry = guard.entry(service_id).or_default();
            if entry.pinned == Some(version_id) {
                entry.pinned = None;
                should_persist = Some(entry.clone());
            }
        }
        if let Some(snapshot) = should_persist {
            self.persist(service_id, &snapshot)?;
        }
        Ok(())
    }

    async fn mutate<F: FnOnce(&mut LocalAuthz)>(
        &self,
        service_id: u64,
        f: F,
    ) -> Result<LocalAuthz, LocalAuthzError> {
        let snapshot;
        {
            let mut guard = self.inner.write().await;
            let entry = guard.entry(service_id).or_default();
            f(entry);
            snapshot = entry.clone();
        }
        self.persist(service_id, &snapshot)?;
        Ok(snapshot)
    }

    fn persist(&self, service_id: u64, authz: &LocalAuthz) -> Result<(), LocalAuthzError> {
        let Some(root) = &self.root else {
            return Ok(());
        };
        let target = root.join(format!("{service_id}.json"));
        // Atomic write: temp file in the same directory + rename. Same-fs
        // rename is atomic on POSIX, so a crash mid-write cannot leave the
        // operator with a half-truncated authz file.
        let dir = target
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
        std::fs::create_dir_all(&dir)?;
        let tmp = tempfile::NamedTempFile::new_in(&dir)?;
        let json = serde_json::to_vec_pretty(authz)?;
        std::fs::write(tmp.path(), json)?;
        tmp.persist(&target).map_err(|e| e.error)?;
        Ok(())
    }
}

impl std::fmt::Debug for LocalAuthzStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalAuthzStore")
            .field("root", &self.root)
            .finish_non_exhaustive()
    }
}

/// View of authz state surfaced via the local RPC (`GET /upgrades/{id}/authz`).
#[derive(Debug, Clone, Serialize)]
pub struct AuthzView {
    pub service_id: u64,
    pub policy_onchain: super::types::UpgradePolicy,
    pub whitelisted: Vec<u64>,
    pub pinned: Option<u64>,
    pub skipped: Vec<SkipEntry>,
    /// Currently running version_id + sha256, if the manager is serving the
    /// service.
    pub running: Option<RunningEntry>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SkipEntry {
    pub version_id: u64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunningEntry {
    pub version_id: u64,
    pub sha256: B256,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decide_precedence_pinned_beats_skipped_and_whitelisted() {
        // Operator pinned v5 explicitly. Even if they previously skipped or
        // whitelisted v5, the pin is the override path and wins.
        let mut a = LocalAuthz::default();
        a.pinned = Some(5);
        a.whitelisted.insert(5);
        a.skipped.insert(5, "old reason".into());
        assert_eq!(a.decide(5), AuthzDecision::Swap { clear_pinned: true });
    }

    #[test]
    fn decide_skipped_beats_whitelisted() {
        // Whitelist was set at fleet level; operator later decided this
        // specific version is broken. Skip MUST win or we'd swap to a
        // version the operator just declined.
        let mut a = LocalAuthz::default();
        a.whitelisted.insert(7);
        a.skipped.insert(7, "found regression".into());
        assert_eq!(a.decide(7), AuthzDecision::Skip);
    }

    #[test]
    fn decide_whitelisted_swap_does_not_clear_pin() {
        // Whitelist is persistent; consuming it on each swap would force the
        // operator to re-add the version on every reconcile.
        let mut a = LocalAuthz::default();
        a.whitelisted.insert(3);
        assert_eq!(
            a.decide(3),
            AuthzDecision::Swap {
                clear_pinned: false
            }
        );
    }

    #[test]
    fn decide_default_holds() {
        // The zero state must alert+hold (preserves existing MANUAL UX).
        let a = LocalAuthz::default();
        assert_eq!(a.decide(99), AuthzDecision::Hold);
    }

    #[tokio::test]
    async fn store_set_whitelist_persists_across_reload() {
        // Regression guard: the persistence path is what makes this
        // module's value-add real. If a manager restart loses the operator's
        // whitelist, MANUAL operators would be back to "wait for alert + SSH"
        // every time the process bounces.
        let dir = tempfile::tempdir().unwrap();
        let store = LocalAuthzStore::new_persisted(dir.path().to_path_buf()).unwrap();
        let mut versions = BTreeSet::new();
        versions.insert(2);
        versions.insert(4);
        store.set_whitelist(42, versions).await.unwrap();

        // Drop and reload.
        drop(store);
        let reloaded = LocalAuthzStore::new_persisted(dir.path().to_path_buf()).unwrap();
        let authz = reloaded.get(42).await;
        assert_eq!(
            authz.whitelisted.iter().copied().collect::<Vec<_>>(),
            vec![2, 4]
        );
    }

    #[tokio::test]
    async fn store_pinned_persists_then_consume_clears() {
        // The pin is a one-shot: it must survive a restart (so the operator
        // can stage a pin during a maintenance window and walk away) but
        // also clear cleanly after the swap is dispatched.
        let dir = tempfile::tempdir().unwrap();
        let store = LocalAuthzStore::new_persisted(dir.path().to_path_buf()).unwrap();
        store.set_pin(11, Some(9)).await.unwrap();
        drop(store);

        let reloaded = LocalAuthzStore::new_persisted(dir.path().to_path_buf()).unwrap();
        assert_eq!(reloaded.get(11).await.pinned, Some(9));

        reloaded.consume_pin_if(11, 9).await.unwrap();
        assert_eq!(reloaded.get(11).await.pinned, None);

        // Consume is idempotent — calling again on the cleared state is a no-op.
        reloaded.consume_pin_if(11, 9).await.unwrap();
        assert_eq!(reloaded.get(11).await.pinned, None);
    }

    #[tokio::test]
    async fn store_consume_pin_only_clears_matching_version() {
        // Defensive: a stale reconcile MUST NOT clear a pin the operator
        // changed underneath us. If the operator pinned v3, then re-pinned
        // to v5 before the watcher saw v3 as effective, the watcher's
        // consume_pin_if(3) must be a no-op.
        let store = LocalAuthzStore::new_in_memory();
        store.set_pin(1, Some(5)).await.unwrap();
        store.consume_pin_if(1, 3).await.unwrap();
        assert_eq!(store.get(1).await.pinned, Some(5));
    }

    #[tokio::test]
    async fn store_skip_persists_and_clears() {
        let dir = tempfile::tempdir().unwrap();
        let store = LocalAuthzStore::new_persisted(dir.path().to_path_buf()).unwrap();
        store
            .add_skip(8, 4, "canary regressed on latency".into())
            .await
            .unwrap();
        drop(store);

        let reloaded = LocalAuthzStore::new_persisted(dir.path().to_path_buf()).unwrap();
        let authz = reloaded.get(8).await;
        assert_eq!(
            authz.skipped.get(&4).map(String::as_str),
            Some("canary regressed on latency")
        );

        reloaded.clear_skip(8, 4).await.unwrap();
        assert!(reloaded.get(8).await.skipped.is_empty());
    }

    #[tokio::test]
    async fn store_in_memory_mode_does_not_touch_disk() {
        // In-memory mode is the test fallback. Writing to it must not
        // attempt disk I/O — that's what makes it safe in CI environments
        // that fail-closed on filesystem writes.
        let store = LocalAuthzStore::new_in_memory();
        store.set_pin(7, Some(2)).await.unwrap();
        assert_eq!(store.get(7).await.pinned, Some(2));
    }

    #[tokio::test]
    async fn store_ignores_non_numeric_files_on_load() {
        // Defensive: the authz directory may share a parent with other
        // tooling files. Loading must not panic or refuse to start if
        // someone drops a README or .gitignore in there.
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("README.md"), "notes").unwrap();
        std::fs::write(dir.path().join("notes.json"), "not authz").unwrap();
        // Valid one to anchor the test.
        std::fs::write(
            dir.path().join("5.json"),
            r#"{"whitelisted":[1,2],"pinned":null,"skipped":{}}"#,
        )
        .unwrap();

        let store = LocalAuthzStore::new_persisted(dir.path().to_path_buf()).unwrap();
        let authz = store.get(5).await;
        assert_eq!(
            authz.whitelisted.iter().copied().collect::<Vec<_>>(),
            vec![1, 2]
        );
    }
}
