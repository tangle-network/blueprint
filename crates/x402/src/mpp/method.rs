//! `BlueprintEvmChargeMethod` — MPP `ChargeMethod` impl that delegates to the
//! configured x402 facilitator.
//!
//! The Blueprint MPP method is intentionally **a thin envelope** over x402:
//! the credential the client returns is a base64url-encoded x402 v1
//! `PaymentPayload<ExactScheme, ExactEvmPayload>` (i.e. an EIP-3009 / Permit2
//! authorization) wrapped in an MPP `PaymentCredential`. We rebuild the
//! canonical x402 `PaymentRequirements` from our `AcceptedToken` table at
//! verify time and forward `(payload, requirements)` to the existing
//! facilitator's `/verify` then `/settle` endpoints.
//!
//! This means:
//! - Existing x402 client wallets work over the MPP wire format unchanged.
//! - The cross-chain pricing math (`AcceptedToken::convert_wei_to_amount`)
//!   is reused without duplication.
//! - The same facilitator (centralised today, decentralised via the
//!   Facilitator Blueprint tomorrow) handles both ingresses.

use std::sync::Arc;

use alloy_primitives::Address;
use base64::Engine;
use mpp::protocol::core::{PaymentCredential, Receipt};
use mpp::protocol::intents::ChargeRequest;
use mpp::protocol::traits::{ChargeMethod, ErrorCode, VerificationError};
use x402_axum::facilitator_client::FacilitatorClient;
use x402_chain_eip155::v1_eip155_exact::types::{
    ExactScheme, PaymentPayload as ExactPaymentPayload, PaymentRequirements as ExactRequirements,
    PaymentRequirementsExtra, VerifyRequest as ExactVerifyRequest,
};
use x402_types::chain::ChainId;
use x402_types::facilitator::Facilitator;
use x402_types::proto;
use x402_types::proto::v1 as proto_v1;

use crate::config::AcceptedToken;
use crate::mpp::credential::{MppCredentialPayload, MppMethodDetails};
use crate::mpp::state::{PayerCacheEntry, VerifiedPayerCache};

/// MPP method name for the Blueprint EVM/EIP-3009 bridge.
///
/// The MPP spec ABNF for `method-name` is `1*LOWERALPHA` — lowercase
/// ASCII letters only, no digits, no hyphens — so the obvious-looking
/// `"x402-evm"` is rejected by the upstream `MethodName::is_valid()`
/// check and would cause MPP-conformant clients (and our own
/// `parse_www_authenticate` round-trip) to reject every challenge we
/// emit. We use `"blueprintevm"` instead.
///
/// Clients setting up an MPP wallet against a Blueprint MPP gateway
/// should register this method name. The credential payload format is
/// documented in [`MppCredentialPayload`].
pub const METHOD_NAME: &str = "blueprintevm";

/// Default `maxTimeoutSeconds` advertised in PaymentRequirements when the
/// gateway has no per-job override. Matches `x402-chain-eip155`'s default.
const DEFAULT_MAX_TIMEOUT_SECS: u64 = 300;

/// MPP charge method that wraps an x402 v1 EIP-155 "exact" payload.
///
/// Verification flow:
/// 1. Pull `MppCredentialPayload.x402_payload` from the credential payload.
/// 2. base64url-decode and JSON-parse it as an x402 `PaymentPayload`.
/// 3. Pull `MppMethodDetails` from the request.
/// 4. Look up the matching [`AcceptedToken`] by network + asset.
/// 5. Build canonical `PaymentRequirements` from `(token, request)`.
/// 6. Forward to the facilitator's `/verify` endpoint.
/// 7. On `Valid`, stash the facilitator-reported payer in the
///    [`VerifiedPayerCache`] keyed by `credential.challenge.id`, and
///    forward to the facilitator's `/settle` endpoint.
/// 8. Translate the result into an `mpp::Receipt`.
///
/// # Safety
///
/// `BlueprintEvmChargeMethod::verify` is only safe to call from inside a
/// [`mpp::server::Mpp::verify_credential_with_expected_request`] wrapper
/// that pins `(amount, currency, recipient)` to a server-side expected
/// value derived from `(service_id, job_index, price_wei)`. Without that
/// wrapper an attacker can submit a credential whose echoed request claims
/// `amount = "1"` against a job whose true price is much higher. The MPP
/// route handler in [`crate::mpp::routes`] always uses the wrapped variant;
/// other call sites MUST do the same.
#[derive(Clone)]
pub(crate) struct BlueprintEvmChargeMethod {
    facilitator: Arc<FacilitatorClient>,
    accepted_tokens: Arc<Vec<AcceptedToken>>,
    /// Side-channel for the facilitator-reported payer. Populated in
    /// `verify` on `VerifyResponse::Valid` and drained by the MPP route
    /// handler immediately afterwards. See [`VerifiedPayerCache`] for the
    /// rationale.
    verified_payers: VerifiedPayerCache,
}

impl BlueprintEvmChargeMethod {
    /// Build a charge method that talks to the supplied facilitator and
    /// recognises the supplied set of accepted tokens. The `verified_payers`
    /// cache is the same `Arc<DashMap<...>>` held by [`MppGatewayState`];
    /// the route handler reads from it after each `verify` to enforce
    /// `payer_is_caller` policies.
    pub(crate) fn new(
        facilitator: FacilitatorClient,
        accepted_tokens: Vec<AcceptedToken>,
        verified_payers: VerifiedPayerCache,
    ) -> Self {
        Self {
            facilitator: Arc::new(facilitator),
            accepted_tokens: Arc::new(accepted_tokens),
            verified_payers,
        }
    }

    /// Number of accepted tokens this method recognises. Used by tests.
    #[cfg(test)]
    pub(crate) fn accepted_token_count(&self) -> usize {
        self.accepted_tokens.len()
    }
}

impl ChargeMethod for BlueprintEvmChargeMethod {
    fn method(&self) -> &str {
        METHOD_NAME
    }

    fn verify(
        &self,
        credential: &PaymentCredential,
        request: &ChargeRequest,
    ) -> impl std::future::Future<Output = Result<Receipt, VerificationError>> + Send {
        let cred_payload: Result<MppCredentialPayload, VerificationError> = credential
            .payload_as::<MppCredentialPayload>()
            .map_err(|e| {
                VerificationError::with_code(
                    format!("invalid blueprintevm credential payload: {e}"),
                    ErrorCode::InvalidPayload,
                )
            });
        let method_details: Result<MppMethodDetails, VerificationError> = request
            .method_details
            .clone()
            .ok_or_else(|| {
                VerificationError::with_code(
                    "ChargeRequest.methodDetails is required for the blueprintevm method",
                    ErrorCode::InvalidPayload,
                )
            })
            .and_then(|v| {
                serde_json::from_value::<MppMethodDetails>(v).map_err(|e| {
                    VerificationError::with_code(
                        format!("invalid blueprintevm methodDetails: {e}"),
                        ErrorCode::InvalidPayload,
                    )
                })
            });
        let amount_str = request.amount.clone();
        let currency = request.currency.clone();
        let recipient = request.recipient.clone();
        let challenge_id = credential.challenge.id.clone();
        let facilitator = self.facilitator.clone();
        let tokens = self.accepted_tokens.clone();
        let verified_payers = self.verified_payers.clone();

        async move {
            let cred_payload = cred_payload?;
            let method_details = method_details?;

            // Look up the accepted token to recover EIP-712 domain extras and
            // the canonical pay-to address. The MPP `recipient` field is
            // expected to match `accepted_token.pay_to` — if it doesn't the
            // facilitator would reject anyway, but we surface a clearer error.
            let token = tokens
                .iter()
                .find(|t| {
                    t.network == method_details.network && t.asset.eq_ignore_ascii_case(&currency)
                })
                .ok_or_else(|| {
                    VerificationError::with_code(
                        format!(
                            "no accepted token matches network={} asset={}",
                            method_details.network, currency
                        ),
                        ErrorCode::InvalidRecipient,
                    )
                })?;

            let recipient_str = recipient.ok_or_else(|| {
                VerificationError::with_code(
                    "ChargeRequest.recipient is required",
                    ErrorCode::InvalidRecipient,
                )
            })?;
            if !recipient_str.eq_ignore_ascii_case(&token.pay_to) {
                return Err(VerificationError::with_code(
                    format!(
                        "recipient {recipient_str} does not match operator pay_to {} for {}",
                        token.pay_to, token.symbol
                    ),
                    ErrorCode::InvalidRecipient,
                ));
            }

            // Decode the embedded x402 payload.
            let payload_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(cred_payload.x402_payload.as_bytes())
                .or_else(|_| {
                    base64::engine::general_purpose::URL_SAFE
                        .decode(cred_payload.x402_payload.as_bytes())
                })
                .map_err(|e| {
                    VerificationError::with_code(
                        format!("x402_payload is not valid base64url: {e}"),
                        ErrorCode::InvalidPayload,
                    )
                })?;

            let exact_payload: ExactPaymentPayload = serde_json::from_slice(&payload_bytes)
                .map_err(|e| {
                    VerificationError::with_code(
                        format!("x402_payload is not a valid v1 PaymentPayload: {e}"),
                        ErrorCode::InvalidPayload,
                    )
                })?;

            // Parse asset + pay_to as Address.
            let asset_addr = token.asset.parse::<Address>().map_err(|_| {
                VerificationError::with_code(
                    format!("invalid asset address {}", token.asset),
                    ErrorCode::InvalidPayload,
                )
            })?;
            let pay_to_addr = token.pay_to.parse::<Address>().map_err(|_| {
                VerificationError::with_code(
                    format!("invalid pay_to address {}", token.pay_to),
                    ErrorCode::InvalidPayload,
                )
            })?;

            let amount_u256 = amount_str.parse::<alloy_primitives::U256>().map_err(|_| {
                VerificationError::with_code(
                    format!("invalid base-unit amount {amount_str}"),
                    ErrorCode::InvalidAmount,
                )
            })?;

            let extra = if token.transfer_method == "eip3009" {
                Some(PaymentRequirementsExtra {
                    name: token
                        .eip3009_name
                        .clone()
                        .unwrap_or_else(|| "USD Coin".into()),
                    version: token.eip3009_version.clone().unwrap_or_else(|| "2".into()),
                })
            } else {
                None
            };

            // x402 v1 PaymentRequirements expects the chain's wire-format
            // network name (e.g. "base"), not the CAIP-2 form. Parse our
            // CAIP-2 string into a ChainId via its `FromStr` impl, then ask
            // x402-types for the registered network name.
            let chain: ChainId = token.network.parse().map_err(|_| {
                VerificationError::with_code(
                    format!("invalid CAIP-2 network {}", token.network),
                    ErrorCode::ChainIdMismatch,
                )
            })?;
            let network_wire = chain.as_network_name().map(str::to_string).ok_or_else(|| {
                VerificationError::with_code(
                    format!(
                        "no x402-types network name registered for {} (chain id {})",
                        token.network, chain.reference
                    ),
                    ErrorCode::ChainIdMismatch,
                )
            })?;

            let resource_url = format!(
                "blueprint://x402/jobs/{}/{}",
                method_details.service_id, method_details.job_index
            );

            let requirements = ExactRequirements {
                scheme: ExactScheme,
                network: network_wire,
                max_amount_required: amount_u256,
                resource: resource_url,
                description: format!(
                    "Tangle blueprint job service_id={} job_index={}",
                    method_details.service_id, method_details.job_index
                ),
                mime_type: None,
                output_schema: None,
                pay_to: pay_to_addr,
                max_timeout_seconds: DEFAULT_MAX_TIMEOUT_SECS,
                asset: asset_addr,
                extra,
            };

            let typed_verify = ExactVerifyRequest {
                x402_version: proto_v1::X402Version1,
                payment_payload: exact_payload.clone(),
                payment_requirements: requirements.clone(),
            };

            let proto_verify: proto::VerifyRequest =
                typed_verify.try_into().map_err(|e: serde_json::Error| {
                    VerificationError::with_code(
                        format!("failed to encode x402 VerifyRequest: {e}"),
                        ErrorCode::InvalidPayload,
                    )
                })?;

            let verify_resp_proto = facilitator
                .verify(&proto_verify)
                .await
                .map_err(|e| VerificationError::network_error(e.to_string()))?;
            let verify_resp: proto_v1::VerifyResponse = serde_json::from_value(verify_resp_proto.0)
                .map_err(|e| {
                    VerificationError::with_code(
                        format!("failed to decode VerifyResponse: {e}"),
                        ErrorCode::InvalidPayload,
                    )
                })?;

            // Hold the verified payer in a local — we only stash it in the
            // shared `verified_payers` cache once `/settle` confirms the
            // payment is on-chain. This is the only ordering that prevents
            // a leak when any intermediate operation (settle encode, settle
            // network, settle decode) fails between `/verify` and `/settle`.
            let verified_payer: Address = match verify_resp {
                proto_v1::VerifyResponse::Valid { payer } => {
                    payer.parse::<Address>().map_err(|_| {
                        VerificationError::with_code(
                            format!("facilitator returned non-address payer {payer}"),
                            ErrorCode::InvalidPayload,
                        )
                    })?
                }
                proto_v1::VerifyResponse::Invalid { reason, .. } => {
                    return Err(VerificationError::with_code(
                        format!("facilitator rejected payment: {reason}"),
                        ErrorCode::InvalidSignature,
                    ));
                }
            };

            // Settle uses the same shape as VerifyRequest.
            let typed_settle = ExactVerifyRequest {
                x402_version: proto_v1::X402Version1,
                payment_payload: exact_payload,
                payment_requirements: requirements,
            };
            let proto_settle: proto::SettleRequest =
                typed_settle.try_into().map_err(|e: serde_json::Error| {
                    VerificationError::with_code(
                        format!("failed to encode x402 SettleRequest: {e}"),
                        ErrorCode::InvalidPayload,
                    )
                })?;

            let settle_resp_proto = facilitator
                .settle(&proto_settle)
                .await
                .map_err(|e| VerificationError::network_error(e.to_string()))?;
            let settle_resp: proto_v1::SettleResponse = serde_json::from_value(settle_resp_proto.0)
                .map_err(|e| {
                    VerificationError::with_code(
                        format!("failed to decode SettleResponse: {e}"),
                        ErrorCode::InvalidPayload,
                    )
                })?;

            match settle_resp {
                proto_v1::SettleResponse::Success { transaction, .. } => {
                    // Only NOW do we stash the payer for the route handler.
                    // Insert-on-success means there is exactly one path that
                    // ever populates the cache, and the route handler always
                    // drains in the same request lifecycle (or the request
                    // panics, in which case the next sweep collects it — see
                    // `MppGatewayState`'s GC task).
                    verified_payers.insert(challenge_id, PayerCacheEntry::new(verified_payer));
                    Ok(Receipt::success(METHOD_NAME, transaction))
                }
                proto_v1::SettleResponse::Error { reason, .. } => {
                    Err(VerificationError::transaction_failed(reason))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use x402_axum::facilitator_client::FacilitatorClient;

    fn base_usdc() -> AcceptedToken {
        AcceptedToken {
            network: "eip155:8453".into(),
            asset: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".into(),
            symbol: "USDC".into(),
            decimals: 6,
            pay_to: "0x0000000000000000000000000000000000000001".into(),
            rate_per_native_unit: Decimal::from(3200u32),
            markup_bps: 0,
            transfer_method: "eip3009".into(),
            eip3009_name: Some("USD Coin".into()),
            eip3009_version: Some("2".into()),
        }
    }

    fn dummy_method() -> BlueprintEvmChargeMethod {
        let facilitator =
            FacilitatorClient::try_new("https://facilitator.x402.rs/".parse().unwrap())
                .expect("dummy facilitator");
        BlueprintEvmChargeMethod::new(
            facilitator,
            vec![base_usdc()],
            std::sync::Arc::new(dashmap::DashMap::new()),
        )
    }

    #[test]
    fn method_name_is_blueprintevm() {
        assert_eq!(dummy_method().method(), METHOD_NAME);
        assert_eq!(METHOD_NAME, "blueprintevm");
    }

    #[test]
    fn method_name_passes_mpp_abnf_validation() {
        // The upstream `mpp::MethodName::is_valid` ABNF is `1*LOWERALPHA`.
        // If we ever regress and add a digit/hyphen to METHOD_NAME, the
        // upstream parser will reject our own challenges and break the
        // wire format. Pin the invariant explicitly.
        let parsed = mpp::protocol::core::MethodName::from(METHOD_NAME);
        assert!(
            parsed.is_valid(),
            "METHOD_NAME must satisfy mpp's MethodName::is_valid (1*LOWERALPHA)"
        );
    }

    #[test]
    fn carries_configured_tokens() {
        let m = dummy_method();
        assert_eq!(m.accepted_token_count(), 1);
    }
}
