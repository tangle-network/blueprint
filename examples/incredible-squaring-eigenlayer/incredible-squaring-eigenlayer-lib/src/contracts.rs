//! Contract bindings for EigenLayer integration
//!
//! This module provides type-safe bindings for interacting with EigenLayer contracts.

use blueprint_sdk::alloy::sol;
use serde::{Deserialize, Serialize};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    SquaringTask,
    "../contracts/out/SquaringTask.sol/SquaringTask.json"
);

sol!(
    #[allow(missing_docs, clippy::too_many_arguments)]
    #[sol(rpc)]
    #[derive(Debug)]
    RegistryCoordinator,
    "../contracts/out/RegistryCoordinator.sol/RegistryCoordinator.json"
);
