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

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    PauserRegistry,
    "dependencies/eigenlayer-middleware-0.5.4/out/PauserRegistry.sol/PauserRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    EmptyContract,
    "dependencies/eigenlayer-middleware-0.5.4/out/EmptyContract.sol/EmptyContract.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    SlashingRegistryCoordinator,
    "contracts/out/SlashingRegistryCoordinator.sol/SlashingRegistryCoordinator.json"
);

pub mod registry_coordinator {
    use super::sol;
    use super::{Deserialize, Serialize};

    sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        #[derive(Debug, Serialize, Deserialize)]
        RegistryCoordinator,
        "contracts/out/RegistryCoordinator.sol/RegistryCoordinator.json"
    );
}

mod interfaces {
    use super::sol;
    use super::{Deserialize, Serialize};

    sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        #[derive(Debug, Serialize, Deserialize)]
        ISlashingRegistryCoordinator,
        "contracts/out/ISlashingRegistryCoordinator.sol/ISlashingRegistryCoordinator.json"
    );
}

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    InstantSlasher,
    "dependencies/eigenlayer-middleware-0.5.4/out/InstantSlasher.sol/InstantSlasher.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    SocketRegistry,
    "dependencies/eigenlayer-middleware-0.5.4/out/SocketRegistry.sol/SocketRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    StrategyFactory,
    "dependencies/eigenlayer-middleware-0.5.4/out/StrategyFactory.sol/StrategyFactory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    StrategyManager,
    "dependencies/eigenlayer-middleware-0.5.4/out/StrategyManager.sol/StrategyManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    IStrategy,
    "dependencies/eigenlayer-middleware-0.5.4/out/IStrategy.sol/IStrategy.json"
);

pub mod stake_registry {
    use super::sol;
    use super::{Deserialize, Serialize};

    sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        #[derive(Debug, Serialize, Deserialize)]
        StakeRegistry,
        "dependencies/eigenlayer-middleware-0.5.4/out/StakeRegistry.sol/StakeRegistry.json"
    );
}

pub mod bls_apk_registry {
    use super::sol;
    use super::{Deserialize, Serialize};

    sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        #[derive(Debug, Serialize, Deserialize)]
        BLSApkRegistry,
        "dependencies/eigenlayer-middleware-0.5.4/out/BLSApkRegistry.sol/BLSApkRegistry.json"
    );
}
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    IndexRegistry,
    "dependencies/eigenlayer-middleware-0.5.4/out/IndexRegistry.sol/IndexRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    OperatorStateRetriever,
    "dependencies/eigenlayer-middleware-0.5.4/out/OperatorStateRetriever.sol/OperatorStateRetriever.json"
);

// sol!(
//     #[allow(missing_docs)]
//     #[sol(rpc)]
//     #[derive(Debug, Serialize, Deserialize)]
//     StrategyBeacon,
//     "dependencies/eigenlayer-middleware-0.5.4/out/IBeacon.sol/IBeacon.json"
// );

// sol!(
//     #[allow(missing_docs)]
//     #[sol(rpc)]
//     #[derive(Debug, Serialize, Deserialize)]
//     MockERC20,
//     "dependencies/eigenlayer-middleware-0.5.4/out/MockERC20.sol/MockERC20.json"
// );