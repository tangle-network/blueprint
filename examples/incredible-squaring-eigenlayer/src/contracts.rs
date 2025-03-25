//! Contract bindings for EigenLayer integration
//!
//! This module provides type-safe bindings for interacting with EigenLayer contracts.

use alloy_sol_types::sol;
use serde::{Deserialize, Serialize};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    SquaringTask,
    "contracts/out/SquaringTask.sol/SquaringTask.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    SquaringServiceManager,
    "contracts/out/SquaringServiceManager.sol/SquaringServiceManager.json"
);

pub use proxy::IServiceManager;
pub use proxy::ProxyAdmin;
pub use proxy::TransparentUpgradeableProxy;

mod proxy {
    use crate::contracts::sol;
    use serde::{Deserialize, Serialize};

    sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        #[derive(Debug, Serialize, Deserialize)]
        IServiceManager,
        "contracts/out/IServiceManager.sol/IServiceManager.json"
    );

    sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        #[derive(Debug, Serialize, Deserialize)]
        TransparentUpgradeableProxy,
        "contracts/out/TransparentUpgradeableProxy.sol/TransparentUpgradeableProxy.json"
    );

    sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        #[derive(Debug, Serialize, Deserialize)]
        ProxyAdmin,
        "contracts/out/ProxyAdmin.sol/ProxyAdmin.json"
    );
}
