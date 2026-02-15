//! The x402 payment gateway server.
//!
//! Runs an axum HTTP server as a [`BackgroundService`] within the Blueprint runner.
//! Each registered job gets an endpoint protected by the x402 middleware from
//! [`x402_axum`]. When a client pays, the payment is verified via the configured
//! facilitator, and a [`JobCall`] is injected into the runner's producer stream.

use crate::config::X402Config;
use crate::error::X402Error;
use crate::producer::VerifiedPayment;
use crate::quote_registry::QuoteRegistry;
use crate::settlement::SettlementOption;

use alloy_primitives::U256;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use blueprint_runner::BackgroundService;
use blueprint_runner::error::RunnerError;
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use x402_axum::X402Middleware;

/// Shared state for the axum handlers.
#[derive(Clone)]
struct GatewayState {
    config: Arc<X402Config>,
    /// Per-job prices in wei: (service_id, job_index) → U256
    job_pricing: Arc<HashMap<(u64, u32), U256>>,
    /// Quote tracking
    quote_registry: QuoteRegistry,
    /// Channel to send verified payments to the runner's producer
    payment_tx: mpsc::UnboundedSender<VerifiedPayment>,
    /// Monotonic call ID counter
    call_id_counter: Arc<AtomicU64>,
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
    payment_tx: mpsc::UnboundedSender<VerifiedPayment>,
    quote_registry: QuoteRegistry,
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
        if config.accepted_tokens.is_empty() {
            return Err(X402Error::Config(
                "at least one accepted_token must be configured".into(),
            ));
        }

        let (producer, payment_tx) = crate::X402Producer::channel();
        let quote_registry = QuoteRegistry::new(Duration::from_secs(config.quote_ttl_secs));

        let gateway = Self {
            config: Arc::new(config),
            job_pricing: Arc::new(job_pricing),
            payment_tx,
            quote_registry,
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
            quote_registry: self.quote_registry.clone(),
            payment_tx: self.payment_tx.clone(),
            call_id_counter: Arc::new(AtomicU64::new(1)),
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

/// Handle a paid job request.
///
/// Called after the x402 middleware has verified and settled payment.
/// The operator has already been paid on-chain at this point.
async fn handle_job_request(
    State(state): State<GatewayState>,
    Path((service_id, job_index)): Path<(u64, u32)>,
    body: Bytes,
) -> impl IntoResponse {
    let key = (service_id, job_index);

    // Verify the job exists in our pricing config
    let price_wei = match state.job_pricing.get(&key) {
        Some(p) => p,
        None => {
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

    // Register a dynamic quote for tracking
    let quote_digest = state
        .quote_registry
        .insert_dynamic(service_id, job_index, *price_wei);

    // Consume the quote (marks it as paid)
    if state.quote_registry.consume(&quote_digest).is_none() {
        return (
            StatusCode::CONFLICT,
            Json(serde_json::json!({ "error": "quote already consumed or expired" })),
        )
            .into_response();
    }

    let call_id = state.call_id_counter.fetch_add(1, Ordering::Relaxed);

    // Record which payment network/token was used. This is accounting
    // metadata only -- it does not affect job execution. The operator
    // receives payment at their on-chain `pay_to` address regardless.
    //
    // For single-token configs this is exact. For multi-token configs we
    // record all accepted networks. Per-request identification is possible
    // by parsing the X-Payment request header, but isn't needed since the
    // operator can reconcile directly from on-chain payment records.
    let (payment_network, payment_token) = if state.config.accepted_tokens.len() == 1 {
        let token = &state.config.accepted_tokens[0];
        (token.network.clone(), token.symbol.clone())
    } else {
        let networks: Vec<&str> = state
            .config
            .accepted_tokens
            .iter()
            .map(|t| t.network.as_str())
            .collect();
        (networks.join(","), "MULTI".into())
    };

    let payment = VerifiedPayment {
        service_id,
        job_index,
        job_args: body,
        quote_digest,
        payment_network,
        payment_token,
        call_id,
    };

    // Send to the runner's producer stream
    if state.payment_tx.send(payment).is_err() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "error": "service shutting down" })),
        )
            .into_response();
    }

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

// Re-use alloy's hex encoder for quote digests.
use alloy_primitives::hex;

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
    use x402_chain_eip155::chain::{Eip155ChainReference, Eip155TokenDeployment};

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

            let deployment = Eip155TokenDeployment {
                chain_reference: Eip155ChainReference::new(chain_id),
                address,
                decimals: token.decimals,
                eip712: None,
            };

            // Parse amount as U256 to handle large values (18-decimal tokens
            // can exceed u64::MAX).
            let amount = match U256::from_str_radix(&amount_str, 10) {
                Ok(a) => a,
                Err(_) => {
                    tracing::warn!(
                        token = %token.symbol,
                        amount = %amount_str,
                        "failed to parse token amount as U256"
                    );
                    return None;
                }
            };

            Some(V2Eip155Exact::price_tag(pay_to, deployment.amount(amount)))
        })
        .collect()
}
