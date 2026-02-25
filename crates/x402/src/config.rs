//! Configuration types for the x402 payment gateway.
//!
//! Operators configure which tokens they accept, on which chains, and how
//! job prices (denominated in wei) are converted into stablecoin amounts.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::net::SocketAddr;
use std::path::Path;
use url::Url;

use crate::error::X402Error;

/// How a job is exposed to the x402 HTTP ingress.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum X402InvocationMode {
    /// Job cannot be invoked via x402.
    #[default]
    Disabled,
    /// Job is payment-gated but otherwise public.
    PublicPaid,
    /// Job is payment-gated and must pass restricted caller policy.
    RestrictedPaid,
}

/// How caller identity is sourced for restricted x402 jobs.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum X402CallerAuthMode {
    /// Caller is inferred from the settled payment payer.
    #[default]
    PayerIsCaller,
    /// Caller is asserted by headers and must include a valid signature.
    DelegatedCallerSignature,
    /// No caller identity check (invalid for restricted mode).
    PaymentOnly,
}

/// Per-job x402 invocation policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobPolicyConfig {
    /// Tangle service id.
    pub service_id: u64,
    /// Job index.
    pub job_index: u32,
    /// Whether/How this job can be called via x402.
    #[serde(default)]
    pub invocation_mode: X402InvocationMode,
    /// Restricted-mode caller identity strategy.
    #[serde(default)]
    pub auth_mode: X402CallerAuthMode,
    /// RPC endpoint used for permission dry-run (`eth_call`) checks.
    #[serde(default)]
    pub tangle_rpc_url: Option<Url>,
    /// Tangle contract address used for `isPermittedCaller` checks.
    #[serde(default)]
    pub tangle_contract: Option<String>,
}

/// Top-level x402 gateway configuration.
///
/// Loaded from TOML -- see [`X402Config::from_toml`].
///
/// ```toml
/// bind_address = "0.0.0.0:8402"
/// facilitator_url = "https://facilitator.x402.rs"
/// quote_ttl_secs = 300
///
/// # Default job policy if no explicit per-job entry exists.
/// default_invocation_mode = "disabled"
///
/// [[job_policies]]
/// service_id = 1
/// job_index = 0
/// invocation_mode = "public_paid"
///
/// [[job_policies]]
/// service_id = 1
/// job_index = 1
/// invocation_mode = "restricted_paid"
/// auth_mode = "payer_is_caller"
/// tangle_rpc_url = "http://127.0.0.1:8545"
/// tangle_contract = "0x0000000000000000000000000000000000000001"
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

    /// Default invocation mode for jobs missing an explicit `job_policies` entry.
    #[serde(default)]
    pub default_invocation_mode: X402InvocationMode,

    /// Per-job invocation overrides.
    #[serde(default)]
    pub job_policies: Vec<JobPolicyConfig>,

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

        let mut seen = HashSet::new();
        for policy in &self.job_policies {
            if !seen.insert((policy.service_id, policy.job_index)) {
                return Err(X402Error::Config(format!(
                    "duplicate job policy for service_id={} job_index={}",
                    policy.service_id, policy.job_index
                )));
            }

            if policy.invocation_mode == X402InvocationMode::RestrictedPaid {
                if policy.auth_mode == X402CallerAuthMode::PaymentOnly {
                    return Err(X402Error::Config(format!(
                        "restricted_paid policy for service_id={} job_index={} cannot use auth_mode=payment_only",
                        policy.service_id, policy.job_index
                    )));
                }

                if policy.tangle_rpc_url.is_none() {
                    return Err(X402Error::Config(format!(
                        "restricted_paid policy for service_id={} job_index={} requires tangle_rpc_url",
                        policy.service_id, policy.job_index
                    )));
                }

                let Some(contract) = &policy.tangle_contract else {
                    return Err(X402Error::Config(format!(
                        "restricted_paid policy for service_id={} job_index={} requires tangle_contract",
                        policy.service_id, policy.job_index
                    )));
                };

                contract
                    .parse::<alloy_primitives::Address>()
                    .map_err(|_| {
                        X402Error::Config(format!(
                            "restricted_paid policy for service_id={} job_index={} has invalid tangle_contract='{}'",
                            policy.service_id, policy.job_index, contract
                        ))
                    })?;
            }
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

    #[test]
    fn test_wei_to_usdc_conversion() {
        let token = usdc_token(0);
        let wei = U256::from(1_000_000_000_000_000u64);
        let result = token.convert_wei_to_amount(&wei).unwrap();
        assert_eq!(result, "3200000");
    }

    #[test]
    fn test_wei_to_usdc_with_markup() {
        let token = usdc_token(200);
        let wei = U256::from(1_000_000_000_000_000u64);
        let result = token.convert_wei_to_amount(&wei).unwrap();
        assert_eq!(result, "3264000");
    }

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
    fn test_validate_zero_exchange_rate() {
        let mut token = usdc_token(0);
        token.rate_per_native_unit = Decimal::ZERO;
        let err = token.validate().unwrap_err();
        assert!(err.to_string().contains("positive"), "{err}");
    }

    #[test]
    fn test_restricted_policy_requires_rpc_and_contract() {
        let config = X402Config {
            bind_address: default_bind_address(),
            facilitator_url: "https://example.com".parse().unwrap(),
            quote_ttl_secs: 300,
            accepted_tokens: vec![usdc_token(0)],
            default_invocation_mode: X402InvocationMode::Disabled,
            job_policies: vec![JobPolicyConfig {
                service_id: 1,
                job_index: 0,
                invocation_mode: X402InvocationMode::RestrictedPaid,
                auth_mode: X402CallerAuthMode::PayerIsCaller,
                tangle_rpc_url: None,
                tangle_contract: None,
            }],
            service_id: 0,
        };

        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("requires tangle_rpc_url"), "{err}");
    }

    #[test]
    fn test_restricted_policy_rejects_payment_only_auth() {
        let config = X402Config {
            bind_address: default_bind_address(),
            facilitator_url: "https://example.com".parse().unwrap(),
            quote_ttl_secs: 300,
            accepted_tokens: vec![usdc_token(0)],
            default_invocation_mode: X402InvocationMode::Disabled,
            job_policies: vec![JobPolicyConfig {
                service_id: 1,
                job_index: 0,
                invocation_mode: X402InvocationMode::RestrictedPaid,
                auth_mode: X402CallerAuthMode::PaymentOnly,
                tangle_rpc_url: Some("http://127.0.0.1:8545".parse().unwrap()),
                tangle_contract: Some("0x0000000000000000000000000000000000000001".into()),
            }],
            service_id: 0,
        };

        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("payment_only"), "{err}");
    }

    #[test]
    fn test_duplicate_job_policy_rejected() {
        let policy = JobPolicyConfig {
            service_id: 1,
            job_index: 0,
            invocation_mode: X402InvocationMode::PublicPaid,
            auth_mode: X402CallerAuthMode::PayerIsCaller,
            tangle_rpc_url: None,
            tangle_contract: None,
        };

        let config = X402Config {
            bind_address: default_bind_address(),
            facilitator_url: "https://example.com".parse().unwrap(),
            quote_ttl_secs: 300,
            accepted_tokens: vec![usdc_token(0)],
            default_invocation_mode: X402InvocationMode::Disabled,
            job_policies: vec![policy.clone(), policy],
            service_id: 0,
        };

        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("duplicate job policy"), "{err}");
    }
}
