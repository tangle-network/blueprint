//! Axum handlers for the parallel `/mpp/jobs/{service_id}/{job_index}`
//! ingress.
//!
//! These handlers speak the IETF Payment HTTP Authentication Scheme:
//!
//! - On a request without `Authorization: Payment`, return `402 Payment
//!   Required` with a `WWW-Authenticate: Payment ...` challenge.
//! - On a request with `Authorization: Payment <base64url credential>`,
//!   parse the credential, validate the HMAC + expiry + per-route fields
//!   via `mpp::Mpp::verify_credential_with_expected_request`, then call
//!   the shared [`handle_paid_job_inner`] to enforce policy and enqueue
//!   the job. On success, attach a `Payment-Receipt` header.
//!
//! Errors are returned as RFC 9457 Problem Details
//! (`application/problem+json`) per the MPP spec.

use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use alloy_primitives::{Address, U256};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use mpp::protocol::core::headers::{
    PAYMENT_RECEIPT_HEADER, WWW_AUTHENTICATE_HEADER, extract_payment_scheme, format_receipt,
    format_www_authenticate, parse_authorization,
};
use mpp::protocol::core::{Base64UrlJson, PaymentChallenge};
use mpp::protocol::intents::ChargeRequest;
use mpp::protocol::traits::ErrorCode;
use serde_json::json;
use std::sync::atomic::Ordering;

use crate::X402InvocationMode;
use crate::config::AcceptedToken;
use crate::gateway::{
    EnqueuedReceipt, GatewayState, PaymentAttribution, PolicyRejection, handle_paid_job_inner,
    resolve_job_policy,
};
use crate::mpp::credential::{Eip3009Extra, MppMethodDetails};
use crate::mpp::method::METHOD_NAME;
use crate::mpp::state::MppGatewayState;
use crate::settlement::{PaymentProtocol, SettlementOption};

const CONTENT_TYPE_PROBLEM_JSON: &str = "application/problem+json";
const PROBLEM_TYPE_BASE: &str = "https://paymentauth.org/problems/";

/// `POST /mpp/jobs/{service_id}/{job_index}` — the MPP-equivalent of
/// `POST /x402/jobs/{service_id}/{job_index}`.
///
/// See the [module docs](self) for the request/response shape.
pub(crate) async fn handle_mpp_job_request(
    State(state): State<GatewayState>,
    Path((service_id, job_index)): Path<(u64, u32)>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let Some(mpp_state) = state.mpp.clone() else {
        // Routes only registered when MPP is configured; this is defence
        // in depth in case axum dispatches a request after a config flip.
        return problem_response(
            StatusCode::NOT_FOUND,
            "verification-failed",
            "MPP ingress is not enabled",
            None,
            service_id,
            job_index,
        );
    };

    let key = (service_id, job_index);
    let Some(price_wei) = state.job_pricing.get(&key).copied() else {
        state.counters.job_not_found.fetch_add(1, Ordering::Relaxed);
        return problem_response(
            StatusCode::NOT_FOUND,
            "verification-failed",
            "job not found",
            None,
            service_id,
            job_index,
        );
    };

    let policy = resolve_job_policy(&state, service_id, job_index);
    if policy.invocation_mode == X402InvocationMode::Disabled {
        state.counters.policy_denied.fetch_add(1, Ordering::Relaxed);
        return problem_response(
            StatusCode::FORBIDDEN,
            "verification-failed",
            "x402 disabled for this job",
            None,
            service_id,
            job_index,
        );
    }

    // Look at the Authorization header for an existing credential.
    let credential_str = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(extract_payment_scheme)
        .map(str::to_string);

    let credential_str = match credential_str {
        Some(s) => s,
        None => {
            // No credential — issue a fresh challenge for *every* accepted
            // token. Per RFC 9110 multiple WWW-Authenticate headers can be
            // returned, and MPP follows that convention. We pick the first
            // accepted token as the canonical challenge body and emit one
            // header per token via `format_www_authenticate_many`.
            return issue_challenge_response(
                &state,
                &mpp_state,
                service_id,
                job_index,
                &price_wei,
                policy_default_token(&mpp_state).as_ref(),
            );
        }
    };

    let credential = match parse_authorization(&credential_str) {
        Ok(c) => c,
        Err(e) => {
            state
                .counters
                .mpp_verification_failed
                .fetch_add(1, Ordering::Relaxed);
            return problem_response(
                StatusCode::BAD_REQUEST,
                "malformed-credential",
                format!("invalid Authorization: Payment header: {e}"),
                None,
                service_id,
                job_index,
            );
        }
    };

    // Recover the method_details the client signed against so we know which
    // accepted token to validate this credential against.
    let echo_request: ChargeRequest = match credential.challenge.request.decode() {
        Ok(r) => r,
        Err(e) => {
            state
                .counters
                .mpp_verification_failed
                .fetch_add(1, Ordering::Relaxed);
            return problem_response(
                StatusCode::BAD_REQUEST,
                "malformed-credential",
                format!("credential challenge request decode failed: {e}"),
                None,
                service_id,
                job_index,
            );
        }
    };
    let method_details = match echo_request
        .method_details
        .clone()
        .map(serde_json::from_value::<MppMethodDetails>)
    {
        Some(Ok(d)) => d,
        Some(Err(e)) => {
            state
                .counters
                .mpp_verification_failed
                .fetch_add(1, Ordering::Relaxed);
            return problem_response(
                StatusCode::BAD_REQUEST,
                "malformed-credential",
                format!("credential methodDetails decode failed: {e}"),
                None,
                service_id,
                job_index,
            );
        }
        None => {
            state
                .counters
                .mpp_verification_failed
                .fetch_add(1, Ordering::Relaxed);
            return problem_response(
                StatusCode::BAD_REQUEST,
                "malformed-credential",
                "credential is missing methodDetails",
                None,
                service_id,
                job_index,
            );
        }
    };

    // Cross-check that the credential is for *this* (service_id, job_index).
    if method_details.service_id != service_id || method_details.job_index != job_index {
        state
            .counters
            .mpp_verification_failed
            .fetch_add(1, Ordering::Relaxed);
        return problem_response(
            StatusCode::FORBIDDEN,
            "verification-failed",
            format!(
                "credential is bound to service_id={} job_index={}, not this route",
                method_details.service_id, method_details.job_index
            ),
            None,
            service_id,
            job_index,
        );
    }

    // Find the matching accepted token so we can build the canonical expected
    // ChargeRequest with the freshly-converted amount.
    let token = mpp_state
        .accepted_tokens
        .iter()
        .find(|t| {
            t.network == method_details.network
                && t.asset.eq_ignore_ascii_case(&echo_request.currency)
        })
        .cloned();
    let Some(token) = token else {
        state
            .counters
            .mpp_verification_failed
            .fetch_add(1, Ordering::Relaxed);
        return problem_response(
            StatusCode::BAD_REQUEST,
            "method-unsupported",
            format!(
                "no accepted token matches network={} asset={}",
                method_details.network, echo_request.currency
            ),
            None,
            service_id,
            job_index,
        );
    };

    let expected_amount = match token.convert_wei_to_amount(&price_wei) {
        Ok(s) => s,
        Err(e) => {
            return problem_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "verification-failed",
                format!("price conversion failed: {e}"),
                None,
                service_id,
                job_index,
            );
        }
    };

    let expected_request = build_charge_request(
        expected_amount.clone(),
        token.clone(),
        method_details.clone(),
        service_id,
        job_index,
    );

    // Validate HMAC + expiry + amount/currency/recipient match, then run
    // the BlueprintEvmChargeMethod which forwards to the facilitator.
    let receipt = match mpp_state
        .mpp
        .verify_credential_with_expected_request(&credential, &expected_request)
        .await
    {
        Ok(r) => r,
        Err(e) => {
            state
                .counters
                .mpp_verification_failed
                .fetch_add(1, Ordering::Relaxed);
            let (status, code) = match e.code {
                Some(ErrorCode::Expired) => (StatusCode::PAYMENT_REQUIRED, "payment-expired"),
                Some(ErrorCode::InvalidAmount) | Some(ErrorCode::InsufficientBalance) => {
                    (StatusCode::PAYMENT_REQUIRED, "payment-insufficient")
                }
                Some(ErrorCode::CredentialMismatch)
                | Some(ErrorCode::InvalidCredential)
                | Some(ErrorCode::InvalidPayload) => {
                    (StatusCode::BAD_REQUEST, "malformed-credential")
                }
                Some(ErrorCode::ChainIdMismatch) => (StatusCode::BAD_REQUEST, "method-unsupported"),
                _ => (StatusCode::PAYMENT_REQUIRED, "verification-failed"),
            };
            return problem_response(
                status,
                code,
                e.message,
                Some(&credential.challenge.id),
                service_id,
                job_index,
            );
        }
    };

    // Resolve attribution from the verified MPP receipt.
    // `Receipt.reference` is the on-chain settlement transaction hash;
    // we don't try to extract the payer from it (the facilitator already
    // recorded it via VerifyResponse). Use the receipt token by symbol.
    let attribution = PaymentAttribution {
        network: Some(token.network.clone()),
        token: Some(token.symbol.clone()),
        // The facilitator's VerifyResponse already returned a payer to the
        // ChargeMethod, but we don't currently thread it back through the
        // mpp Receipt. For `payer_is_caller` policies the gateway will fall
        // back to the regular header-based settlement context — operators
        // requiring payer-as-caller for MPP today should also configure
        // restricted_paid + DelegatedCallerSignature, which is wire-protocol
        // agnostic. The facilitator-payer plumbing is tracked as TODO.
        settled_payer: None,
    };

    let enqueued = match handle_paid_job_inner(
        &state,
        service_id,
        job_index,
        body,
        &headers,
        attribution,
        PaymentProtocol::Mpp,
    )
    .await
    {
        Ok(r) => r,
        Err(rej) => return policy_rejection_to_problem(rej, service_id, job_index),
    };

    success_response(receipt, enqueued, service_id, job_index)
}

/// `GET /mpp/jobs/{service_id}/{job_index}/price` — discovery endpoint that
/// advertises the MPP settlement options for the job. The shape mirrors the
/// existing `/x402/jobs/.../price` discovery endpoint, but the entries
/// carry `protocol: "mpp"` and use the MPP route URL.
pub(crate) async fn get_mpp_job_price(
    State(state): State<GatewayState>,
    Path((service_id, job_index)): Path<(u64, u32)>,
) -> Response {
    if state.mpp.is_none() {
        return problem_response(
            StatusCode::NOT_FOUND,
            "verification-failed",
            "MPP ingress is not enabled",
            None,
            service_id,
            job_index,
        );
    }

    let key = (service_id, job_index);
    let Some(price_wei) = state.job_pricing.get(&key).copied() else {
        state.counters.job_not_found.fetch_add(1, Ordering::Relaxed);
        return problem_response(
            StatusCode::NOT_FOUND,
            "verification-failed",
            "job not found",
            None,
            service_id,
            job_index,
        );
    };

    let policy = resolve_job_policy(&state, service_id, job_index);
    if policy.invocation_mode == X402InvocationMode::Disabled {
        return problem_response(
            StatusCode::FORBIDDEN,
            "verification-failed",
            "x402 disabled for this job",
            None,
            service_id,
            job_index,
        );
    }

    let options: Result<Vec<SettlementOption>, _> = crate::X402Gateway::mpp_settlement_options(
        &state.config,
        service_id,
        job_index,
        &price_wei,
    );

    match options {
        Ok(opts) => (
            StatusCode::OK,
            Json(json!({
                "service_id": service_id,
                "job_index": job_index,
                "price_wei": price_wei.to_string(),
                "settlement_options": opts,
            })),
        )
            .into_response(),
        Err(e) => problem_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "verification-failed",
            e.to_string(),
            None,
            service_id,
            job_index,
        ),
    }
}

// ───────────────────────────────────────────────────────────────────────────
// Helpers
// ───────────────────────────────────────────────────────────────────────────

/// Pick a default accepted token to use as the headline challenge when the
/// client did not specify a preference. Today this is just the first
/// configured token; once MPP supports multi-token challenges via
/// `format_www_authenticate_many` we'll emit one challenge per token.
fn policy_default_token(mpp_state: &Arc<MppGatewayState>) -> Option<AcceptedToken> {
    mpp_state.accepted_tokens.first().cloned()
}

/// Build the canonical [`ChargeRequest`] for a given (token, price) pair.
///
/// This is the request the server *expects* to see echoed back inside any
/// MPP credential for this route. It mirrors the on-wire shape that
/// [`build_challenge`] embeds in `WWW-Authenticate`.
fn build_charge_request(
    amount: String,
    token: AcceptedToken,
    method_details: MppMethodDetails,
    service_id: u64,
    job_index: u32,
) -> ChargeRequest {
    let _ = (service_id, job_index); // included via method_details
    ChargeRequest {
        amount,
        currency: token.asset.clone(),
        decimals: None,
        recipient: Some(token.pay_to.clone()),
        description: Some(format!(
            "Tangle blueprint job service_id={} job_index={}",
            method_details.service_id, method_details.job_index
        )),
        external_id: None,
        method_details: Some(
            serde_json::to_value(&method_details).unwrap_or(serde_json::Value::Null),
        ),
    }
}

/// Issue a `402 Payment Required` response with one `WWW-Authenticate:
/// Payment ...` challenge per accepted token. We pick the first token as
/// the body of the 402; the rest are emitted as additional WWW-Authenticate
/// headers per RFC 9110.
fn issue_challenge_response(
    state: &GatewayState,
    mpp_state: &Arc<MppGatewayState>,
    service_id: u64,
    job_index: u32,
    price_wei: &U256,
    _headline_token: Option<&AcceptedToken>,
) -> Response {
    if mpp_state.accepted_tokens.is_empty() {
        return problem_response(
            StatusCode::SERVICE_UNAVAILABLE,
            "verification-failed",
            "no accepted tokens configured",
            None,
            service_id,
            job_index,
        );
    }

    state
        .counters
        .mpp_challenge_issued
        .fetch_add(1, Ordering::Relaxed);

    let mut challenges: Vec<PaymentChallenge> = Vec::with_capacity(mpp_state.accepted_tokens.len());
    for token in mpp_state.accepted_tokens.iter() {
        match build_challenge(mpp_state, token, service_id, job_index, price_wei) {
            Ok(c) => challenges.push(c),
            Err(e) => {
                tracing::warn!(
                    service_id,
                    job_index,
                    token = %token.symbol,
                    network = %token.network,
                    error = %e,
                    "skipping accepted token in MPP challenge"
                );
            }
        }
    }

    if challenges.is_empty() {
        return problem_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "verification-failed",
            "no accepted tokens could be encoded into MPP challenges",
            None,
            service_id,
            job_index,
        );
    }

    let mut header_values: Vec<HeaderValue> = Vec::with_capacity(challenges.len());
    for c in &challenges {
        match format_www_authenticate(c)
            .ok()
            .and_then(|s| HeaderValue::from_str(&s).ok())
        {
            Some(v) => header_values.push(v),
            None => {
                tracing::warn!(
                    service_id,
                    job_index,
                    "failed to encode MPP challenge as WWW-Authenticate header"
                );
            }
        }
    }

    let body = json!({
        "type": format!("{PROBLEM_TYPE_BASE}payment-required"),
        "title": "Payment Required",
        "status": 402,
        "detail": "Payment is required to invoke this Tangle blueprint job",
        "instance": format!("/mpp/jobs/{service_id}/{job_index}"),
        "challenge_ids": challenges.iter().map(|c| c.id.clone()).collect::<Vec<_>>(),
        "service_id": service_id,
        "job_index": job_index,
    });

    let mut resp = (StatusCode::PAYMENT_REQUIRED, Json(body)).into_response();
    let headers = resp.headers_mut();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_static(CONTENT_TYPE_PROBLEM_JSON),
    );
    for v in header_values {
        headers.append(WWW_AUTHENTICATE_HEADER, v);
    }
    resp
}

/// Build one [`PaymentChallenge`] for a single accepted token.
fn build_challenge(
    mpp_state: &Arc<MppGatewayState>,
    token: &AcceptedToken,
    service_id: u64,
    job_index: u32,
    price_wei: &U256,
) -> Result<PaymentChallenge, String> {
    let amount = token
        .convert_wei_to_amount(price_wei)
        .map_err(|e| e.to_string())?;

    // Validate token addresses parse cleanly so we don't ship a 402 a client
    // can't act on.
    token
        .asset
        .parse::<Address>()
        .map_err(|_| format!("invalid asset address {}", token.asset))?;
    token
        .pay_to
        .parse::<Address>()
        .map_err(|_| format!("invalid pay_to address {}", token.pay_to))?;

    let method_details = MppMethodDetails {
        network: token.network.clone(),
        scheme: "exact".into(),
        transfer_method: token.transfer_method.clone(),
        extra: if token.transfer_method == "eip3009" {
            Some(Eip3009Extra {
                name: token
                    .eip3009_name
                    .clone()
                    .unwrap_or_else(|| "USD Coin".into()),
                version: token.eip3009_version.clone().unwrap_or_else(|| "2".into()),
            })
        } else {
            None
        },
        decimals: token.decimals,
        service_id,
        job_index,
    };

    let request =
        build_charge_request(amount, token.clone(), method_details, service_id, job_index);
    let request_b64 = Base64UrlJson::from_typed(&request).map_err(|e| e.to_string())?;

    let expires =
        unix_iso8601(SystemTime::now() + Duration::from_secs(mpp_state.challenge_ttl_secs))
            .map_err(|e| e.to_string())?;

    Ok(PaymentChallenge::with_secret_key_full(
        mpp_state.secret_key.as_str(),
        mpp_state.realm.clone(),
        METHOD_NAME,
        "charge",
        request_b64,
        Some(&expires),
        None,
        Some(&format!(
            "Tangle blueprint job service_id={service_id} job_index={job_index} via {}",
            token.symbol
        )),
        None,
    ))
}

/// Format a SystemTime as an ISO 8601 / RFC 3339 timestamp.
fn unix_iso8601(t: SystemTime) -> Result<String, String> {
    let secs = t
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs() as i64;
    let datetime = time::OffsetDateTime::from_unix_timestamp(secs).map_err(|e| e.to_string())?;
    datetime
        .format(&time::format_description::well_known::Rfc3339)
        .map_err(|e| e.to_string())
}

/// Build a successful 202 response carrying both the JSON enqueue receipt
/// and the MPP `Payment-Receipt` header so the client can verify settlement.
fn success_response(
    receipt: mpp::protocol::core::Receipt,
    enqueued: EnqueuedReceipt,
    service_id: u64,
    job_index: u32,
) -> Response {
    let body = json!({
        "status": "accepted",
        "receipt": enqueued.quote_digest_hex,
        "service_id": service_id,
        "job_index": job_index,
        "call_id": enqueued.call_id,
    });

    let mut resp = (StatusCode::ACCEPTED, Json(body)).into_response();
    if let Ok(receipt_val) = format_receipt(&receipt) {
        if let Ok(header_val) = HeaderValue::from_str(&receipt_val) {
            resp.headers_mut()
                .insert(PAYMENT_RECEIPT_HEADER, header_val);
        }
    }
    resp
}

/// Convert a [`PolicyRejection`] from the shared ingress helper into an
/// RFC 9457 Problem Details response.
fn policy_rejection_to_problem(rej: PolicyRejection, service_id: u64, job_index: u32) -> Response {
    let code = match rej.code {
        "x402_disabled" => "verification-failed",
        "job_not_found" => "verification-failed",
        "quote_conflict" => "verification-failed",
        "shutting_down" => "verification-failed",
        "signature_replay" => "verification-failed",
        "caller_not_permitted" => "verification-failed",
        _ => "verification-failed",
    };
    problem_response(rej.status, code, rej.detail, None, service_id, job_index)
}

/// Build an RFC 9457 Problem Details response.
fn problem_response(
    status: StatusCode,
    code: &str,
    detail: impl Into<String>,
    instance_challenge_id: Option<&str>,
    service_id: u64,
    job_index: u32,
) -> Response {
    let detail = detail.into();
    let mut body = json!({
        "type": format!("{PROBLEM_TYPE_BASE}{code}"),
        "title": status.canonical_reason().unwrap_or("Error"),
        "status": status.as_u16(),
        "detail": detail,
        "instance": format!("/mpp/jobs/{service_id}/{job_index}"),
        "service_id": service_id,
        "job_index": job_index,
    });
    if let Some(challenge_id) = instance_challenge_id {
        body.as_object_mut()
            .expect("json object")
            .insert("challenge_id".into(), json!(challenge_id));
    }
    let mut resp = (status, Json(body)).into_response();
    resp.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_static(CONTENT_TYPE_PROBLEM_JSON),
    );
    resp
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    fn token() -> AcceptedToken {
        AcceptedToken {
            network: "eip155:8453".into(),
            asset: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".into(),
            symbol: "USDC".into(),
            decimals: 6,
            pay_to: "0x000000000000000000000000000000000000dEaD".into(),
            rate_per_native_unit: Decimal::from(3200u32),
            markup_bps: 0,
            transfer_method: "eip3009".into(),
            eip3009_name: Some("USD Coin".into()),
            eip3009_version: Some("2".into()),
        }
    }

    #[test]
    fn build_charge_request_round_trip() {
        let req = build_charge_request(
            "10000".into(),
            token(),
            MppMethodDetails {
                network: "eip155:8453".into(),
                scheme: "exact".into(),
                transfer_method: "eip3009".into(),
                extra: Some(Eip3009Extra {
                    name: "USD Coin".into(),
                    version: "2".into(),
                }),
                decimals: 6,
                service_id: 1,
                job_index: 0,
            },
            1,
            0,
        );
        assert_eq!(req.amount, "10000");
        assert_eq!(req.currency.to_lowercase(), token().asset.to_lowercase());
        assert_eq!(req.recipient.as_deref(), Some(token().pay_to.as_str()));
        let details: MppMethodDetails =
            serde_json::from_value(req.method_details.unwrap()).unwrap();
        assert_eq!(details.network, "eip155:8453");
    }

    #[test]
    fn problem_response_is_application_problem_json() {
        let resp = problem_response(
            StatusCode::BAD_REQUEST,
            "malformed-credential",
            "broken",
            None,
            1,
            0,
        );
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let ct = resp
            .headers()
            .get(axum::http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default();
        assert_eq!(ct, CONTENT_TYPE_PROBLEM_JSON);
    }
}
