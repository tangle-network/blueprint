//! Coinbase exchange rate API oracle.
//!
//! Fetches spot exchange rates from the public Coinbase API. No API key
//! required for the exchange-rates endpoint.
//!
//! ```rust,ignore
//! use x402_blueprint::oracle::CoinbaseOracle;
//!
//! let oracle = CoinbaseOracle::new();
//! let rate = oracle.rate("ETH", "USDC").await?;
//! // rate ~ 3200.0
//! ```

use super::{ExchangeRateProvider, OracleError};
use rust_decimal::Decimal;
use std::str::FromStr;

/// Fetches exchange rates from the Coinbase public API.
///
/// Uses the `GET /v2/exchange-rates?currency={base}` endpoint which returns
/// spot rates for all supported quote currencies.
pub struct CoinbaseOracle {
    client: reqwest::Client,
}

impl CoinbaseOracle {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl Default for CoinbaseOracle {
    fn default() -> Self {
        Self::new()
    }
}

impl ExchangeRateProvider for CoinbaseOracle {
    fn rate(
        &self,
        base: &str,
        quote: &str,
    ) -> impl std::future::Future<Output = Result<Decimal, OracleError>> + Send {
        let url = format!("https://api.coinbase.com/v2/exchange-rates?currency={base}");
        let quote_owned = quote.to_owned();
        let client = self.client.clone();

        async move {
            let resp = client
                .get(&url)
                .send()
                .await
                .map_err(|e| OracleError::Http(e.to_string()))?;

            if !resp.status().is_success() {
                return Err(OracleError::Http(format!(
                    "Coinbase API returned {}",
                    resp.status()
                )));
            }

            let body: serde_json::Value = resp
                .json()
                .await
                .map_err(|e| OracleError::Parse(e.to_string()))?;

            let rate_str = body["data"]["rates"][&quote_owned]
                .as_str()
                .ok_or_else(|| {
                    OracleError::NotFound(format!("no rate for {quote_owned} in Coinbase response"))
                })?;

            Decimal::from_str(rate_str).map_err(|e| OracleError::Parse(format!("{rate_str}: {e}")))
        }
    }
}
