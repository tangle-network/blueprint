/// Tangle Protocol implementation for blueprint manager
///
/// This module contains the Tangle-specific client and event handler.

pub mod client;
pub mod event_handler;

#[cfg(test)]
mod tests;

pub use client::TangleProtocolClient;
pub use event_handler::TangleEventHandler;
