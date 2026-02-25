//! # Blueprint x402 Payment Gateway
//!
//! Cross-chain EVM settlement for Blueprint SDK job execution via the
//! [x402 payment protocol](https://x402.org).
//!
//! This crate provides an HTTP server that accepts x402 payments (stablecoins on any
//! supported EVM chain) and translates them into job executions within the Blueprint
//! runner. It integrates with the existing RFQ/pricing system so that operator-signed
//! price quotes serve as chain-agnostic invoices, settled via x402 on whichever EVM
//! chain the client prefers.
//!
//! ## Architecture
//!
//! ```text
//! Client ──► x402 HTTP Server ──► Payment Verified ──► JobCall injected
//!               (axum + x402-axum)     (facilitator)       (Producer stream)
//!                                                              │
//!                                                              ▼
//!                                                      Router dispatches
//!                                                              │
//!                                                              ▼
//!                                                      Result returned
//!                                                      (HTTP 200 or receipt)
//! ```
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use blueprint_x402::{X402Gateway, X402Config};
//! use blueprint_runner::BlueprintRunner;
//! use blueprint_router::Router;
//!
//! let config = X402Config::from_toml("x402.toml")?;
//! let (gateway, producer) = X402Gateway::new(config, job_pricing)?;
//!
//! BlueprintRunner::builder((), env)
//!     .router(router)
//!     .producer(producer)
//!     .background_service(gateway)
//!     .run()
//!     .await?;
//! ```
//!
//! ## Supported Chains
//!
//! All EVM chains that have USDC/DAI with `transferWithAuthorization` (EIP-3009):
//! Base, Ethereum, Polygon, Arbitrum, Optimism, etc.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod config;
pub mod error;
pub mod gateway;
pub mod producer;
pub mod quote_registry;
pub mod settlement;

pub use config::{JobPolicyConfig, X402CallerAuthMode, X402Config, X402InvocationMode};
pub use error::X402Error;
pub use gateway::X402Gateway;
pub use producer::X402Producer;
pub use quote_registry::QuoteRegistry;
pub use settlement::SettlementOption;
