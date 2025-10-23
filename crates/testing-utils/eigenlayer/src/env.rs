use alloy_primitives::Uint;
use alloy_primitives::{Address, address};
use blueprint_chain_setup::anvil::get_receipt;
use blueprint_core::info;
use blueprint_evm_extra::util::get_provider_http;
use blueprint_runner::eigenlayer::config::EigenlayerProtocolSettings;
use eigensdk::utils::slashing::middleware::registry_coordinator::ISlashingRegistryCoordinatorTypes::OperatorSetParam;
use eigensdk::utils::slashing::middleware::registry_coordinator::IStakeRegistryTypes::StrategyParams;
use eigensdk::utils::slashing::middleware::registry_coordinator::RegistryCoordinator;
use url::Url;

/// The default Token address for our Squaring Example
pub const TOKEN_ADDR: Address = address!("4826533b4897376654bb4d4ad88b7fafd0c98528");

/// Sets up the test environment for the EigenLayer Blueprint for non-empty Anvil testnet
///
/// # Description
/// - Creates a quorum for operator registration
/// - Returns a [`EigenlayerProtocolSettings`] struct containing the test environment state.
#[allow(clippy::missing_panics_doc)]
pub async fn setup_eigenlayer_test_environment<T: TryInto<Url>>(
    http_endpoint: T,
) -> EigenlayerProtocolSettings
where
    <T as TryInto<Url>>::Error: std::fmt::Debug
{
    let http_endpoint = http_endpoint.try_into().unwrap();
    let provider = get_provider_http(http_endpoint.clone());

    let default_eigenlayer_protocol_settings = EigenlayerProtocolSettings::default();

    let registry_coordinator =
        RegistryCoordinator::new(default_eigenlayer_protocol_settings.registry_coordinator_address, provider.clone());

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

    default_eigenlayer_protocol_settings
}
