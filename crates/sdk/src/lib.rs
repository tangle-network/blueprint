//! ![Tangle Network Banner](https://raw.githubusercontent.com/tangle-network/tangle/refs/heads/main/assets/Tangle%20%20Banner.png)
//!
//! <h1 align="center">Blueprint SDK</h1>
//!
//! <p align="center"><em>A comprehensive toolkit for building, deploying, and managing blueprints on the Tangle Network.</em></p>
//!
//! <p align="center">
//! <a href="https://github.com/tangle-network/blueprint/actions"><img src="https://img.shields.io/github/actions/workflow/status/tangle-network/blueprint/ci.yml?branch=main&logo=github" alt="Build Status"></a>
//! <a href="https://github.com/tangle-network/blueprint/releases"><img src="https://img.shields.io/github/v/release/tangle-network/blueprint?sort=semver&filter=blueprint-sdk-*&display_name=release" alt="Latest Release"></a>
//! <a href="https://github.com/tangle-network/blueprint/blob/main/LICENSE"><img src="https://img.shields.io/crates/l/blueprint-sdk" alt="License"></a>
//! <a href="https://discord.com/invite/cv8EfJu3Tn"><img src="https://img.shields.io/discord/833784453251596298?label=Discord" alt="Discord"></a>
//! <a href="https://t.me/tanglenet"><img src="https://img.shields.io/endpoint?color=neon&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2Ftanglenet" alt="Telegram"></a>
//! </p>
//!
//! ## Overview
//!
//! Blueprints are Infrastructure-as-Code templates that allow developers to quickly build crypto services. The Blueprint SDK
//! comes equipped with a variety of tools, from event listeners and p2p networking to flexible keystores, allowing you to rapidly
//! prototype distributed systems. With these tools, developers can get started building anything from oracles to bridge
//! security systems, zk prover networks, AI agent orchestration systems. Deploy these applications on the [Tangle Network], [Eigenlayer], or natively.
//!
//! We also have a [documentation site](https://docs.tangle.tools/) on all things Tangle to help you get started.
//!
//! [Tangle Network]: https://tangle.tools
//! [Eigenlayer]: https://eigenlayer.xyz
//!
//! ## Features
#![doc = document_features::document_features!()]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(
    html_logo_url = "https://cdn.prod.website-files.com/6494562b44a28080aafcbad4/65aaf8b0818b1d504cbdf81b_Tnt%20Logo.png"
)]
//!
//! ## Logging Targets
//!
//! The SDK is split into multiple logging targets to make debugging different components easier.
//! When testing, by default, only `ERROR`, `WARN`, and `INFO` logs will be printed. This can be controlled
//! with the [RUST_LOG] environment variable.
//!
//! An example use-case would be setting `RUST_LOG=tangle-consumer=trace` to determine the cause of a failing
//! job submission.
//!
//! ### Producers
//!
//! * `evm-polling-producer` - [`PollingProducer`]
//! * `tangle-producer` - [`TangleProducer`]
//!
//! ### Consumers
//!
//! * `tangle-consumer` - [`TangleConsumer`]
//!
//! ### Runner
//!
//! * `blueprint-runner` - [`BlueprintRunner`]
//! * `blueprint-router` - [`Router`]
//! * `blueprint-rejection` - All [`Job`] call failures
//!
//! ### Other
//!
//! * `tangle-node` - The stdout of a local Anvil/Tangle node
//!     * These are spawned by `cargo tangle blueprint run --protocol tangle` when targeting local testnets.
//! * `build-output` - The stderr of `cargo build` when deploying with [`cargo tangle`]
//!     * By default, the output of `cargo build` is hidden. If diagnosing a build error, use `RUST_LOG=build-output=debug`.
//!
//! [RUST_LOG]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives
//! [`PollingProducer`]: evm::producer::PollingProducer
//! [`TangleProducer`]: tangle::producer::TangleProducer
//! [`TangleConsumer`]: tangle::consumer::TangleConsumer
//! [`BlueprintRunner`]: runner::BlueprintRunner
//! [`cargo tangle`]: https://docs.rs/cargo_tangle

// == Core utilities ==

// Expose the core module to the outside world
pub use blueprint_core as core;
pub use core::*;

/// Core cryptographic primitives and utilities
pub use blueprint_crypto as crypto;

pub use blueprint_clients as clients;
pub use blueprint_contexts as contexts;

pub use blueprint_keystore as keystore;
pub use blueprint_qos as qos;
pub use blueprint_std as std;
pub use tokio;

pub mod error;
pub use error::Error;

/// Re-export the core extractors from the `blueprint_core` crate.
pub mod extract {
    #[cfg(feature = "macros")]
    pub use blueprint_macros::FromRef;

    pub use blueprint_core::extract::*;
}

/// Blueprint execution and runtime utilities
pub use blueprint_runner as runner;

/// Blueprint authentication proxy and utilities
pub use blueprint_auth as auth;
pub use blueprint_auth::request_auth::AuthContext;

/// Manager <-> service communication bridge
pub use blueprint_manager_bridge as bridge;

pub mod producers {
    #[cfg(feature = "cronjob")]
    pub use blueprint_producers_extra::cron::CronJob;
}

pub use blueprint_router as router;
pub use blueprint_router::Router;

pub mod registration;

#[cfg(feature = "macros")]
pub mod macros {
    pub mod context {
        pub use blueprint_context_derive::*;
    }

    pub use blueprint_macros::*;
}

// == Protocol-specific utilities ==

#[cfg(any(feature = "evm", feature = "eigenlayer", feature = "tangle"))]
pub use alloy;

#[cfg(any(feature = "evm", feature = "eigenlayer"))]
mod evm_feat {
    pub mod evm {
        pub use blueprint_evm_extra::*;
    }
}
#[cfg(any(feature = "evm", feature = "eigenlayer"))]
pub use evm_feat::*;

#[cfg(feature = "tangle")]
mod tangle_feat {
    pub mod tangle {
        pub use blueprint_tangle_extra::*;
    }
}
#[cfg(feature = "tangle")]
pub use tangle_feat::*;

#[cfg(feature = "eigenlayer")]
pub use eigensdk;

#[cfg(feature = "eigenlayer")]
pub mod eigenlayer {
    pub use blueprint_eigenlayer_extra::*;
}

// == Development utilities ==

#[cfg(feature = "testing")]
/// Testing utilities and helpers
pub mod testing {
    /// Utilities for creating and interacting with local chains
    pub mod chain_setup {
        pub use blueprint_chain_setup::*;
    }

    /// General testing utilities for blueprints
    pub mod utils {
        pub use blueprint_testing_utils::*;
    }

    /// Temporary file and directory management for tests
    pub use tempfile;
}

// Build utilities
#[cfg(feature = "build")]
/// Build-time utilities for blueprint compilation
pub mod build {
    pub use blueprint_build_utils::*;
}

// x402 payment gateway
#[cfg(feature = "x402")]
/// x402 payment gateway for cross-chain job settlement.
///
/// Enables operators to accept payments in any supported token on any supported
/// chain (Base, Ethereum, Solana, etc.) and translate them into job executions.
pub mod x402 {
    pub use blueprint_x402::*;
}

// Webhook producer for external HTTP event triggers
#[cfg(feature = "webhooks")]
/// Webhook producer for triggering jobs from external HTTP events.
///
/// Enables blueprints to receive webhooks from external services (TradingView,
/// exchange APIs, monitoring systems, etc.) and convert them into job executions.
pub mod webhooks {
    pub use blueprint_webhooks::*;
}

// Remote cloud deployment providers
#[cfg(feature = "remote-providers")]
/// Remote cloud deployment providers for Blueprint instances
pub mod remote {
    pub use blueprint_remote_providers::*;
}

#[cfg(feature = "networking")]
/// Networking utilities for blueprints
pub mod networking {
    pub use blueprint_networking::*;
    #[cfg(feature = "round-based-compat")]
    pub use blueprint_networking_round_based_extension as round_based_compat;

    /// Gossip protocol primitives (deduplication, message store, etc.)
    #[cfg(feature = "gossip")]
    pub mod gossip {
        pub use blueprint_gossip_primitives::*;
    }

    /// BLS signature aggregation via P2P gossip protocol
    #[cfg(feature = "agg-sig-gossip")]
    pub mod agg_sig_gossip {
        pub use blueprint_networking_agg_sig_gossip_extension::*;
    }
}

#[cfg(feature = "local-store")]
pub use blueprint_stores as stores;
