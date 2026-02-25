//! The x402 payment gateway server.
//!
//! Runs an axum HTTP server as a [`BackgroundService`] within the Blueprint runner.
//! Each registered job gets an endpoint protected by the x402 middleware from
//! [`x402_axum`]. When a client pays, the payment is verified via the configured
//! facilitator, and a [`JobCall`] is injected into the runner's producer stream.

use crate::config::{JobPolicyConfig, X402CallerAuthMode, X402Config, X402InvocationMode};
use crate::error::X402Error;
use crate::producer::VerifiedPayment;
use crate::quote_registry::QuoteRegistry;
use crate::settlement::SettlementOption;

use alloy_primitives::{Address, Signature, U256, hex};
use alloy_provider::ProviderBuilder;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use blueprint_runner::BackgroundService;
use blueprint_runner::error::RunnerError;
use bytes::Bytes;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tnt_core_bindings::bindings::r#i_tangle::ITangle;
use tokio::sync::{Mutex, mpsc, oneshot};
use url::Url;
use x402_axum::X402Middleware;
use x402_types::proto::v1::SettleResponse;
use x402_types::util::Base64Bytes;

const HEADER_CALLER: &str = "X-TANGLE-CALLER";
const HEADER_CALLER_SIG: &str = "X-TANGLE-CALLER-SIG";
const HEADER_CALLER_NONCE: &str = "X-TANGLE-CALLER-NONCE";
const HEADER_CALLER_EXPIRY: &str = "X-TANGLE-CALLER-EXPIRY";
const HEADER_SETTLEMENT: &str = "X-Payment-Response";
const HEADER_PAYMENT_V1: &str = "X-PAYMENT";
const HEADER_PAYMENT_V2: &str = "Payment-Signature";

/// Shared state for the axum handlers.
#[derive(Clone)]
struct GatewayState {
    config: Arc<X402Config>,
    /// Per-job prices in wei: (service_id, job_index) → U256
    job_pricing: Arc<HashMap<(u64, u32), U256>>,
    /// Per-job x402 invocation policies.
    job_policies: Arc<HashMap<(u64, u32), JobPolicyConfig>>,
    /// Quote tracking
    quote_registry: QuoteRegistry,
    /// Channel to send verified payments to the runner's producer
    payment_tx: mpsc::UnboundedSender<VerifiedPayment>,
    /// Monotonic call ID counter
    call_id_counter: Arc<AtomicU64>,
    /// Replay protection for delegated caller assertions.
    replay_guard: Arc<DelegatedReplayGuard>,
    /// Request counters for lightweight observability.
    counters: Arc<GatewayCounters>,
}

#[derive(Default)]
struct GatewayCounters {
    accepted: AtomicU64,
    policy_denied: AtomicU64,
    policy_error: AtomicU64,
    replay_denied: AtomicU64,
    enqueue_failed: AtomicU64,
    job_not_found: AtomicU64,
    quote_conflict: AtomicU64,
    auth_dry_run_allowed: AtomicU64,
    auth_dry_run_denied: AtomicU64,
    auth_dry_run_error: AtomicU64,
}

impl GatewayCounters {
    fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({
            "accepted": self.accepted.load(Ordering::Relaxed),
            "policy_denied": self.policy_denied.load(Ordering::Relaxed),
            "policy_error": self.policy_error.load(Ordering::Relaxed),
            "replay_denied": self.replay_denied.load(Ordering::Relaxed),
            "enqueue_failed": self.enqueue_failed.load(Ordering::Relaxed),
            "job_not_found": self.job_not_found.load(Ordering::Relaxed),
            "quote_conflict": self.quote_conflict.load(Ordering::Relaxed),
            "auth_dry_run_allowed": self.auth_dry_run_allowed.load(Ordering::Relaxed),
            "auth_dry_run_denied": self.auth_dry_run_denied.load(Ordering::Relaxed),
            "auth_dry_run_error": self.auth_dry_run_error.load(Ordering::Relaxed),
        })
    }
}

#[derive(Default)]
struct DelegatedReplayGuard {
    // key: "{caller}:{service_id}:{job_index}:{nonce}" => expiry_unix_secs
    seen_nonces: Mutex<HashMap<String, u64>>,
}

impl DelegatedReplayGuard {
    async fn reserve(
        &self,
        caller: Address,
        service_id: u64,
        job_index: u32,
        nonce: &str,
        expiry: u64,
    ) -> Result<(), PolicyRejection> {
        let now = current_unix_timestamp_secs().map_err(|e| {
            PolicyRejection::service_unavailable("clock_error", format!("clock error: {e}"))
        })?;

        let mut guard = self.seen_nonces.lock().await;
        guard.retain(|_, stored_expiry| *stored_expiry > now);

        let key = format!("{caller:#x}:{service_id}:{job_index}:{nonce}");
        if guard.contains_key(&key) {
            return Err(PolicyRejection::conflict(
                "signature_replay",
                "delegated signature nonce already used for this job scope",
            ));
        }

        guard.insert(key, expiry);
        Ok(())
    }
}

#[derive(Debug)]
struct PolicyRejection {
    status: StatusCode,
    code: &'static str,
    detail: String,
}

impl PolicyRejection {
    fn denied(code: &'static str, detail: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code,
            detail: detail.into(),
        }
    }

    fn bad_request(code: &'static str, detail: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code,
            detail: detail.into(),
        }
    }

    fn service_unavailable(code: &'static str, detail: impl Into<String>) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code,
            detail: detail.into(),
        }
    }

    fn conflict(code: &'static str, detail: impl Into<String>) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            code,
            detail: detail.into(),
        }
    }

    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            Json(serde_json::json!({
                "error": self.detail,
                "code": self.code,
            })),
        )
            .into_response()
    }
}

#[derive(Debug, Clone)]
struct SettlementDetails {
    payer: Option<Address>,
    network: Option<String>,
}

#[derive(Debug, Clone)]
struct PaymentAttribution {
    network: Option<String>,
    token: Option<String>,
    settled_payer: Option<Address>,
}

/// The x402 payment gateway.
///
/// Implements [`BackgroundService`] so it can be plugged directly into
/// [`BlueprintRunner::builder`](blueprint_runner::BlueprintRunner).
///
/// # Usage
///
/// ```rust,ignore
/// let (gateway, producer) = X402Gateway::new(config, job_pricing)?;
///
/// BlueprintRunner::builder((), env)
///     .router(router)
///     .producer(producer)
///     .background_service(gateway)
///     .run()
///     .await?;
/// ```
pub struct X402Gateway {
    config: Arc<X402Config>,
    job_pricing: Arc<HashMap<(u64, u32), U256>>,
    job_policies: Arc<HashMap<(u64, u32), JobPolicyConfig>>,
    payment_tx: mpsc::UnboundedSender<VerifiedPayment>,
    quote_registry: QuoteRegistry,
    replay_guard: Arc<DelegatedReplayGuard>,
    counters: Arc<GatewayCounters>,
}

impl X402Gateway {
    /// Create a new gateway and its paired [`X402Producer`](crate::X402Producer).
    ///
    /// `job_pricing` maps `(service_id, job_index)` to the price in wei.
    /// This is the same `JobPricingConfig` used by the pricing engine.
    pub fn new(
        config: X402Config,
        job_pricing: HashMap<(u64, u32), U256>,
    ) -> Result<(Self, crate::X402Producer), X402Error> {
        config.validate()?;

        if config.accepted_tokens.is_empty() {
            return Err(X402Error::Config(
                "at least one accepted_token must be configured".into(),
            ));
        }

        validate_price_tag_amount_bounds(&config, &job_pricing)?;

        let job_policies = config
            .job_policies
            .iter()
            .cloned()
            .map(|policy| ((policy.service_id, policy.job_index), policy))
            .collect();

        let (producer, payment_tx) = crate::X402Producer::channel();
        let quote_registry = QuoteRegistry::new(Duration::from_secs(config.quote_ttl_secs));

        let gateway = Self {
            config: Arc::new(config),
            job_pricing: Arc::new(job_pricing),
            job_policies: Arc::new(job_policies),
            payment_tx,
            quote_registry,
            replay_guard: Arc::new(DelegatedReplayGuard::default()),
            counters: Arc::new(GatewayCounters::default()),
        };

        Ok((gateway, producer))
    }

    /// Compute settlement options for a given job, converting the wei price
    /// to each accepted token denomination.
    pub fn settlement_options(
        config: &X402Config,
        service_id: u64,
        job_index: u32,
        price_wei: &U256,
    ) -> Result<Vec<SettlementOption>, X402Error> {
        let base_url = format!(
            "http://{}/x402/jobs/{}/{}",
            config.bind_address, service_id, job_index
        );

        config
            .accepted_tokens
            .iter()
            .map(|token| {
                let amount = token.convert_wei_to_amount(price_wei)?;
                Ok(SettlementOption {
                    network: token.network.clone(),
                    asset: token.asset.clone(),
                    symbol: token.symbol.clone(),
                    amount,
                    pay_to: token.pay_to.clone(),
                    scheme: "exact".into(),
                    x402_endpoint: base_url.clone(),
                })
            })
            .collect()
    }

    /// Build the axum router with x402-protected job endpoints.
    fn build_router(&self) -> Router {
        let state = GatewayState {
            config: self.config.clone(),
            job_pricing: self.job_pricing.clone(),
            job_policies: self.job_policies.clone(),
            quote_registry: self.quote_registry.clone(),
            payment_tx: self.payment_tx.clone(),
            call_id_counter: Arc::new(AtomicU64::new(1)),
            replay_guard: self.replay_guard.clone(),
            counters: self.counters.clone(),
        };

        // Base job execution route handler, protected by the x402 middleware.
        // The middleware automatically:
        //   1. Returns 402 with payment requirements when no payment header is present
        //   2. Verifies payment via the facilitator when a payment header is found
        //   3. Settles payment before passing the request to our handler
        let x402 =
            X402Middleware::new(self.config.facilitator_url.as_str()).settle_before_execution();

        let config = self.config.clone();
        let job_pricing = self.job_pricing.clone();

        let layer = x402.with_dynamic_price(
            move |_headers: &http::header::HeaderMap,
                  uri: &http::Uri,
                  _base_url: Option<&url::Url>| {
                let config = config.clone();
                let job_pricing = job_pricing.clone();
                let uri = uri.clone();
                async move { build_evm_price_tags(&config, &job_pricing, &uri) }
            },
        );

        let job_route = post(handle_job_request).layer(layer);

        Router::new()
            .route("/x402/jobs/{service_id}/{job_index}", job_route)
            // Health/discovery endpoints are unprotected
            .route("/x402/health", axum::routing::get(health_check))
            .route("/x402/stats", axum::routing::get(get_stats))
            .route(
                "/x402/jobs/{service_id}/{job_index}/auth-dry-run",
                post(post_auth_dry_run),
            )
            .route(
                "/x402/jobs/{service_id}/{job_index}/price",
                axum::routing::get(get_job_price),
            )
            .with_state(state)
    }
}

impl BackgroundService for X402Gateway {
    async fn start(&self) -> Result<oneshot::Receiver<Result<(), RunnerError>>, RunnerError> {
        let (tx, rx) = oneshot::channel();
        let router = self.build_router();
        let addr = self.config.bind_address;
        let registry = self.quote_registry.clone();

        tokio::spawn(async move {
            tracing::info!(%addr, "x402 payment gateway starting");

            // Spawn a background GC task for expired quotes
            let gc_registry = registry.clone();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(30));
                loop {
                    interval.tick().await;
                    gc_registry.gc();
                }
            });

            let listener = match tokio::net::TcpListener::bind(addr).await {
                Ok(l) => l,
                Err(e) => {
                    let _ = tx.send(Err(RunnerError::Other(Box::new(e))));
                    return;
                }
            };

            tracing::info!(%addr, "x402 payment gateway listening");

            if let Err(e) = axum::serve(listener, router).await {
                tracing::error!(error = %e, "x402 gateway server error");
                let _ = tx.send(Err(RunnerError::Other(Box::new(X402Error::Server(
                    e.to_string(),
                )))));
            }
        });

        Ok(rx)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Axum Handlers
// ═══════════════════════════════════════════════════════════════════════════

/// Health check endpoint.
async fn health_check() -> &'static str {
    "ok"
}

/// Lightweight stats endpoint for operator diagnostics.
async fn get_stats(State(state): State<GatewayState>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "ok",
            "counters": state.counters.snapshot(),
        })),
    )
}

/// Get settlement options for a job (discovery endpoint).
///
/// Returns the available payment methods and amounts for a given job,
/// without requiring payment. Clients use this to know what to pay before
/// sending the actual x402 request.
async fn get_job_price(
    State(state): State<GatewayState>,
    Path((service_id, job_index)): Path<(u64, u32)>,
) -> impl IntoResponse {
    let key = (service_id, job_index);
    let price_wei = match state.job_pricing.get(&key) {
        Some(p) => p,
        None => {
            state.counters.job_not_found.fetch_add(1, Ordering::Relaxed);
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "job not found",
                    "service_id": service_id,
                    "job_index": job_index,
                })),
            )
                .into_response();
        }
    };
    let policy = resolve_job_policy(&state, service_id, job_index);

    if policy.invocation_mode == X402InvocationMode::Disabled {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "error": "job is not enabled for x402",
                "code": "x402_disabled",
                "service_id": service_id,
                "job_index": job_index,
            })),
        )
            .into_response();
    }

    match X402Gateway::settlement_options(&state.config, service_id, job_index, price_wei) {
        Ok(options) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "service_id": service_id,
                "job_index": job_index,
                "price_wei": price_wei.to_string(),
                "settlement_options": options,
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// Dry-run policy evaluation endpoint.
///
/// Runs the same restricted auth + `eth_call` permission check logic as paid
/// invocation, but does not enqueue a job call.
async fn post_auth_dry_run(
    State(state): State<GatewayState>,
    Path((service_id, job_index)): Path<(u64, u32)>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let key = (service_id, job_index);
    if !state.job_pricing.contains_key(&key) {
        state.counters.job_not_found.fetch_add(1, Ordering::Relaxed);
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "allowed": false,
                "error": "job not found",
                "service_id": service_id,
                "job_index": job_index,
            })),
        )
            .into_response();
    }

    let policy = resolve_job_policy(&state, service_id, job_index);
    match policy.invocation_mode {
        X402InvocationMode::Disabled => {
            state
                .counters
                .auth_dry_run_denied
                .fetch_add(1, Ordering::Relaxed);
            (
                StatusCode::FORBIDDEN,
                Json(serde_json::json!({
                    "allowed": false,
                    "code": "x402_disabled",
                    "error": "job is not enabled for x402 invocation",
                    "service_id": service_id,
                    "job_index": job_index,
                })),
            )
                .into_response()
        }
        X402InvocationMode::PublicPaid => {
            state
                .counters
                .auth_dry_run_allowed
                .fetch_add(1, Ordering::Relaxed);
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "allowed": true,
                    "mode": "public_paid",
                    "service_id": service_id,
                    "job_index": job_index,
                })),
            )
                .into_response()
        }
        X402InvocationMode::RestrictedPaid => {
            match authorize_restricted_job(
                &state, &policy, service_id, job_index, &body, &headers, false,
            )
            .await
            {
                Ok(caller) => {
                    state
                        .counters
                        .auth_dry_run_allowed
                        .fetch_add(1, Ordering::Relaxed);
                    (
                        StatusCode::OK,
                        Json(serde_json::json!({
                            "allowed": true,
                            "mode": "restricted_paid",
                            "caller": format!("{caller:#x}"),
                            "service_id": service_id,
                            "job_index": job_index,
                        })),
                    )
                        .into_response()
                }
                Err(rejection) => {
                    if rejection.status == StatusCode::FORBIDDEN
                        || rejection.status == StatusCode::CONFLICT
                    {
                        state
                            .counters
                            .auth_dry_run_denied
                            .fetch_add(1, Ordering::Relaxed);
                    } else {
                        state
                            .counters
                            .auth_dry_run_error
                            .fetch_add(1, Ordering::Relaxed);
                    }
                    rejection.into_response()
                }
            }
        }
    }
}

/// Handle a paid job request.
///
/// Called after the x402 middleware has verified and settled payment.
/// The operator has already been paid on-chain at this point.
async fn handle_job_request(
    State(state): State<GatewayState>,
    Path((service_id, job_index)): Path<(u64, u32)>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let key = (service_id, job_index);

    // Verify the job exists in our pricing config
    let price_wei = match state.job_pricing.get(&key) {
        Some(p) => p,
        None => {
            state.counters.job_not_found.fetch_add(1, Ordering::Relaxed);
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "job not found",
                    "service_id": service_id,
                    "job_index": job_index,
                })),
            )
                .into_response();
        }
    };

    let policy = resolve_job_policy(&state, service_id, job_index);
    let caller = match policy.invocation_mode {
        X402InvocationMode::Disabled => {
            state.counters.policy_denied.fetch_add(1, Ordering::Relaxed);
            tracing::warn!(
                service_id,
                job_index,
                reason = "x402_disabled",
                "x402 policy denied"
            );
            return PolicyRejection::denied(
                "x402_disabled",
                "job is not enabled for x402 invocation",
            )
            .into_response();
        }
        X402InvocationMode::PublicPaid => None,
        X402InvocationMode::RestrictedPaid => {
            match authorize_restricted_job(
                &state, &policy, service_id, job_index, &body, &headers, true,
            )
            .await
            {
                Ok(caller) => Some(caller),
                Err(rejection) => {
                    if rejection.status == StatusCode::FORBIDDEN
                        || rejection.status == StatusCode::CONFLICT
                    {
                        state.counters.policy_denied.fetch_add(1, Ordering::Relaxed);
                        if rejection.code == "signature_replay" {
                            state.counters.replay_denied.fetch_add(1, Ordering::Relaxed);
                        }
                    } else {
                        state.counters.policy_error.fetch_add(1, Ordering::Relaxed);
                    }
                    tracing::warn!(
                        service_id,
                        job_index,
                        status = %rejection.status,
                        code = rejection.code,
                        reason = "policy_rejected",
                        "x402 restricted policy failed"
                    );
                    return rejection.into_response();
                }
            }
        }
    };

    // Register a dynamic quote for tracking
    let quote_digest = state
        .quote_registry
        .insert_dynamic(service_id, job_index, *price_wei);

    // Consume the quote (marks it as paid)
    if state.quote_registry.consume(&quote_digest).is_none() {
        state
            .counters
            .quote_conflict
            .fetch_add(1, Ordering::Relaxed);
        return (
            StatusCode::CONFLICT,
            Json(serde_json::json!({ "error": "quote already consumed or expired" })),
        )
            .into_response();
    }

    let call_id = state.call_id_counter.fetch_add(1, Ordering::Relaxed);

    let attribution = extract_payment_attribution(&headers, &state.config);
    let (payment_network, payment_token) = resolved_payment_metadata(&state.config, &attribution);

    let payment = VerifiedPayment {
        service_id,
        job_index,
        job_args: body,
        quote_digest,
        payment_network,
        payment_token,
        call_id,
        caller,
    };

    // Send to the runner's producer stream
    if state.payment_tx.send(payment).is_err() {
        state
            .counters
            .enqueue_failed
            .fetch_add(1, Ordering::Relaxed);
        tracing::error!(
            service_id,
            job_index,
            reason = "enqueue_failed",
            "x402 enqueue failed"
        );
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "error": "service shutting down" })),
        )
            .into_response();
    }

    state.counters.accepted.fetch_add(1, Ordering::Relaxed);

    let digest_hex = hex::encode(quote_digest);

    (
        StatusCode::ACCEPTED,
        Json(serde_json::json!({
            "status": "accepted",
            "receipt": digest_hex,
            "service_id": service_id,
            "job_index": job_index,
            "call_id": call_id,
        })),
    )
        .into_response()
}

// ═══════════════════════════════════════════════════════════════════════════
// Policy + Attribution Helpers
// ═══════════════════════════════════════════════════════════════════════════

fn resolve_job_policy(state: &GatewayState, service_id: u64, job_index: u32) -> JobPolicyConfig {
    state
        .job_policies
        .get(&(service_id, job_index))
        .cloned()
        .unwrap_or(JobPolicyConfig {
            service_id,
            job_index,
            invocation_mode: state.config.default_invocation_mode,
            auth_mode: X402CallerAuthMode::PaymentOnly,
            tangle_rpc_url: None,
            tangle_contract: None,
        })
}

async fn authorize_restricted_job(
    state: &GatewayState,
    policy: &JobPolicyConfig,
    service_id: u64,
    job_index: u32,
    body: &Bytes,
    headers: &HeaderMap,
    enforce_replay_guard: bool,
) -> Result<Address, PolicyRejection> {
    let caller = match policy.auth_mode {
        X402CallerAuthMode::PayerIsCaller => {
            let settlement = parse_settlement_details(headers).ok_or_else(|| {
                PolicyRejection::bad_request(
                    "missing_settlement_context",
                    "X-Payment-Response settlement context is required",
                )
            })?;
            settlement.payer.ok_or_else(|| {
                PolicyRejection::bad_request(
                    "missing_settled_payer",
                    "settled payer is required for auth_mode=payer_is_caller",
                )
            })?
        }
        X402CallerAuthMode::DelegatedCallerSignature => {
            let assertion = verify_delegated_signature(service_id, job_index, body, headers)?;
            if enforce_replay_guard {
                state
                    .replay_guard
                    .reserve(
                        assertion.caller,
                        service_id,
                        job_index,
                        &assertion.nonce,
                        assertion.expiry,
                    )
                    .await?;
            }
            assertion.caller
        }
        X402CallerAuthMode::PaymentOnly => {
            return Err(PolicyRejection::service_unavailable(
                "invalid_policy",
                "restricted_paid cannot use auth_mode=payment_only",
            ));
        }
    };

    let rpc_url = policy.tangle_rpc_url.as_ref().ok_or_else(|| {
        PolicyRejection::service_unavailable(
            "invalid_policy",
            "restricted_paid policy missing tangle_rpc_url",
        )
    })?;

    let contract = policy
        .tangle_contract
        .as_deref()
        .ok_or_else(|| {
            PolicyRejection::service_unavailable(
                "invalid_policy",
                "restricted_paid policy missing tangle_contract",
            )
        })
        .and_then(|raw| {
            Address::from_str(raw).map_err(|_| {
                PolicyRejection::service_unavailable(
                    "invalid_policy",
                    "restricted_paid policy has invalid tangle_contract",
                )
            })
        })?;

    let permitted = is_permitted_caller_eth_call(rpc_url, contract, service_id, caller)
        .await
        .map_err(|e| PolicyRejection::service_unavailable("permission_check_failed", e))?;

    if !permitted {
        return Err(PolicyRejection::denied(
            "caller_not_permitted",
            format!(
                "caller {} is not permitted for service_id={} via on-chain policy",
                caller, service_id
            ),
        ));
    }

    Ok(caller)
}

fn verify_delegated_signature(
    service_id: u64,
    job_index: u32,
    body: &Bytes,
    headers: &HeaderMap,
) -> Result<DelegatedAssertion, PolicyRejection> {
    let caller_raw = header_string(headers, HEADER_CALLER).ok_or_else(|| {
        PolicyRejection::bad_request(
            "missing_caller",
            "X-TANGLE-CALLER header is required for delegated signature auth",
        )
    })?;

    let caller = Address::from_str(&caller_raw)
        .map_err(|_| PolicyRejection::bad_request("invalid_caller", "invalid X-TANGLE-CALLER"))?;

    let nonce = header_string(headers, HEADER_CALLER_NONCE).ok_or_else(|| {
        PolicyRejection::bad_request(
            "missing_signature_nonce",
            "X-TANGLE-CALLER-NONCE header is required for delegated signature auth",
        )
    })?;

    if nonce.trim().is_empty() || nonce.len() > 128 {
        return Err(PolicyRejection::bad_request(
            "invalid_signature_nonce",
            "X-TANGLE-CALLER-NONCE must be non-empty and <= 128 chars",
        ));
    }

    let sig_raw = header_string(headers, HEADER_CALLER_SIG).ok_or_else(|| {
        PolicyRejection::bad_request(
            "missing_signature",
            "X-TANGLE-CALLER-SIG header is required for delegated signature auth",
        )
    })?;

    let signature = Signature::from_str(&sig_raw).map_err(|_| {
        PolicyRejection::bad_request(
            "invalid_signature",
            "invalid X-TANGLE-CALLER-SIG; expected 65-byte hex signature",
        )
    })?;

    let expiry = header_string(headers, HEADER_CALLER_EXPIRY)
        .ok_or_else(|| {
            PolicyRejection::bad_request(
                "missing_signature_expiry",
                "X-TANGLE-CALLER-EXPIRY header is required for delegated signature auth",
            )
        })?
        .parse::<u64>()
        .map_err(|_| {
            PolicyRejection::bad_request(
                "invalid_signature_expiry",
                "X-TANGLE-CALLER-EXPIRY must be a unix timestamp",
            )
        })?;

    let now = current_unix_timestamp_secs().map_err(|e| {
        PolicyRejection::service_unavailable("clock_error", format!("clock error: {e}"))
    })?;

    if now > expiry {
        return Err(PolicyRejection::denied(
            "signature_expired",
            "delegated signature has expired",
        ));
    }

    let payload = delegated_auth_payload(service_id, job_index, body, &nonce, expiry);

    let recovered = signature
        .recover_address_from_msg(payload.as_bytes())
        .map_err(|_| {
            PolicyRejection::bad_request(
                "invalid_signature_recovery",
                "failed to recover signer from delegated signature",
            )
        })?;

    if recovered != caller {
        return Err(PolicyRejection::denied(
            "signature_mismatch",
            "delegated signature does not match X-TANGLE-CALLER",
        ));
    }

    Ok(DelegatedAssertion {
        caller,
        nonce,
        expiry,
    })
}

#[derive(Debug, Clone)]
struct DelegatedAssertion {
    caller: Address,
    nonce: String,
    expiry: u64,
}

fn delegated_auth_payload(
    service_id: u64,
    job_index: u32,
    body: &Bytes,
    nonce: &str,
    expiry: u64,
) -> String {
    let body_hash = alloy_primitives::keccak256(body);
    format!(
        "x402-authorize:{service_id}:{job_index}:{}:{nonce}:{expiry}",
        hex::encode(body_hash)
    )
}

async fn is_permitted_caller_eth_call(
    rpc_url: &Url,
    tangle_contract: Address,
    service_id: u64,
    caller: Address,
) -> Result<bool, String> {
    let provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_http(rpc_url.clone());

    let tangle = ITangle::new(tangle_contract, &provider);

    tangle
        .isPermittedCaller(service_id, caller)
        .call()
        .await
        .map_err(|e| format!("eth_call isPermittedCaller failed: {e}"))
}

fn extract_payment_attribution(headers: &HeaderMap, config: &X402Config) -> PaymentAttribution {
    let settlement = parse_settlement_details(headers);

    let mut network = settlement.as_ref().and_then(|s| s.network.clone());
    let settled_payer = settlement.and_then(|s| s.payer);

    let mut asset = None;

    for header_name in [HEADER_PAYMENT_V1, HEADER_PAYMENT_V2] {
        if let Some(json) = decode_base64_json_header(headers, header_name) {
            if network.is_none() {
                network = json
                    .get("network")
                    .and_then(serde_json::Value::as_str)
                    .map(ToOwned::to_owned)
                    .or_else(|| {
                        json.get("accepted")
                            .and_then(|a| a.get("network"))
                            .and_then(serde_json::Value::as_str)
                            .map(ToOwned::to_owned)
                    });
            }

            asset = json
                .get("asset")
                .and_then(serde_json::Value::as_str)
                .map(ToOwned::to_owned)
                .or_else(|| {
                    json.get("accepted")
                        .and_then(|a| a.get("asset"))
                        .and_then(serde_json::Value::as_str)
                        .map(ToOwned::to_owned)
                });

            if asset.is_some() {
                break;
            }
        }
    }

    let token = resolve_token_label(config, network.as_deref(), asset.as_deref());

    PaymentAttribution {
        network,
        token,
        settled_payer,
    }
}

fn resolve_token_label(
    config: &X402Config,
    network: Option<&str>,
    asset: Option<&str>,
) -> Option<String> {
    match (network, asset) {
        (Some(network), Some(asset)) => {
            let asset_lc = asset.to_ascii_lowercase();
            config
                .accepted_tokens
                .iter()
                .find(|token| {
                    token.network == network && token.asset.to_ascii_lowercase() == asset_lc
                })
                .map(|token| token.symbol.clone())
                .or_else(|| Some(asset.to_string()))
        }
        (None, Some(asset)) => Some(asset.to_string()),
        _ => None,
    }
}

fn resolved_payment_metadata(
    config: &X402Config,
    attribution: &PaymentAttribution,
) -> (String, String) {
    if let (Some(network), Some(token)) = (&attribution.network, &attribution.token) {
        return (network.clone(), token.clone());
    }

    if config.accepted_tokens.len() == 1 {
        let token = &config.accepted_tokens[0];
        return (token.network.clone(), token.symbol.clone());
    }

    let networks: Vec<&str> = config
        .accepted_tokens
        .iter()
        .map(|token| token.network.as_str())
        .collect();
    (networks.join(","), "MULTI".into())
}

fn parse_settlement_details(headers: &HeaderMap) -> Option<SettlementDetails> {
    let raw = headers.get(HEADER_SETTLEMENT)?.as_bytes();
    let decoded = Base64Bytes::from(raw).decode().ok()?;
    let settlement: SettleResponse = serde_json::from_slice(&decoded).ok()?;

    match settlement {
        SettleResponse::Success { payer, network, .. } => {
            let payer = Address::from_str(&payer).ok();
            Some(SettlementDetails {
                payer,
                network: Some(network),
            })
        }
        SettleResponse::Error { network, .. } => Some(SettlementDetails {
            payer: None,
            network: Some(network),
        }),
    }
}

fn decode_base64_json_header(headers: &HeaderMap, header_name: &str) -> Option<serde_json::Value> {
    let raw = headers.get(header_name)?.as_bytes();
    let decoded = Base64Bytes::from(raw).decode().ok()?;
    serde_json::from_slice(&decoded).ok()
}

fn header_string(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(ToOwned::to_owned)
}

fn current_unix_timestamp_secs() -> Result<u64, std::time::SystemTimeError> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

// ═══════════════════════════════════════════════════════════════════════════
// EVM Price Tag Construction
// ═══════════════════════════════════════════════════════════════════════════

/// Build V2 x402 price tags for all accepted EVM tokens, given a job's URI.
///
/// Parses `service_id` and `job_index` from the URI path, looks up the
/// wei-denominated price, and converts to each accepted token's denomination.
fn build_evm_price_tags(
    config: &X402Config,
    job_pricing: &HashMap<(u64, u32), U256>,
    uri: &http::Uri,
) -> Vec<x402_types::proto::v2::PriceTag> {
    use x402_chain_eip155::V2Eip155Exact;
    use x402_chain_eip155::chain::{
        AssetTransferMethod, Eip155ChainReference, Eip155TokenDeployment,
    };

    // Parse service_id and job_index from URI: /x402/jobs/{service_id}/{job_index}
    let segments: Vec<&str> = uri.path().split('/').collect();
    let service_id: u64 = match segments.get(3).and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => {
            tracing::warn!(uri = %uri, "failed to parse service_id from URI");
            return vec![];
        }
    };
    let job_index: u32 = match segments.get(4).and_then(|s| s.parse().ok()) {
        Some(idx) => idx,
        None => {
            tracing::warn!(uri = %uri, "failed to parse job_index from URI");
            return vec![];
        }
    };

    let price_wei = match job_pricing.get(&(service_id, job_index)) {
        Some(p) => p,
        None => return vec![],
    };

    config
        .accepted_tokens
        .iter()
        .filter(|t| t.network.starts_with("eip155:"))
        .filter_map(|token| {
            let amount_str = match token.convert_wei_to_amount(price_wei) {
                Ok(s) => s,
                Err(e) => {
                    tracing::warn!(
                        token = %token.symbol,
                        network = %token.network,
                        error = %e,
                        "failed to convert wei price to token amount"
                    );
                    return None;
                }
            };

            let chain_id: u64 = match token.network.strip_prefix("eip155:") {
                Some(s) => match s.parse() {
                    Ok(id) => id,
                    Err(_) => {
                        tracing::warn!(
                            network = %token.network,
                            "invalid chain ID in token network"
                        );
                        return None;
                    }
                },
                None => return None,
            };

            let address: alloy_primitives::Address = match token.asset.parse() {
                Ok(a) => a,
                Err(_) => {
                    tracing::warn!(
                        token = %token.symbol,
                        asset = %token.asset,
                        "invalid asset address"
                    );
                    return None;
                }
            };

            let pay_to: alloy_primitives::Address = match token.pay_to.parse() {
                Ok(a) => a,
                Err(_) => {
                    tracing::warn!(
                        token = %token.symbol,
                        pay_to = %token.pay_to,
                        "invalid pay_to address"
                    );
                    return None;
                }
            };

            let transfer_method = if token.transfer_method == "eip3009" {
                let name = token
                    .eip3009_name
                    .clone()
                    .unwrap_or_else(|| "USD Coin".into());
                let version = token.eip3009_version.clone().unwrap_or_else(|| "2".into());
                AssetTransferMethod::Eip3009 { name, version }
            } else {
                AssetTransferMethod::Permit2
            };

            let deployment = Eip155TokenDeployment {
                chain_reference: Eip155ChainReference::new(chain_id),
                address,
                decimals: token.decimals,
                transfer_method,
            };

            let amount: u64 = match amount_str.parse::<u64>() {
                Ok(a) => a,
                Err(_) => {
                    tracing::error!(
                        token = %token.symbol,
                        amount = %amount_str,
                        service_id,
                        job_index,
                        reason = "amount_exceeds_u64",
                        "token amount exceeds u64; this should be blocked by startup validation"
                    );
                    return None;
                }
            };

            Some(V2Eip155Exact::price_tag(pay_to, deployment.amount(amount)))
        })
        .collect()
}

fn validate_price_tag_amount_bounds(
    config: &X402Config,
    job_pricing: &HashMap<(u64, u32), U256>,
) -> Result<(), X402Error> {
    for ((service_id, job_index), price_wei) in job_pricing {
        for token in &config.accepted_tokens {
            let amount_str = token.convert_wei_to_amount(price_wei)?;
            if amount_str.parse::<u64>().is_err() {
                return Err(X402Error::Config(format!(
                    "price-tag amount overflow for service_id={} job_index={} token={} network={} amount={} (must fit u64)",
                    service_id, job_index, token.symbol, token.network, amount_str
                )));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Json as AxumJson;
    use axum::http::HeaderValue;
    use serde_json::json;
    use std::net::SocketAddr;
    use tokio::task::JoinHandle;

    const TEST_CALLER: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
    const TEST_SIG: &str = "0x814d0eea80800217fd84c9cb9daa1b72c17f30745366958bd0e2798190875c145a27d3ce7ab1dfd8937282633c978a81ba053734cc2869d016af6457b3b5c5e81b";
    const TEST_NONCE: &str = "nonce-123";
    const TEST_EXPIRY: &str = "4102444800"; // 2100-01-01T00:00:00Z

    fn delegated_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CALLER, HeaderValue::from_static(TEST_CALLER));
        headers.insert(HEADER_CALLER_NONCE, HeaderValue::from_static(TEST_NONCE));
        headers.insert(HEADER_CALLER_SIG, HeaderValue::from_static(TEST_SIG));
        headers.insert(HEADER_CALLER_EXPIRY, HeaderValue::from_static(TEST_EXPIRY));
        headers
    }

    #[test]
    fn delegated_payload_binds_nonce() {
        let body = Bytes::from_static(b"hello");
        let payload = delegated_auth_payload(1, 7, &body, TEST_NONCE, 4_102_444_800);
        assert_eq!(
            payload,
            "x402-authorize:1:7:1c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac8:nonce-123:4102444800"
        );
    }

    #[test]
    fn verify_delegated_signature_accepts_valid_headers() {
        let headers = delegated_headers();
        let body = Bytes::from_static(b"hello");
        let assertion = verify_delegated_signature(1, 7, &body, &headers).expect("valid assertion");
        assert_eq!(assertion.caller, Address::from_str(TEST_CALLER).unwrap());
        assert_eq!(assertion.nonce, TEST_NONCE);
        assert_eq!(assertion.expiry, 4_102_444_800);
    }

    #[test]
    fn verify_delegated_signature_requires_nonce_header() {
        let mut headers = delegated_headers();
        headers.remove(HEADER_CALLER_NONCE);
        let body = Bytes::from_static(b"hello");
        let err = verify_delegated_signature(1, 7, &body, &headers).unwrap_err();
        assert_eq!(err.status, StatusCode::BAD_REQUEST);
        assert_eq!(err.code, "missing_signature_nonce");
    }

    #[tokio::test]
    async fn replay_guard_rejects_duplicate_nonce_same_scope() {
        let guard = DelegatedReplayGuard::default();
        let caller = Address::from_str(TEST_CALLER).unwrap();
        guard
            .reserve(caller, 1, 7, TEST_NONCE, 4_102_444_800)
            .await
            .expect("first use should pass");

        let err = guard
            .reserve(caller, 1, 7, TEST_NONCE, 4_102_444_800)
            .await
            .unwrap_err();
        assert_eq!(err.status, StatusCode::CONFLICT);
        assert_eq!(err.code, "signature_replay");
    }

    #[tokio::test]
    async fn replay_guard_allows_same_nonce_other_job_scope() {
        let guard = DelegatedReplayGuard::default();
        let caller = Address::from_str(TEST_CALLER).unwrap();

        guard
            .reserve(caller, 1, 7, TEST_NONCE, 4_102_444_800)
            .await
            .expect("first use should pass");
        guard
            .reserve(caller, 1, 8, TEST_NONCE, 4_102_444_800)
            .await
            .expect("same nonce in different job scope should pass");
    }

    async fn start_mock_eth_rpc() -> (Url, JoinHandle<()>) {
        async fn handler(
            AxumJson(payload): AxumJson<serde_json::Value>,
        ) -> AxumJson<serde_json::Value> {
            let id = payload.get("id").cloned().unwrap_or(json!(1));
            AxumJson(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": "0x0000000000000000000000000000000000000000000000000000000000000001"
            }))
        }

        let app = Router::new().route("/", post(handler));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind mock rpc");
        let addr: SocketAddr = listener.local_addr().expect("local addr");
        let handle = tokio::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });

        (format!("http://{addr}").parse().expect("valid url"), handle)
    }

    #[tokio::test]
    async fn authorize_restricted_job_enforces_replay_in_paid_path() {
        let (rpc_url, rpc_handle) = start_mock_eth_rpc().await;
        let body = Bytes::from_static(b"hello");
        let headers = delegated_headers();
        let policy = JobPolicyConfig {
            service_id: 1,
            job_index: 7,
            invocation_mode: X402InvocationMode::RestrictedPaid,
            auth_mode: X402CallerAuthMode::DelegatedCallerSignature,
            tangle_rpc_url: Some(rpc_url),
            tangle_contract: Some("0x0000000000000000000000000000000000000001".into()),
        };

        let (payment_tx, _payment_rx) = mpsc::unbounded_channel();
        let state = GatewayState {
            config: Arc::new(X402Config {
                bind_address: "127.0.0.1:8402".parse().unwrap(),
                facilitator_url: "https://x402.org/facilitator".parse().unwrap(),
                quote_ttl_secs: 300,
                accepted_tokens: vec![],
                default_invocation_mode: X402InvocationMode::Disabled,
                job_policies: vec![],
                service_id: 1,
            }),
            job_pricing: Arc::new(HashMap::new()),
            job_policies: Arc::new(HashMap::new()),
            quote_registry: QuoteRegistry::new(Duration::from_secs(30)),
            payment_tx,
            call_id_counter: Arc::new(AtomicU64::new(1)),
            replay_guard: Arc::new(DelegatedReplayGuard::default()),
            counters: Arc::new(GatewayCounters::default()),
        };

        let first = authorize_restricted_job(&state, &policy, 1, 7, &body, &headers, true).await;
        assert!(first.is_ok(), "first authorization should pass");

        let second = authorize_restricted_job(&state, &policy, 1, 7, &body, &headers, true).await;
        let err = second.expect_err("replayed nonce should fail");
        assert_eq!(err.status, StatusCode::CONFLICT);
        assert_eq!(err.code, "signature_replay");

        rpc_handle.abort();
    }

    #[tokio::test]
    async fn authorize_restricted_job_dry_run_does_not_consume_nonce() {
        let (rpc_url, rpc_handle) = start_mock_eth_rpc().await;
        let body = Bytes::from_static(b"hello");
        let headers = delegated_headers();
        let policy = JobPolicyConfig {
            service_id: 1,
            job_index: 7,
            invocation_mode: X402InvocationMode::RestrictedPaid,
            auth_mode: X402CallerAuthMode::DelegatedCallerSignature,
            tangle_rpc_url: Some(rpc_url),
            tangle_contract: Some("0x0000000000000000000000000000000000000001".into()),
        };

        let (payment_tx, _payment_rx) = mpsc::unbounded_channel();
        let state = GatewayState {
            config: Arc::new(X402Config {
                bind_address: "127.0.0.1:8402".parse().unwrap(),
                facilitator_url: "https://x402.org/facilitator".parse().unwrap(),
                quote_ttl_secs: 300,
                accepted_tokens: vec![],
                default_invocation_mode: X402InvocationMode::Disabled,
                job_policies: vec![],
                service_id: 1,
            }),
            job_pricing: Arc::new(HashMap::new()),
            job_policies: Arc::new(HashMap::new()),
            quote_registry: QuoteRegistry::new(Duration::from_secs(30)),
            payment_tx,
            call_id_counter: Arc::new(AtomicU64::new(1)),
            replay_guard: Arc::new(DelegatedReplayGuard::default()),
            counters: Arc::new(GatewayCounters::default()),
        };

        // Dry-run path should not reserve nonce.
        let dry_run_1 =
            authorize_restricted_job(&state, &policy, 1, 7, &body, &headers, false).await;
        assert!(dry_run_1.is_ok(), "first dry-run should pass");
        let dry_run_2 =
            authorize_restricted_job(&state, &policy, 1, 7, &body, &headers, false).await;
        assert!(dry_run_2.is_ok(), "second dry-run should also pass");

        // First paid execution should still pass, because dry-runs did not consume nonce.
        let paid = authorize_restricted_job(&state, &policy, 1, 7, &body, &headers, true).await;
        assert!(paid.is_ok(), "paid path should pass after dry-runs");

        rpc_handle.abort();
    }
}
