//! Crusoe Energy — clean-energy GPU cloud (L40S, A100, H100).
//!
//! <https://docs.crusoecloud.com/api>
//!
//! Env vars:
//! - `CRUSOE_API_KEY` — API key ID from the Crusoe console.
//! - `CRUSOE_API_SECRET` — API secret.
//! - `CRUSOE_PROJECT_ID` — project ID for resource creation.
//! - `CRUSOE_REGION` — location slug, e.g. `us-east1`, `us-central1`, `us-northwest1`.

mod adapter;
mod instance_mapper;

pub use adapter::CrusoeAdapter;
pub use instance_mapper::CrusoeInstanceMapper;
