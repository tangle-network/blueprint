use alloy_primitives::Uint;
use alloy_primitives::{Address, address};
use alloy_provider::Provider;
use blueprint_chain_setup::anvil::get_receipt;
use blueprint_core::info;
use blueprint_evm_extra::util::get_provider_http;
use blueprint_runner::eigenlayer::config::EigenlayerProtocolSettings;
use eigensdk::utils::slashing::middleware::registry_coordinator::ISlashingRegistryCoordinatorTypes::OperatorSetParam;
use eigensdk::utils::slashing::middleware::registry_coordinator::IStakeRegistryTypes::StrategyParams;
use eigensdk::utils::slashing::middleware::registry_coordinator::RegistryCoordinator;
use url::Url;

/// The default Allocation Manager address
pub const ALLOCATION_MANAGER_ADDR: Address = address!("8a791620dd6260079bf849dc5567adc3f2fdc318");
/// The default AVS Directory address
pub const AVS_DIRECTORY_ADDR: Address = address!("5fc8d32690cc91d4c39d9d3abcbd16989f875707");
/// The default Delegation address
pub const DELEGATION_MANAGER_ADDR: Address = address!("cf7ed3acca5a467e9e704c703e8d87f634fb0fc9");
/// The default Strategy Manager address
pub const STRATEGY_MANAGER_ADDR: Address = address!("a513e6e4b8f2a923d98304ec87f64353c4d5c853");
/// The default Rewards Coordinator address
pub const REWARDS_COORDINATOR_ADDR: Address = address!("b7f8bc63bbcad18155201308c8f3540b07f84f5e");
/// The default Permission Controller address
pub const PERMISSION_CONTROLLER_ADDR: Address = address!("3aa5ebb10dc797cac828524e59a333d0a371443c");
/// The default Strategy address for our Squaring Example
pub const STRATEGY_ADDR: Address = address!("524f04724632eed237cba3c37272e018b3a7967e");
/// The default Stake Registry address (Differs when using ECDSA Base)
pub const STAKE_REGISTRY_ADDR: Address = address!("4c5859f0f772848b2d91f1d83e2fe57935348029");
/// The default Operator State Retriever address
pub const OPERATOR_STATE_RETRIEVER_ADDR: Address = address!("b0d4afd8879ed9f52b28595d31b441d079b2ca07");
/// The default Registry Coordinator address
pub const REGISTRY_COORDINATOR_ADDR: Address = address!("cd8a1c3ba11cf5ecfa6267617243239504a98d90");
/// The default Service Manager address (Depends on AVS, this is the proxy)
pub const SERVICE_MANAGER_ADDR: Address = address!("36c02da8a0983159322a80ffe9f24b1acff8b570");
/// The default Token address for our Squaring Example
pub const TOKEN_ADDR: Address = address!("4826533b4897376654bb4d4ad88b7fafd0c98528");

pub struct EigenlayerTestEnvironment {
    pub http_endpoint: String,
    pub ws_endpoint: String,
    pub accounts: Vec<Address>,
    pub eigenlayer_contract_addresses: EigenlayerProtocolSettings,
}

/// Sets up the test environment for the EigenLayer Blueprint.
///
/// # Description
/// - Deploys all EigenLayer contracts programmatically to the testnet
/// - Sets up AVS permissions and metadata
/// - Creates a quorum for operator registration
/// - Returns a [`EigenlayerTestEnvironment`] struct containing the test environment state.
#[allow(clippy::missing_panics_doc)]
pub async fn setup_eigenlayer_test_environment<T: TryInto<Url>, U: TryInto<Url>>(
    http_endpoint: T,
    ws_endpoint: U,
) -> EigenlayerTestEnvironment
where
    <T as TryInto<Url>>::Error: std::fmt::Debug,
    <U as TryInto<Url>>::Error: std::fmt::Debug,
{
    let http_endpoint = http_endpoint.try_into().unwrap();
    let ws_endpoint = ws_endpoint.try_into().unwrap();
    let provider = get_provider_http(http_endpoint.clone());
    // Get Anvil's accounts
    let accounts = provider.get_accounts().await.unwrap();

    unsafe {
        std::env::set_var("ALLOCATION_MANAGER_ADDR", ALLOCATION_MANAGER_ADDR.to_string());
        std::env::set_var("AVS_DIRECTORY_ADDR", AVS_DIRECTORY_ADDR.to_string());
        std::env::set_var("DELEGATION_MANAGER_ADDR", DELEGATION_MANAGER_ADDR.to_string());
        std::env::set_var("STRATEGY_MANAGER_ADDR", STRATEGY_MANAGER_ADDR.to_string());
        std::env::set_var("REWARDS_COORDINATOR_ADDR", REWARDS_COORDINATOR_ADDR.to_string());
        std::env::set_var("PERMISSION_CONTROLLER_ADDR", PERMISSION_CONTROLLER_ADDR.to_string());
        std::env::set_var("STRATEGY_ADDR", STRATEGY_ADDR.to_string());
        std::env::set_var("STAKE_REGISTRY_ADDR", STAKE_REGISTRY_ADDR.to_string());
        std::env::set_var("OPERATOR_STATE_RETRIEVER_ADDR", OPERATOR_STATE_RETRIEVER_ADDR.to_string());
        std::env::set_var("REGISTRY_COORDINATOR_ADDR", REGISTRY_COORDINATOR_ADDR.to_string());
        std::env::set_var("SERVICE_MANAGER_ADDR", SERVICE_MANAGER_ADDR.to_string());
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
            strategy_address: STRATEGY_ADDR,
            // Registration parameters (use defaults for testing)
            allocation_delay: 0,
            deposit_amount: 5_000_000_000_000_000_000_000,
            stake_amount: 1_000_000_000_000_000_000,
            operator_sets: vec![0],
            staker_opt_out_window_blocks: 50400,
            metadata_url: "https://github.com/tangle-network/blueprint".to_string(),
        }
    }
}
