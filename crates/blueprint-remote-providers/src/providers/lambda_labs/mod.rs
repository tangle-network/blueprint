//! Lambda Labs — on-demand GPU cloud with A100/H100 instances.
//!
//! <https://cloud.lambdalabs.com/api/v1/docs>
//!
//! API pattern: REST + Basic auth with API key as username. We represent this as
//! Bearer auth because reqwest normalizes both; Lambda Labs accepts a Bearer token.
//!
//! Env vars:
//! - `LAMBDA_LABS_API_KEY` — API key from the Lambda Labs dashboard.
//! - `LAMBDA_LABS_REGION` — default region slug, e.g. `us-west-1`, `us-east-1`.
//! - `LAMBDA_LABS_SSH_KEY_NAME` — SSH key name pre-registered in the dashboard.

mod adapter;
mod instance_mapper;

pub use adapter::LambdaLabsAdapter;
pub use instance_mapper::LambdaLabsInstanceMapper;
