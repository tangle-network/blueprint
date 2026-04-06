//! Settlement option types for cross-chain x402 / MPP payment.
//!
//! These are returned alongside RFQ quotes to advertise how a quote can be settled.

use serde::{Deserialize, Serialize};

/// Wire protocol used to deliver a payment to the gateway.
///
/// Both protocols accept the same EIP-3009 / Permit2 EVM payments and feed
/// the same downstream producer; they differ only in HTTP wire format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentProtocol {
    /// x402 wire format (X-PAYMENT / X-PAYMENT-RESPONSE headers).
    X402,
    /// MPP / IETF Payment authentication scheme
    /// (`WWW-Authenticate: Payment` / `Authorization: Payment` / `Payment-Receipt`).
    Mpp,
}

impl PaymentProtocol {
    /// Wire-friendly name for this protocol.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            PaymentProtocol::X402 => "x402",
            PaymentProtocol::Mpp => "mpp",
        }
    }
}

/// A settlement option describing one way a client can pay for a job.
///
/// Included in RFQ responses so clients know which chains/tokens the operator accepts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementOption {
    /// Wire protocol the client should use (x402 or MPP).
    #[serde(default = "default_protocol")]
    pub protocol: PaymentProtocol,

    /// CAIP-2 network identifier, e.g. `"eip155:8453"` for Base.
    pub network: String,

    /// Token contract address on the EVM chain.
    pub asset: String,

    /// Human-readable symbol, e.g. `"USDC"`.
    pub symbol: String,

    /// Payment amount in the token's smallest unit (e.g. 6-decimal USDC).
    pub amount: String,

    /// Operator's receive address on this chain.
    pub pay_to: String,

    /// Payment scheme. For x402 this is `"exact"`. For MPP this is the
    /// charge intent name (`"charge"`).
    pub scheme: String,

    /// Endpoint URL where the client should send payment. For x402 this is
    /// `/x402/jobs/{sid}/{idx}`; for MPP this is `/mpp/jobs/{sid}/{idx}`.
    pub x402_endpoint: String,
}

fn default_protocol() -> PaymentProtocol {
    PaymentProtocol::X402
}
