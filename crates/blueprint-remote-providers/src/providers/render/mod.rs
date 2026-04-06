//! Render Network Dispersed — decentralized AI compute platform launched in late
//! 2025 by the team behind Render Network's GPU rendering market.
//!
//! <https://dispersed.com/docs>
//!
//! Status: **young / experimental.** Dispersed went public in late 2025 and the
//! REST surface is still stabilizing (full v1 contract expected Q2-Q3 2026).
//! Operators should verify credentials end-to-end with a smoke deployment before
//! relying on Render in production. Endpoint shapes documented in `adapter.rs`
//! reflect the public preview spec and may shift.
//!
//! Env vars:
//! - `RENDER_API_KEY` — Bearer token from the Dispersed dashboard.
//! - `RENDER_REGION` — preferred region slug (default `na-east`).
//! - `RENDER_IMAGE` — container image to launch on the node (default Ubuntu + CUDA).
//! - `RENDER_SSH_KEY_ID` — pre-registered SSH key id from the Dispersed dashboard.
//! - `RENDER_SSH_KEY_PATH` — path to the matching private key on the orchestrator.

mod adapter;
mod instance_mapper;

pub use adapter::RenderAdapter;
pub use instance_mapper::RenderInstanceMapper;
