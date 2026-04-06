//! Fluidstack — GPU-focused cloud with a REST API. Serves RTX/A100/H100 at low
//! hourly rates from a distributed datacenter pool.
//!
//! <https://docs.fluidstack.io/reference/overview>
//!
//! Env vars:
//! - `FLUIDSTACK_API_KEY` — API key from the Fluidstack dashboard.
//! - `FLUIDSTACK_REGION` — region slug, e.g. `us_east`, `eu_west`.

mod adapter;
mod instance_mapper;

pub use adapter::FluidstackAdapter;
pub use instance_mapper::FluidstackInstanceMapper;
