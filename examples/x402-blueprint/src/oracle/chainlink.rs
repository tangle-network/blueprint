//! Chainlink AggregatorV3 price feed oracle.
//!
//! Reads the latest round data from a Chainlink price feed contract on any
//! EVM chain. Feed addresses are registered per trading pair.
//!
//! ```rust,ignore
//! use alloy_primitives::address;
//! use x402_blueprint::oracle::ChainlinkOracle;
//!
//! let oracle = ChainlinkOracle::new(
//!     provider,
//!     vec![
//!         // ETH/USD on Ethereum mainnet
//!         ("ETH", "USD", address!("5f4eC3Df9cbd43714FE2740f5E3616155c5b8419")),
//!     ],
//! );
//! let rate = oracle.rate("ETH", "USD").await?;
//! ```

use super::{ExchangeRateProvider, OracleError};
use alloy_primitives::Address;
use alloy_provider::RootProvider;
use alloy_sol_types::sol;
use rust_decimal::Decimal;
use std::collections::HashMap;

sol! {
    #[sol(rpc)]
    interface IAggregatorV3 {
        function latestRoundData() external view returns (
            uint80 roundId,
            int256 answer,
            uint256 startedAt,
            uint256 updatedAt,
            uint80 answeredInRound
        );
        function decimals() external view returns (uint8);
    }
}

/// Reads exchange rates from Chainlink AggregatorV3 price feed contracts.
pub struct ChainlinkOracle {
    provider: RootProvider,
    feeds: HashMap<(String, String), Address>,
}

impl ChainlinkOracle {
    /// Create a new Chainlink oracle.
    ///
    /// `feeds` is a list of `(base, quote, feed_address)` tuples. For example:
    /// `("ETH", "USD", 0x5f4e...)` for the ETH/USD feed on Ethereum mainnet.
    pub fn new<'a>(
        provider: RootProvider,
        feeds: impl IntoIterator<Item = (&'a str, &'a str, Address)>,
    ) -> Self {
        let feeds = feeds
            .into_iter()
            .map(|(base, quote, addr)| ((base.to_owned(), quote.to_owned()), addr))
            .collect();
        Self { provider, feeds }
    }
}

impl ExchangeRateProvider for ChainlinkOracle {
    fn rate(
        &self,
        base: &str,
        quote: &str,
    ) -> impl std::future::Future<Output = Result<Decimal, OracleError>> + Send {
        let key = (base.to_owned(), quote.to_owned());
        let feed_address = self.feeds.get(&key).copied();
        let provider = self.provider.clone();

        async move {
            let address = feed_address
                .ok_or_else(|| OracleError::NotFound(format!("{base}/{quote}")))?;

            let feed = IAggregatorV3::new(address, &provider);

            let dec_result = feed
                .decimals()
                .call()
                .await
                .map_err(|e| OracleError::Provider(format!("decimals(): {e}")))?;

            let round = feed
                .latestRoundData()
                .call()
                .await
                .map_err(|e| OracleError::Provider(format!("latestRoundData(): {e}")))?;

            // answer is int256; negative means the feed is stale/invalid.
            let answer_i128: i128 = round
                .answer
                .try_into()
                .map_err(|_| OracleError::Parse("answer overflows i128".into()))?;

            if answer_i128 <= 0 {
                return Err(OracleError::Provider("negative or zero price".into()));
            }

            // Scale: answer has `feed_decimals` decimal places.
            // rate = answer / 10^feed_decimals
            let feed_decimals: u8 = dec_result;
            let scale = Decimal::from(10u64.pow(u32::from(feed_decimals)));
            let rate = Decimal::from(answer_i128) / scale;

            Ok(rate)
        }
    }
}
