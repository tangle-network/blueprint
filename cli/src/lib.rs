pub mod command;
pub mod foundry;
pub mod utils;

#[cfg(test)]
mod tests;

pub use blueprint_chain_setup::anvil;
use blueprint_tangle_extra::util::TxProgressExt;
use tangle_subxt::subxt::{Config, blocks::ExtrinsicEvents, client::OnlineClientT, tx::TxProgress};
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::field::BoundedString;

pub(crate) async fn wait_for_in_block_success<T: Config>(
    res: TxProgress<T, impl OnlineClientT<T>>,
) -> ExtrinsicEvents<T> {
    res.wait_for_in_block()
        .await
        .unwrap()
        .fetch_events()
        .await
        .unwrap()
}

/// Helper function to decode a `BoundedString` to a regular String
pub(crate) fn decode_bounded_string(bounded_string: &BoundedString) -> String {
    String::from_utf8_lossy(&bounded_string.0.0).to_string()
}

/// This force installs the default crypto provider.
///
/// This is necessary in case there are more than one available backends enabled in rustls (ring,
/// aws-lc-rs).
///
/// This should be called high in the main fn.
///
/// See also:
///   <https://github.com/snapview/tokio-tungstenite/issues/353#issuecomment-2455100010>
///   <https://github.com/awslabs/aws-sdk-rust/discussions/1257>
///
/// # Panics
/// If the default crypto provider cannot be installed, this function will panic.
pub fn install_crypto_provider() {
    // https://github.com/snapview/tokio-tungstenite/issues/353
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install default rustls crypto provider");
}
