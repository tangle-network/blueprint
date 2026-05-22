//! In-memory state shared between the upgrade watcher, the local RPC, and the
//! CLI surface.
//!
//! - `running` tracks the (versionId, sha256) the manager is currently
//!   serving for each service. Compared against `effectiveBinaryVersion` to
//!   decide whether a swap is needed.
//! - `policies` is a local cache of `getServiceUpgradePolicy` so the RPC can
//!   answer without an extra RPC round-trip; the watcher refreshes this on
//!   `ServiceUpgradePolicySet` events.
//! - `pending` holds upgrades that are visible to operators (APPROVE policy)
//!   or downgrades-from-AUTO (attestation failure).
//! - `history` is a bounded ring of recent swap results for `GET /upgrades/history`.

use super::types::{PendingUpgrade, UpgradeHistoryEntry, UpgradePolicy};
use alloy_primitives::B256;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Maximum upgrade-history entries kept in memory. Anything beyond this is
/// dropped from the head; an operator should ship the structured logs to
/// long-term storage if they want full history.
const HISTORY_CAP: usize = 256;

#[derive(Debug, Clone, Copy)]
pub struct RunningBinary {
    pub version_id: u64,
    pub sha256: B256,
}

#[derive(Default)]
struct Inner {
    running: HashMap<u64, RunningBinary>,
    policies: HashMap<u64, UpgradePolicy>,
    pending: HashMap<u64, PendingUpgrade>,
    history: Vec<UpgradeHistoryEntry>,
}

#[derive(Clone, Default)]
pub struct UpgradeState {
    inner: Arc<RwLock<Inner>>,
}

impl UpgradeState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn set_running(&self, service_id: u64, running: RunningBinary) {
        self.inner.write().await.running.insert(service_id, running);
    }

    pub async fn clear_running(&self, service_id: u64) {
        let mut guard = self.inner.write().await;
        guard.running.remove(&service_id);
        guard.pending.remove(&service_id);
    }

    pub async fn running(&self, service_id: u64) -> Option<RunningBinary> {
        self.inner.read().await.running.get(&service_id).copied()
    }

    pub async fn set_policy(&self, service_id: u64, policy: UpgradePolicy) {
        self.inner.write().await.policies.insert(service_id, policy);
    }

    pub async fn policy(&self, service_id: u64) -> Option<UpgradePolicy> {
        self.inner.read().await.policies.get(&service_id).copied()
    }

    pub async fn record_pending(&self, upgrade: PendingUpgrade) {
        self.inner
            .write()
            .await
            .pending
            .insert(upgrade.service_id, upgrade);
    }

    pub async fn clear_pending(&self, service_id: u64) {
        self.inner.write().await.pending.remove(&service_id);
    }

    pub async fn pending(&self, service_id: u64) -> Option<PendingUpgrade> {
        self.inner.read().await.pending.get(&service_id).cloned()
    }

    pub async fn list_pending(&self) -> Vec<PendingUpgrade> {
        self.inner.read().await.pending.values().cloned().collect()
    }

    pub async fn push_history(&self, entry: UpgradeHistoryEntry) {
        let mut guard = self.inner.write().await;
        if guard.history.len() >= HISTORY_CAP {
            guard.history.remove(0);
        }
        guard.history.push(entry);
    }

    pub async fn history(&self) -> Vec<UpgradeHistoryEntry> {
        self.inner.read().await.history.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::upgrade::types::{PendingUpgrade, UpgradeOutcome, UpgradePolicy};
    use std::time::SystemTime;

    fn dummy_pending(service_id: u64) -> PendingUpgrade {
        PendingUpgrade {
            service_id,
            blueprint_id: 1,
            current_version_id: Some(0),
            current_sha256: Some(B256::ZERO),
            available_version_id: 1,
            available_sha256: B256::repeat_byte(0xab),
            available_uri: "ipfs://test".into(),
            available_attestation_hash: B256::ZERO,
            policy: UpgradePolicy::Approve,
            detected_at: SystemTime::now(),
        }
    }

    #[tokio::test]
    async fn running_binary_round_trip() {
        let state = UpgradeState::new();
        state
            .set_running(
                7,
                RunningBinary {
                    version_id: 3,
                    sha256: B256::repeat_byte(0xcd),
                },
            )
            .await;
        let got = state.running(7).await.unwrap();
        assert_eq!(got.version_id, 3);
        assert_eq!(got.sha256, B256::repeat_byte(0xcd));
    }

    #[tokio::test]
    async fn clearing_running_also_clears_pending() {
        // Invariant: if the manager stops serving a service we should not keep
        // its pending upgrade exposed to operators.
        let state = UpgradeState::new();
        state.record_pending(dummy_pending(9)).await;
        state
            .set_running(
                9,
                RunningBinary {
                    version_id: 0,
                    sha256: B256::ZERO,
                },
            )
            .await;
        state.clear_running(9).await;
        assert!(state.pending(9).await.is_none());
        assert!(state.running(9).await.is_none());
    }

    #[tokio::test]
    async fn history_is_bounded_by_cap() {
        // Guards a memory-leak regression: a long-running manager swapping
        // binaries hourly should not grow unbounded.
        let state = UpgradeState::new();
        for i in 0..(HISTORY_CAP + 32) {
            state
                .push_history(UpgradeHistoryEntry {
                    service_id: i as u64,
                    blueprint_id: 1,
                    from_version_id: None,
                    from_sha256: None,
                    to_version_id: 1,
                    to_sha256: B256::ZERO,
                    policy: UpgradePolicy::Auto,
                    completed_at: SystemTime::now(),
                    outcome: UpgradeOutcome::Swapped,
                })
                .await;
        }
        assert_eq!(state.history().await.len(), HISTORY_CAP);
        // Oldest entries dropped from the head — newest should still be 287.
        assert_eq!(
            state.history().await.last().unwrap().service_id,
            (HISTORY_CAP + 32 - 1) as u64
        );
    }
}
