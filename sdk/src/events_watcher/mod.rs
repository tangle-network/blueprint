//! ## Overview
//!
//! Event watcher traits handle the syncing and listening of events for a given network.
//! The event watcher calls into a storage for handling of important state. The run implementation
//! of an event watcher polls for blocks. Implementations of the event watcher trait define an
//! action to take when the specified event is found in a block at the `handle_event` api.

/// Error type for the event watcher module.
pub mod error;
pub use error::Error;

#[cfg(feature = "std")]
pub mod evm;
mod retry;
pub mod substrate;
pub mod tangle;
