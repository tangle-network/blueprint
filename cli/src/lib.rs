pub mod command;
#[cfg(feature = "full")]
pub mod foundry;
#[cfg(feature = "full")]
pub mod settings;
pub mod telemetry;
#[cfg(feature = "full")]
pub mod utils;
#[cfg(feature = "full")]
pub mod workspace;

#[cfg(all(test, feature = "full"))]
mod tests;

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
