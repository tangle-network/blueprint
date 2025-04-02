use crate::bindings::core::rewardscoordinator::IRewardsCoordinatorTypes::RewardsCoordinatorConstructorParams;
use crate::bindings::{
    AVSDirectory, AllocationManager, DelegationManager, EigenPod, EigenPodManager, PauserRegistry,
    PermissionController, ProxyAdmin, RewardsCoordinator, StrategyBase, StrategyFactory,
    StrategyManager, UpgradeableBeacon,
};
use crate::helpers::{deploy_empty_proxy, upgrade_proxy};
use alloy_primitives::{Address, Bytes, U256};
use alloy_sol_types::SolCall;
use blueprint_sdk::evm::util::get_provider_from_signer;
use blueprint_sdk::info;
use serde::{Deserialize, Serialize};

pub const MIDDLEWARE_VERSION: &str = "v1.4.0-testnet-holesky";

/// Contract Addresses of EigenLayer core contracts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployedCoreContracts {
    /// Proxy admin address
    pub proxy_admin: Address,
    /// Delegation manager address
    pub delegation_manager: Address,
    /// Delegation manager implementation address
    pub delegation_manager_impl: Address,
    /// AVS directory address
    pub avs_directory: Address,
    /// AVS directory implementation address
    pub avs_directory_impl: Address,
    /// Strategy manager address
    pub strategy_manager: Address,
    /// Strategy manager implementation address
    pub strategy_manager_impl: Address,
    /// `EigenPod` manager address
    pub eigen_pod_manager: Address,
    /// `EigenPod` manager implementation address
    pub eigen_pod_manager_impl: Address,
    /// Allocation manager address
    pub allocation_manager: Address,
    /// Allocation manager implementation address
    pub allocation_manager_impl: Address,
    /// Rewards coordinator address
    pub rewards_coordinator: Address,
    /// Rewards coordinator implementation address
    pub rewards_coordinator_impl: Address,
    /// `EigenPod` beacon address
    pub eigen_pod_beacon: Address,
    /// Pauser registry address
    pub pauser_registry: Address,
    /// Strategy factory address
    pub strategy_factory: Address,
    /// Strategy factory implementation address
    pub strategy_factory_impl: Address,
    /// Strategy beacon address
    pub strategy_beacon: Address,
    /// Permission controller address
    pub permission_controller: Address,
    /// Permission controller implementation address
    pub permission_controller_impl: Address,
}

/// Configuration for the strategy manager
#[derive(Debug, Clone)]
pub struct StrategyManagerConfig {
    /// Initial paused status
    pub init_paused_status: U256,
    /// Initial withdrawal delay blocks
    pub init_withdrawal_delay_blocks: u32,
}

/// Configuration for the delegation manager
#[derive(Debug, Clone)]
pub struct DelegationManagerConfig {
    /// Initial paused status
    pub init_paused_status: U256,
    /// Withdrawal delay blocks
    pub withdrawal_delay_blocks: u32,
}

/// Configuration for the `EigenPod` manager
#[derive(Debug, Clone)]
pub struct EigenPodManagerConfig {
    /// Initial paused status
    pub init_paused_status: U256,
}

/// Configuration for the rewards coordinator
#[derive(Debug, Clone)]
pub struct RewardsCoordinatorConfig {
    /// Initial paused status
    pub init_paused_status: U256,
    /// Maximum rewards duration
    pub max_rewards_duration: u32,
    /// Maximum retroactive length
    pub max_retroactive_length: u32,
    /// Maximum future length
    pub max_future_length: u32,
    /// Genesis rewards timestamp
    pub genesis_rewards_timestamp: u32,
    /// Updater address
    pub updater: Address,
    /// Activation delay
    pub activation_delay: u32,
    /// Calculation interval seconds
    pub calculation_interval_seconds: u32,
    /// Global operator commission in basis points
    pub global_operator_commission_bips: u16,
}

/// Configuration for the strategy factory
#[derive(Debug, Clone)]
pub struct StrategyFactoryConfig {
    /// Initial paused status
    pub init_paused_status: U256,
}

/// Configuration for all core contracts
#[derive(Debug, Clone)]
pub struct DeploymentConfigData {
    /// Strategy manager configuration
    pub strategy_manager: StrategyManagerConfig,
    /// Delegation manager configuration
    pub delegation_manager: DelegationManagerConfig,
    /// `EigenPod` manager configuration
    pub eigen_pod_manager: EigenPodManagerConfig,
    /// Rewards coordinator configuration
    pub rewards_coordinator: RewardsCoordinatorConfig,
    /// Strategy factory configuration
    pub strategy_factory: StrategyFactoryConfig,
}

/// Deploys the EigenLayer core contracts
///
/// This function deploys all the necessary core contracts for EigenLayer
/// following the logic from the provided Solidity script.
///
/// # Arguments
///
/// * `http_endpoint` - HTTP endpoint for the RPC provider
/// * `private_key` - Private key for the deployer account
/// * `deployer_address` - Deployer address
/// * `config_data` - Configuration data for the deployment
/// * `eth_pos_deposit` - Address of the ETH POS deposit contract (optional, for mainnet)
/// * `genesis_time` - Genesis time for the `EigenPod` (optional)
///
/// # Errors
/// Returns an error if any of the contract deployments fail or if any of the contract calls fail.
#[allow(clippy::too_many_lines)]
pub async fn deploy_core_contracts(
    http_endpoint: &str,
    private_key: &str,
    deployer_address: Address,
    config_data: DeploymentConfigData,
    eth_pos_deposit: Option<Address>,
    genesis_time: Option<u64>,
) -> color_eyre::eyre::Result<DeployedCoreContracts> {
    info!("Starting EigenLayer core contracts deployment...");

    let wallet = get_provider_from_signer(private_key, http_endpoint);

    info!("Deployer address: {}", deployer_address);

    // Deploy ProxyAdmin
    let proxy_admin = ProxyAdmin::deploy(&wallet).await?;
    let &proxy_admin_addr = proxy_admin.address();
    info!("ProxyAdmin deployed at: {}", proxy_admin_addr);

    // Deploy empty proxies
    info!("Deploying empty proxies...");
    let delegation_manager_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await?;
    info!(
        "DelegationManager proxy deployed at: {}",
        delegation_manager_proxy
    );
    let avs_directory_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await?;
    info!("AVSDirectory proxy deployed at: {}", avs_directory_proxy);
    let strategy_manager_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await?;
    info!(
        "StrategyManager proxy deployed at: {}",
        strategy_manager_proxy
    );
    let allocation_manager_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await?;
    info!(
        "AllocationManager proxy deployed at: {}",
        allocation_manager_proxy
    );
    let rewards_coordinator_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await?;
    info!(
        "RewardsCoordinator proxy deployed at: {}",
        rewards_coordinator_proxy
    );
    let eigen_pod_beacon_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await?;
    info!(
        "EigenPodBeacon proxy deployed at: {}",
        eigen_pod_beacon_proxy
    );
    let strategy_factory_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await?;
    info!(
        "StrategyFactory proxy deployed at: {}",
        strategy_factory_proxy
    );
    let eigen_pod_manager_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await?;
    info!(
        "EigenPodManager proxy deployed at: {}",
        eigen_pod_manager_proxy
    );
    let permission_controller_proxy = deploy_empty_proxy(&wallet, proxy_admin_addr).await?;
    info!(
        "PermissionController proxy deployed at: {}",
        permission_controller_proxy
    );

    // Deploy PauserRegistry
    let pauser_registry =
        PauserRegistry::deploy(&wallet, Vec::<Address>::new(), proxy_admin_addr).await?;
    let &pauser_registry_addr = pauser_registry.address();
    info!("PauserRegistry deployed at: {}", pauser_registry_addr);

    // Deploy implementation contracts
    info!("Deploying implementation contracts...");

    // Deploy DelegationManager implementation
    let delegation_manager_impl = DelegationManager::deploy(
        &wallet,
        strategy_manager_proxy,
        eigen_pod_manager_proxy,
        allocation_manager_proxy,
        pauser_registry_addr,
        permission_controller_proxy,
        0u32, // minWithdrawalDelay
        MIDDLEWARE_VERSION.to_string(),
    )
    .await?;
    let &delegation_manager_impl_addr = delegation_manager_impl.address();
    info!(
        "DelegationManager implementation deployed at: {}",
        delegation_manager_impl_addr
    );

    // Deploy PermissionController implementation
    let permission_controller_impl =
        PermissionController::deploy(&wallet, MIDDLEWARE_VERSION.to_string()).await?;
    let &permission_controller_impl_addr = permission_controller_impl.address();
    info!(
        "PermissionController implementation deployed at: {}",
        permission_controller_impl_addr
    );

    // Deploy AVSDirectory implementation
    let avs_directory_impl = AVSDirectory::deploy(
        &wallet,
        delegation_manager_proxy,
        pauser_registry_addr,
        MIDDLEWARE_VERSION.to_string(),
    )
    .await?;
    let &avs_directory_impl_addr = avs_directory_impl.address();
    info!(
        "AVSDirectory implementation deployed at: {}",
        avs_directory_impl_addr
    );

    // Deploy StrategyManager implementation
    let strategy_manager_impl = StrategyManager::deploy(
        &wallet,
        delegation_manager_proxy,
        pauser_registry_addr,
        MIDDLEWARE_VERSION.to_string(),
    )
    .await?;
    let &strategy_manager_impl_addr = strategy_manager_impl.address();
    info!(
        "StrategyManager implementation deployed at: {}",
        strategy_manager_impl_addr
    );

    // Deploy StrategyFactory implementation
    let strategy_factory_impl = StrategyFactory::deploy(
        &wallet,
        strategy_manager_proxy,
        pauser_registry_addr,
        MIDDLEWARE_VERSION.to_string(),
    )
    .await?;
    let &strategy_factory_impl_addr = strategy_factory_impl.address();
    info!(
        "StrategyFactory implementation deployed at: {}",
        strategy_factory_impl_addr
    );

    // Deploy AllocationManager implementation
    let allocation_manager_impl = AllocationManager::deploy(
        &wallet,
        delegation_manager_proxy,
        pauser_registry_addr,
        permission_controller_proxy,
        0u32, // _DEALLOCATION_DELAY
        0u32, // _ALLOCATION_CONFIGURATION_DELAY
        MIDDLEWARE_VERSION.to_string(),
    )
    .await?;
    let &allocation_manager_impl_addr = allocation_manager_impl.address();
    info!(
        "AllocationManager implementation deployed at: {}",
        allocation_manager_impl_addr
    );

    // Determine ETH POS deposit address
    let eth_pos_deposit_addr = match eth_pos_deposit {
        Some(addr) => addr,
        None => {
            // For non-mainnet chains, you might want to deploy a mock
            // This is a placeholder address, should be replaced with actual implementation
            Address::from_slice(&[0u8; 20])
        }
    };

    // Deploy EigenPodManager implementation
    let eigen_pod_manager_impl = EigenPodManager::deploy(
        &wallet,
        eth_pos_deposit_addr,
        eigen_pod_beacon_proxy,
        delegation_manager_proxy,
        pauser_registry_addr,
        MIDDLEWARE_VERSION.to_string(),
    )
    .await?;
    let &eigen_pod_manager_impl_addr = eigen_pod_manager_impl.address();
    info!(
        "EigenPodManager implementation deployed at: {}",
        eigen_pod_manager_impl_addr
    );

    // Configure RewardsCoordinator parameters
    let calculation_interval_seconds = config_data.rewards_coordinator.calculation_interval_seconds;
    let max_rewards_duration = config_data.rewards_coordinator.max_rewards_duration;
    let max_retroactive_length = config_data.rewards_coordinator.max_retroactive_length;
    let max_future_length = config_data.rewards_coordinator.max_future_length;
    let genesis_rewards_timestamp = config_data.rewards_coordinator.genesis_rewards_timestamp;

    // Deploy RewardsCoordinator implementation
    let rewards_coordinator_impl = RewardsCoordinator::deploy(
        &wallet,
        RewardsCoordinatorConstructorParams {
            delegationManager: delegation_manager_proxy,
            strategyManager: strategy_manager_proxy,
            allocationManager: allocation_manager_proxy,
            pauserRegistry: pauser_registry_addr,
            permissionController: permission_controller_proxy,
            version: MIDDLEWARE_VERSION.to_string(),
            CALCULATION_INTERVAL_SECONDS: calculation_interval_seconds,
            MAX_REWARDS_DURATION: max_rewards_duration,
            MAX_RETROACTIVE_LENGTH: max_retroactive_length,
            MAX_FUTURE_LENGTH: max_future_length,
            GENESIS_REWARDS_TIMESTAMP: genesis_rewards_timestamp,
        },
    )
    .await?;
    let &rewards_coordinator_impl_addr = rewards_coordinator_impl.address();
    info!(
        "RewardsCoordinator implementation deployed at: {}",
        rewards_coordinator_impl_addr
    );

    // Determine genesis time for EigenPod
    let genesis_time_value = genesis_time.unwrap_or(1_564_000);

    // Deploy EigenPod implementation
    let eigen_pod_impl = EigenPod::deploy(
        &wallet,
        eth_pos_deposit_addr,
        eigen_pod_manager_proxy,
        genesis_time_value,
        MIDDLEWARE_VERSION.to_string(),
    )
    .await?;
    let &eigen_pod_impl_addr = eigen_pod_impl.address();
    info!(
        "EigenPod implementation deployed at: {}",
        eigen_pod_impl_addr
    );

    // Deploy StrategyBase implementation
    let base_strategy_impl = StrategyBase::deploy(
        &wallet,
        strategy_manager_proxy,
        pauser_registry_addr,
        MIDDLEWARE_VERSION.to_string(),
    )
    .await?;
    let &base_strategy_impl_addr = base_strategy_impl.address();
    info!(
        "StrategyBase implementation deployed at: {}",
        base_strategy_impl_addr
    );

    // Deploy and configure the strategy beacon
    let strategy_beacon = UpgradeableBeacon::deploy(&wallet, base_strategy_impl_addr).await?;
    let &strategy_beacon_addr = strategy_beacon.address();
    info!("StrategyBeacon deployed at: {}", strategy_beacon_addr);

    // Upgrade contracts with implementations and initialize them
    info!("Upgrading proxies with implementations and initializing...");

    // Upgrade DelegationManager
    let delegation_manager_init_data = DelegationManager::initializeCall {
        initialOwner: deployer_address,
        initialPausedStatus: config_data.delegation_manager.init_paused_status,
    }
    .abi_encode()
    .into();

    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        delegation_manager_proxy,
        delegation_manager_impl_addr,
        delegation_manager_init_data,
    )
    .await?;
    info!("DelegationManager proxy upgraded and initialized");

    // Upgrade StrategyManager
    let strategy_manager_init_data = StrategyManager::initializeCall {
        initialOwner: deployer_address,
        initialStrategyWhitelister: strategy_factory_proxy,
        initialPausedStatus: config_data.strategy_manager.init_paused_status,
    }
    .abi_encode()
    .into();

    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        strategy_manager_proxy,
        strategy_manager_impl_addr,
        strategy_manager_init_data,
    )
    .await?;
    info!("StrategyManager proxy upgraded and initialized");

    // Upgrade PermissionController
    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        permission_controller_proxy,
        permission_controller_impl_addr,
        Bytes::new(),
    )
    .await?;
    info!("PermissionController proxy upgraded");

    // Upgrade StrategyFactory
    let strategy_factory_init_data = StrategyFactory::initializeCall {
        _initialOwner: deployer_address,
        _initialPausedStatus: config_data.strategy_factory.init_paused_status,
        _strategyBeacon: strategy_beacon_addr,
    }
    .abi_encode()
    .into();

    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        strategy_factory_proxy,
        strategy_factory_impl_addr,
        strategy_factory_init_data,
    )
    .await?;
    info!("StrategyFactory proxy upgraded and initialized");

    // Upgrade EigenPodManager
    let eigen_pod_manager_init_data = EigenPodManager::initializeCall {
        initialOwner: deployer_address,
        _initPausedStatus: config_data.eigen_pod_manager.init_paused_status,
    }
    .abi_encode()
    .into();

    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        eigen_pod_manager_proxy,
        eigen_pod_manager_impl_addr,
        eigen_pod_manager_init_data,
    )
    .await?;
    info!("EigenPodManager proxy upgraded and initialized");

    // Upgrade AVSDirectory
    let avs_directory_init_data = AVSDirectory::initializeCall {
        initialOwner: deployer_address,
        initialPausedStatus: U256::from(0), // Using 0 as default
    }
    .abi_encode()
    .into();

    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        avs_directory_proxy,
        avs_directory_impl_addr,
        avs_directory_init_data,
    )
    .await?;
    info!("AVSDirectory proxy upgraded and initialized");

    // Upgrade RewardsCoordinator
    let rewards_coordinator_init_data = RewardsCoordinator::initializeCall {
        initialOwner: deployer_address,
        initialPausedStatus: config_data.rewards_coordinator.init_paused_status,
        _rewardsUpdater: deployer_address, // Using deployer as rewards updater
        _activationDelay: config_data.rewards_coordinator.activation_delay,
        _defaultSplitBips: config_data
            .rewards_coordinator
            .global_operator_commission_bips,
    }
    .abi_encode()
    .into();

    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        rewards_coordinator_proxy,
        rewards_coordinator_impl_addr,
        rewards_coordinator_init_data,
    )
    .await?;
    info!("RewardsCoordinator proxy upgraded and initialized");

    // Upgrade EigenPod beacon
    let eigen_pod_init_data = EigenPod::initializeCall {
        _podOwner: eigen_pod_manager_proxy,
    }
    .abi_encode()
    .into();

    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        eigen_pod_beacon_proxy,
        eigen_pod_impl_addr,
        eigen_pod_init_data,
    )
    .await?;
    info!("EigenPod beacon proxy upgraded and initialized");

    // Upgrade AllocationManager
    let allocation_manager_init_data = AllocationManager::initializeCall {
        initialOwner: deployer_address,
        initialPausedStatus: config_data.delegation_manager.init_paused_status,
    }
    .abi_encode()
    .into();

    upgrade_proxy(
        &wallet,
        proxy_admin_addr,
        allocation_manager_proxy,
        allocation_manager_impl_addr,
        allocation_manager_init_data,
    )
    .await?;
    info!("AllocationManager proxy upgraded and initialized");

    info!("EigenLayer core contracts deployment completed successfully!");

    // Return deployed contract addresses
    let deployed_contracts = DeployedCoreContracts {
        proxy_admin: proxy_admin_addr,
        delegation_manager: delegation_manager_proxy,
        delegation_manager_impl: delegation_manager_impl_addr,
        avs_directory: avs_directory_proxy,
        avs_directory_impl: avs_directory_impl_addr,
        strategy_manager: strategy_manager_proxy,
        strategy_manager_impl: strategy_manager_impl_addr,
        eigen_pod_manager: eigen_pod_manager_proxy,
        eigen_pod_manager_impl: eigen_pod_manager_impl_addr,
        allocation_manager: allocation_manager_proxy,
        allocation_manager_impl: allocation_manager_impl_addr,
        rewards_coordinator: rewards_coordinator_proxy,
        rewards_coordinator_impl: rewards_coordinator_impl_addr,
        eigen_pod_beacon: eigen_pod_beacon_proxy,
        pauser_registry: pauser_registry_addr,
        strategy_factory: strategy_factory_proxy,
        strategy_factory_impl: strategy_factory_impl_addr,
        strategy_beacon: strategy_beacon_addr,
        permission_controller: permission_controller_proxy,
        permission_controller_impl: permission_controller_impl_addr,
    };

    Ok(deployed_contracts)
}
