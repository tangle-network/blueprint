//! Contract bindings for EigenLayer integration
//!
//! This module provides type-safe bindings for interacting with EigenLayer contracts.

use blueprint_sdk::alloy::sol;
use serde::{Deserialize, Serialize};

pub mod task_manager {
    use super::*;
    sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        #[derive(Debug, Serialize, Deserialize)]
        SquaringTask,
        "../contracts/out/SquaringTask.sol/SquaringTask.json"
    );
}

pub mod registry_coordinator {
    use super::*;
    sol!(
        #[allow(missing_docs, clippy::too_many_arguments)]
        #[sol(rpc)]
        #[derive(Debug)]
        RegistryCoordinator,
        "../contracts/out/RegistryCoordinator.sol/RegistryCoordinator.json"
    );
}
