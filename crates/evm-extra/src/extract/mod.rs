//! Extraction utilities for EVM-specific data
//!
//! This module provides extractors for EVM blocks, events, and jobs, following the same
//! pattern as Tangle for consistency and reusability.

pub mod block;
pub mod contract;
pub mod event;
pub mod job;
pub mod tx;

pub use block::{BlockHash, BlockNumber, BlockTimestamp};
pub use contract::ContractAddress;
pub use event::{BlockEvents, Events, FirstEvent, LastEvent};
pub use job::{
    CallId, CallIdRejection, Caller, CallerRejection, InvalidCallId, InvalidCaller,
    InvalidJobIndex, InvalidServiceId, JobIndex, JobIndexRejection, JobInputs, MissingCallId,
    MissingCaller, MissingJobIndex, MissingJobInputs, MissingServiceId, ServiceId,
    ServiceIdRejection,
};
pub use tx::Tx;
