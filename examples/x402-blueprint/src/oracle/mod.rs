//! Exchange rate oracles for dynamic x402 pricing.
//!
//! The [`ExchangeRateProvider`] trait abstracts over different price feed
//! sources. Implementations fetch the current exchange rate (e.g. ETH/USDC)
//! and return it as a `Decimal`. The [`refresh_rates`] helper applies fresh
//! rates to an [`X402Config`] before starting (or restarting) the gateway.
//!
//! Three implementations are provided:
//!
//! | Oracle | Feature | Source |
//! |--------|---------|--------|
//! | [`ChainlinkOracle`] | `chainlink` | On-chain AggregatorV3 price feed |
//! | [`UniswapV3TwapOracle`] | `uniswap` | On-chain Uniswap V3 TWAP |
//! | [`CoinbaseOracle`] | `coinbase` | Coinbase REST API |
//!
//! All three can be wrapped in [`CachedRateProvider`] to avoid hitting the
//! data source on every call.

use blueprint_x402::X402Config;
#[cfg(test)]
use blueprint_x402::config::X402InvocationMode;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[cfg(feature = "chainlink")]
mod chainlink;
#[cfg(feature = "chainlink")]
pub use chainlink::ChainlinkOracle;

#[cfg(feature = "uniswap")]
mod uniswap;
#[cfg(feature = "uniswap")]
pub use uniswap::UniswapV3TwapOracle;

#[cfg(feature = "coinbase")]
mod coinbase;
#[cfg(feature = "coinbase")]
pub use coinbase::CoinbaseOracle;

// ---- Error ----

/// Errors from exchange rate oracle operations.
#[derive(Debug)]
pub enum OracleError {
    /// The requested trading pair has no configured feed.
    NotFound(String),
    /// The upstream data source returned an error.
    Provider(String),
    /// Failed to parse the rate from the data source.
    Parse(String),
    /// HTTP request failed (Coinbase API).
    Http(String),
}

impl fmt::Display for OracleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "feed not found: {msg}"),
            Self::Provider(msg) => write!(f, "provider error: {msg}"),
            Self::Parse(msg) => write!(f, "parse error: {msg}"),
            Self::Http(msg) => write!(f, "http error: {msg}"),
        }
    }
}

impl std::error::Error for OracleError {}

// ---- Trait ----

/// Provides the current exchange rate for a trading pair.
///
/// Returns the rate as a `Decimal`: how many `quote` units per 1 `base` unit.
/// For example, `rate("ETH", "USDC")` returns ~3200 (1 ETH = 3200 USDC).
pub trait ExchangeRateProvider: Send + Sync {
    fn rate(
        &self,
        base: &str,
        quote: &str,
    ) -> impl std::future::Future<Output = Result<Decimal, OracleError>> + Send;
}

// ---- Config helper ----

/// Update all `rate_per_native_unit` fields in an [`X402Config`] from a live
/// exchange rate provider.
///
/// Call this at startup (or periodically) to ensure the gateway converts
/// wei-denominated job prices into accurate stablecoin amounts.
///
/// ```rust,ignore
/// let oracle = CoinbaseOracle::new();
/// refresh_rates(&mut config, &oracle, "ETH").await?;
/// let (gateway, producer) = X402Gateway::new(config, pricing)?;
/// ```
pub async fn refresh_rates<P: ExchangeRateProvider>(
    config: &mut X402Config,
    provider: &P,
    native_currency: &str,
) -> Result<(), OracleError> {
    for token in &mut config.accepted_tokens {
        let rate = provider.rate(native_currency, &token.symbol).await?;
        token.rate_per_native_unit = rate;
    }
    Ok(())
}

// ---- Cached wrapper ----

/// Wraps any [`ExchangeRateProvider`] with a time-based cache.
///
/// Calls to [`rate()`](ExchangeRateProvider::rate) return the cached value if
/// it was fetched within the TTL. Otherwise, the inner provider is queried and
/// the cache is updated.
pub struct CachedRateProvider<P> {
    inner: P,
    cache: Mutex<HashMap<(String, String), (Decimal, Instant)>>,
    ttl: Duration,
}

impl<P> CachedRateProvider<P> {
    pub fn new(inner: P, ttl: Duration) -> Self {
        Self {
            inner,
            cache: Mutex::new(HashMap::new()),
            ttl,
        }
    }
}

impl<P: ExchangeRateProvider> ExchangeRateProvider for CachedRateProvider<P> {
    fn rate(
        &self,
        base: &str,
        quote: &str,
    ) -> impl std::future::Future<Output = Result<Decimal, OracleError>> + Send {
        let base_owned = base.to_owned();
        let quote_owned = quote.to_owned();
        async move {
            let key = (base_owned.clone(), quote_owned.clone());

            // Check cache (brief lock, no await while held).
            {
                let cache = self.cache.lock().unwrap();
                if let Some((rate, fetched_at)) = cache.get(&key) {
                    if fetched_at.elapsed() < self.ttl {
                        return Ok(*rate);
                    }
                }
            }

            // Fetch from inner provider.
            let rate = self.inner.rate(&base_owned, &quote_owned).await?;

            // Update cache.
            {
                let mut cache = self.cache.lock().unwrap();
                cache.insert(key, (rate, Instant::now()));
            }

            Ok(rate)
        }
    }
}

// ---- Fixed rate (for testing) ----

/// A fixed-rate provider that always returns the same rate. Useful for tests
/// and local development.
pub struct FixedRateProvider {
    rates: HashMap<(String, String), Decimal>,
}

impl FixedRateProvider {
    pub fn new(rates: HashMap<(String, String), Decimal>) -> Self {
        Self { rates }
    }

    /// Convenience: single-pair provider.
    pub fn single(base: &str, quote: &str, rate: Decimal) -> Self {
        let mut rates = HashMap::new();
        rates.insert((base.to_owned(), quote.to_owned()), rate);
        Self { rates }
    }
}

impl ExchangeRateProvider for FixedRateProvider {
    fn rate(
        &self,
        base: &str,
        quote: &str,
    ) -> impl std::future::Future<Output = Result<Decimal, OracleError>> + Send {
        let result = self
            .rates
            .get(&(base.to_owned(), quote.to_owned()))
            .copied()
            .ok_or_else(|| OracleError::NotFound(format!("{base}/{quote}")));
        async move { result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fixed_rate_provider() {
        let provider = FixedRateProvider::single("ETH", "USDC", Decimal::from(3200u32));
        let rate = provider.rate("ETH", "USDC").await.unwrap();
        assert_eq!(rate, Decimal::from(3200u32));
    }

    #[tokio::test]
    async fn test_fixed_rate_not_found() {
        let provider = FixedRateProvider::single("ETH", "USDC", Decimal::from(3200u32));
        assert!(provider.rate("BTC", "USDC").await.is_err());
    }

    #[tokio::test]
    async fn test_cached_provider_returns_cached_value() {
        let inner = FixedRateProvider::single("ETH", "USDC", Decimal::from(3200u32));
        let cached = CachedRateProvider::new(inner, Duration::from_secs(60));

        let r1 = cached.rate("ETH", "USDC").await.unwrap();
        let r2 = cached.rate("ETH", "USDC").await.unwrap();
        assert_eq!(r1, r2);
        assert_eq!(r1, Decimal::from(3200u32));
    }

    #[tokio::test]
    async fn test_refresh_rates_updates_config() {
        let provider = FixedRateProvider::single("ETH", "USDC", Decimal::from(4000u32));

        let mut config = X402Config {
            bind_address: "127.0.0.1:0".parse().unwrap(),
            facilitator_url: "https://example.com".parse().unwrap(),
            quote_ttl_secs: 300,
            accepted_tokens: vec![blueprint_x402::config::AcceptedToken {
                network: "eip155:8453".into(),
                asset: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".into(),
                symbol: "USDC".into(),
                decimals: 6,
                pay_to: "0x0000000000000000000000000000000000000001".into(),
                rate_per_native_unit: Decimal::from(3200u32),
                markup_bps: 0,
                transfer_method: "permit2".into(),
                eip3009_name: None,
                eip3009_version: None,
            }],
            default_invocation_mode: X402InvocationMode::Disabled,
            job_policies: vec![],
            service_id: 0,
        };

        assert_eq!(
            config.accepted_tokens[0].rate_per_native_unit,
            Decimal::from(3200u32)
        );
        refresh_rates(&mut config, &provider, "ETH").await.unwrap();
        assert_eq!(
            config.accepted_tokens[0].rate_per_native_unit,
            Decimal::from(4000u32)
        );
    }
}
