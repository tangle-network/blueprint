use blueprint_clients::Error;
pub use blueprint_clients::eigenlayer::client::EigenlayerClient;
use blueprint_runner::config::BlueprintEnvironment;

/// Provides access to Eigenlayer utilities through its [`EigenlayerClient`].
pub trait EigenlayerContext {
    async fn eigenlayer_client(&self) -> Result<EigenlayerClient, Error>;
}

impl EigenlayerContext for BlueprintEnvironment {
    async fn eigenlayer_client(&self) -> Result<EigenlayerClient, Error> {
        Ok(EigenlayerClient::new(self.clone()))
    }
}
