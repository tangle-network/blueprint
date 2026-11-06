//! RunPod — GPU pod cloud with secure and community tiers.
//!
//! <https://docs.runpod.io/api-reference-v2/pods/create-a-pod>
//!
//! Env vars:
//! - `RUNPOD_API_KEY` — API key from the RunPod console.
//! - `RUNPOD_REGION` — native data-center ID constraint.
//! - `RUNPOD_CLOUD_TYPE` — `SECURE` or `COMMUNITY`.

mod adapter;
mod instance_mapper;

pub use adapter::RunPodAdapter;
pub use instance_mapper::RunPodInstanceMapper;
