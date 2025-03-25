use alloy_primitives::{Address, U256};
use alloy_provider::RootProvider;
use alloy_sol_types::SolCall;
use alloy_sol_types::sol;
use blueprint_sdk::evm::util::get_provider_from_signer;
use blueprint_sdk::info;
use blueprint_sdk::evm::util::{get_provider_http};
use blueprint_sdk::testing::chain_setup::anvil::get_receipt;
use color_eyre::eyre::eyre;
use eigensdk::utils::slashing::middleware::blsapkregistry::BLSApkRegistry;
use eigensdk::utils::slashing::middleware::indexregistry::IndexRegistry;
use eigensdk::utils::slashing::middleware::operatorstateretriever::OperatorStateRetriever;
use eigensdk::utils::slashing::middleware::stakeregistry::StakeRegistry;
use eigensdk::utils::slashing::core::istrategy::IStrategy;
use eigensdk::utils::slashing::sdk::mockerc20::MockERC20;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::contracts::{
    SquaringServiceManager, SquaringTask, 
    ProxyAdmin, TransparentUpgradeableProxy
};

// Import EigenLayer contracts using the sol! macro
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    PauserRegistry,
    "dependencies/eigenlayer-middleware-0.5.4/out/PauserRegistry.sol/PauserRegistry.json"
);

// sol!(
//     #[allow(missing_docs)]
//     #[sol(rpc)]
//     #[derive(Debug, Serialize, Deserialize)]
//     IAVSDirectory,
//     "contracts/out/IAVSDirectory.sol/IAVSDirectory.json"
// );

// sol!(
//     #[allow(missing_docs)]
//     #[sol(rpc)]
//     #[derive(Debug, Serialize, Deserialize)]
//     IDelegationManager,
//     "contracts/out/IDelegationManager.sol/IDelegationManager.json"
// );

// sol!(
//     #[allow(missing_docs)]
//     #[sol(rpc)]
//     #[derive(Debug, Serialize, Deserialize)]
//     IStrategyManager,
//     "contracts/out/IStrategyManager.sol/IStrategyManager.json"
// );

// sol!(
//     #[allow(missing_docs)]
//     #[sol(rpc)]
//     #[derive(Debug, Serialize, Deserialize)]
//     IRegistryCoordinator,
//     "contracts/out/IRegistryCoordinator.sol/IRegistryCoordinator.json"
// );

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    SlashingRegistryCoordinator,
    "contracts/out/SlashingRegistryCoordinator.sol/SlashingRegistryCoordinator.json"
);

mod interfaces {
    use super::sol;
    use super::{Serialize, Deserialize};

    sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        #[derive(Debug, Serialize, Deserialize)]
        ISlashingRegistryCoordinator,
        "contracts/out/ISlashingRegistryCoordinator.sol/ISlashingRegistryCoordinator.json"
    );
}

// sol!(
//     #[allow(missing_docs)]
//     #[sol(rpc)]
//     #[derive(Debug, Serialize, Deserialize)]
//     BLSApkRegistry,
//     "contracts/out/BLSApkRegistry.sol/BLSApkRegistry.json"
// );

// sol!(
//     #[allow(missing_docs)]
//     #[sol(rpc)]
//     #[derive(Debug, Serialize, Deserialize)]
//     IIndexRegistry,
//     "contracts/out/IIndexRegistry.sol/IIndexRegistry.json"
// );

// sol!(
//     #[allow(missing_docs)]
//     #[sol(rpc)]
//     #[derive(Debug, Serialize, Deserialize)]
//     IStakeRegistry,
//     "contracts/out/IStakeRegistry.sol/IStakeRegistry.json"
// );

// sol!(
//     #[allow(missing_docs)]
//     #[sol(rpc)]
//     #[derive(Debug, Serialize, Deserialize)]
//     OperatorStateRetriever,
//     "contracts/out/OperatorStateRetriever.sol/OperatorStateRetriever.json"
// );

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

// sol!(
//     #[allow(missing_docs)]
//     #[sol(rpc)]
//     #[derive(Debug, Serialize, Deserialize)]
//     MockERC20,
//     "contracts/out/MockERC20.sol/MockERC20.json"
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
/// * `operator_params` - Operator parameters for each quorum
/// * `operator_addr` - Address of the operator
/// * `operator_2_addr` - Address of the second operator
/// * `contracts_registry_addr` - Address of the contracts registry
/// * `task_generator_addr` - Address of the task generator
/// * `aggregator_addr` - Address of the aggregator
/// * `rewards_owner_addr` - Address of the rewards owner
/// * `rewards_initiator_addr` - Address of the rewards initiator
/// * `task_response_window_block` - Task response window in blocks
/// * `eigenlayer_addresses` - Map of EigenLayer contract addresses
///
/// # Returns
///
/// * `Result<DeployedContracts>` - The deployed contract addresses
pub async fn deploy_avs_contracts(
    http_endpoint: &str,
    private_key: &str,
    deployer_address: Address,
    num_quorums: u32,
    operator_params: Vec<u32>,
    operator_addr: Address,
    permission_controller_address: Address,
    allocation_manager_address: Address,
    task_generator_addr: Address,
    aggregator_addr: Address,
    task_response_window_block: u32,
    eigenlayer_addresses: HashMap<String, Address>,
) -> color_eyre::eyre::Result<DeployedContracts> {
    info!("Starting AVS deployment...");
    
    let provider = get_provider_http(http_endpoint);

    let wallet = get_provider_from_signer(private_key, http_endpoint);
    // let deployer_address = wallet.address();
    
    info!("Deployer address: {}", deployer_address);
    
    // Get EigenLayer contract addresses
    let avs_directory_addr = *eigenlayer_addresses.get("avsDirectory")
        .ok_or_else(|| eyre!("AVS Directory address not found"))?;
    let delegation_manager_addr = *eigenlayer_addresses.get("delegation")
        .ok_or_else(|| eyre!("Delegation Manager address not found"))?;
    let strategy_manager_addr = *eigenlayer_addresses.get("strategyManager")
        .ok_or_else(|| eyre!("Strategy Manager address not found"))?;
    let delayed_withdrawal_router_addr = *eigenlayer_addresses.get("delayedWithdrawalRouter")
        .ok_or_else(|| eyre!("Delayed Withdrawal Router address not found"))?;
    let eigen_layer_pauser_reg_addr = *eigenlayer_addresses.get("eigenLayerPauserReg")
        .ok_or_else(|| eyre!("EigenLayer Pauser Registry address not found"))?;
    let rewards_coordinator_addr = eigenlayer_addresses.get("rewardsCoordinator")
        .cloned()
        .unwrap_or(Address::ZERO);
    
    // Deploy MockERC20 token
    info!("Deploying MockERC20 token...");
    let mock_erc20 = MockERC20::deploy(&wallet).await?;
    let &mock_erc20_addr = mock_erc20.address();
    info!("MockERC20 deployed at: {}", mock_erc20_addr);
    
    // Deploy ProxyAdmin
    info!("Deploying ProxyAdmin...");
    let proxy_admin = ProxyAdmin::deploy(&wallet).await?;
    let &proxy_admin_addr = proxy_admin.address();
    info!("ProxyAdmin deployed at: {}", proxy_admin_addr);
    
    // Deploy PauserRegistry
    info!("Deploying PauserRegistry...");
    let pausers = vec![deployer_address];
    let pauser_registry = PauserRegistry::deploy(&wallet, pausers, deployer_address).await?;
    let &pauser_registry_addr = pauser_registry.address();
    info!("PauserRegistry deployed at: {}", pauser_registry_addr);
    
    // Deploy empty proxies
    info!("Deploying empty proxies...");
    let squaring_service_manager_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await.unwrap();
    let squaring_task_manager_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await.unwrap();
    let registry_coordinator_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await.unwrap();
    let bls_apk_registry_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await.unwrap();
    let index_registry_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await.unwrap();
    let stake_registry_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await.unwrap();
    let socket_registry_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await.unwrap();
    let instant_slasher_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await.unwrap();
    
    // Deploy OperatorStateRetriever
    info!("Deploying OperatorStateRetriever...");
    let operator_state_retriever = OperatorStateRetriever::deploy(&wallet).await?;
    let &operator_state_retriever_addr = operator_state_retriever.address();
    info!("OperatorStateRetriever deployed at: {}", operator_state_retriever_addr);
    
    // Deploy implementation contracts
    info!("Deploying implementation contracts...");
    
    // Deploy StakeRegistry implementation
    let stake_registry_impl = StakeRegistry::deploy(
        &wallet,
        registry_coordinator_proxy,
        delegation_manager_addr,
        avs_directory_addr,
        delayed_withdrawal_router_addr,
    ).await?;
    let &stake_registry_impl_addr = stake_registry_impl.address();
    info!("StakeRegistry implementation deployed at: {}", stake_registry_impl_addr);
    
    // Deploy BLSApkRegistry implementation
    let bls_apk_registry_impl = BLSApkRegistry::deploy(
        &wallet,
        registry_coordinator_proxy,
    ).await?;
    let &bls_apk_registry_impl_addr = bls_apk_registry_impl.address();
    info!("BLSApkRegistry implementation deployed at: {}", bls_apk_registry_impl_addr);
    
    // Deploy IndexRegistry implementation
    let index_registry_impl = IndexRegistry::deploy(
        &wallet,
        registry_coordinator_proxy,
    ).await?;
    let &index_registry_impl_addr = index_registry_impl.address();
    info!("IndexRegistry implementation deployed at: {}", index_registry_impl_addr);
    
    // Deploy InstantSlasher implementation
    let instant_slasher_impl = InstantSlasher::deploy(
        &wallet,
        delayed_withdrawal_router_addr,
        registry_coordinator_proxy,
        squaring_task_manager_proxy,
    ).await?;
    let &instant_slasher_impl_addr = instant_slasher_impl.address();
    info!("InstantSlasher implementation deployed at: {}", instant_slasher_impl_addr);
    
    // Deploy SocketRegistry implementation
    let socket_registry_impl = SocketRegistry::deploy(
        &wallet,
        registry_coordinator_proxy,
    ).await?;
    let &socket_registry_impl_addr = socket_registry_impl.address();
    info!("SocketRegistry implementation deployed at: {}", socket_registry_impl_addr);
    
    // Deploy RegistryCoordinator implementation
    let registry_coordinator_impl = SlashingRegistryCoordinator::deploy(
        &wallet,
        stake_registry_proxy,
        bls_apk_registry_proxy,
        index_registry_proxy,
        socket_registry_proxy,
        delayed_withdrawal_router_addr,
        eigen_layer_pauser_reg_addr,
        "v1.4.0-tangle-testnet".to_string(),
    ).await?;
    let &registry_coordinator_impl_addr = registry_coordinator_impl.address();
    info!("RegistryCoordinator implementation deployed at: {}", registry_coordinator_impl_addr);
    
    // Deploy SquaringServiceManager implementation
    let squaring_service_manager_impl = SquaringServiceManager::deploy(
        &wallet,
        avs_directory_addr,
        registry_coordinator_proxy,
        stake_registry_proxy,
        rewards_coordinator_addr,
        squaring_task_manager_proxy,
        permission_controller_address,
        allocation_manager_address,
    ).await?;
    let &squaring_service_manager_impl_addr = squaring_service_manager_impl.address();
    info!("SquaringServiceManager implementation deployed at: {}", squaring_service_manager_impl_addr);
    
    // Deploy SquaringTask implementation
    let squaring_task_impl = SquaringTask::deploy(
        &wallet,
        registry_coordinator_impl_addr,
        task_response_window_block,
        // task_generator_addr,
        // squaring_service_manager_proxy,
    ).await?;
    let &squaring_task_impl_addr = squaring_task_impl.address();
    info!("SquaringTask implementation deployed at: {}", squaring_task_impl_addr);
    
    // Upgrade proxies with implementations
    info!("Upgrading proxies with implementations...");
    
    // Upgrade StakeRegistry
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        stake_registry_proxy,
        stake_registry_impl_addr,
        alloy_primitives::Bytes::new(),
    ).await?;
    info!("StakeRegistry proxy upgraded");
    
    // Upgrade BLSApkRegistry
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        bls_apk_registry_proxy,
        bls_apk_registry_impl_addr,
        alloy_primitives::Bytes::new(),
    ).await?;
    info!("BLSApkRegistry proxy upgraded");
    
    // Upgrade IndexRegistry
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        index_registry_proxy,
        index_registry_impl_addr,
        alloy_primitives::Bytes::new(),
    ).await?;
    info!("IndexRegistry proxy upgraded");
    
    // Upgrade SocketRegistry
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        socket_registry_proxy,
        socket_registry_impl_addr,
        alloy_primitives::Bytes::new(),
    ).await?;
    info!("SocketRegistry proxy upgraded");
    
    // Upgrade InstantSlasher
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        instant_slasher_proxy,
        instant_slasher_impl_addr,
        alloy_primitives::Bytes::new(),
    ).await?;
    info!("InstantSlasher proxy upgraded");
    
    // Initialize RegistryCoordinator
    let registry_coordinator_init_data = SlashingRegistryCoordinator::initializeCall {
        initialOwner: deployer_address,
        churnApprover: deployer_address,
        ejector: deployer_address,
        initialPausedStatus: U256::from(0),
        avs: squaring_service_manager_proxy,
    }.abi_encode().into();

    
    // Upgrade RegistryCoordinator with initialization
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        registry_coordinator_proxy,
        registry_coordinator_impl_addr,
        registry_coordinator_init_data,
    ).await?;
    info!("RegistryCoordinator proxy upgraded and initialized");
    
    // Initialize SquaringServiceManager
    let service_manager_init_data = SquaringServiceManager::initializeCall {
        initialOwner: deployer_address,
        rewardsInitiator: deployer_address
    }.abi_encode().into();
    
    // Upgrade SquaringServiceManager with initialization
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        squaring_service_manager_proxy,
        squaring_service_manager_impl_addr,
        service_manager_init_data,
    ).await?;
    info!("SquaringServiceManager proxy upgraded and initialized");
    
    // Initialize SquaringTask
    let task_manager_init_data = SquaringTask::initializeCall {
        _aggregator: aggregator_addr,
        _generator: task_generator_addr,
        initialOwner: deployer_address
    }.abi_encode().into();
    
    // Upgrade SquaringTask with initialization
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        squaring_task_manager_proxy,
        squaring_task_impl_addr,
        task_manager_init_data,
    ).await?;
    info!("SquaringTask proxy upgraded and initialized");
    
    // Create quorums
    info!("Creating quorums...");
    let registry_coordinator = SlashingRegistryCoordinator::new(registry_coordinator_proxy, wallet.clone());

    // let quorum_operator_set_params = interfaces::ISlashingRegistryCoordinatorTypes::OperatorSetParam {
    //     maxOperatorCount: 100u32,
    //     kickBIPsOfOperatorStake: 100u16,
    //     kickBIPsOfTotalStake: 100u16,
    // };

    let strategy = IStrategy::deploy(wallet.clone()).await?;
    let strategy_addr = strategy.address().clone();

    let deployed_strategies = vec![strategy];
    let num_strategies = deployed_strategies.len();

    let mut quorums_operator_set_params = Vec::with_capacity(num_quorums as usize);
    
    for i in 0..num_quorums {
        quorums_operator_set_params.push(interfaces::ISlashingRegistryCoordinatorTypes::OperatorSetParam {
            maxOperatorCount: 100u32,
            kickBIPsOfOperatorStake: 100u16,
            kickBIPsOfTotalStake: 100u16,
        });
    }

    let mut quorums_strategy_params = Vec::with_capacity(num_quorums as usize);

    for i in 0..num_quorums {
        let operator_param = num_quorums;
        let quorum_operator_set_param = interfaces::ISlashingRegistryCoordinatorTypes::OperatorSetParam {
                maxOperatorCount: operator_param as u32,
                kickBIPsOfOperatorStake: operator_params[i as usize + 1] as u16,
                kickBIPsOfTotalStake: operator_params[i as usize + 2] as u16,
            };
            quorums_operator_set_params.push(quorum_operator_set_param);

            let mut quorum_strategy_param = Vec::with_capacity(num_strategies);
            let multiplier = alloy_primitives::Uint::<96, 2>::from(1u64);
            for j in 0..num_strategies {
                let strategy_param = interfaces::IStakeRegistryTypes::StrategyParams {
                    strategy: deployed_strategies[j].address().clone(),
                    multiplier,
                };
                quorum_strategy_param.push(strategy_param);
            }
            quorums_strategy_params.push(quorum_strategy_param);
        }

    
    // Fund operators with tokens
    info!("Funding operators with tokens...");
    let token = MockERC20::new(mock_erc20_addr, wallet);
    
    // Mint tokens to operator
    let mint_call = token.mint(operator_addr, U256::from(1000000000000000000u64));
    let mint_receipt = get_receipt(mint_call).await?;
    if !mint_receipt.status() {
        return Err(eyre!("Failed to mint tokens to operator: {}", operator_addr));
    }
    info!("Minted tokens to operator: {}", operator_addr);
    
    // // Mint tokens to operator 2
    // let _ = token.mint(operator_2_addr, U256::from(1000000000000000000u64)).await?;
    // info!("Minted tokens to operator 2: {}", operator_2_addr);
    
    info!("AVS deployment completed successfully!");
    
    // Return deployed contract addresses
    let deployed_contracts = DeployedContracts {
        proxy_admin: proxy_admin_addr,
        squaring_service_manager: squaring_service_manager_proxy,
        squaring_service_manager_impl: squaring_service_manager_impl_addr,
        squaring_task_manager: squaring_task_manager_proxy,
        registry_coordinator: registry_coordinator_proxy,
        bls_apk_registry: bls_apk_registry_proxy,
        index_registry: index_registry_proxy,
        stake_registry: stake_registry_proxy,
        operator_state_retriever: operator_state_retriever_addr,
        strategy: mock_erc20_addr,
        pauser_registry: pauser_registry_addr,
        token: mock_erc20_addr,
        instant_slasher: instant_slasher_proxy,
        socket_registry: socket_registry_proxy,
    };
    
    Ok(deployed_contracts)
}

/// Helper function to deploy an empty proxy
async fn deploy_empty_proxy(
    wallet: &RootProvider,
    proxy_admin: Address,
) -> color_eyre::eyre::Result<Address> {
    let implementation = Address::ZERO;
    let data = alloy_primitives::Bytes::new();
    
    let proxy = TransparentUpgradeableProxy::deploy(
        wallet,
        implementation,
        proxy_admin,
        data,
    ).await?;
    
    Ok(proxy.address().clone())
}

/// Helper function to upgrade a proxy with an implementation
async fn upgrade_proxy(
    wallet: &RootProvider,
    proxy_admin_addr: Address,
    proxy_addr: Address,
    implementation_addr: Address,
    data: alloy_primitives::Bytes,
) -> color_eyre::eyre::Result<()> {
    let proxy_admin = ProxyAdmin::new(proxy_admin_addr, wallet.clone());
    
    let receipt = if data.is_empty() {
        let call = proxy_admin.upgrade(proxy_addr, implementation_addr);
        get_receipt(call).await?
    } else {
        let call = proxy_admin.upgradeAndCall(proxy_addr, implementation_addr, data);
        get_receipt(call).await?
    };

    if !receipt.status() {
        return Err(color_eyre::eyre::eyre!("Failed to upgrade proxy"));
    }
    
    Ok(())
}