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

/// The default Allocation Manager address on our testnet
pub const ALLOCATION_MANAGER_ADDR: Address = address!("8a791620dd6260079bf849dc5567adc3f2fdc318");
/// The default AVS Directory address on our testnet
pub const AVS_DIRECTORY_ADDR: Address = address!("b7f8bc63bbcad18155201308c8f3540b07f84f5e");
/// The default Delegation Manager address on our testnet
pub const DELEGATION_MANAGER_ADDR: Address = address!("cf7ed3acca5a467e9e704c703e8d87f634fb0fc9");
/// The default ERC20 Mock address on our testnet
pub const ERC20_MOCK_ADDR: Address = address!("eC4cFde48EAdca2bC63E94BB437BbeAcE1371bF3");
/// The default Operator State Retriever address on our testnet
pub const OPERATOR_STATE_RETRIEVER_ADDR: Address =
    address!("1429859428c0abc9c2c47c8ee9fbaf82cfa0f20f");
/// The default Permission Controller address on our testnet
pub const PERMISSION_CONTROLLER_ADDR: Address =
    address!("322813fd9a801c5507c9de605d63cea4f2ce6c44");
/// The default Registry Coordinator address on our testnet
pub const REGISTRY_COORDINATOR_ADDR: Address = address!("7969c5ed335650692bc04293b07f5bf2e7a673c0");
/// The default Rewards Coordinator address on our testnet
pub const REWARDS_COORDINATOR_ADDR: Address = address!("0dcd1bf9a1b36ce34237eeafef220932846bcd82");
/// The default Service Manager address on our testnet (Depends on AVS, this is the proxy)
pub const SERVICE_MANAGER_ADDR: Address = address!("b7278a61aa25c888815afc32ad3cc52ff24fe575");
/// The default Strategy Manager address on our testnet
pub const STRATEGY_MANAGER_ADDR: Address = address!("a513e6e4b8f2a923d98304ec87f64353c4d5c853");
/// The default Stake Registry address on our testnet (Differs when using ECDSA Base)
pub const STAKE_REGISTRY_ADDR: Address = address!("82e01223d51eb87e16a03e24687edf0f294da6f1");

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
            "ALLOCATION_MANAGER_ADDR",
            ALLOCATION_MANAGER_ADDR.to_string(),
        );
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
            avs_directory_address: Address::default(),
            rewards_coordinator_address: Address::default(),
            permission_controller_address: PERMISSION_CONTROLLER_ADDR,
        },
    }
}
