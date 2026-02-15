//! x402 Blueprint Example
//!
//! Demonstrates the full pipeline from TOML pricing config through the x402
//! HTTP gateway to job dispatch via the Blueprint router.
//!
//! Two jobs are provided:
//! - **echo** (job 0): returns the request body unchanged.
//! - **hash** (job 1): returns the keccak256 digest of the body.
//!
//! Pricing supports both static TOML-based prices and dynamic oracle-based
//! pricing via the [`PriceOracle`] trait. Exchange rate oracles (Chainlink,
//! Uniswap V3 TWAP, Coinbase API) are available in the [`oracle`] module
//! behind feature flags.

pub mod oracle;

use alloy_primitives::U256;
use blueprint_router::Router;
use bytes::Bytes;
use std::collections::HashMap;

// ---- Job IDs ----

pub const ECHO_JOB: u32 = 0;
pub const HASH_JOB: u32 = 1;

// ---- Job handlers ----

pub async fn echo(body: Bytes) -> Bytes {
    body
}

pub async fn hash(body: Bytes) -> Bytes {
    Bytes::copy_from_slice(alloy_primitives::keccak256(&body).as_slice())
}

// ---- Router ----

pub fn router() -> Router {
    Router::new().route(ECHO_JOB, echo).route(HASH_JOB, hash)
}

// ---- Price Oracle ----

/// A price oracle resolves the current price (in wei) for a given job.
///
/// Static TOML config works for fixed pricing. For production deployments that
/// need live exchange rates, surge pricing, or per-caller discounts, implement
/// this trait with your own oracle (Chainlink, Pyth, Redstone, or a custom
/// off-chain feed).
pub trait PriceOracle: Send + Sync {
    /// Look up the price in wei for (service_id, job_index).
    /// Returns `None` if the job is not priced (i.e. not offered via x402).
    fn price_wei(&self, service_id: u64, job_index: u32) -> Option<U256>;

    /// Snapshot the full pricing table as a `HashMap`.
    /// Used to seed the gateway at startup.
    fn snapshot(&self) -> HashMap<(u64, u32), U256>;
}

/// Static price oracle backed by a `HashMap`. Prices never change after init.
#[derive(Debug, Clone)]
pub struct StaticPriceOracle {
    prices: HashMap<(u64, u32), U256>,
}

impl StaticPriceOracle {
    pub fn new(prices: HashMap<(u64, u32), U256>) -> Self {
        Self { prices }
    }
}

impl PriceOracle for StaticPriceOracle {
    fn price_wei(&self, service_id: u64, job_index: u32) -> Option<U256> {
        self.prices.get(&(service_id, job_index)).copied()
    }

    fn snapshot(&self) -> HashMap<(u64, u32), U256> {
        self.prices.clone()
    }
}

/// Multiplier-based oracle that wraps a base oracle and applies a dynamic
/// factor. Useful for surge pricing or exchange-rate adjustments.
///
/// `effective_price = base_price * numerator / denominator`
#[derive(Debug, Clone)]
pub struct ScaledPriceOracle<O> {
    inner: O,
    numerator: U256,
    denominator: U256,
}

impl<O> ScaledPriceOracle<O> {
    /// Create a scaled oracle. The factor is `numerator / denominator`.
    /// For example, (3, 2) applies a 1.5x multiplier.
    ///
    /// Returns an error if `denominator` is zero.
    pub fn new(inner: O, numerator: U256, denominator: U256) -> Result<Self, &'static str> {
        if denominator.is_zero() {
            return Err("denominator must be non-zero");
        }
        Ok(Self {
            inner,
            numerator,
            denominator,
        })
    }
}

impl<O: PriceOracle> PriceOracle for ScaledPriceOracle<O> {
    fn price_wei(&self, service_id: u64, job_index: u32) -> Option<U256> {
        let base = self.inner.price_wei(service_id, job_index)?;
        Some(base * self.numerator / self.denominator)
    }

    fn snapshot(&self) -> HashMap<(u64, u32), U256> {
        self.inner
            .snapshot()
            .into_iter()
            .map(|(k, v)| (k, v * self.numerator / self.denominator))
            .collect()
    }
}

// ---- TOML loader ----

/// Parse a job pricing TOML string into the `HashMap<(u64, u32), U256>` that
/// `X402Gateway::new` expects.
///
/// Format:
/// ```toml
/// [service_id]
/// job_index = "price_in_wei"
/// ```
pub fn load_job_pricing(content: &str) -> Result<HashMap<(u64, u32), U256>, String> {
    let parsed: toml::Value = toml::from_str(content).map_err(|e| e.to_string())?;

    let table = parsed
        .as_table()
        .ok_or_else(|| "job pricing TOML must be a table".to_string())?;

    let mut config = HashMap::new();

    for (service_key, jobs) in table {
        let service_id: u64 = service_key
            .parse()
            .map_err(|_| format!("invalid service ID: {service_key}"))?;

        let jobs_table = jobs
            .as_table()
            .ok_or_else(|| format!("service {service_id}: expected a table"))?;

        for (job_key, price_val) in jobs_table {
            let job_index: u32 = job_key
                .parse()
                .map_err(|_| format!("service {service_id}: invalid job index: {job_key}"))?;

            let price_str = price_val.as_str().ok_or_else(|| {
                format!("service {service_id} job {job_index}: price must be a string")
            })?;

            let price = U256::from_str_radix(price_str, 10).map_err(|_| {
                format!("service {service_id} job {job_index}: invalid wei value: {price_str}")
            })?;

            config.insert((service_id, job_index), price);
        }
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_job_pricing() {
        let toml = r#"
[1]
0 = "1000000000000000"
1 = "10000000000000000"
"#;
        let prices = load_job_pricing(toml).unwrap();
        assert_eq!(prices[&(1, 0)], U256::from(1_000_000_000_000_000u64));
        assert_eq!(prices[&(1, 1)], U256::from(10_000_000_000_000_000u64));
    }

    #[test]
    fn test_static_oracle() {
        let mut prices = HashMap::new();
        prices.insert((1, 0), U256::from(100u64));
        let oracle = StaticPriceOracle::new(prices);

        assert_eq!(oracle.price_wei(1, 0), Some(U256::from(100u64)));
        assert_eq!(oracle.price_wei(1, 99), None);
    }

    #[test]
    fn test_scaled_oracle() {
        let mut prices = HashMap::new();
        prices.insert((1, 0), U256::from(1000u64));
        let base = StaticPriceOracle::new(prices);
        // 1.5x multiplier: 3/2
        let oracle = ScaledPriceOracle::new(base, U256::from(3u64), U256::from(2u64)).unwrap();

        assert_eq!(oracle.price_wei(1, 0), Some(U256::from(1500u64)));
        assert_eq!(oracle.snapshot()[&(1, 0)], U256::from(1500u64));
    }

    #[test]
    fn test_scaled_oracle_2x() {
        let mut prices = HashMap::new();
        prices.insert((1, 0), U256::from(500u64));
        let base = StaticPriceOracle::new(prices);
        let oracle = ScaledPriceOracle::new(base, U256::from(2u64), U256::from(1u64)).unwrap();

        assert_eq!(oracle.price_wei(1, 0), Some(U256::from(1000u64)));
    }

    #[test]
    fn test_scaled_oracle_zero_denominator() {
        let prices = HashMap::new();
        let base = StaticPriceOracle::new(prices);
        let result = ScaledPriceOracle::new(base, U256::from(1u64), U256::ZERO);
        assert!(result.is_err());
    }
}
