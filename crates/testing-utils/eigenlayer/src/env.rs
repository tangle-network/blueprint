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
pub const ALLOCATION_MANAGER_ADDR: Address = address!("8A791620dd6260079BF849Dc5567aDC3F2FdC318");
/// The default AVS Directory address on our testnet
pub const AVS_DIRECTORY_ADDR: Address = address!("b7f8bc63bbcad18155201308c8f3540b07f84f5e");
/// The default AVS Directory Implementation address on our testnet
pub const AVS_DIRECTORY_IMPLEMENTATION_ADDR: Address =
    address!("7a2088a1bfc9d81c55368ae168c2c02570cb814f");
/// The default Base Strategy Implementation address on our testnet
pub const BASE_STRATEGY_IMPLEMENTATION_ADDR: Address =
    address!("a85233C63b9Ee964Add6F2cffe00Fd84eb32338f");
/// The default Delayed Withdrawal Router address on our testnet
pub const DELAYED_WITHDRAWAL_ROUTER_ADDR: Address =
    address!("8A791620dd6260079BF849Dc5567aDC3F2FdC318");
/// The default Delayed Withdrawal Router Implementation address on our testnet
pub const DELAYED_WITHDRAWAL_ROUTER_IMPLEMENTATION_ADDR: Address =
    address!("9A9f2CCfdE556A7E9Ff0848998Aa4a0CFD8863AE");
/// The default Delegation address on our testnet
pub const DELEGATION_MANAGER_ADDR: Address = address!("Dc64a140Aa3E981100a9becA4E685f962f0cF6C9");
/// The default Delegation Implementation address on our testnet
pub const DELEGATION_IMPLEMENTATION_ADDR: Address =
    address!("A51c1fc2f0D1a1b8494Ed1FE312d7C3a78Ed91C0");
/// The default EigenLayer Pauser Reg address on our testnet
pub const EIGEN_LAYER_PAUSER_REG_ADDR: Address =
    address!("9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0");
/// The default EigenLayer Proxy Admin address on our testnet
pub const EIGEN_LAYER_PROXY_ADMIN_ADDR: Address =
    address!("e7f1725E7734CE288F8367e1Bb143E90bb3F0512");
/// The default EigenPod Beacon address on our testnet
pub const EIGEN_POD_BEACON_ADDR: Address = address!("B7f8BC63BbcaD18155201308C8f3540b07f84F5e");
/// The default EigenPod Implementation address on our testnet
pub const EIGEN_POD_IMPLEMENTATION_ADDR: Address =
    address!("610178dA211FEF7D417bC0e6FeD39F05609AD788");
/// The default EigenPod Manager address on our testnet
pub const EIGEN_POD_MANAGER_ADDR: Address = address!("2279B7A0a67DB372996a5FaB50D91eAA73d2eBe6");
/// The default EigenPod Manager Implementation address on our testnet
pub const EIGEN_POD_MANAGER_IMPLEMENTATION_ADDR: Address =
    address!("959922bE3CAee4b8Cd9a407cc3ac1C251C2007B1");
/// The default Empty Contract address on our testnet
pub const EMPTY_CONTRACT_ADDR: Address = address!("Cf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9");
/// The default Slasher address on our testnet
pub const SLASHER_ADDR: Address = address!("a513E6E4b8f2a923D98304ec87F64353C4D5C853");
/// The default Slasher Implementation address on our testnet
pub const SLASHER_IMPLEMENTATION_ADDR: Address =
    address!("0B306BF915C4d645ff596e518fAf3F9669b97016");
/// The default Strategy Manager address on our testnet
pub const STRATEGY_MANAGER_ADDR: Address = address!("5FC8d32690cc91D4c39d9d3abcBD16989F875707");
/// The default Strategy Manager Implementation address on our testnet
pub const STRATEGY_MANAGER_IMPLEMENTATION_ADDR: Address =
    address!("0DCd1Bf9A1b36cE34237eEaFef220932846BCD82");

/// The default ERC20 Mock address on our testnet
pub const ERC20_MOCK_ADDR: Address = address!("82e01223d51Eb87e16A03E24687EDF0F294da6f1");
/// The default ERC20 Mock Strategy address on our testnet
pub const ERC20_MOCK_STRATEGY_ADDR: Address = address!("7969c5eD335650692Bc04293B07F5BF2e7A673C0");

/// The default Operator State Retriever address on our testnet
pub const OPERATOR_STATE_RETRIEVER_ADDR: Address =
    address!("e3011a37a904ab90c8881a99bd1f6e21401f1522");
/// The default Permission Controller address on our testnet
pub const PERMISSION_CONTROLLER_ADDR: Address =
    address!("322813fd9a801c5507c9de605d63cea4f2ce6c44");
/// The default Registry Coordinator address on our testnet
pub const REGISTRY_COORDINATOR_ADDR: Address = address!("a7c59f010700930003b33ab25a7a0679c860f29c");
/// The default Rewards Coordinator address on our testnet
pub const REWARDS_COORDINATOR_ADDR: Address = address!("0dcd1bf9a1b36ce34237eeafef220932846bcd82");
/// The default Service Manager address on our testnet (Depends on AVS, this is the proxy)
pub const SERVICE_MANAGER_ADDR: Address = address!("c0f115a19107322cfbf1cdbc7ea011c19ebdb4f8");
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
        std::env::set_var("EIGEN_POD_MANAGER_ADDR", EIGEN_POD_MANAGER_ADDR.to_string());
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
            allocation_manager_address: DELAYED_WITHDRAWAL_ROUTER_ADDR,
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
