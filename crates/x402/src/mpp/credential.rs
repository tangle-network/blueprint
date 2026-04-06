//! Credential payload schema for the Blueprint MPP `x402-evm` method.
//!
//! The Blueprint MPP method ([`METHOD_NAME`](super::METHOD_NAME) = `"x402-evm"`)
//! intentionally wraps the same EIP-3009 / Permit2 `PaymentPayload` that x402
//! clients already produce. This means:
//!
//! - Existing x402 wallets work over the MPP wire format unchanged — they
//!   sign the same EIP-712 typed data, they just put the result in
//!   `Authorization: Payment` instead of `X-PAYMENT`.
//! - Verification delegates to the same x402 facilitator URL configured for
//!   the legacy ingress; no second facilitator service is needed.
//! - The cross-chain pricing math (`AcceptedToken::convert_wei_to_amount`)
//!   is reused without duplication.
//!
//! # Wire shape
//!
//! The MPP `PaymentCredential.payload` JSON for this method looks like:
//!
//! ```json
//! {
//!   "x402_payload": "<base64url(serialized x402 v1 PaymentPayload JSON)>"
//! }
//! ```
//!
//! And the MPP `PaymentChallenge.request` field (a base64url-encoded
//! `ChargeRequest`) carries the per-method context the verifier needs in
//! `methodDetails`:
//!
//! ```json
//! {
//!   "amount": "10000",
//!   "currency": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
//!   "recipient": "0xYourPayToOnBase",
//!   "methodDetails": {
//!     "network": "eip155:8453",
//!     "scheme": "exact",
//!     "transferMethod": "eip3009",
//!     "extra": { "name": "USD Coin", "version": "2" },
//!     "decimals": 6,
//!     "service_id": 1,
//!     "job_index": 0
//!   }
//! }
//! ```

use serde::{Deserialize, Serialize};

/// Method-specific extension fields embedded in the MPP `ChargeRequest.methodDetails`.
///
/// The Blueprint MPP server fills this in at challenge time so that clients
/// have all the context they need to build an x402-compatible
/// `transferWithAuthorization` (or `permit2`) signature.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MppMethodDetails {
    /// CAIP-2 network identifier, e.g. `"eip155:8453"` for Base.
    pub network: String,
    /// x402 scheme name. Today this is always `"exact"`.
    pub scheme: String,
    /// Transfer method: `"eip3009"` or `"permit2"`.
    #[serde(rename = "transferMethod")]
    pub transfer_method: String,
    /// EIP-3009 domain extras. Required when `transfer_method == "eip3009"`.
    /// Carries the token contract's EIP-712 `name` and `version`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Eip3009Extra>,
    /// Token decimals. Used by clients to format amounts.
    pub decimals: u8,
    /// Tangle service id this challenge is bound to.
    pub service_id: u64,
    /// Tangle job index this challenge is bound to.
    pub job_index: u32,
}

/// EIP-3009 EIP-712 domain extras carried in [`MppMethodDetails::extra`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Eip3009Extra {
    /// Token contract EIP-712 domain `name`.
    pub name: String,
    /// Token contract EIP-712 domain `version`.
    pub version: String,
}

/// MPP credential payload for the Blueprint `x402-evm` method.
///
/// The single `x402_payload` field carries a base64url-encoded x402 v1
/// `PaymentPayload` JSON, which the verifier hands directly to the
/// configured x402 facilitator's `/verify` and `/settle` endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MppCredentialPayload {
    /// Base64url-encoded x402 v1 `PaymentPayload` JSON. The verifier
    /// `base64url`-decodes this and parses it as
    /// `x402_types::proto::v1::PaymentPayload<x402_chain_eip155::ExactScheme,
    /// x402_chain_eip155::ExactPayload>`.
    pub x402_payload: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_details_round_trip_with_eip3009_extras() {
        let details = MppMethodDetails {
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
        };

        let json = serde_json::to_string(&details).unwrap();
        assert!(json.contains("\"transferMethod\":\"eip3009\""));
        assert!(json.contains("\"extra\""));
        let parsed: MppMethodDetails = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, details);
    }

    #[test]
    fn method_details_omits_extra_for_permit2() {
        let details = MppMethodDetails {
            network: "eip155:1".into(),
            scheme: "exact".into(),
            transfer_method: "permit2".into(),
            extra: None,
            decimals: 6,
            service_id: 7,
            job_index: 3,
        };

        let json = serde_json::to_string(&details).unwrap();
        assert!(!json.contains("\"extra\""));
    }

    #[test]
    fn credential_payload_round_trip() {
        let payload = MppCredentialPayload {
            x402_payload: "eyJ4NDAyVmVyc2lvbiI6MX0".into(),
        };
        let json = serde_json::to_string(&payload).unwrap();
        let parsed: MppCredentialPayload = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.x402_payload, payload.x402_payload);
    }
}
