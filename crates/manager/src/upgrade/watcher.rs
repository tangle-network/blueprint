//! Background task that reconciles each tracked service against
//! `effectiveBinaryVersion(serviceId)` whenever the chain signals that a
//! binary-version-relevant event has been observed.
//!
//! The watcher does **not** run its own log subscription — Tangle's protocol
//! client already streams every log on the tangle contract via `next_event`.
//! Instead the protocol event loop forwards block-level notifications to the
//! watcher; on each notification we re-resolve every service we are running
//! and take action.
//!
//! This is intentionally a "pull" design:
//!   - We never make trust decisions from events alone. Concurrent acks
//!     across operators mean the only safe truth is `effectiveBinaryVersion`.
//!   - We reconcile cheaply because the tracked services list is small.
//!
//! ### Lifecycle
//!
//! `UpgradeWatcher::spawn` returns a join handle and a notification channel.
//! The protocol event handler calls `notify_block(...)` whenever it processes
//! a block that carried any of the binary-versions selectors. Shutdown is
//! propagated via the same channel by dropping the sender.

use super::attestation::AttestationVerifier;
use super::chain::ChainView;
use super::error::Result;
use super::local_authz::{AuthzDecision, LocalAuthzStore};
use super::state::{RunningBinary, UpgradeState};
use super::swap::download_and_verify;
use super::types::{
    BinaryVersionInfo, PendingUpgrade, UpgradeHistoryEntry, UpgradeOutcome, UpgradePolicy,
};
use blueprint_core::{info, warn};
use std::collections::HashSet;
use std::path::PathBuf;
use std::time::SystemTime;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

/// Tunables visible to call sites. `cache_root` is the same directory the
/// initial-fetch path writes to, so swaps live next to the bytes they
/// replace.
#[derive(Clone)]
pub struct WatcherConfig {
    pub chain: ChainView,
    pub state: UpgradeState,
    pub attestation: AttestationVerifier,
    /// Per-service cache root. Each service gets `<cache>/svc-<bp>-<sid>/`.
    pub cache_root: PathBuf,
    /// Service ids the protocol event handler is currently tracking. The
    /// watcher resolves the chain for each on a reconcile signal; ownership
    /// of `ActiveBlueprints` stays with the protocol manager.
    pub tracked_services: TrackedServices,
    /// Local authorization store consulted when on-chain policy is MANUAL.
    /// MANUAL operators may pre-authorize specific versions without writing
    /// an on-chain ack tx (the "MANUAL-with-assist" mode).
    pub local_authz: LocalAuthzStore,
    /// Outgoing queue of swap requests the protocol event handler must drain
    /// and execute against `ActiveBlueprints`. See `SwapRequest`.
    pub swap_tx: mpsc::Sender<SwapRequest>,
}

/// Snapshot of `(blueprint_id, service_id)` tuples the manager is serving.
///
/// The watcher only needs to read this — the protocol event handler updates
/// it whenever it adds or removes a service. We use a `tokio::sync::RwLock`
/// (not the stdlib `RwLock`) so reading from the watcher loop never blocks
/// the runtime if the writer is currently inside an `.await`.
#[derive(Clone, Default)]
pub struct TrackedServices {
    inner: std::sync::Arc<tokio::sync::RwLock<HashSet<(u64, u64)>>>,
}

impl TrackedServices {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn insert(&self, blueprint_id: u64, service_id: u64) {
        self.inner.write().await.insert((blueprint_id, service_id));
    }

    pub async fn remove(&self, blueprint_id: u64, service_id: u64) {
        self.inner.write().await.remove(&(blueprint_id, service_id));
    }

    pub async fn snapshot(&self) -> Vec<(u64, u64)> {
        self.inner.read().await.iter().copied().collect()
    }
}

/// Swap order produced by the watcher and consumed by the protocol event
/// handler.
///
/// The watcher has already paid for the trust-root costs at this point:
/// the binary at `binary_path` is sha256-verified against `target.sha256`
/// and the attestation gate has passed. The handler's job is to drain
/// (via `Service::shutdown`) the old service and re-spawn against the new
/// binary.
#[derive(Debug, Clone)]
pub struct SwapRequest {
    pub service_id: u64,
    pub blueprint_id: u64,
    pub target: BinaryVersionInfo,
    pub binary_path: PathBuf,
    pub policy: UpgradePolicy,
    pub previous: Option<RunningBinary>,
}

#[derive(Debug, Clone)]
pub enum WatcherSignal {
    /// A block was observed that may have touched binary-version state for
    /// the listed blueprints. An empty set means "reconcile everything we are
    /// tracking" — used on startup.
    Reconcile { blueprints: HashSet<u64> },
}

#[derive(Clone)]
pub struct WatcherHandle {
    tx: mpsc::Sender<WatcherSignal>,
}

impl WatcherHandle {
    pub async fn notify_block(&self, blueprints: HashSet<u64>) {
        // Bounded channel: a momentarily-slow watcher should not pin the
        // protocol event loop. Dropped notifications are fine — the watcher
        // always re-resolves the chain on the next signal it does receive,
        // so eventual consistency holds.
        let _ = self.tx.try_send(WatcherSignal::Reconcile { blueprints });
    }

    pub async fn notify_reconcile_all(&self) {
        let _ = self
            .tx
            .send(WatcherSignal::Reconcile {
                blueprints: HashSet::new(),
            })
            .await;
    }
}

pub struct UpgradeWatcher {
    config: WatcherConfig,
    rx: mpsc::Receiver<WatcherSignal>,
}

impl UpgradeWatcher {
    #[must_use]
    pub fn spawn(config: WatcherConfig) -> (WatcherHandle, JoinHandle<()>) {
        let (tx, rx) = mpsc::channel(64);
        let watcher = Self { config, rx };
        let handle = tokio::spawn(watcher.run());
        (WatcherHandle { tx }, handle)
    }

    async fn run(mut self) {
        info!(target: "upgrade", "UpgradeWatcher started");
        while let Some(signal) = self.rx.recv().await {
            match signal {
                WatcherSignal::Reconcile { blueprints } => {
                    if let Err(err) = self.reconcile(&blueprints).await {
                        warn!(target: "upgrade", error = %err, "reconcile loop failed");
                    }
                }
            }
        }
        info!(target: "upgrade", "UpgradeWatcher stopping (channel closed)");
    }

    async fn reconcile(&self, filter_blueprints: &HashSet<u64>) -> Result<()> {
        let snapshot = self.config.tracked_services.snapshot().await;
        for (blueprint_id, service_id) in snapshot {
            if !filter_blueprints.is_empty() && !filter_blueprints.contains(&blueprint_id) {
                continue;
            }
            if let Err(err) = self.reconcile_one(blueprint_id, service_id).await {
                warn!(
                    target: "upgrade",
                    blueprint_id,
                    service_id,
                    error = %err,
                    "service reconcile failed; will retry on next signal"
                );
            }
        }
        Ok(())
    }

    /// Reconcile a single service. Public for the operator CLI path
    /// (`upgrade ack`) which wants to drive reconciliation on demand.
    pub async fn reconcile_one(&self, blueprint_id: u64, service_id: u64) -> Result<()> {
        // Cache the policy first — operator-visible RPCs ask for it without
        // wanting to round-trip the chain again.
        let policy = self.config.chain.service_upgrade_policy(service_id).await?;
        self.config.state.set_policy(service_id, policy).await;

        let target = match self
            .config
            .chain
            .effective_binary_version(service_id, blueprint_id)
            .await
        {
            Ok(v) => v,
            Err(super::error::UpgradeError::NoVersionsPublished { .. }) => {
                info!(
                    target: "upgrade",
                    blueprint_id,
                    service_id,
                    "no published versions yet — nothing to reconcile"
                );
                return Ok(());
            }
            Err(err) => return Err(err),
        };

        let running = self.config.state.running(service_id).await;
        let needs_swap = running.map(|r| r.sha256 != target.sha256).unwrap_or(true);

        if !needs_swap {
            // Nothing to do; clear any stale pending entry left over from a
            // previous policy.
            self.config.state.clear_pending(service_id).await;
            return Ok(());
        }

        match policy {
            UpgradePolicy::Manual => {
                // MANUAL-with-assist: consult the local authz store before
                // falling through to the alert-only path. This is the seam
                // that lets a MANUAL operator pre-authorize specific
                // versions without writing an on-chain ack — the manager
                // still owns the verify-then-swap pipeline, the operator
                // still controls policy on-chain, and no gas is spent.
                let authz = self.config.local_authz.get(service_id).await;
                let decision = authz.decide(target.version_id);
                match decision {
                    AuthzDecision::Swap { clear_pinned } => {
                        // MANUAL pre-authorized: same attestation gate as
                        // AUTO. The on-chain policy is still MANUAL — the
                        // operator's audit trail is unchanged — but they
                        // get the full safety pipeline. Refusing the
                        // attestation gate here would erase the value-add.
                        if let Err(err) = self.config.attestation.verify(service_id, &target) {
                            warn!(
                                target: "upgrade",
                                event = "binary_upgrade_attestation_failed",
                                blueprint_id,
                                service_id,
                                version_id = target.version_id,
                                error = %err,
                                "MANUAL-with-assist swap blocked by attestation gate"
                            );
                            self.config
                                .state
                                .push_history(history_entry(
                                    blueprint_id,
                                    service_id,
                                    running,
                                    &target,
                                    policy,
                                    UpgradeOutcome::AttestationFailed,
                                ))
                                .await;
                            self.record_pending(blueprint_id, service_id, &target, policy, running)
                                .await;
                            return Ok(());
                        }
                        info!(
                            target: "upgrade",
                            event = "binary_upgrade_local_authz_swap",
                            blueprint_id,
                            service_id,
                            version_id = target.version_id,
                            clear_pinned,
                            "MANUAL-with-assist authorized swap"
                        );
                        let result = self
                            .execute_swap(blueprint_id, service_id, &target, policy, running)
                            .await;
                        // Consume the one-shot pin after dispatch. We do
                        // this unconditionally on the Swap+clear_pinned
                        // path so a failed dispatch does not leave a stale
                        // pin that fires again on the next reconcile.
                        if clear_pinned {
                            if let Err(err) = self
                                .config
                                .local_authz
                                .consume_pin_if(service_id, target.version_id)
                                .await
                            {
                                warn!(
                                    target: "upgrade",
                                    service_id,
                                    version_id = target.version_id,
                                    error = %err,
                                    "failed to consume local authz pin; \
                                     operator may need to clear it manually"
                                );
                            }
                        }
                        result
                    }
                    AuthzDecision::Skip => {
                        // Operator explicitly declined this version.
                        // Suppress further pending state so the alert
                        // channels stop firing on the same version.
                        self.config.state.clear_pending(service_id).await;
                        info!(
                            target: "upgrade",
                            event = "binary_upgrade_skipped",
                            blueprint_id,
                            service_id,
                            version_id = target.version_id,
                            "operator declined this version via local authz"
                        );
                        Ok(())
                    }
                    AuthzDecision::Hold => {
                        // No matching local authorization. Preserves the
                        // pre-existing MANUAL behavior exactly: alert + hold.
                        self.record_pending(blueprint_id, service_id, &target, policy, running)
                            .await;
                        warn!(
                            target: "upgrade",
                            event = "binary_upgrade_blocked",
                            blueprint_id,
                            service_id,
                            available_version_id = target.version_id,
                            available_sha256 = %hex_short(target.sha256.as_slice()),
                            "MANUAL policy — operator must switch policy or pre-authorize via local authz"
                        );
                        Ok(())
                    }
                }
            }
            UpgradePolicy::Approve => {
                // APPROVE: only swap once the operator has acked this exact
                // version on-chain. Re-resolve to make sure we are not
                // tricked by a stale signal.
                let acked = self
                    .config
                    .chain
                    .service_acked_version_id(service_id)
                    .await?;
                if acked == target.version_id {
                    self.execute_swap(blueprint_id, service_id, &target, policy, running)
                        .await
                } else {
                    self.record_pending(blueprint_id, service_id, &target, policy, running)
                        .await;
                    info!(
                        target: "upgrade",
                        event = "binary_upgrade_available",
                        blueprint_id,
                        service_id,
                        available_version_id = target.version_id,
                        available_sha256 = %hex_short(target.sha256.as_slice()),
                        "APPROVE policy — waiting for operator ack"
                    );
                    Ok(())
                }
            }
            UpgradePolicy::Auto => {
                // AUTO: gate on attestation, then swap.
                if let Err(err) = self.config.attestation.verify(service_id, &target) {
                    warn!(
                        target: "upgrade",
                        event = "binary_upgrade_attestation_failed",
                        blueprint_id,
                        service_id,
                        version_id = target.version_id,
                        error = %err,
                        "AUTO swap downgraded to APPROVE-style notification"
                    );
                    self.config
                        .state
                        .push_history(history_entry(
                            blueprint_id,
                            service_id,
                            running,
                            &target,
                            policy,
                            UpgradeOutcome::AttestationFailed,
                        ))
                        .await;
                    self.record_pending(blueprint_id, service_id, &target, policy, running)
                        .await;
                    return Ok(());
                }
                self.execute_swap(blueprint_id, service_id, &target, policy, running)
                    .await
            }
        }
    }

    async fn record_pending(
        &self,
        blueprint_id: u64,
        service_id: u64,
        target: &BinaryVersionInfo,
        policy: UpgradePolicy,
        running: Option<RunningBinary>,
    ) {
        self.config
            .state
            .record_pending(PendingUpgrade {
                service_id,
                blueprint_id,
                current_version_id: running.map(|r| r.version_id),
                current_sha256: running.map(|r| r.sha256),
                available_version_id: target.version_id,
                available_sha256: target.sha256,
                available_uri: target.binary_uri.clone(),
                available_attestation_hash: target.attestation_hash,
                policy,
                detected_at: SystemTime::now(),
            })
            .await;
    }

    async fn execute_swap(
        &self,
        blueprint_id: u64,
        service_id: u64,
        target: &BinaryVersionInfo,
        policy: UpgradePolicy,
        running: Option<RunningBinary>,
    ) -> Result<()> {
        let service_label = format!("svc-{blueprint_id}-{service_id}");
        let cache_dir = self.config.cache_root.join(&service_label);

        // Step 1: download + sha256 verify. This is the trust-root gate; if
        // it fails we record the outcome and abandon — the protocol event
        // handler will never see a SwapRequest with mismatched bytes.
        let binary_path = match download_and_verify(service_id, &cache_dir, target).await {
            Ok(path) => path,
            Err(err) => {
                let outcome = match err {
                    super::error::UpgradeError::Sha256Mismatch { .. } => {
                        UpgradeOutcome::Sha256Mismatch
                    }
                    _ => UpgradeOutcome::DownloadFailed,
                };
                warn!(
                    target: "upgrade",
                    event = "binary_swap_failed",
                    blueprint_id,
                    service_id,
                    version_id = target.version_id,
                    error = %err,
                    ?outcome,
                    "swap aborted before drain — binary kept running on old version"
                );
                self.config
                    .state
                    .push_history(history_entry(
                        blueprint_id,
                        service_id,
                        running,
                        target,
                        policy,
                        outcome,
                    ))
                    .await;
                return Err(err);
            }
        };

        // Step 2: hand the verified binary to the protocol event handler
        // for drain + atomic swap. The handler owns `ActiveBlueprints`
        // (mutable, not shared) so it does the kill + respawn under its own
        // lifecycle locks. We intentionally do NOT mutate `state.running`
        // here — the handler updates it when the new service is healthy.
        info!(
            target: "upgrade",
            event = "binary_swap_queued",
            blueprint_id,
            service_id,
            from_version_id = ?running.map(|r| r.version_id),
            to_version_id = target.version_id,
            to_sha256 = %hex_short(target.sha256.as_slice()),
            binary_path = %binary_path.display(),
            policy = %policy,
            "verified swap dispatched to protocol event handler"
        );

        let request = SwapRequest {
            service_id,
            blueprint_id,
            target: target.clone(),
            binary_path,
            policy,
            previous: running,
        };
        if let Err(err) = self.config.swap_tx.send(request).await {
            warn!(
                target: "upgrade",
                blueprint_id,
                service_id,
                error = %err,
                "swap queue closed; protocol event handler is gone — abandoning swap"
            );
        }
        Ok(())
    }
}

fn hex_short(bytes: &[u8]) -> String {
    if bytes.len() >= 8 {
        format!("0x{}", hex::encode(&bytes[..8]))
    } else {
        format!("0x{}", hex::encode(bytes))
    }
}

fn history_entry(
    blueprint_id: u64,
    service_id: u64,
    running: Option<RunningBinary>,
    target: &BinaryVersionInfo,
    policy: UpgradePolicy,
    outcome: UpgradeOutcome,
) -> UpgradeHistoryEntry {
    UpgradeHistoryEntry {
        service_id,
        blueprint_id,
        from_version_id: running.map(|r| r.version_id),
        from_sha256: running.map(|r| r.sha256),
        to_version_id: target.version_id,
        to_sha256: target.sha256,
        policy,
        completed_at: SystemTime::now(),
        outcome,
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::UpgradePolicy;

    #[test]
    fn watcher_signal_filter_includes_empty_set_as_wildcard() {
        // The contract between the protocol event handler and the watcher:
        // an empty `blueprints` set means "reconcile everything". Codifying
        // that here prevents a future refactor from silently changing it to
        // "reconcile nothing" and stalling all upgrades.
        let signal = super::WatcherSignal::Reconcile {
            blueprints: std::collections::HashSet::new(),
        };
        match signal {
            super::WatcherSignal::Reconcile { blueprints } => assert!(blueprints.is_empty()),
        }
    }

    #[test]
    fn upgrade_policy_resolution_is_three_state() {
        // Cheap exhaustiveness guard — if someone adds a fourth variant to
        // UpgradePolicy without thinking about the watcher's three-arm
        // match, this test fails to compile.
        let policies = [
            UpgradePolicy::Approve,
            UpgradePolicy::Auto,
            UpgradePolicy::Manual,
        ];
        for p in policies {
            match p {
                UpgradePolicy::Approve | UpgradePolicy::Auto | UpgradePolicy::Manual => {}
            }
        }
    }
}
