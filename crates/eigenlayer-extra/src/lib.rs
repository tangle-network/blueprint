//! EigenLayer AVS Framework - High-level services for AVS development
//!
//! This crate provides production-ready abstractions on top of eigensdk-rs for building
//! Actively Validated Services (AVS) on EigenLayer. It mirrors the design of blueprint-tangle-extra
//! but specifically augments the EigenLayer protocol with developer-friendly services.
//!
//! # Architecture
//!
//! - **registration**: AVS registration state management (shared with CLI)
//! - **discovery**: On-chain AVS discovery and operator status queries
//! - **services**: High-level operator services (rewards, slashing, lifecycle)
//! - **util**: Utility functions for AVS development
//!
//! # Design Principles
//!
//! 1. **Production-Ready**: No mocks, real contract integration
//! 2. **Performant**: Async-first with efficient event processing
//! 3. **Type-Safe**: Leverage Rust's type system for correctness
//! 4. **Framework, Not Wrapper**: Augment eigensdk, don't just wrap it
//! 5. **Modular**: Shared logic between CLI and manager

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod discovery;
pub mod error;
pub mod generic_task_aggregation;
pub mod registration;
pub mod services;
pub mod sidecar;
pub mod util;

// Re-exports for convenience
pub use discovery::{AvsDiscoveryService, DiscoveredAvs, OperatorStatus};
pub use registration::{
    AvsRegistration, AvsRegistrationConfig, AvsRegistrations, RegistrationStateManager,
    RegistrationStatus, RuntimeTarget,
};
pub use services::{
    lifecycle::OperatorLifecycleManager, rewards::RewardsManager, slashing::SlashingMonitor,
};
