//! Vast.ai — spot GPU marketplace. Instances are rented from other users'
//! hardware, so reliability varies and prices fluctuate.
//!
//! <https://vast.ai/docs/api>
//!
//! Adapter strategy: search for cheapest available GPU offer matching the
//! requested spec, place a bid within `max_price_per_hour`, wait until running.
//!
//! Env vars:
//! - `VAST_AI_API_KEY` — API key from the Vast.ai console.
//! - `VAST_AI_MAX_PRICE_PER_HOUR` — bid ceiling in USD.
//! - `VAST_AI_MIN_RELIABILITY` — minimum reliability score (0.0 - 1.0).

mod adapter;
mod instance_mapper;

pub use adapter::VastAiAdapter;
pub use instance_mapper::VastAiInstanceMapper;
