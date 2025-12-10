//! Tangle EVM Context
//!
//! Provides the TangleEvmClientContext trait for accessing the TangleEvmClient
//! from the blueprint environment.

pub use blueprint_client_tangle_evm::Error;
pub use blueprint_client_tangle_evm::TangleEvmClient;
pub use blueprint_client_tangle_evm::TangleEvmClientConfig;
pub use blueprint_client_tangle_evm::TangleEvmSettings;
use blueprint_runner::config::BlueprintEnvironment;

/// TangleEvmClientContext trait provides access to the Tangle EVM client from the context.
pub trait TangleEvmClientContext {
    /// Returns the Tangle EVM client instance
    fn tangle_evm_client(
        &self,
    ) -> impl core::future::Future<Output = Result<TangleEvmClient, Error>> + Send;
}

impl TangleEvmClientContext for BlueprintEnvironment {
    async fn tangle_evm_client(&self) -> Result<TangleEvmClient, Error> {
        let keystore = self.keystore();

        // Get the tangle-evm protocol settings from environment
        let settings = self
            .protocol_settings
            .tangle_evm()
            .map_err(|e| Error::Config(e.to_string()))?;

        // Create the client config from the environment
        let config = TangleEvmClientConfig {
            http_rpc_endpoint: self.http_rpc_endpoint.clone(),
            ws_rpc_endpoint: self.ws_rpc_endpoint.clone(),
            keystore_uri: self.keystore_uri.clone(),
            data_dir: self.data_dir.clone(),
            settings: TangleEvmSettings {
                blueprint_id: settings.blueprint_id,
                service_id: settings.service_id,
                tangle_contract: settings.tangle_contract,
                restaking_contract: settings.restaking_contract,
                status_registry_contract: settings.status_registry_contract,
            },
            test_mode: self.test_mode,
        };

        TangleEvmClient::with_keystore(config, keystore).await
    }
}
