//! Configuration types for the x402 payment gateway.
//!
//! Operators configure which tokens they accept, on which chains, and how
//! job prices (denominated in wei) are converted into stablecoin amounts.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::Path;
use url::Url;

use crate::error::X402Error;

/// Top-level x402 gateway configuration.
///
/// Loaded from TOML — see [`X402Config::from_toml`].
///
/// ```toml
/// bind_address = "0.0.0.0:8402"
/// facilitator_url = "https://facilitator.x402.rs"
/// quote_ttl_secs = 300
///
/// [[accepted_tokens]]
/// network = "eip155:8453"
/// asset = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"
/// symbol = "USDC"
/// decimals = 6
/// pay_to = "0xYourOperatorAddressOnBase"
/// rate_per_native_unit = "3200.00"
/// markup_bps = 200
///
/// [job_overrides.direct]
/// job_indices = [0, 1, 2]
///
/// [job_overrides.relay]
/// job_indices = [6, 7]
/// gas_budget_wei = "500000000000000"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X402Config {
    /// Socket address to bind the HTTP server to.
    #[serde(default = "default_bind_address")]
    pub bind_address: SocketAddr,

    /// URL of the x402 facilitator for payment verification and settlement.
    pub facilitator_url: Url,

    /// How long (seconds) a dynamically-generated quote remains valid.
    #[serde(default = "default_quote_ttl")]
    pub quote_ttl_secs: u64,

    /// Tokens the operator accepts for x402 settlement.
    #[serde(default)]
    pub accepted_tokens: Vec<AcceptedToken>,

    /// Per-job execution mode overrides.
    /// If a job index is not listed in any override, it defaults to [`ExecutionMode::Direct`].
    #[serde(default)]
    pub job_overrides: JobOverrides,

    /// The service ID this gateway serves (set at runtime, not from TOML).
    #[serde(default)]
    pub service_id: u64,
}

/// A token the operator accepts for x402 payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptedToken {
    /// CAIP-2 network identifier, e.g. `"eip155:8453"` for Base.
    pub network: String,

    /// Token contract address on the EVM chain.
    pub asset: String,

    /// Human-readable symbol, e.g. `"USDC"`.
    pub symbol: String,

    /// Token decimals (6 for USDC, 18 for DAI, etc.).
    pub decimals: u8,

    /// Operator's receive address on this chain.
    pub pay_to: String,

    /// Exchange rate: how many token units equal 1 native unit (e.g. 1 ETH = 3200 USDC).
    /// Used to convert wei-denominated job prices into token amounts.
    pub rate_per_native_unit: Decimal,

    /// Markup in basis points (1 bps = 0.01%). Applied on top of the converted price
    /// to cover cross-chain settlement risk and operator margin.
    #[serde(default)]
    pub markup_bps: u16,
}

/// Per-job execution mode configuration (reserved for future use).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobOverrides {
    /// Jobs that execute directly (no gas, HTTP response).
    #[serde(default)]
    pub direct: Option<ModeConfig>,
}

/// Configuration for a specific execution mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeConfig {
    /// Which job indices use this mode.
    pub job_indices: Vec<u32>,
}

impl X402Config {
    /// Load configuration from a TOML file.
    pub fn from_toml(path: impl AsRef<Path>) -> Result<Self, X402Error> {
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| X402Error::Config(format!("failed to read config file: {e}")))?;
        toml::from_str(&content).map_err(X402Error::from)
    }

    /// Check whether a job index is configured for direct execution.
    ///
    /// All jobs default to direct execution. This method exists for
    /// future expansion when additional execution modes are supported.
    pub fn is_direct_execution(&self, _job_index: u32) -> bool {
        true
    }

    /// Build a lookup from job_index → list of accepted tokens.
    /// All jobs share the same accepted token list; this is a convenience
    /// for the gateway to quickly build x402 price tags.
    pub fn accepted_tokens_map(&self) -> &[AcceptedToken] {
        &self.accepted_tokens
    }

    /// Convert a price in wei to a token amount string, applying the exchange rate
    /// and markup for the given token.
    pub fn convert_wei_to_token(
        &self,
        wei_price: &alloy_primitives::U256,
        token: &AcceptedToken,
    ) -> Result<String, X402Error> {
        // Convert U256 wei to Decimal
        let wei_decimal = Decimal::from_str_exact(&wei_price.to_string())
            .map_err(|e| X402Error::PriceConversion(format!("wei to decimal: {e}")))?;

        // 1 ETH = 10^18 wei. Convert wei to native units.
        let native_unit = Decimal::from(10u64.pow(18));
        let native_amount = wei_decimal / native_unit;

        // Apply exchange rate: native_amount * rate = token amount in whole units
        let token_amount = native_amount * token.rate_per_native_unit;

        // Apply markup
        let markup_multiplier =
            Decimal::ONE + Decimal::from(token.markup_bps) / Decimal::from(10_000u32);
        let final_amount = token_amount * markup_multiplier;

        // Convert to smallest token units (e.g. 6 decimals for USDC)
        let token_unit = Decimal::from(10u64.pow(u32::from(token.decimals)));
        let smallest_units = (final_amount * token_unit).floor().to_string();

        Ok(smallest_units)
    }
}

fn default_bind_address() -> SocketAddr {
    "0.0.0.0:8402".parse().expect("valid default address")
}

fn default_quote_ttl() -> u64 {
    300
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wei_to_usdc_conversion() {
        let config = X402Config {
            bind_address: default_bind_address(),
            facilitator_url: "https://example.com".parse().unwrap(),
            quote_ttl_secs: 300,
            accepted_tokens: vec![],
            job_overrides: JobOverrides::default(),
            service_id: 0,
        };

        let token = AcceptedToken {
            network: "eip155:8453".into(),
            asset: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".into(),
            symbol: "USDC".into(),
            decimals: 6,
            pay_to: "0x0000000000000000000000000000000000000001".into(),
            rate_per_native_unit: Decimal::from(3200u32), // 1 ETH = 3200 USDC
            markup_bps: 0,
        };

        // 0.001 ETH = 1_000_000_000_000_000 wei
        let wei = alloy_primitives::U256::from(1_000_000_000_000_000u64);
        let result = config.convert_wei_to_token(&wei, &token).unwrap();
        // 0.001 ETH * 3200 USDC/ETH = 3.2 USDC = 3_200_000 smallest units
        assert_eq!(result, "3200000");
    }

    #[test]
    fn test_wei_to_usdc_with_markup() {
        let config = X402Config {
            bind_address: default_bind_address(),
            facilitator_url: "https://example.com".parse().unwrap(),
            quote_ttl_secs: 300,
            accepted_tokens: vec![],
            job_overrides: JobOverrides::default(),
            service_id: 0,
        };

        let token = AcceptedToken {
            network: "eip155:8453".into(),
            asset: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".into(),
            symbol: "USDC".into(),
            decimals: 6,
            pay_to: "0x0000000000000000000000000000000000000001".into(),
            rate_per_native_unit: Decimal::from(3200u32),
            markup_bps: 200, // 2% markup
        };

        let wei = alloy_primitives::U256::from(1_000_000_000_000_000u64);
        let result = config.convert_wei_to_token(&wei, &token).unwrap();
        // 3.2 USDC * 1.02 = 3.264 USDC = 3_264_000 smallest units
        assert_eq!(result, "3264000");
    }

    #[test]
    fn test_all_jobs_default_to_direct() {
        let config = X402Config {
            bind_address: default_bind_address(),
            facilitator_url: "https://example.com".parse().unwrap(),
            quote_ttl_secs: 300,
            accepted_tokens: vec![],
            job_overrides: JobOverrides::default(),
            service_id: 0,
        };
        assert!(config.is_direct_execution(0));
        assert!(config.is_direct_execution(99));
    }
}
