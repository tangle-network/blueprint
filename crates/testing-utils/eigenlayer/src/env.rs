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
pub const ALLOCATION_MANAGER_ADDR: Address = address!("d0141e899a65c95a556fe2b27e5982a6de7fdd7a");
/// The default AVS Directory address on our testnet
pub const AVS_DIRECTORY_ADDR: Address = address!("f8e31cb472bc70500f08cd84917e5a1912ec8397");
/// The default Delegation address on our testnet
pub const DELEGATION_MANAGER_ADDR: Address = address!("cace1b78160ae76398f486c8a18044da0d66d86d");
/// The default Strategy Manager address on our testnet
pub const STRATEGY_MANAGER_ADDR: Address = address!("c96304e3c037f81da488ed9dea1d8f2a48278a75");
/// The default Strategy Factory address on our testnet
pub const STRATEGY_FACTORY_ADDR: Address = address!("3347b4d90ebe72befb30444c9966b2b990ae9fcb");
/// The default Rewards Coordinator address on our testnet
pub const REWARDS_COORDINATOR_ADDR: Address = address!("22753e4264fddc6181dc7cce468904a80a363e44");
/// The default Pauser Registry address on our testnet
pub const PAUSER_REGISTRY_ADDR: Address = address!("efab0beb0a557e452b398035ea964948c750b2fd");
/// The default Strategy Beacon address on our testnet
pub const STRATEGY_BEACON_ADDR: Address = address!("40918ba7f132e0acba2ce4de4c4baf9bd2d7d849");
/// The default Permission Controller address on our testnet
pub const PERMISSION_CONTROLLER_ADDR: Address =
    address!("3aade2dcd2df6a8cac689ee797591b2913658659");
/// The default Strategy address for our Squaring Example
pub const STRATEGY_ADDR: Address = address!("f8a8b047683062b5bbbbe9d104c9177d6b6cc086");
/// The default Token address for our Squaring Example
pub const TOKEN_ADDR: Address = address!("d42912755319665397ff090fbb63b1a31ae87cee");
/// The default Stake Registry address on our testnet (Differs when using ECDSA Base)
pub const STAKE_REGISTRY_ADDR: Address = address!("fd6f7a6a5c21a3f503ebae7a473639974379c351");

// ================= Incredible Squaring Deployment Addresses =================
/// The default Operator State Retriever address on our testnet
pub const OPERATOR_STATE_RETRIEVER_ADDR: Address =
    address!("c582bc0317dbb0908203541971a358c44b1f3766");
/// The default Registry Coordinator address on our testnet
pub const REGISTRY_COORDINATOR_ADDR: Address = address!("4bf010f1b9beda5450a8dd702ed602a104ff65ee");
/// The default Service Manager address on our testnet (Depends on AVS, this is the proxy)
pub const SERVICE_MANAGER_ADDR: Address = address!("638a246f0ec8883ef68280293ffe8cfbabe61b44");
/// The default Slasher address on our testnet
pub const SLASHER_ADDR: Address = address!("e1fd27f4390dcbe165f4d60dbf821e4b9bb02ded");

// /// The default Empty Contract address on our testnet
// pub const EMPTY_CONTRACT_ADDR: Address = address!("Cf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9");
// /// The default Slasher Implementation address on our testnet
// pub const SLASHER_IMPLEMENTATION_ADDR: Address =
//     address!("0B306BF915C4d645ff596e518fAf3F9669b97016");
// /// The default Strategy Manager Implementation address on our testnet
// pub const STRATEGY_MANAGER_IMPLEMENTATION_ADDR: Address =
//     address!("7a2088a1bfc9d81c55368ae168c2c02570cb814f");
// /// The default ERC20 Mock address on our testnet
// pub const ERC20_MOCK_ADDR: Address = address!("8f86403a4de0bb5791fa46b8e795c547942fe4cf");
// /// The default ERC20 Mock Strategy address on our testnet
// pub const ERC20_MOCK_STRATEGY_ADDR: Address = address!("7969c5eD335650692Bc04293B07F5BF2e7A673C0");

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
        strategy: TOKEN_ADDR,
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
