//! Local axum router exposing upgrade state to operator tooling.
//!
//! This is intentionally a small surface — the manager is not a full RPC node.
//! Mount it behind the auth proxy or on a localhost-only listener; tx-signing
//! endpoints submit on-chain calls using the manager's keystore key.
//!
//! Routes:
//!   `GET  /upgrades/pending`              list of pending upgrades
//!   `GET  /upgrades/history`              recent swap outcomes (bounded ring)
//!   `GET  /upgrades/policy/:service_id`   current policy + cached on-chain value
//!   `POST /upgrades/policy/:service_id`   set policy (signs on-chain tx)
//!   `POST /upgrades/:service_id/ack`      ack a specific version on-chain
//!
//! Local-authz (MANUAL-with-assist) routes — NO on-chain tx:
//!   `GET  /upgrades/{service_id}/available`     enumerate published versions
//!   `GET  /upgrades/{service_id}/authz`         show current local-authz state
//!   `POST /upgrades/{service_id}/pin`           one-shot pin to a version
//!   `POST /upgrades/{service_id}/whitelist`     replace whitelist (idempotent)
//!   `POST /upgrades/{service_id}/skip`          add a skip entry with a reason
//!
//! Errors carry a JSON body `{ "error": "<message>" }` and an HTTP status
//! drawn from the failure mode (4xx = caller misuse, 5xx = chain failure).

use super::chain::ChainView;
use super::local_authz::{AuthzView, LocalAuthzStore, RunningEntry, SkipEntry};
use super::state::UpgradeState;
use super::types::{PendingUpgrade, UpgradeHistoryEntry, UpgradePolicy};
use super::watcher::TrackedServices;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::sync::Arc;

#[derive(Clone)]
pub struct UpgradeApi {
    chain: Arc<ChainView>,
    state: UpgradeState,
    local_authz: LocalAuthzStore,
    tracked_services: TrackedServices,
}

impl UpgradeApi {
    #[must_use]
    pub fn new(
        chain: ChainView,
        state: UpgradeState,
        local_authz: LocalAuthzStore,
        tracked_services: TrackedServices,
    ) -> Self {
        Self {
            chain: Arc::new(chain),
            state,
            local_authz,
            tracked_services,
        }
    }

    /// Access the chain view for callers (e.g. the Tangle event handler)
    /// that need to read on-chain state without going through HTTP.
    #[must_use]
    pub fn chain(&self) -> &ChainView {
        &self.chain
    }

    /// Access the in-memory upgrade state.
    #[must_use]
    pub fn state(&self) -> &UpgradeState {
        &self.state
    }

    /// Access the local authz store for callers that drive it
    /// programmatically (tests, embedding scenarios).
    #[must_use]
    pub fn local_authz(&self) -> &LocalAuthzStore {
        &self.local_authz
    }

    #[must_use]
    pub fn router(self) -> Router {
        Router::new()
            .route("/upgrades/pending", get(list_pending))
            .route("/upgrades/history", get(list_history))
            .route("/upgrades/policy/{service_id}", get(get_policy))
            .route("/upgrades/policy/{service_id}", post(set_policy))
            .route("/upgrades/{service_id}/ack", post(submit_ack))
            .route("/upgrades/{service_id}/available", get(list_available))
            .route("/upgrades/{service_id}/authz", get(get_authz))
            .route("/upgrades/{service_id}/pin", post(set_pin))
            .route("/upgrades/{service_id}/whitelist", post(set_whitelist))
            .route("/upgrades/{service_id}/skip", post(add_skip))
            .with_state(self)
    }
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: String,
}

struct ApiError(StatusCode, String);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, Json(ErrorBody { error: self.1 })).into_response()
    }
}

#[derive(Debug, Serialize)]
struct PendingList {
    pending: Vec<PendingUpgrade>,
}

#[derive(Debug, Serialize)]
struct HistoryList {
    history: Vec<UpgradeHistoryEntry>,
}

#[derive(Debug, Serialize)]
struct PolicyView {
    service_id: u64,
    /// Cached value (last `getServiceUpgradePolicy` we observed).
    cached: Option<UpgradePolicy>,
    /// Freshly fetched value from the chain.
    onchain: UpgradePolicy,
    /// Operator's `getServiceAckedVersionId`.
    acked_version_id: u64,
}

#[derive(Debug, Deserialize)]
struct SetPolicyBody {
    policy: UpgradePolicy,
}

#[derive(Debug, Serialize)]
struct TxReceiptView {
    tx_hash: String,
}

#[derive(Debug, Deserialize)]
struct AckBody {
    version_id: u64,
}

async fn list_pending(State(api): State<UpgradeApi>) -> Result<Json<PendingList>, ApiError> {
    Ok(Json(PendingList {
        pending: api.state.list_pending().await,
    }))
}

async fn list_history(State(api): State<UpgradeApi>) -> Result<Json<HistoryList>, ApiError> {
    Ok(Json(HistoryList {
        history: api.state.history().await,
    }))
}

async fn get_policy(
    State(api): State<UpgradeApi>,
    Path(service_id): Path<u64>,
) -> Result<Json<PolicyView>, ApiError> {
    let cached = api.state.policy(service_id).await;
    let onchain = api
        .chain
        .service_upgrade_policy(service_id)
        .await
        .map_err(|e| ApiError(StatusCode::BAD_GATEWAY, e.to_string()))?;
    api.state.set_policy(service_id, onchain).await;
    let acked = api
        .chain
        .service_acked_version_id(service_id)
        .await
        .map_err(|e| ApiError(StatusCode::BAD_GATEWAY, e.to_string()))?;
    Ok(Json(PolicyView {
        service_id,
        cached,
        onchain,
        acked_version_id: acked,
    }))
}

async fn set_policy(
    State(api): State<UpgradeApi>,
    Path(service_id): Path<u64>,
    Json(body): Json<SetPolicyBody>,
) -> Result<Json<TxReceiptView>, ApiError> {
    let tx = api
        .chain
        .set_service_upgrade_policy(service_id, body.policy)
        .await
        .map_err(|e| ApiError(StatusCode::BAD_GATEWAY, e.to_string()))?;
    api.state.set_policy(service_id, body.policy).await;
    Ok(Json(TxReceiptView {
        tx_hash: format!("0x{}", hex::encode(tx.as_slice())),
    }))
}

async fn submit_ack(
    State(api): State<UpgradeApi>,
    Path(service_id): Path<u64>,
    Json(body): Json<AckBody>,
) -> Result<Json<TxReceiptView>, ApiError> {
    let tx = api
        .chain
        .ack_binary_version(service_id, body.version_id)
        .await
        .map_err(|e| ApiError(StatusCode::BAD_GATEWAY, e.to_string()))?;
    Ok(Json(TxReceiptView {
        tx_hash: format!("0x{}", hex::encode(tx.as_slice())),
    }))
}

// ──────────────────────────────────────────────────────────────────────
// Local-authz routes (MANUAL-with-assist) — NO on-chain tx.
// ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct AvailableEntry {
    version_id: u64,
    sha256: String,
    binary_uri: String,
    attestation_hash: String,
    published_at: u64,
    deprecated: bool,
    /// True iff this version's sha256 matches what the manager is currently
    /// running for the service. Useful for tooling to render an "active" badge.
    running: bool,
}

#[derive(Debug, Serialize)]
struct AvailableList {
    service_id: u64,
    blueprint_id: u64,
    active_version_id: u64,
    versions: Vec<AvailableEntry>,
}

async fn list_available(
    State(api): State<UpgradeApi>,
    Path(service_id): Path<u64>,
) -> Result<Json<AvailableList>, ApiError> {
    // Resolve the blueprint via the tracked-services map — that is the
    // single source of truth for "which blueprint is this service running"
    // inside this manager process. We deliberately do not accept
    // blueprint_id as a query param: the operator's tool should never be
    // able to ask about a foreign blueprint.
    let running = api.state.running(service_id).await;
    let blueprint_id = resolve_blueprint_id(&api.tracked_services, service_id).await?;
    let count = api
        .chain
        .binary_version_count(blueprint_id)
        .await
        .map_err(|e| ApiError(StatusCode::BAD_GATEWAY, e.to_string()))?;
    let active_version_id = api
        .chain
        .active_binary_version_id(blueprint_id)
        .await
        .map_err(|e| ApiError(StatusCode::BAD_GATEWAY, e.to_string()))?;

    let mut versions = Vec::with_capacity(count as usize);
    for i in 0..count {
        let info = api
            .chain
            .get_binary_version(blueprint_id, i)
            .await
            .map_err(|e| ApiError(StatusCode::BAD_GATEWAY, e.to_string()))?;
        let is_running = running.map(|r| r.sha256 == info.sha256).unwrap_or(false);
        versions.push(AvailableEntry {
            version_id: info.version_id,
            sha256: format!("0x{}", hex::encode(info.sha256.as_slice())),
            binary_uri: info.binary_uri,
            attestation_hash: format!("0x{}", hex::encode(info.attestation_hash.as_slice())),
            published_at: info.published_at,
            deprecated: info.deprecated,
            running: is_running,
        });
    }
    Ok(Json(AvailableList {
        service_id,
        blueprint_id,
        active_version_id,
        versions,
    }))
}

async fn get_authz(
    State(api): State<UpgradeApi>,
    Path(service_id): Path<u64>,
) -> Result<Json<AuthzView>, ApiError> {
    let onchain = api
        .chain
        .service_upgrade_policy(service_id)
        .await
        .map_err(|e| ApiError(StatusCode::BAD_GATEWAY, e.to_string()))?;
    let authz = api.local_authz.get(service_id).await;
    let running = api.state.running(service_id).await.map(|r| RunningEntry {
        version_id: r.version_id,
        sha256: r.sha256,
    });
    Ok(Json(AuthzView {
        service_id,
        policy_onchain: onchain,
        whitelisted: authz.whitelisted.into_iter().collect(),
        pinned: authz.pinned,
        skipped: authz
            .skipped
            .into_iter()
            .map(|(version_id, reason)| SkipEntry { version_id, reason })
            .collect(),
        running,
    }))
}

#[derive(Debug, Deserialize)]
struct PinBody {
    version_id: u64,
    #[serde(default)]
    dry_run: bool,
}

#[derive(Debug, Serialize)]
struct PinResult {
    ok: bool,
    /// The decision the watcher will take when it next reconciles this
    /// service. `pinned_swap` means it will run the verify-then-swap path;
    /// `already_running` means the requested version is the one currently
    /// served (no-op); `not_published` means the version was not found on
    /// chain (the pin is still stored, but flagged for the caller).
    status: String,
    pinned: Option<u64>,
}

async fn set_pin(
    State(api): State<UpgradeApi>,
    Path(service_id): Path<u64>,
    Json(body): Json<PinBody>,
) -> Result<Json<PinResult>, ApiError> {
    // Resolve so we can give the caller a useful status. The pin itself is
    // safe to store even if the version doesn't exist yet (the watcher will
    // simply hold), but failing-fast is friendlier.
    let blueprint_id = resolve_blueprint_id(&api.tracked_services, service_id).await?;
    let count = api
        .chain
        .binary_version_count(blueprint_id)
        .await
        .map_err(|e| ApiError(StatusCode::BAD_GATEWAY, e.to_string()))?;
    let status = if body.version_id >= count {
        "not_published"
    } else {
        let running = api.state.running(service_id).await;
        let target = api
            .chain
            .get_binary_version(blueprint_id, body.version_id)
            .await
            .map_err(|e| ApiError(StatusCode::BAD_GATEWAY, e.to_string()))?;
        if running.map(|r| r.sha256 == target.sha256).unwrap_or(false) {
            "already_running"
        } else {
            "pinned_swap"
        }
    };
    if body.dry_run {
        return Ok(Json(PinResult {
            ok: true,
            status: format!("dry_run:{status}"),
            pinned: api.local_authz.get(service_id).await.pinned,
        }));
    }
    api.local_authz
        .set_pin(service_id, Some(body.version_id))
        .await
        .map_err(|e| ApiError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(PinResult {
        ok: true,
        status: status.into(),
        pinned: Some(body.version_id),
    }))
}

#[derive(Debug, Deserialize)]
struct WhitelistBody {
    /// New whitelist contents. Replaces any previous value; pass `[]` to clear.
    versions: Vec<u64>,
}

#[derive(Debug, Serialize)]
struct WhitelistResult {
    whitelisted: Vec<u64>,
}

async fn set_whitelist(
    State(api): State<UpgradeApi>,
    Path(service_id): Path<u64>,
    Json(body): Json<WhitelistBody>,
) -> Result<Json<WhitelistResult>, ApiError> {
    let set: BTreeSet<u64> = body.versions.into_iter().collect();
    let authz = api
        .local_authz
        .set_whitelist(service_id, set)
        .await
        .map_err(|e| ApiError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(WhitelistResult {
        whitelisted: authz.whitelisted.into_iter().collect(),
    }))
}

#[derive(Debug, Deserialize)]
struct SkipBody {
    version_id: u64,
    reason: String,
}

#[derive(Debug, Serialize)]
struct SkipResult {
    skipped: Vec<SkipEntry>,
}

async fn add_skip(
    State(api): State<UpgradeApi>,
    Path(service_id): Path<u64>,
    Json(body): Json<SkipBody>,
) -> Result<Json<SkipResult>, ApiError> {
    if body.reason.trim().is_empty() {
        return Err(ApiError(
            StatusCode::BAD_REQUEST,
            "reason must be non-empty (this lands in the operator's audit log)".into(),
        ));
    }
    let authz = api
        .local_authz
        .add_skip(service_id, body.version_id, body.reason)
        .await
        .map_err(|e| ApiError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(SkipResult {
        skipped: authz
            .skipped
            .into_iter()
            .map(|(version_id, reason)| SkipEntry { version_id, reason })
            .collect(),
    }))
}

/// Resolve `service_id -> blueprint_id` via the tracked-services map. Returns
/// a 404-equivalent ApiError when the manager isn't serving the service —
/// callers should never be able to drive local authz for foreign services.
async fn resolve_blueprint_id(tracked: &TrackedServices, service_id: u64) -> Result<u64, ApiError> {
    for (bp, sid) in tracked.snapshot().await {
        if sid == service_id {
            return Ok(bp);
        }
    }
    Err(ApiError(
        StatusCode::NOT_FOUND,
        format!("service {service_id} is not tracked by this manager"),
    ))
}
