#![allow(dead_code)]

pub mod contexts;
pub mod error;
pub mod jobs;
#[cfg(test)]
pub mod tests;

use alloy_primitives::{Address, address};
use alloy_sol_types::sol;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::LazyLock;

pub static TASK_MANAGER_ADDRESS: LazyLock<Address> = LazyLock::new(|| {
    env::var("TASK_MANAGER_ADDRESS")
        .map(|addr| addr.parse().expect("Invalid TASK_MANAGER_ADDRESS"))
        .unwrap_or_else(|_| address!("0000000000000000000000000000000000000000"))
});
pub static PRIVATE_KEY: LazyLock<String> = LazyLock::new(|| {
    env::var("PRIVATE_KEY").unwrap_or_else(|_| {
        "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string()
    })
});
pub static AGGREGATOR_PRIVATE_KEY: LazyLock<String> = LazyLock::new(|| {
    env::var("PRIVATE_KEY").unwrap_or_else(|_| {
        "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6".to_string()
    })
});

sol!(
    #[allow(missing_docs, clippy::too_many_arguments)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    SquaringTask,
    "contracts/out/SquaringTask.sol/SquaringTask.json"
);

sol!(
    #[allow(missing_docs, clippy::too_many_arguments)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    SquaringServiceManager,
    "contracts/out/SquaringServiceManager.sol/SquaringServiceManager.json"
);
