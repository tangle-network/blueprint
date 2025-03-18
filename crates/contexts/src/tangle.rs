pub use blueprint_clients::Error;
pub use blueprint_clients::tangle::client::TangleClient;
use blueprint_runner::config::BlueprintEnvironment;

/// `TangleContext` trait provides access to the Tangle client from the context.
pub trait TangleClientContext {
    /// Returns the Tangle client instance
    async fn tangle_client(&self) -> Result<TangleClient, Error>;
}

impl TangleClientContext for BlueprintEnvironment {
    async fn tangle_client(&self) -> Result<TangleClient, Error> {
        TangleClient::new(self.clone()).await.map_err(Into::into)
    }
}
