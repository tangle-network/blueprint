//! TensorDock — GPU marketplace with a REST API. Hosts A100s and RTX GPUs at
//! marketplace prices via a JSON API.
//!
//! <https://documenter.getpostman.com/view/10839702/UVByJrCz>
//!
//! Env vars:
//! - `TENSORDOCK_API_KEY` — API key.
//! - `TENSORDOCK_API_TOKEN` — API token (TensorDock uses both).
//! - `TENSORDOCK_REGION` — optional region hint.

mod adapter;
mod instance_mapper;

pub use adapter::TensorDockAdapter;
pub use instance_mapper::TensorDockInstanceMapper;
