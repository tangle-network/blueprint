pub use blueprint_core_testing_utils::Error;
use tangle_subxt::subxt_core::utils::AccountId32;

#[cfg(test)]
mod tests;

pub mod blueprint;
pub mod harness;
pub mod keys;
pub mod multi_node;
pub mod runner;

// Re-export commonly used types
pub use harness::TangleTestHarness;

pub type InputValue = tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::field::Field<AccountId32>;
pub type OutputValue = tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::field::Field<AccountId32>;
