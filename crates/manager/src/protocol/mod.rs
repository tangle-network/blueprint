/// Protocol abstraction layer for blueprint manager
///
/// This module provides a clean abstraction over different blockchain protocols
/// (Tangle, EigenLayer, etc.) that the blueprint manager can execute on.
///
/// # Architecture
///
/// The protocol layer is designed around these core traits:
/// - `ProtocolClient`: Defines how to connect to and listen to a protocol
/// - `ProtocolEventHandler`: Handles protocol-specific events
/// - `ProtocolConfig`: Protocol-specific configuration
use crate::blueprint::ActiveBlueprints;
use crate::config::BlueprintManagerContext;
use crate::error::Result;
use blueprint_runner::config::BlueprintEnvironment;

pub mod eigenlayer;
pub mod tangle;
pub mod types;

pub use types::{ProtocolEvent, ProtocolType};

/// Protocol manager that handles both client and event processing
///
/// Uses enum dispatch for zero-cost abstraction over protocols
pub enum ProtocolManager {
    Tangle {
        client: tangle::TangleProtocolClient,
        handler: tangle::TangleEventHandler,
    },
    Eigenlayer {
        client: eigenlayer::EigenlayerProtocolClient,
        handler: eigenlayer::EigenlayerEventHandler,
    },
}

impl ProtocolManager {
    /// Create a new protocol manager for the given protocol type
    ///
    /// # Errors
    ///
    /// Returns an error if the protocol client fails to initialize
    pub async fn new(
        protocol: ProtocolType,
        env: BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
    ) -> Result<Self> {
        match protocol {
            ProtocolType::Tangle => {
                let client = tangle::TangleProtocolClient::new(env, ctx).await?;
                let handler = tangle::TangleEventHandler::new();
                Ok(Self::Tangle { client, handler })
            }
            ProtocolType::Eigenlayer => {
                let client = eigenlayer::EigenlayerProtocolClient::new(env, ctx).await?;
                let handler = eigenlayer::EigenlayerEventHandler::new();
                Ok(Self::Eigenlayer { client, handler })
            }
        }
    }

    /// Initialize the protocol (query initial state, etc.)
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails (e.g., loading state, spawning blueprints)
    pub async fn initialize(
        &mut self,
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        match self {
            Self::Tangle { client, handler } => {
                handler
                    .initialize(client, env, ctx, active_blueprints)
                    .await
            }
            Self::Eigenlayer { client, handler } => {
                handler
                    .initialize(client, env, ctx, active_blueprints)
                    .await
            }
        }
    }

    /// Get the next event from the protocol
    pub async fn next_event(&mut self) -> Option<ProtocolEvent> {
        match self {
            Self::Tangle { client, .. } => client.next_event().await,
            Self::Eigenlayer { client, .. } => client.next_event().await,
        }
    }

    /// Handle a protocol event
    ///
    /// # Errors
    ///
    /// Returns an error if event handling fails (e.g., spawning services, contract calls)
    pub async fn handle_event(
        &mut self,
        event: &ProtocolEvent,
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        match self {
            Self::Tangle { client, handler } => {
                handler
                    .handle_event(client, event, env, ctx, active_blueprints)
                    .await
            }
            Self::Eigenlayer { handler, .. } => {
                handler
                    .handle_event(event, env, ctx, active_blueprints)
                    .await
            }
        }
    }

    /// Run the protocol event loop
    ///
    /// # Errors
    ///
    /// Returns an error if initialization or event handling fails
    pub async fn run(
        &mut self,
        env: &BlueprintEnvironment,
        ctx: &BlueprintManagerContext,
        active_blueprints: &mut ActiveBlueprints,
    ) -> Result<()> {
        // Initialize
        self.initialize(env, ctx, active_blueprints).await?;

        // Event loop
        while let Some(event) = self.next_event().await {
            self.handle_event(&event, env, ctx, active_blueprints)
                .await?;
        }

        Err(crate::error::Error::ClientDied)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::Address;
    use blueprint_runner::config::ProtocolSettings;
    use blueprint_runner::tangle::config::TangleProtocolSettings;

    /// Test that `ProtocolType` correctly converts from `ProtocolSettings`
    #[test]
    fn test_protocol_type_from_settings() {
        // Test Tangle conversion
        let tangle_settings = ProtocolSettings::Tangle(TangleProtocolSettings {
            blueprint_id: 1,
            service_id: Some(0),
            tangle_contract: Address::ZERO,
            restaking_contract: Address::ZERO,
            status_registry_contract: Address::ZERO,
        });
        let protocol_type: ProtocolType = (&tangle_settings).into();
        assert!(matches!(protocol_type, ProtocolType::Tangle));

        // Test None defaults to Tangle
        let none_settings = ProtocolSettings::None;
        let protocol_type: ProtocolType = (&none_settings).into();
        assert!(matches!(protocol_type, ProtocolType::Tangle));
    }

    /// Test that `ProtocolEvent` correctly identifies its variant
    #[test]
    fn test_protocol_event_variant_checking() {
        use crate::protocol::types::EigenlayerProtocolEvent;

        let eigenlayer_event = ProtocolEvent::Eigenlayer(EigenlayerProtocolEvent {
            block_number: 100,
            block_hash: vec![0u8; 32],
            logs: vec![],
        });

        // Should not be Tangle
        assert!(eigenlayer_event.as_tangle().is_none());
        // Should be EigenLayer
        assert!(eigenlayer_event.as_eigenlayer().is_some());
        // Block number should be preserved
        assert_eq!(eigenlayer_event.block_number(), 100);
    }
}
