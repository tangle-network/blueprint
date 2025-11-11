/// EigenLayer Protocol implementation for blueprint manager
///
/// This module contains the EigenLayer-specific client, event handler, and registration management.
pub mod client;
pub mod event_handler;

pub use client::EigenlayerProtocolClient;
pub use event_handler::EigenlayerEventHandler;

// Re-export registration types from blueprint-eigenlayer-extra
pub use blueprint_eigenlayer_extra::registration::{
    AvsRegistration, AvsRegistrationConfig, AvsRegistrations, RegistrationStateManager,
    RegistrationStatus,
};
