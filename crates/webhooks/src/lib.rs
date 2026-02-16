//! # Blueprint Webhook Producer
//!
//! Trigger blueprint jobs from external HTTP webhooks (TradingView alerts,
//! price feeds, exchange notifications, monitoring systems, etc.).
//!
//! ## Architecture
//!
//! ```text
//! External Event ──► Webhook HTTP Server ──► Auth Verified ──► JobCall injected
//!                        (axum)                (HMAC/Bearer)      (Producer stream)
//!                                                                      │
//!                                                                      ▼
//!                                                              Router dispatches
//! ```
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use blueprint_webhooks::{WebhookGateway, WebhookConfig};
//! use blueprint_runner::BlueprintRunner;
//!
//! let config = WebhookConfig::from_toml("webhooks.toml")?;
//! let (gateway, producer) = WebhookGateway::new(config)?;
//!
//! BlueprintRunner::builder((), env)
//!     .router(router)
//!     .producer(producer)
//!     .background_service(gateway)
//!     .run()
//!     .await?;
//! ```
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod auth;
pub mod config;
pub mod error;
pub mod gateway;
pub mod producer;

pub use config::WebhookConfig;
pub use error::WebhookError;
pub use gateway::WebhookGateway;
pub use producer::WebhookProducer;
