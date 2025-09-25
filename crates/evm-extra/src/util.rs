use alloy_network::EthereumWallet;
use alloy_primitives::FixedBytes;
use alloy_primitives::U256;
use alloy_provider::{
    PendingTransactionBuilder, PendingTransactionError, Provider, ProviderBuilder, RootProvider,
    WsConnect,
};
use alloy_rpc_types::eth::TransactionReceipt;
use alloy_signer_local::PrivateKeySigner;
use alloy_transport::TransportErrorKind;
use blueprint_std::str::FromStr;
use reqwest::Url as ReqwestUrl;
use url::Url;

fn to_reqwest_url(url: Url) -> ReqwestUrl {
    ReqwestUrl::parse(url.as_str()).expect("valid URL")
}

/// 1 day
pub const SIGNATURE_EXPIRY: U256 = U256::from_limbs([86400, 0, 0, 0]);

/// Get the provider for a http endpoint
///
/// # Returns
/// - [`RootProvider`] - The provider
///
/// # Panics
/// - If the provided http endpoint is not a valid URL
#[must_use]
pub fn get_provider_http<T: TryInto<Url>>(http_endpoint: T) -> RootProvider
where
    <T as TryInto<Url>>::Error: std::fmt::Debug,
{
    let endpoint: Url = http_endpoint.try_into().unwrap();
    let provider = ProviderBuilder::new().connect_http(to_reqwest_url(endpoint));
    provider.root().clone()
}

/// Get the provider for a http endpoint with the specified [`Wallet`](EthereumWallet)
///
/// # Returns
/// - [`RootProvider<BoxTransport>`] - The provider
///
/// # Panics
/// - If the provided http endpoint is not a valid URL
#[must_use]
pub fn get_wallet_provider_http<T: TryInto<Url>>(
    http_endpoint: T,
    wallet: EthereumWallet,
) -> RootProvider
where
    <T as TryInto<Url>>::Error: std::fmt::Debug,
{
    let endpoint: Url = http_endpoint.try_into().unwrap();
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect_http(to_reqwest_url(endpoint));
    provider.root().clone()
}

/// Get the provider for a websocket endpoint
///
/// # Returns
/// - [`RootProvider`] - The provider
///
/// # Panics
/// - If the provided websocket endpoint is not a valid URL
#[must_use]
pub async fn get_provider_ws(ws_endpoint: &str) -> RootProvider {
    let provider = ProviderBuilder::new()
        .connect_ws(WsConnect::new(ws_endpoint))
        .await
        .unwrap();
    provider.root().clone()
}

#[allow(clippy::type_complexity)]
/// Get the provider for an http endpoint with the [`Wallet`](EthereumWallet) for the specified private key
///
/// # Returns
/// - [`RootProvider`] - The provider
///
/// # Panics
/// - If the provided http endpoint is not a valid URL
#[must_use]
pub fn get_provider_from_signer<T: TryInto<Url>>(key: &str, rpc_url: T) -> RootProvider
where
    <T as TryInto<Url>>::Error: std::fmt::Debug,
{
    let signer = PrivateKeySigner::from_str(key).expect("wrong key ");
    let wallet = EthereumWallet::from(signer);
    let endpoint: Url = rpc_url.try_into().unwrap();
    let provider = ProviderBuilder::new()
        .wallet(wallet.clone())
        .connect_http(to_reqwest_url(endpoint));
    provider.root().clone()
}

/// Wait for a transaction to finish and return its receipt.
///
/// # Arguments
///
/// `rpc_url` - The RPC URL.
/// `tx_hash` - The hash of the transaction.
///
/// # Returns
///
/// A [`TransportResult`] containing the transaction hash.
///
/// # Errors
///
/// - [`TransportErrorKind::custom_str("Invalid RPC URL")`] - If the provided RPC URL is invalid.
/// - [`PendingTransactionError`] - If the receipt cannot be retrieved.
///
/// [`TransportResult`]: alloy_transport::TransportResult
pub async fn wait_transaction(
    rpc_url: impl TryInto<Url>,
    tx_hash: FixedBytes<32>,
) -> Result<TransactionReceipt, PendingTransactionError> {
    let url = rpc_url
        .try_into()
        .map_err(|_| TransportErrorKind::custom_str("Invalid RPC URL"))?;
    let reqwest_url = to_reqwest_url(url);
    let provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_http(reqwest_url);
    let root_provider = provider.root().clone();
    let pending_tx = PendingTransactionBuilder::new(root_provider, tx_hash);
    pending_tx.get_receipt().await
}
