use alloy_primitives::{Address, U256};
use alloy_sol_types::SolCall;
use alloy_sol_types::sol;
use blueprint_sdk::evm::util::get_provider_from_signer;
use blueprint_sdk::info;
use blueprint_sdk::testing::chain_setup::anvil::get_receipt;
use color_eyre::eyre::eyre;
use eigensdk::utils::slashing::sdk::mockerc20::MockERC20;
use serde::{Deserialize, Serialize};

use crate::contracts::{ProxyAdmin, SquaringServiceManager, SquaringTask};
use crate::tests::helpers::{deploy_empty_proxy, upgrade_proxy};

// Import EigenLayer contracts using the sol! macro
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

/// Data structure to hold deployed contract addresses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployedContracts {
    /// Proxy admin address
    pub proxy_admin: Address,
    /// Squaring service manager address
    pub squaring_service_manager: Address,
    /// Squaring service manager implementation address
    pub squaring_service_manager_impl: Address,
    /// Squaring task manager address
    pub squaring_task_manager: Address,
    /// Registry coordinator address
    pub registry_coordinator: Address,
    /// BLS APK registry address
    pub bls_apk_registry: Address,
    /// Index registry address
    pub index_registry: Address,
    /// Stake registry address
    pub stake_registry: Address,
    /// Operator state retriever address
    pub operator_state_retriever: Address,
    /// Strategy address
    pub strategy: Address,
    /// Pauser registry address
    pub pauser_registry: Address,
    /// Token address
    pub token: Address,
    /// Instant slasher address
    pub instant_slasher: Address,
    /// Socket registry address
    pub socket_registry: Address,
}

/// Deploys the EigenLayer AVS contracts
///
/// This function deploys all the necessary contracts for the EigenLayer AVS
/// following the logic from the provided Solidity script.
///
/// # Arguments
///
/// * `http_endpoint` - HTTP endpoint for the RPC provider
/// * `private_key` - Private key for the deployer account
/// * `num_quorums` - Number of quorums
/// * `permission_controller_address` - Address of the permission controller
/// * `allocation_manager_address` - Address of the allocation manager
/// * `avs_directory_addr` - Address of the AVS directory
/// * `delegation_manager_addr` - Address of the delegation manager
/// * `eigen_layer_pauser_reg_addr` - Address of the EigenLayer pauser registry
/// * `rewards_coordinator_addr` - Address of the rewards coordinator
/// * `task_generator_addr` - Address of the task generator
/// * `aggregator_addr` - Address of the aggregator
/// * `task_response_window_block` - Task response window in blocks
///
/// # Returns
///
/// * `Result<DeployedContracts>` - The deployed contract addresses
#[allow(clippy::too_many_arguments)]
pub async fn deploy_avs_contracts(
    http_endpoint: &str,
    private_key: &str,
    deployer_address: Address,
    num_quorums: u32,
    permission_controller_address: Address,
    allocation_manager_address: Address,
    avs_directory_addr: Address,
    delegation_manager_addr: Address,
    pauser_registry_addr: Address,
    rewards_coordinator_addr: Address,
    strategy_factory_addr: Address,
    task_generator_addr: Address,
    aggregator_addr: Address,
    task_response_window_block: u32,
) -> color_eyre::eyre::Result<DeployedContracts> {
    info!("Starting AVS deployment...");

    let wallet = get_provider_from_signer(private_key, http_endpoint);

    info!("Deployer address: {}", deployer_address);

    info!("Initializing token...");
    let mock_erc20 = MockERC20::deploy(wallet.clone()).await?;
    // let &mock_erc20_addr = mock_erc20.address();
    let token = mock_erc20;
    // let token = MockERC20::new(TOKEN_ADDR, wallet.clone());

    let mint_call = token.mint(deployer_address, U256::from(15000000000000000000000u128));
    let mint_receipt = get_receipt(mint_call).await?;
    info!("Token mint receipt: {:?}", mint_receipt);
    if !mint_receipt.status() {
        return Err(eyre!("Failed to mint tokens to deployer"));
    }
    info!("Minted tokens to deployer: {}", deployer_address);

    let mint_call = token.mint(task_generator_addr, U256::from(30000000000000000000000u128));
    let mint_receipt = get_receipt(mint_call).await?;
    info!("Token mint receipt: {:?}", mint_receipt);
    if !mint_receipt.status() {
        return Err(eyre!("Failed to mint tokens to task generator"));
    }
    info!("Minted tokens to task generator: {}", task_generator_addr);

    let balance = token.balanceOf(deployer_address).call().await?._0;
    info!("Deployer token balance: {}", balance);
    let balance = token.balanceOf(task_generator_addr).call().await?._0;
    info!("Task generator token balance: {}", balance);

    let token_total_supply = token.totalSupply().call().await?._0;
    info!("Token total supply: {}", token_total_supply);

    let strategy_factory = StrategyFactory::new(strategy_factory_addr, wallet.clone());

    let new_strategy_call = strategy_factory.deployNewStrategy(*token.address());
    let new_strategy_receipt = get_receipt(new_strategy_call).await?;
    let strategy_addr = if let Some(last_log) = new_strategy_receipt.logs().last() {
        let data = last_log.data().data.clone();
        if data.len() >= 32 {
            // The address is in the last 20 bytes of the 32-byte data field
            let mut addr_bytes = [0u8; 20];
            addr_bytes.copy_from_slice(&data[12..32]);
            Address::from_slice(&addr_bytes)
        } else {
            return Err(color_eyre::eyre::eyre!("Invalid log data format"));
        }
    } else {
        return Err(eyre!("Failed to get strategy address from receipt"));
    };
    info!("Strategy deployed at: {}", strategy_addr);
    let squaring_strategy = IStrategy::new(strategy_addr, wallet.clone());

    // Deploy ProxyAdmin
    info!("Deploying ProxyAdmin...");
    let proxy_admin = ProxyAdmin::deploy(&wallet).await?;
    let &proxy_admin_addr = proxy_admin.address();
    info!("ProxyAdmin deployed at: {}", proxy_admin_addr);

    // First, deploy all empty proxies
    info!("Deploying empty proxies...");
    let squaring_service_manager_proxy =
        deploy_empty_proxy(&wallet, proxy_admin_addr).await.unwrap();
    let stake_registry_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await.unwrap();
    let squaring_task_manager_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await.unwrap();
    let slashing_registry_coordinator_proxy =
        deploy_empty_proxy(&wallet, proxy_admin_addr).await.unwrap();
    let bls_apk_registry_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await.unwrap();
    let index_registry_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await.unwrap();
    let socket_registry_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await.unwrap();
    let instant_slasher_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await.unwrap();

    // Deploy OperatorStateRetriever
    info!("Deploying OperatorStateRetriever...");
    let operator_state_retriever = OperatorStateRetriever::deploy(&wallet).await?;
    let &operator_state_retriever_addr = operator_state_retriever.address();
    info!(
        "OperatorStateRetriever deployed at: {}",
        operator_state_retriever_addr
    );

    // Deploy implementation contracts
    info!("Deploying implementation contracts...");

    // Deploy StakeRegistry implementation
    let stake_registry_impl = stake_registry::StakeRegistry::deploy(
        &wallet,
        slashing_registry_coordinator_proxy,
        delegation_manager_addr,
        avs_directory_addr,
        allocation_manager_address,
    )
    .await?;
    let &stake_registry_impl_addr = stake_registry_impl.address();
    info!(
        "StakeRegistry implementation deployed at: {}",
        stake_registry_impl_addr
    );

    // Deploy BLSApkRegistry implementation
    let bls_apk_registry_impl =
        bls_apk_registry::BLSApkRegistry::deploy(&wallet, slashing_registry_coordinator_proxy)
            .await?;
    let &bls_apk_registry_impl_addr = bls_apk_registry_impl.address();
    info!(
        "BLSApkRegistry implementation deployed at: {}",
        bls_apk_registry_impl_addr
    );

    // Deploy IndexRegistry implementation
    let index_registry_impl =
        IndexRegistry::deploy(&wallet, slashing_registry_coordinator_proxy).await?;
    let &index_registry_impl_addr = index_registry_impl.address();
    info!(
        "IndexRegistry implementation deployed at: {}",
        index_registry_impl_addr
    );

    // Deploy InstantSlasher implementation
    let instant_slasher_impl = InstantSlasher::deploy(
        &wallet,
        allocation_manager_address,
        slashing_registry_coordinator_proxy,
        squaring_task_manager_proxy,
    )
    .await?;
    let &instant_slasher_impl_addr = instant_slasher_impl.address();
    info!(
        "InstantSlasher implementation deployed at: {}",
        instant_slasher_impl_addr
    );

    // Deploy RegistryCoordinator implementation
    let registry_coordinator_impl = SlashingRegistryCoordinator::deploy(
        &wallet,
        stake_registry_proxy,
        bls_apk_registry_proxy,
        index_registry_proxy,
        socket_registry_proxy,
        allocation_manager_address,
        pauser_registry_addr,
        "v1.4.0-testnet-holesky".to_string(),
    )
    .await?;
    let &registry_coordinator_impl_addr = registry_coordinator_impl.address();
    info!(
        "Registry Coordinator implementation deployed at: {}",
        registry_coordinator_impl_addr
    );

    let pausers = vec![deployer_address, deployer_address];
    let pauser_registry = PauserRegistry::deploy(&wallet, pausers, deployer_address).await?;
    let &pauser_registry_addr = pauser_registry.address();
    info!("Pauser Registry deployed at: {}", pauser_registry_addr);

    let mut quorums_operator_set_params = Vec::with_capacity(num_quorums as usize);

    let mut quorums_strategy_params = Vec::with_capacity(num_quorums as usize);
    let deployed_strategies = [squaring_strategy];
    let num_strategies = deployed_strategies.len();

    for _i in 0..num_quorums {
        let quorum_operator_set_param =
            interfaces::ISlashingRegistryCoordinatorTypes::OperatorSetParam {
                maxOperatorCount: 10000u32,
                kickBIPsOfOperatorStake: 15000u16,
                kickBIPsOfTotalStake: 100u16,
            };
        quorums_operator_set_params.push(quorum_operator_set_param);

        let mut quorum_strategy_param = Vec::with_capacity(num_strategies);
        let multiplier = alloy_primitives::Uint::<96, 2>::from(1u64);
        for j in deployed_strategies.iter().take(num_strategies) {
            let strategy_param = interfaces::IStakeRegistryTypes::StrategyParams {
                strategy: *j.address(),
                multiplier,
            };
            quorum_strategy_param.push(strategy_param);
        }
        quorums_strategy_params.push(quorum_strategy_param);
    }

    // Initialize RegistryCoordinator
    let registry_coordinator_init_data = SlashingRegistryCoordinator::initializeCall {
        initialOwner: deployer_address,
        churnApprover: deployer_address,
        ejector: deployer_address,
        initialPausedStatus: U256::from(0),
        avs: squaring_service_manager_proxy,
    }
    .abi_encode()
    .into();

    // Upgrade proxies with implementations
    info!("Upgrading proxies with implementations...");

    // Upgrade StakeRegistry
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        stake_registry_proxy,
        stake_registry_impl_addr,
        alloy_primitives::Bytes::new(),
    )
    .await?;
    info!("StakeRegistry proxy upgraded");

    // Upgrade BLSApkRegistry
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        bls_apk_registry_proxy,
        bls_apk_registry_impl_addr,
        alloy_primitives::Bytes::new(),
    )
    .await?;
    info!("BLSApkRegistry proxy upgraded");

    // Upgrade IndexRegistry
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        index_registry_proxy,
        index_registry_impl_addr,
        alloy_primitives::Bytes::new(),
    )
    .await?;
    info!("IndexRegistry proxy upgraded");

    // Upgrade RegistryCoordinator with initialization
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        slashing_registry_coordinator_proxy,
        registry_coordinator_impl_addr,
        registry_coordinator_init_data,
    )
    .await?;
    info!("RegistryCoordinator proxy upgraded and initialized");

    // Deploy SquaringServiceManager implementation
    let squaring_service_manager_impl = SquaringServiceManager::deploy(
        &wallet,
        avs_directory_addr,
        slashing_registry_coordinator_proxy,
        stake_registry_proxy,
        rewards_coordinator_addr,
        squaring_task_manager_proxy,
        permission_controller_address,
        allocation_manager_address,
    )
    .await?;
    let &squaring_service_manager_impl_addr = squaring_service_manager_impl.address();
    info!(
        "SquaringServiceManager implementation deployed at: {}",
        squaring_service_manager_impl_addr
    );

    // Deploy SquaringTask implementation
    let squaring_task_impl = SquaringTask::deploy(
        &wallet,
        slashing_registry_coordinator_proxy,
        task_response_window_block,
    )
    .await?;
    let &squaring_task_impl_addr = squaring_task_impl.address();
    info!(
        "SquaringTask implementation deployed at: {}",
        squaring_task_impl_addr
    );

    // Initialize SquaringServiceManager
    let service_manager_init_data = SquaringServiceManager::initializeCall {
        initialOwner: deployer_address,
        rewardsInitiator: deployer_address,
    }
    .abi_encode()
    .into();

    // Upgrade SquaringServiceManager with initialization
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        squaring_service_manager_proxy,
        squaring_service_manager_impl_addr,
        service_manager_init_data,
    )
    .await?;
    info!("SquaringServiceManager proxy upgraded and initialized");

    // Initialize SquaringTask
    let task_manager_init_data = SquaringTask::initializeCall {
        _aggregator: aggregator_addr,
        _generator: task_generator_addr,
        initialOwner: deployer_address,
    }
    .abi_encode()
    .into();

    // Upgrade SquaringTask with initialization
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        squaring_task_manager_proxy,
        squaring_task_impl_addr,
        task_manager_init_data,
    )
    .await?;
    info!("SquaringTask proxy upgraded and initialized");

    // Upgrade InstantSlasher
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        instant_slasher_proxy,
        instant_slasher_impl_addr,
        alloy_primitives::Bytes::new(),
    )
    .await?;
    info!("InstantSlasher proxy upgraded");

    // Deploy SocketRegistry implementation
    let socket_registry_impl =
        SocketRegistry::deploy(&wallet, slashing_registry_coordinator_proxy).await?;
    let &socket_registry_impl_addr = socket_registry_impl.address();
    info!(
        "SocketRegistry implementation deployed at: {}",
        socket_registry_impl_addr
    );

    // Upgrade SocketRegistry
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        socket_registry_proxy,
        socket_registry_impl_addr,
        alloy_primitives::Bytes::new(),
    )
    .await?;
    info!("SocketRegistry proxy upgraded");

    info!("AVS deployment completed successfully!");

    // Return deployed contract addresses
    let deployed_contracts = DeployedContracts {
        proxy_admin: proxy_admin_addr,
        squaring_service_manager: squaring_service_manager_proxy,
        squaring_service_manager_impl: squaring_service_manager_impl_addr,
        squaring_task_manager: squaring_task_manager_proxy,
        registry_coordinator: slashing_registry_coordinator_proxy,
        bls_apk_registry: bls_apk_registry_proxy,
        index_registry: index_registry_proxy,
        stake_registry: stake_registry_proxy,
        operator_state_retriever: operator_state_retriever_addr,
        strategy: *deployed_strategies[0].address(),
        pauser_registry: pauser_registry_addr,
        token: *token.address(),
        instant_slasher: instant_slasher_proxy,
        socket_registry: socket_registry_proxy,
    };

    Ok(deployed_contracts)
}
