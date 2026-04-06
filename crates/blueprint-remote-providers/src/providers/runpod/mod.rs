//! RunPod — GPU pod cloud with secure and community tiers.
//!
//! <https://docs.runpod.io/references/api>
//!
//! Env vars:
//! - `RUNPOD_API_KEY` — API key from the RunPod console.
//! - `RUNPOD_REGION` — region hint (RunPod picks a datacenter automatically).
//! - `RUNPOD_CLOUD_TYPE` — `SECURE` or `COMMUNITY`.

mod adapter;
mod instance_mapper;

pub use adapter::RunPodAdapter;
pub use instance_mapper::RunPodInstanceMapper;
