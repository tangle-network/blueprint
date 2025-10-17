use alloy_primitives::{Address, Uint, address};
use alloy_primitives::aliases::U96;
use alloy_provider::Provider;
use blueprint_chain_setup::anvil::get_receipt;
use blueprint_core::info;
use blueprint_evm_extra::util::{get_provider_from_signer, get_provider_http};
use blueprint_runner::eigenlayer::config::EigenlayerProtocolSettings;
use eigenlayer_contract_deployer::bindings::core::registry_coordinator::ISlashingRegistryCoordinatorTypes::OperatorSetParam;
use eigenlayer_contract_deployer::bindings::core::registry_coordinator::IStakeRegistryTypes::StrategyParams;
use eigenlayer_contract_deployer::bindings::RegistryCoordinator;
use eigenlayer_contract_deployer::core::{
    deploy_core_contracts, DelegationManagerConfig, DeploymentConfigData,
    EigenPodManagerConfig, RewardsCoordinatorConfig, StrategyFactoryConfig,
    StrategyManagerConfig
};
use eigenlayer_contract_deployer::deploy::{deploy_avs_contracts, DeployedContracts};
use eigenlayer_contract_deployer::permissions::setup_avs_permissions;
use url::Url;

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

    let accounts = provider.get_accounts().await.unwrap();

    // Use Anvil's default accounts
    let owner_account = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    let task_generator_account = address!("15d34AAf54267DB7D7c367839AAf71A00a2C6A65");
    let aggregator_account = address!("a0Ee7A142d267C1f36714E4a8F75612F20a79720");
    let private_key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    info!("Deploying EigenLayer core contracts...");

    let core_config = DeploymentConfigData {
        strategy_manager: StrategyManagerConfig {
            init_paused_status: Uint::from(0),
            init_withdrawal_delay_blocks: 1u32,
        },
        delegation_manager: DelegationManagerConfig {
            init_paused_status: Uint::from(0),
            withdrawal_delay_blocks: 0u32,
        },
        eigen_pod_manager: EigenPodManagerConfig {
            init_paused_status: Uint::from(0),
        },
        rewards_coordinator: RewardsCoordinatorConfig {
            init_paused_status: Uint::from(0),
            max_rewards_duration: 864_000_u32,
            max_retroactive_length: 432_000_u32,
            max_future_length: 864_000_u32,
            genesis_rewards_timestamp: 1_672_531_200_u32,
            updater: owner_account,
            activation_delay: 0u32,
            calculation_interval_seconds: 86400u32,
            global_operator_commission_bips: 1000u16,
        },
        strategy_factory: StrategyFactoryConfig {
            init_paused_status: Uint::from(0),
        },
    };

    let core_contracts = deploy_core_contracts(
        &http_endpoint.to_string(),
        private_key,
        owner_account,
        core_config,
        Some(address!("00000000219ab540356cBB839Cbe05303d7705Fa")),
        Some(1_564_000),
    )
    .await
    .unwrap();

    info!("Deploying AVS contracts...");

    let avs_contracts = deploy_avs_contracts(
        &http_endpoint.to_string(),
        private_key,
        owner_account,
        1,
        core_contracts.permission_controller,
        core_contracts.allocation_manager,
        core_contracts.avs_directory,
        core_contracts.delegation_manager,
        core_contracts.pauser_registry,
        core_contracts.rewards_coordinator,
        core_contracts.strategy_factory,
        task_generator_account,
        aggregator_account,
        10,
    )
    .await
    .unwrap();

    let DeployedContracts {
        registry_coordinator: registry_coordinator_address,
        operator_state_retriever: operator_state_retriever_address,
        stake_registry: stake_registry_address,
        strategy: strategy_address,
        squaring_service_manager: service_manager_address,
        ..
    } = avs_contracts;

    info!("Setting AVS permissions and metadata...");
    let signer_wallet = get_provider_from_signer(private_key, http_endpoint.as_str());

    setup_avs_permissions(
        &core_contracts,
        &avs_contracts,
        &signer_wallet,
        owner_account,
        "https://github.com/tangle-network/avs/blob/main/metadata.json".to_string(),
    )
    .await
    .unwrap();

    info!("Creating quorum...");
    let registry_coordinator =
        RegistryCoordinator::new(registry_coordinator_address, signer_wallet.clone());

    let operator_set_param = OperatorSetParam {
        maxOperatorCount: 10,
        kickBIPsOfOperatorStake: 100,
        kickBIPsOfTotalStake: 1000,
    };

    let strategy_params = StrategyParams {
        strategy: strategy_address,
        multiplier: U96::from(1),
    };

    let create_quorum_call = registry_coordinator.createTotalDelegatedStakeQuorum(
        operator_set_param,
        U96::from(0),
        vec![strategy_params],
    );

    let _receipt = get_receipt(create_quorum_call).await.unwrap();

    info!("EigenLayer test environment setup complete");

    EigenlayerTestEnvironment {
        http_endpoint: http_endpoint.to_string(),
        ws_endpoint: ws_endpoint.to_string(),
        accounts,
        eigenlayer_contract_addresses: EigenlayerProtocolSettings {
            allocation_manager_address: core_contracts.allocation_manager,
            registry_coordinator_address,
            operator_state_retriever_address,
            delegation_manager_address: core_contracts.delegation_manager,
            service_manager_address,
            stake_registry_address,
            strategy_manager_address: core_contracts.strategy_manager,
            avs_directory_address: core_contracts.avs_directory,
            rewards_coordinator_address: core_contracts.rewards_coordinator,
            permission_controller_address: core_contracts.permission_controller,
            strategy_address,
        },
    }
}
