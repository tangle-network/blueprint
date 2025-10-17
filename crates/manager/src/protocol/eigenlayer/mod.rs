/// EigenLayer Protocol implementation for blueprint manager
///
/// This module contains the EigenLayer-specific client and event handler.

pub mod client;
pub mod event_handler;

pub use client::EigenlayerProtocolClient;
pub use event_handler::EigenlayerEventHandler;
