use alloy_primitives::Address;
use alloy_sol_types::SolCall;
use blueprint_sdk::testing::chain_setup::anvil::get_receipt;
use color_eyre::eyre::Result;
use tracing::error;

use crate::contracts::IServiceManager;
use crate::tests::core::{
    AllocationManager, DeployedCoreContracts, iallocation_manager::IAllocationManager,
};
use crate::tests::deploy::DeployedContracts;

/// Sets permissions and metadata for the AVS at the service manager of the given deployed contracts
pub async fn setup_avs_permissions(
    core_contracts: &DeployedCoreContracts,
    deployed_contracts: &DeployedContracts,
    signer_wallet: &alloy_provider::RootProvider,
    deployer_address: Address,
    avs_metadata_uri: String,
) -> Result<()> {
    let service_manager_address = deployed_contracts.squaring_service_manager;
    let registry_coordinator_address = deployed_contracts.registry_coordinator;
    let allocation_manager_address = core_contracts.allocation_manager;
    let slasher_address = deployed_contracts.instant_slasher;

    // Get contract instances
    let service_manager = IServiceManager::new(service_manager_address, signer_wallet.clone());

    let set_appointee_call = service_manager.setAppointee(
        deployer_address,
        allocation_manager_address,
        AllocationManager::setAVSRegistrarCall::SELECTOR.into(),
    );
    let set_appointee_receipt = get_receipt(set_appointee_call).await?;
    if !set_appointee_receipt.status() {
        error!("Failed to set appointee");
        return Err(color_eyre::eyre::eyre!("Failed to set appointee").into());
    }

    let allocation_manager = IAllocationManager::new(
        core_contracts.allocation_manager_impl,
        signer_wallet.clone(),
    );
    allocation_manager
        .setAVSRegistrar(
            deployed_contracts.squaring_service_manager,
            deployed_contracts.registry_coordinator,
        )
        .send()
        .await?;

    service_manager
        .setAppointee(
            registry_coordinator_address,
            allocation_manager_address,
            AllocationManager::createOperatorSetsCall::SELECTOR.into(),
        )
        .send()
        .await?;
    service_manager
        .setAppointee(
            slasher_address,
            allocation_manager_address,
            AllocationManager::slashOperatorCall::SELECTOR.into(),
        )
        .send()
        .await?;
    service_manager
        .setAppointee(
            deployer_address,
            allocation_manager_address,
            AllocationManager::updateAVSMetadataURICall::SELECTOR.into(),
        )
        .send()
        .await?;
    allocation_manager
        .updateAVSMetadataURI(service_manager_address, avs_metadata_uri)
        .send()
        .await?;

    Ok(())
}
