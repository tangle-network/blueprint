//! Tangle EVM Context
//!
//! Provides the TangleEvmClientContext trait for accessing the TangleEvmClient
//! from the blueprint environment.

pub use blueprint_clients::tangle_evm::client::TangleEvmClient;
pub use blueprint_clients::tangle_evm::Error;
use blueprint_runner::config::BlueprintEnvironment;

/// TangleEvmClientContext trait provides access to the Tangle EVM client from the context.
pub trait TangleEvmClientContext {
    /// Returns the Tangle EVM client instance
    fn tangle_evm_client(&self) -> impl core::future::Future<Output = Result<TangleEvmClient, Error>> + Send;
}

impl TangleEvmClientContext for BlueprintEnvironment {
    async fn tangle_evm_client(&self) -> Result<TangleEvmClient, Error> {
        let keystore = self.keystore();
        TangleEvmClient::with_keystore(self.clone(), keystore)
            .await
            .map_err(Into::into)
    }
}
