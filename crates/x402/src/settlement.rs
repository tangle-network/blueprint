//! Settlement option types for cross-chain x402 payment.
//!
//! These are returned alongside RFQ quotes to advertise how a quote can be settled.

use serde::{Deserialize, Serialize};

/// A settlement option describing one way a client can pay for a job.
///
/// Included in RFQ responses so clients know which chains/tokens the operator accepts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementOption {
    /// CAIP-2 network identifier, e.g. `"eip155:8453"` for Base.
    pub network: String,

    /// Token identifier — contract address (EVM) or mint (Solana).
    pub asset: String,

    /// Human-readable symbol, e.g. `"USDC"`.
    pub symbol: String,

    /// Payment amount in the token's smallest unit (e.g. 6-decimal USDC).
    pub amount: String,

    /// Operator's receive address on this chain.
    pub pay_to: String,

    /// x402 payment scheme, e.g. `"exact"`.
    pub scheme: String,

    /// x402 endpoint URL where the client should send payment.
    pub x402_endpoint: String,
}
