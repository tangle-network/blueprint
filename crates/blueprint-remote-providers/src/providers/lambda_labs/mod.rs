//! Lambda Cloud instance pricing, capacity, and lifecycle.
//! <https://docs.lambda.com/api/cloud>
//! Fixed node SKUs require an exact GPU count from the SDK catalog.
//! Credentials may be supplied by the caller or LAMBDA_LABS_API_KEY.

mod adapter;
mod instance_mapper;

pub use adapter::LambdaLabsAdapter;
pub use instance_mapper::LambdaLabsInstanceMapper;
