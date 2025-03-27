use alloy_primitives::Uint;
use alloy_primitives::{Address, address};
use alloy_provider::Provider;
use blueprint_chain_setup::anvil::get_receipt;
use blueprint_core::info;
use blueprint_evm_extra::util::get_provider_http;
use blueprint_runner::eigenlayer::config::EigenlayerProtocolSettings;
use eigensdk::utils::slashing::middleware::registrycoordinator::ISlashingRegistryCoordinatorTypes::OperatorSetParam;
use eigensdk::utils::slashing::middleware::registrycoordinator::IStakeRegistryTypes::StrategyParams;
use eigensdk::utils::slashing::middleware::registrycoordinator::RegistryCoordinator;

// ================= Core Eigenlayer Deployment Addresses =================
/// The default Allocation Manager address on our testnet
pub const ALLOCATION_MANAGER_ADDR: Address = address!("8a791620dd6260079bf849dc5567adc3f2fdc318");
/// The default AVS Directory address on our testnet
pub const AVS_DIRECTORY_ADDR: Address = address!("b7f8bc63bbcad18155201308c8f3540b07f84f5e");
/// The default Delegation address on our testnet
pub const DELEGATION_MANAGER_ADDR: Address = address!("cf7ed3acca5a467e9e704c703e8d87f634fb0fc9");
/// The default Strategy Manager address on our testnet
pub const STRATEGY_MANAGER_ADDR: Address = address!("a513e6e4b8f2a923d98304ec87f64353c4d5c853");
/// The default Strategy Factory address on our testnet
pub const STRATEGY_FACTORY_ADDR: Address = address!("68b1d87f95878fe05b998f19b66f4baba5de1aed");
/// The default Rewards Coordinator address on our testnet
pub const REWARDS_COORDINATOR_ADDR: Address = address!("0dcd1bf9a1b36ce34237eeafef220932846bcd82");
/// The default Pauser Registry address on our testnet
pub const PAUSER_REGISTRY_ADDR: Address = address!("959922be3caee4b8cd9a407cc3ac1c251c2007b1");
/// The default Strategy Beacon address on our testnet
pub const STRATEGY_BEACON_ADDR: Address = address!("9e545e3c0baab3e08cdfd552c960a1050f373042");
/// The default Permission Controller address on our testnet
pub const PERMISSION_CONTROLLER_ADDR: Address =
    address!("4ed7c70f96b99c776995fb64377f0d4ab3b0e1c1");
/// The default Strategy address for our Squaring Example
pub const STRATEGY_ADDR: Address = address!("5e3d0fde6f793b3115a9e7f5ebc195bbeed35d6c");
/// The default Token address for our Squaring Example
pub const TOKEN_ADDR: Address = address!("8f86403a4de0bb5791fa46b8e795c547942fe4cf");

// ================= Incredible Squaring Deployment Addresses =================
/// The default Operator State Retriever address on our testnet
pub const OPERATOR_STATE_RETRIEVER_ADDR: Address =
    address!("ab16a69a5a8c12c732e0deff4be56a70bb64c926");
/// The default Registry Coordinator address on our testnet
pub const REGISTRY_COORDINATOR_ADDR: Address = address!("22753e4264fddc6181dc7cce468904a80a363e44");
/// The default Service Manager address on our testnet (Depends on AVS, this is the proxy)
pub const SERVICE_MANAGER_ADDR: Address = address!("f8e31cb472bc70500f08cd84917e5a1912ec8397");

/// The default Empty Contract address on our testnet
pub const EMPTY_CONTRACT_ADDR: Address = address!("Cf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9");
/// The default Slasher address on our testnet
pub const SLASHER_ADDR: Address = address!("a513E6E4b8f2a923D98304ec87F64353C4D5C853");
/// The default Slasher Implementation address on our testnet
pub const SLASHER_IMPLEMENTATION_ADDR: Address =
    address!("0B306BF915C4d645ff596e518fAf3F9669b97016");
/// The default Strategy Manager Implementation address on our testnet
pub const STRATEGY_MANAGER_IMPLEMENTATION_ADDR: Address =
    address!("7a2088a1bfc9d81c55368ae168c2c02570cb814f");
/// The default ERC20 Mock address on our testnet
pub const ERC20_MOCK_ADDR: Address = address!("8f86403a4de0bb5791fa46b8e795c547942fe4cf");
/// The default ERC20 Mock Strategy address on our testnet
pub const ERC20_MOCK_STRATEGY_ADDR: Address = address!("7969c5eD335650692Bc04293B07F5BF2e7A673C0");
/// The default Stake Registry address on our testnet (Differs when using ECDSA Base)
pub const STAKE_REGISTRY_ADDR: Address = address!("34b40ba116d5dec75548a9e9a8f15411461e8c70");

pub struct EigenlayerTestEnvironment {
    pub http_endpoint: String,
    pub ws_endpoint: String,
    pub accounts: Vec<Address>,
    pub eigenlayer_contract_addresses: EigenlayerProtocolSettings,
}

/// Sets up the test environment for the EigenLayer Blueprint.
///
/// # Description
/// - Sets all the necessary environment variables for the necessary EigenLayer Contract Addresses.
/// - Returns a [`EigenlayerTestEnvironment`] struct containing the test environment state.
#[allow(clippy::missing_panics_doc)]
pub async fn setup_eigenlayer_test_environment(
    http_endpoint: &str,
    ws_endpoint: &str,
) -> EigenlayerTestEnvironment {
    let provider = get_provider_http(http_endpoint);

    let accounts = provider.get_accounts().await.unwrap();

    unsafe {
        std::env::set_var(
            "REGISTRY_COORDINATOR_ADDR",
            REGISTRY_COORDINATOR_ADDR.to_string(),
        );
        std::env::set_var(
            "OPERATOR_STATE_RETRIEVER_ADDR",
            OPERATOR_STATE_RETRIEVER_ADDR.to_string(),
        );
        std::env::set_var(
            "DELEGATION_MANAGER_ADDR",
            DELEGATION_MANAGER_ADDR.to_string(),
        );
        std::env::set_var(
            "PERMISSION_CONTROLLER_ADDR",
            PERMISSION_CONTROLLER_ADDR.to_string(),
        );
        std::env::set_var("SERVICE_MANAGER_ADDR", SERVICE_MANAGER_ADDR.to_string());
        std::env::set_var("STAKE_REGISTRY_ADDR", STAKE_REGISTRY_ADDR.to_string());
        std::env::set_var("STRATEGY_MANAGER_ADDR", STRATEGY_MANAGER_ADDR.to_string());
        std::env::set_var("ERC20_MOCK_ADDR", ERC20_MOCK_ADDR.to_string());
        std::env::set_var("AVS_DIRECTORY_ADDR", AVS_DIRECTORY_ADDR.to_string());
        std::env::set_var("SLASHER_ADDR", SLASHER_ADDR.to_string());
    }

    let registry_coordinator =
        RegistryCoordinator::new(REGISTRY_COORDINATOR_ADDR, provider.clone());

    let operator_set_params = OperatorSetParam {
        maxOperatorCount: 10,
        kickBIPsOfOperatorStake: 100,
        kickBIPsOfTotalStake: 1000,
    };
    let strategy_params = StrategyParams {
        strategy: ERC20_MOCK_ADDR,
        multiplier: Uint::from(1),
    };

    info!("Creating Quorum...");
    let _receipt = get_receipt(registry_coordinator.createTotalDelegatedStakeQuorum(
        operator_set_params,
        Uint::from(0),
        vec![strategy_params],
    ))
    .await
    .unwrap();

    info!("Setup Eigenlayer test environment");

    EigenlayerTestEnvironment {
        http_endpoint: http_endpoint.to_string(),
        ws_endpoint: ws_endpoint.to_string(),
        accounts,
        eigenlayer_contract_addresses: EigenlayerProtocolSettings {
            allocation_manager_address: ALLOCATION_MANAGER_ADDR,
            registry_coordinator_address: REGISTRY_COORDINATOR_ADDR,
            operator_state_retriever_address: OPERATOR_STATE_RETRIEVER_ADDR,
            delegation_manager_address: DELEGATION_MANAGER_ADDR,
            service_manager_address: SERVICE_MANAGER_ADDR,
            stake_registry_address: STAKE_REGISTRY_ADDR,
            strategy_manager_address: STRATEGY_MANAGER_ADDR,
            avs_directory_address: AVS_DIRECTORY_ADDR,
            rewards_coordinator_address: REWARDS_COORDINATOR_ADDR,
            permission_controller_address: PERMISSION_CONTROLLER_ADDR,
        },
    }
}
