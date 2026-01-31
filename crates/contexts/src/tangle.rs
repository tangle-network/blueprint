//! Tangle Context
//!
//! Provides the TangleClientContext trait for accessing the TangleClient
//! from the blueprint environment.

pub use blueprint_client_tangle::Error;
pub use blueprint_client_tangle::TangleClient;
pub use blueprint_client_tangle::TangleClientConfig;
pub use blueprint_client_tangle::TangleSettings;
use blueprint_runner::config::BlueprintEnvironment;

/// TangleClientContext trait provides access to the Tangle client from the context.
pub trait TangleClientContext {
    /// Returns the Tangle client instance
    fn tangle_client(
        &self,
    ) -> impl core::future::Future<Output = Result<TangleClient, Error>> + Send;
}

impl TangleClientContext for BlueprintEnvironment {
    async fn tangle_client(&self) -> Result<TangleClient, Error> {
        let keystore = self.keystore();

        // Get the tangle protocol settings from environment
        let settings = self
            .protocol_settings
            .tangle()
            .map_err(|e| Error::Config(e.to_string()))?;

        // Create the client config from the environment
        let config = TangleClientConfig {
            http_rpc_endpoint: self.http_rpc_endpoint.clone(),
            ws_rpc_endpoint: self.ws_rpc_endpoint.clone(),
            keystore_uri: self.keystore_uri.clone(),
            data_dir: self.data_dir.clone(),
            settings: TangleSettings {
                blueprint_id: settings.blueprint_id,
                service_id: settings.service_id,
                tangle_contract: settings.tangle_contract,
                restaking_contract: settings.restaking_contract,
                status_registry_contract: settings.status_registry_contract,
            },
            test_mode: self.test_mode,
            dry_run: self.dry_run,
        };

        TangleClient::with_keystore(config, keystore).await
    }
}
