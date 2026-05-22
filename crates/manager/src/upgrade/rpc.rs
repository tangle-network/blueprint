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
//! Errors carry a JSON body `{ "error": "<message>" }` and an HTTP status
//! drawn from the failure mode (4xx = caller misuse, 5xx = chain failure).

use super::chain::ChainView;
use super::state::UpgradeState;
use super::types::{PendingUpgrade, UpgradeHistoryEntry, UpgradePolicy};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct UpgradeApi {
    chain: Arc<ChainView>,
    state: UpgradeState,
}

impl UpgradeApi {
    #[must_use]
    pub fn new(chain: ChainView, state: UpgradeState) -> Self {
        Self {
            chain: Arc::new(chain),
            state,
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

    #[must_use]
    pub fn router(self) -> Router {
        Router::new()
            .route("/upgrades/pending", get(list_pending))
            .route("/upgrades/history", get(list_history))
            .route("/upgrades/policy/{service_id}", get(get_policy))
            .route("/upgrades/policy/{service_id}", post(set_policy))
            .route("/upgrades/{service_id}/ack", post(submit_ack))
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
