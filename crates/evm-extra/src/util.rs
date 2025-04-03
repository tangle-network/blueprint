use alloy_network::EthereumWallet;
use alloy_primitives::U256;
use alloy_provider::{Provider, ProviderBuilder, RootProvider, WsConnect, PendingTransactionBuilder, PendingTransactionError};
use alloy_signer_local::PrivateKeySigner;
use blueprint_std::str::FromStr;
use url::Url;
use alloy_primitives::FixedBytes;
use alloy_rpc_types::eth::TransactionReceipt;
use alloy_transport::TransportErrorKind;

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
pub fn get_provider_http(http_endpoint: &str) -> RootProvider {
    ProviderBuilder::new()
        .on_http(http_endpoint.parse().unwrap())
        .root()
        .clone()
}

/// Get the provider for a http endpoint with the specified [`Wallet`](EthereumWallet)
///
/// # Returns
/// - [`RootProvider<BoxTransport>`] - The provider
///
/// # Panics
/// - If the provided http endpoint is not a valid URL
#[must_use]
pub fn get_wallet_provider_http(http_endpoint: &str, wallet: EthereumWallet) -> RootProvider {
    ProviderBuilder::new()
        .wallet(wallet)
        .on_http(http_endpoint.parse().unwrap())
        .root()
        .clone()
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
    ProviderBuilder::new()
        .on_ws(WsConnect::new(ws_endpoint))
        .await
        .unwrap()
        .root()
        .clone()
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
pub fn get_provider_from_signer(key: &str, rpc_url: &str) -> RootProvider {
    let signer = PrivateKeySigner::from_str(key).expect("wrong key ");
    let wallet = EthereumWallet::from(signer);
    let url = Url::parse(rpc_url).expect("Wrong rpc url");
    ProviderBuilder::new()
        .wallet(wallet.clone())
        .on_http(url)
        .root()
        .clone()
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
pub async fn wait_transaction(
    rpc_url: &str,
    tx_hash: FixedBytes<32>,
) -> Result<TransactionReceipt, PendingTransactionError> {
    let url = Url::parse(rpc_url).map_err(|_| TransportErrorKind::custom_str("Invalid RPC URL"))?;
    let root_provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .on_http(url);
    let pending_tx = PendingTransactionBuilder::new(root_provider, tx_hash);
    pending_tx.get_receipt().await
}
