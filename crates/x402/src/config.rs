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
/// Loaded from TOML -- see [`X402Config::from_toml`].
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

    /// Transfer method for the token. Determines how the x402 facilitator
    /// moves funds: `"permit2"` (default) or `"eip3009"`.
    ///
    /// For USDC on most chains, use `"eip3009"` with `eip3009_name = "USD Coin"`
    /// and `eip3009_version = "2"`. For other tokens, `"permit2"` is universal.
    #[serde(default = "default_transfer_method")]
    pub transfer_method: String,

    /// EIP-3009 domain name (required when `transfer_method = "eip3009"`).
    #[serde(default)]
    pub eip3009_name: Option<String>,

    /// EIP-3009 domain version (required when `transfer_method = "eip3009"`).
    #[serde(default)]
    pub eip3009_version: Option<String>,
}

fn default_transfer_method() -> String {
    "permit2".into()
}

impl AcceptedToken {
    /// Convert a price in wei to a token amount string in the token's smallest unit.
    ///
    /// Applies the exchange rate and markup. Returns the amount as a decimal
    /// string suitable for x402 price tags and settlement options.
    pub fn convert_wei_to_amount(
        &self,
        wei_price: &alloy_primitives::U256,
    ) -> Result<String, X402Error> {
        let wei_decimal = Decimal::from_str_exact(&wei_price.to_string())
            .map_err(|e| X402Error::PriceConversion(format!("wei to decimal: {e}")))?;

        // 1 ETH = 10^18 wei. Convert wei to native units.
        let native_unit = Decimal::from(10u64.pow(18));
        let native_amount = wei_decimal / native_unit;

        // Apply exchange rate: native_amount * rate = token amount in whole units
        let token_amount = native_amount * self.rate_per_native_unit;

        // Apply markup
        let markup_multiplier =
            Decimal::ONE + Decimal::from(self.markup_bps) / Decimal::from(10_000u32);
        let final_amount = token_amount * markup_multiplier;

        // Convert to smallest token units (e.g. 6 decimals for USDC)
        let token_unit = Decimal::from(10u64.pow(u32::from(self.decimals)));
        let smallest_units = (final_amount * token_unit).floor().to_string();

        Ok(smallest_units)
    }

    /// Validate token configuration at load time.
    ///
    /// Checks that the network is a valid CAIP-2 EVM identifier, addresses
    /// parse as valid EVM addresses, and the exchange rate is positive.
    pub fn validate(&self) -> Result<(), X402Error> {
        // Network must be eip155:<chain_id>
        let chain_str = self.network.strip_prefix("eip155:").ok_or_else(|| {
            X402Error::Config(format!(
                "token {}: network must start with 'eip155:', got '{}'",
                self.symbol, self.network
            ))
        })?;
        chain_str.parse::<u64>().map_err(|_| {
            X402Error::Config(format!(
                "token {}: invalid chain ID in network '{}': '{chain_str}' is not a valid u64",
                self.symbol, self.network
            ))
        })?;

        // Asset address must parse as a valid EVM address
        self.asset
            .parse::<alloy_primitives::Address>()
            .map_err(|_| {
                X402Error::Config(format!(
                    "token {} on {}: invalid asset address '{}'",
                    self.symbol, self.network, self.asset
                ))
            })?;

        // Pay-to address must parse as a valid EVM address
        self.pay_to
            .parse::<alloy_primitives::Address>()
            .map_err(|_| {
                X402Error::Config(format!(
                    "token {} on {}: invalid pay_to address '{}'",
                    self.symbol, self.network, self.pay_to
                ))
            })?;

        // Exchange rate must be positive
        if self.rate_per_native_unit <= Decimal::ZERO {
            return Err(X402Error::Config(format!(
                "token {} on {}: rate_per_native_unit must be positive, got {}",
                self.symbol, self.network, self.rate_per_native_unit
            )));
        }

        Ok(())
    }
}

impl X402Config {
    /// Load configuration from a TOML file.
    pub fn from_toml(path: impl AsRef<Path>) -> Result<Self, X402Error> {
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| X402Error::Config(format!("failed to read config file: {e}")))?;
        let config: Self = toml::from_str(&content).map_err(X402Error::from)?;
        config.validate()?;
        Ok(config)
    }

    /// Validate the full configuration.
    pub fn validate(&self) -> Result<(), X402Error> {
        for token in &self.accepted_tokens {
            token.validate()?;
        }
        Ok(())
    }

    /// Build a lookup from job_index to list of accepted tokens.
    /// All jobs share the same accepted token list; this is a convenience
    /// for the gateway to quickly build x402 price tags.
    pub fn accepted_tokens_map(&self) -> &[AcceptedToken] {
        &self.accepted_tokens
    }

    /// Convert a price in wei to a token amount string, applying the exchange rate
    /// and markup for the given token.
    ///
    /// Delegates to [`AcceptedToken::convert_wei_to_amount`].
    pub fn convert_wei_to_token(
        &self,
        wei_price: &alloy_primitives::U256,
        token: &AcceptedToken,
    ) -> Result<String, X402Error> {
        token.convert_wei_to_amount(wei_price)
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
    use alloy_primitives::U256;

    fn usdc_token(markup_bps: u16) -> AcceptedToken {
        AcceptedToken {
            network: "eip155:8453".into(),
            asset: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".into(),
            symbol: "USDC".into(),
            decimals: 6,
            pay_to: "0x0000000000000000000000000000000000000001".into(),
            rate_per_native_unit: Decimal::from(3200u32),
            markup_bps,
            transfer_method: "eip3009".into(),
            eip3009_name: Some("USD Coin".into()),
            eip3009_version: Some("2".into()),
        }
    }

    fn dai_token(markup_bps: u16) -> AcceptedToken {
        AcceptedToken {
            network: "eip155:1".into(),
            asset: "0x6B175474E89094C44Da98b954EedeAC495271d0F".into(),
            symbol: "DAI".into(),
            decimals: 18,
            pay_to: "0x0000000000000000000000000000000000000002".into(),
            rate_per_native_unit: Decimal::from(3200u32),
            markup_bps,
            transfer_method: default_transfer_method(),
            eip3009_name: None,
            eip3009_version: None,
        }
    }

    // ---- Basic conversion tests ----

    #[test]
    fn test_wei_to_usdc_conversion() {
        let token = usdc_token(0);
        // 0.001 ETH = 1_000_000_000_000_000 wei
        let wei = U256::from(1_000_000_000_000_000u64);
        let result = token.convert_wei_to_amount(&wei).unwrap();
        // 0.001 ETH * 3200 USDC/ETH = 3.2 USDC = 3_200_000 smallest units
        assert_eq!(result, "3200000");
    }

    #[test]
    fn test_wei_to_usdc_with_markup() {
        let token = usdc_token(200); // 2% markup
        let wei = U256::from(1_000_000_000_000_000u64);
        let result = token.convert_wei_to_amount(&wei).unwrap();
        // 3.2 USDC * 1.02 = 3.264 USDC = 3_264_000 smallest units
        assert_eq!(result, "3264000");
    }

    // ---- Edge case tests ----

    #[test]
    fn test_zero_wei_returns_zero() {
        let token = usdc_token(200);
        let result = token.convert_wei_to_amount(&U256::ZERO).unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_one_wei_rounds_to_zero_for_usdc() {
        // 1 wei is vanishingly small: 1e-18 ETH * 3200 = 3.2e-15 USDC
        // In 6-decimal smallest units: 3.2e-9, which floors to 0.
        let token = usdc_token(0);
        let result = token.convert_wei_to_amount(&U256::from(1u64)).unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_18_decimal_token_large_amount() {
        // DAI has 18 decimals. 1 ETH = 3200 DAI.
        // 10 ETH = 10_000_000_000_000_000_000 wei = 32000 DAI
        // In 18-decimal units: 32000 * 10^18 = 32000000000000000000000
        let token = dai_token(0);
        let wei = U256::from(10u64) * U256::from(10u64).pow(U256::from(18u64));
        let result = token.convert_wei_to_amount(&wei).unwrap();
        assert_eq!(result, "32000000000000000000000");
    }

    #[test]
    fn test_18_decimal_token_exceeds_u64() {
        // Verify amounts that would overflow u64 are handled correctly.
        // u64::MAX = 18_446_744_073_709_551_615
        // 100 ETH * 3200 DAI/ETH = 320000 DAI
        // In 18-decimal units: 320000 * 10^18 = 320000000000000000000000 (> u64::MAX)
        let token = dai_token(0);
        let wei = U256::from(100u64) * U256::from(10u64).pow(U256::from(18u64));
        let result = token.convert_wei_to_amount(&wei).unwrap();
        assert_eq!(result, "320000000000000000000000");
        // Verify this actually exceeds u64
        assert!(
            result.parse::<u128>().unwrap() > u64::MAX as u128,
            "this amount must exceed u64::MAX to validate the overflow fix"
        );
    }

    #[test]
    fn test_full_markup_100_percent() {
        let mut token = usdc_token(0);
        token.markup_bps = 10_000; // 100% markup
        let wei = U256::from(1_000_000_000_000_000u64); // 0.001 ETH
        let result = token.convert_wei_to_amount(&wei).unwrap();
        // 3.2 USDC * 2.0 = 6.4 USDC = 6_400_000
        assert_eq!(result, "6400000");
    }

    #[test]
    fn test_unit_exchange_rate() {
        let mut token = usdc_token(0);
        token.rate_per_native_unit = Decimal::ONE;
        let wei = U256::from(10u64).pow(U256::from(18u64)); // 1 ETH
        let result = token.convert_wei_to_amount(&wei).unwrap();
        // 1 ETH * 1 USDC/ETH = 1 USDC = 1_000_000
        assert_eq!(result, "1000000");
    }

    // ---- Validation tests ----

    #[test]
    fn test_validate_good_token() {
        let token = usdc_token(200);
        assert!(token.validate().is_ok());
    }

    #[test]
    fn test_validate_bad_network_prefix() {
        let mut token = usdc_token(0);
        token.network = "solana:mainnet".into();
        let err = token.validate().unwrap_err();
        assert!(err.to_string().contains("eip155:"), "{err}");
    }

    #[test]
    fn test_validate_bad_chain_id() {
        let mut token = usdc_token(0);
        token.network = "eip155:not_a_number".into();
        let err = token.validate().unwrap_err();
        assert!(err.to_string().contains("invalid chain ID"), "{err}");
    }

    #[test]
    fn test_validate_bad_asset_address() {
        let mut token = usdc_token(0);
        token.asset = "not_an_address".into();
        let err = token.validate().unwrap_err();
        assert!(err.to_string().contains("invalid asset address"), "{err}");
    }

    #[test]
    fn test_validate_bad_pay_to_address() {
        let mut token = usdc_token(0);
        token.pay_to = "0xZZZ".into();
        let err = token.validate().unwrap_err();
        assert!(err.to_string().contains("invalid pay_to address"), "{err}");
    }

    #[test]
    fn test_validate_zero_exchange_rate() {
        let mut token = usdc_token(0);
        token.rate_per_native_unit = Decimal::ZERO;
        let err = token.validate().unwrap_err();
        assert!(err.to_string().contains("positive"), "{err}");
    }

    #[test]
    fn test_validate_negative_exchange_rate() {
        let mut token = usdc_token(0);
        token.rate_per_native_unit = Decimal::from(-1i32);
        let err = token.validate().unwrap_err();
        assert!(err.to_string().contains("positive"), "{err}");
    }

    // ---- Config-level conversion (backwards compat) ----

    #[test]
    fn test_config_convert_delegates_to_token() {
        let token = usdc_token(0);
        let config = X402Config {
            bind_address: default_bind_address(),
            facilitator_url: "https://example.com".parse().unwrap(),
            quote_ttl_secs: 300,
            accepted_tokens: vec![token.clone()],

            service_id: 0,
        };
        let wei = U256::from(1_000_000_000_000_000u64);
        let from_config = config.convert_wei_to_token(&wei, &token).unwrap();
        let from_token = token.convert_wei_to_amount(&wei).unwrap();
        assert_eq!(from_config, from_token);
    }
}
