//! Blueprint binary version upgrade orchestration.
//!
//! Pieces:
//!
//! - [`chain`]: read/write helpers against the on-chain
//!   `BlueprintsBinaryVersions` surface (currently behind a local ABI stub
//!   pending tnt-core-bindings v0.18).
//! - [`watcher`]: background task that reconciles every tracked service
//!   against `effectiveBinaryVersion(serviceId)` whenever the chain produces
//!   a relevant signal.
//! - [`swap`]: download + sha256 verify + drain + atomic swap pipeline.
//! - [`attestation`]: gate for non-zero `attestationHash` rows; production
//!   verification lands separately in cargo-tangle.
//! - [`rpc`]: small axum router for operator tooling.
//! - [`state`]: in-memory state shared across all of the above.
//!
//! Operator-facing CLI commands (`ack`, `set-policy`, etc.) live in
//! `cargo-tangle` (this repo's `cli/` crate) — that path already owns
//! keystore loading and signing UX.
//!
//! ## Safety contract (do not weaken)
//!
//! 1. Every binary the manager runs MUST have been gated by
//!    `verify_binary_digest` against `effectiveBinaryVersion(...).sha256Hash`.
//!    The same helper handles the initial-fetch path; never fork.
//! 2. The drain-then-swap sequence in `watcher::execute_swap` is non-negotiable.
//!    Killing a service mid-job loses operator slot work.
//! 3. AUTO swap requires sha256 match AND attestation verify. Failure on the
//!    attestation step downgrades to an APPROVE-style pending notification
//!    rather than swapping silently.
//! 4. The default policy is `APPROVE`. A new operator joining a service does
//!    NOT auto-update.
//! 5. `serviceAckedVersionId` is a single slot any operator can write —
//!    after observing `OperatorBinaryAcked` the watcher re-resolves via
//!    `effectiveBinaryVersion` and never assumes the ack reflects this
//!    manager's intent.

pub mod abi;
pub mod attestation;
pub mod chain;
pub mod error;
pub mod local_authz;
pub mod rpc;
pub mod state;
pub mod swap;
pub mod types;
pub mod watcher;

pub use attestation::AttestationVerifier;
pub use chain::ChainView;
pub use error::{Result, UpgradeError};
pub use local_authz::{
    AuthzDecision, AuthzView, LocalAuthz, LocalAuthzError, LocalAuthzStore, RunningEntry, SkipEntry,
};
pub use rpc::UpgradeApi;
pub use state::{RunningBinary, UpgradeState};
pub use types::{
    BinaryVersionInfo, PendingUpgrade, UpgradeHistoryEntry, UpgradeOutcome, UpgradePolicy,
};
pub use watcher::{
    SwapRequest, TrackedServices, UpgradeWatcher, WatcherConfig, WatcherHandle, WatcherSignal,
};

/// Bundle of upgrade-flow handles consumed by the Tangle event handler.
///
/// Built once at manager start (from `executor::run_blueprint_manager_with_keystore`)
/// and threaded through `ProtocolManager` so the handler can:
///
/// 1. Subscribe to swap requests produced by the watcher
///    (`Receiver<SwapRequest>`).
/// 2. Update `state` and `tracked_services` as services spin up / down.
/// 3. Notify the watcher when a relevant on-chain event lands
///    (`WatcherHandle`).
/// 4. Surface the API to the local RPC (`UpgradeApi`).
pub struct UpgradePipeline {
    pub watcher: WatcherHandle,
    pub state: UpgradeState,
    pub tracked_services: TrackedServices,
    pub swap_rx: tokio::sync::mpsc::Receiver<SwapRequest>,
    pub api: UpgradeApi,
    pub local_authz: LocalAuthzStore,
}

impl std::fmt::Debug for UpgradePipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpgradePipeline").finish_non_exhaustive()
    }
}

/// Inputs needed to assemble an [`UpgradePipeline`].
pub struct UpgradePipelineBuilder {
    pub chain: ChainView,
    pub cache_root: std::path::PathBuf,
    pub attestation: AttestationVerifier,
    /// Optional persistence root for `LocalAuthzStore`. When set, MANUAL
    /// services may pre-authorize swaps locally; when `None` the watcher
    /// behaves exactly as before (alert + hold).
    pub local_authz_root: Option<std::path::PathBuf>,
}

impl UpgradePipelineBuilder {
    /// Spin up the upgrade watcher and return the operator-side pipeline +
    /// the join handle for the watcher background task.
    ///
    /// Caller is responsible for ensuring the join handle's lifetime is
    /// bound to the manager so a panic in the watcher surfaces in shutdown.
    pub fn spawn(self) -> (UpgradePipeline, tokio::task::JoinHandle<()>) {
        let state = UpgradeState::new();
        let tracked_services = TrackedServices::new();
        let (swap_tx, swap_rx) = tokio::sync::mpsc::channel::<SwapRequest>(32);
        let local_authz = match self.local_authz_root {
            Some(root) => match LocalAuthzStore::new_persisted(root.clone()) {
                Ok(store) => store,
                Err(err) => {
                    blueprint_core::warn!(
                        target: "upgrade",
                        root = %root.display(),
                        error = %err,
                        "failed to initialize persisted local authz store; falling back to in-memory"
                    );
                    LocalAuthzStore::new_in_memory()
                }
            },
            None => LocalAuthzStore::new_in_memory(),
        };
        let api = UpgradeApi::new(
            self.chain.clone(),
            state.clone(),
            local_authz.clone(),
            tracked_services.clone(),
        );

        let config = WatcherConfig {
            chain: self.chain,
            state: state.clone(),
            attestation: self.attestation,
            cache_root: self.cache_root,
            tracked_services: tracked_services.clone(),
            local_authz: local_authz.clone(),
            swap_tx,
        };
        let (watcher, join) = UpgradeWatcher::spawn(config);

        (
            UpgradePipeline {
                watcher,
                state,
                tracked_services,
                swap_rx,
                api,
                local_authz,
            },
            join,
        )
    }
}

/// Selectors of the events the watcher should react to. The protocol event
/// handler matches log topic 0 against these and, on a hit, forwards a
/// reconcile signal to the watcher.
pub mod event_selectors {
    use alloy_sol_types::SolEvent;

    use super::abi::IBlueprintBinaryVersions;

    pub fn binary_version_published() -> alloy_primitives::B256 {
        IBlueprintBinaryVersions::BinaryVersionPublished::SIGNATURE_HASH
    }

    pub fn binary_active_version_changed() -> alloy_primitives::B256 {
        IBlueprintBinaryVersions::BinaryActiveVersionChanged::SIGNATURE_HASH
    }

    pub fn binary_version_deprecated() -> alloy_primitives::B256 {
        IBlueprintBinaryVersions::BinaryVersionDeprecated::SIGNATURE_HASH
    }

    pub fn operator_binary_acked() -> alloy_primitives::B256 {
        IBlueprintBinaryVersions::OperatorBinaryAcked::SIGNATURE_HASH
    }

    pub fn service_upgrade_policy_set() -> alloy_primitives::B256 {
        IBlueprintBinaryVersions::ServiceUpgradePolicySet::SIGNATURE_HASH
    }

    /// True iff the given topic0 matches any of the binary-versions events.
    pub fn matches_any(topic0: &alloy_primitives::B256) -> bool {
        [
            binary_version_published(),
            binary_active_version_changed(),
            binary_version_deprecated(),
            operator_binary_acked(),
            service_upgrade_policy_set(),
        ]
        .iter()
        .any(|sig| sig == topic0)
    }
}
