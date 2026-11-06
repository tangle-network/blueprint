//! Akash Console managed-wallet API.
//!
//! [`AkashAdapter::prepare_auction`] explicitly opens an unleased auction using
//! the catalog class/count/VRAM, caller image/key, and maximum hourly spend.
//! Add the returned adapter to `CloudProvisioner::with_providers` to compare its
//! current bids with other providers. Close an unselected auction with
//! `close_auction`. No blueprint needs its own rental HTTP or SDL builder.
//!
//! `quote` reads exact live bids and current chain timestamps without creating
//! resources. Only native USD-denominated ACT-credit bids are selectable;
//! historical GPU price distributions and unconverted AKT bids are excluded.
//! Allocation records the deployment id before accepting a bid. Retirement
//! closes that deployment and verifies its chain state through the Console API.
//!
//! <https://akash.network/docs/api-documentation/console-api/getting-started/>

mod adapter;
mod instance_mapper;
mod sdl;
pub use adapter::AkashAdapter;
pub use instance_mapper::AkashInstanceMapper;
