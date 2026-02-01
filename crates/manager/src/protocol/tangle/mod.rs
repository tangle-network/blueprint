/// Tangle Protocol implementation for blueprint manager.
///
/// This module wires the manager into the EVM-native Tangle contracts by
/// reusing the shared `blueprint-client-tangle` crate.
pub mod client;
pub mod event_handler;
pub mod metadata;

pub use client::TangleProtocolClient;
pub use event_handler::TangleEventHandler;
