use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
use crate::protocol::types::{ProtocolEvent, TangleEvmProtocolEvent};
use blueprint_client_tangle_evm::{
    TangleEvmClient, TangleEvmClientConfig, TangleEvmEvent, TangleEvmSettings,
};
use blueprint_keystore::{Keystore, KeystoreConfig};
use blueprint_runner::config::BlueprintEnvironment;

/// Client used by the manager to observe Tangle EVM blocks/events.
pub struct TangleEvmProtocolClient {
    client: TangleEvmClient,
}

impl TangleEvmProtocolClient {
    /// Build a new client from the blueprint environment.
    ///
    /// # Errors
    ///
    /// Returns an error if protocol settings cannot be loaded, the keystore
    /// initialization fails, or the underlying client cannot be constructed.
    pub async fn new(env: BlueprintEnvironment, _ctx: &BlueprintManagerContext) -> Result<Self> {
        let settings = env
            .protocol_settings
            .tangle_evm()
            .map_err(|e| Error::Other(e.to_string()))?;

        let client_config = TangleEvmClientConfig {
            http_rpc_endpoint: env.http_rpc_endpoint.clone(),
            ws_rpc_endpoint: env.ws_rpc_endpoint.clone(),
            keystore_uri: env.keystore_uri.clone(),
            data_dir: env.data_dir.clone(),
            settings: TangleEvmSettings {
                blueprint_id: settings.blueprint_id,
                service_id: settings.service_id,
                tangle_contract: settings.tangle_contract,
                restaking_contract: settings.restaking_contract,
                status_registry_contract: settings.status_registry_contract,
            },
            test_mode: env.test_mode,
        };

        let keystore = Keystore::new(KeystoreConfig::new().fs_root(&env.keystore_uri))?;
        let client = TangleEvmClient::with_keystore(client_config, keystore)
            .await
            .map_err(Error::from)?;

        Ok(Self { client })
    }

    /// Accessor for the inner client.
    #[must_use]
    pub fn client(&self) -> &TangleEvmClient {
        &self.client
    }

    /// Get the next protocol event.
    pub async fn next_event(&mut self) -> Option<ProtocolEvent> {
        self.client.next_event().await.map(Self::map_event)
    }

    fn map_event(event: TangleEvmEvent) -> ProtocolEvent {
        ProtocolEvent::TangleEvm(TangleEvmProtocolEvent {
            block_number: event.block_number,
            block_hash: event.block_hash,
            timestamp: event.timestamp,
            logs: event.logs.clone(),
            inner: event,
        })
    }
}
