//! Uniswap V3 TWAP (time-weighted average price) oracle.
//!
//! Reads tick cumulatives from a Uniswap V3 pool's `observe()` function and
//! computes the arithmetic mean tick over a configurable window. The tick is
//! then converted to a human-readable exchange rate.
//!
//! ```rust,ignore
//! use alloy_primitives::address;
//! use x402_blueprint::oracle::UniswapV3TwapOracle;
//!
//! let oracle = UniswapV3TwapOracle::new(
//!     provider,
//!     vec![PoolConfig {
//!         base: "ETH".into(),
//!         quote: "USDC".into(),
//!         pool: address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"),
//!         token0_decimals: 6,   // USDC
//!         token1_decimals: 18,  // WETH
//!         base_is_token0: false, // ETH = token1
//!     }],
//!     600, // 10-minute TWAP
//! );
//! let rate = oracle.rate("ETH", "USDC").await?;
//! ```

use super::{ExchangeRateProvider, OracleError};
use alloy_primitives::Address;
use alloy_provider::RootProvider;
use alloy_sol_types::sol;
use rust_decimal::Decimal;
use std::collections::HashMap;

sol! {
    #[sol(rpc)]
    interface IUniswapV3Pool {
        function observe(uint32[] calldata secondsAgos) external view returns (
            int56[] memory tickCumulatives,
            uint160[] memory secondsPerLiquidityCumulativeX128s
        );
    }
}

/// Configuration for a single Uniswap V3 pool used as a price source.
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Base currency symbol (e.g. "ETH").
    pub base: String,
    /// Quote currency symbol (e.g. "USDC").
    pub quote: String,
    /// Pool contract address.
    pub pool: Address,
    /// Decimals of token0 in the pool.
    pub token0_decimals: u8,
    /// Decimals of token1 in the pool.
    pub token1_decimals: u8,
    /// Whether the base currency is token0 in the pool.
    /// For the USDC/WETH pool (token0=USDC, token1=WETH), set this to `false`
    /// when querying ETH as the base.
    pub base_is_token0: bool,
}

/// Reads exchange rates from Uniswap V3 pool TWAP oracles.
pub struct UniswapV3TwapOracle {
    provider: RootProvider,
    pools: HashMap<(String, String), PoolConfig>,
    twap_seconds: u32,
}

impl UniswapV3TwapOracle {
    /// Create a new Uniswap V3 TWAP oracle.
    ///
    /// `twap_seconds` is the lookback window for the time-weighted average.
    /// Common values: 300 (5 min), 600 (10 min), 1800 (30 min).
    pub fn new(
        provider: RootProvider,
        pools: impl IntoIterator<Item = PoolConfig>,
        twap_seconds: u32,
    ) -> Self {
        let pools = pools
            .into_iter()
            .map(|p| ((p.base.clone(), p.quote.clone()), p))
            .collect();
        Self {
            provider,
            pools,
            twap_seconds,
        }
    }
}

impl ExchangeRateProvider for UniswapV3TwapOracle {
    fn rate(
        &self,
        base: &str,
        quote: &str,
    ) -> impl std::future::Future<Output = Result<Decimal, OracleError>> + Send {
        let key = (base.to_owned(), quote.to_owned());
        let pool_config = self.pools.get(&key).cloned();
        let twap_seconds = self.twap_seconds;
        let provider = self.provider.clone();

        async move {
            let config = pool_config
                .ok_or_else(|| OracleError::NotFound(format!("{base}/{quote}")))?;

            let pool = IUniswapV3Pool::new(config.pool, &provider);

            let seconds_agos = vec![twap_seconds, 0u32];
            let result = pool
                .observe(seconds_agos)
                .call()
                .await
                .map_err(|e| OracleError::Provider(format!("observe(): {e}")))?;

            if result.tickCumulatives.len() < 2 {
                return Err(OracleError::Provider(
                    "observe() returned fewer than 2 tick cumulatives".into(),
                ));
            }

            // Arithmetic mean tick over the TWAP window.
            let tick_past: i64 = result.tickCumulatives[0]
                .try_into()
                .map_err(|_| OracleError::Parse("tick cumulative overflow".into()))?;
            let tick_now: i64 = result.tickCumulatives[1]
                .try_into()
                .map_err(|_| OracleError::Parse("tick cumulative overflow".into()))?;
            let mean_tick = (tick_now - tick_past) / i64::from(twap_seconds);

            let rate = tick_to_rate(
                mean_tick as i32,
                config.token0_decimals,
                config.token1_decimals,
                config.base_is_token0,
            );

            Decimal::try_from(rate).map_err(|e| OracleError::Parse(format!("decimal: {e}")))
        }
    }
}

/// Convert a Uniswap V3 tick to a human-readable exchange rate.
///
/// In Uniswap V3, `1.0001^tick` gives the raw price (token1 per token0 in
/// smallest units). We adjust for decimal differences and optionally invert
/// so the result is always "quote per base" in human units.
fn tick_to_rate(tick: i32, token0_decimals: u8, token1_decimals: u8, base_is_token0: bool) -> f64 {
    // 1.0001^tick = token1_raw / token0_raw
    let raw_price = 1.0001_f64.powi(tick);

    // Adjust for decimal difference to get human-readable price.
    // human_price = raw_price * 10^(token0_decimals - token1_decimals)
    // This gives "how many token1 per 1 token0" in human units.
    let decimal_adj =
        10_f64.powi(i32::from(token0_decimals) - i32::from(token1_decimals));
    let token1_per_token0 = raw_price * decimal_adj;

    if base_is_token0 {
        // base = token0, quote = token1
        // rate = token1 per token0 = quote per base
        token1_per_token0
    } else {
        // base = token1, quote = token0
        // rate = token0 per token1 = 1 / (token1 per token0)
        1.0 / token1_per_token0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_to_rate_eth_usdc() {
        // USDC/WETH pool: token0=USDC(6), token1=WETH(18), base=ETH(token1)
        // At ~$3200 ETH: tick ~ 195600
        let rate = tick_to_rate(195_600, 6, 18, false);
        // Should be approximately 3200 USDC per ETH
        assert!(rate > 3000.0 && rate < 3500.0, "rate was {rate}");
    }

    #[test]
    fn test_tick_to_rate_inverse() {
        // If base is token0, the rate should be the inverse of base=token1.
        let rate_base_t1 = tick_to_rate(195_600, 6, 18, false);
        let rate_base_t0 = tick_to_rate(195_600, 6, 18, true);
        let product = rate_base_t0 * rate_base_t1;
        assert!(
            (product - 1.0).abs() < 0.01,
            "product was {product}"
        );
    }
}
