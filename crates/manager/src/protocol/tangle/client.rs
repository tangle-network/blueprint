/// Tangle Protocol Client
///
/// Handles connection to Tangle network and streams finality notifications.
use crate::config::BlueprintManagerContext;
use crate::error::Result;
use crate::protocol::types::{ProtocolEvent, TangleProtocolEvent};
use blueprint_clients::tangle::EventsClient;
use blueprint_clients::tangle::client::TangleClient;
use blueprint_keystore::{Keystore, KeystoreConfig};
use blueprint_runner::config::BlueprintEnvironment;

/// Tangle protocol client implementation
pub struct TangleProtocolClient {
    client: TangleClient,
}

impl TangleProtocolClient {
    /// Create a new Tangle protocol client
    pub async fn new(env: BlueprintEnvironment, _ctx: &BlueprintManagerContext) -> Result<Self> {
        let keystore = Keystore::new(KeystoreConfig::new().fs_root(&env.keystore_uri))?;
        let client = TangleClient::with_keystore(env, keystore).await?;

        Ok(Self { client })
    }

    /// Get a reference to the underlying Tangle client
    ///
    /// This is useful for protocol-specific operations like querying services
    #[must_use] pub fn tangle_client(&self) -> &TangleClient {
        &self.client
    }
}

impl TangleProtocolClient {
    /// Get the next protocol event from Tangle
    pub async fn next_event(&mut self) -> Option<ProtocolEvent> {
        self.client.latest_event().await.map(|tangle_event| {
            ProtocolEvent::Tangle(TangleProtocolEvent {
                block_number: tangle_event.number,
                block_hash: tangle_event.hash,
                inner: tangle_event,
            })
        })
    }
}
