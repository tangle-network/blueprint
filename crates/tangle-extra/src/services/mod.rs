//! Lifecycle automation services for Tangle v2 contracts
//!
//! This module provides background keepers that automate lifecycle operations
//! on Tangle v2 contracts. These can be run by blueprint operators to ensure
//! timely execution of periodic operations.
//!
//! ## Available Keepers
//!
//! - [`EpochKeeper`] - Monitors and triggers epoch distributions on InflationPool
//! - [`RoundKeeper`] - Advances rounds on MultiAssetDelegation when ready
//! - [`StreamKeeper`] - Drips streaming payments for operators
//! - [`SubscriptionBillingKeeper`] - Bills subscription services when payment intervals elapse
//!
//! ## Usage
//!
//! ```rust,ignore
//! use blueprint_tangle_extra::services::{
//!     BackgroundKeeper, EpochKeeper, RoundKeeper, StreamKeeper,
//!     KeeperConfig, KeeperHandle,
//! };
//!
//! // Create keepers with shared config
//! let config = KeeperConfig::new(http_rpc, keystore)
//!     .with_inflation_pool(inflation_pool_address)
//!     .with_multi_asset_delegation(mad_address)
//!     .with_streaming_payment_manager(spm_address);
//!
//! // Start background services
//! let epoch_handle = EpochKeeper::start(config.clone(), shutdown.subscribe());
//! let round_handle = RoundKeeper::start(config.clone(), shutdown.subscribe());
//! let stream_handle = StreamKeeper::start(config.clone(), shutdown.subscribe());
//!
//! // Wait for shutdown
//! shutdown.send(()).ok();
//! epoch_handle.await?;
//! round_handle.await?;
//! stream_handle.await?;
//! ```

mod billing;
mod epoch;
mod keeper;
mod round;
mod stream;

pub use billing::SubscriptionBillingKeeper;
pub use epoch::EpochKeeper;
pub use keeper::{BackgroundKeeper, KeeperConfig, KeeperError, KeeperHandle, KeeperResult};
pub use round::RoundKeeper;
pub use stream::StreamKeeper;
