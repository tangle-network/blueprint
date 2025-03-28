use alloy_primitives::Address;
use alloy_sol_types::SolCall;
use blueprint_sdk::testing::chain_setup::anvil::get_receipt;
use blueprint_sdk::info;
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
    let allocation_manager = IAllocationManager::new(
        core_contracts.allocation_manager_impl,
        signer_wallet.clone(),
    );

    // Set Deployer Account as appointee for setAVSRegistrar on AllocationManager
    info!("Setting Deployer Account as appointee for setAVSRegistrar on AllocationManager");
    let set_appointee_call = service_manager.setAppointee(
        deployer_address,
        allocation_manager_address,
        AllocationManager::setAVSRegistrarCall::SELECTOR.into(),
    );
    let set_appointee_receipt = get_receipt(set_appointee_call).await?;
    if !set_appointee_receipt.status() {
        error!("Failed to set deployer as appointee for setAVSRegistrar on AllocationManager");
        return Err(color_eyre::eyre::eyre!("Failed to set deployer as appointee for setAVSRegistrar on AllocationManager").into());
    }

    // Set AVS Registrar
    info!("Setting AVS Registrar");
    let set_avs_registrar_call = allocation_manager
        .setAVSRegistrar(
            deployed_contracts.squaring_service_manager,
            deployed_contracts.registry_coordinator,
        );
    let set_avs_registrar_receipt = get_receipt(set_avs_registrar_call).await?;
    if !set_avs_registrar_receipt.status() {
        error!("Failed to set AVS registrar");
        return Err(color_eyre::eyre::eyre!("Failed to set AVS registrar").into());
    }

    // Set Deployer Account as appointee for createOperatorSets on AllocationManager
    info!("Setting Deployer Account as appointee for createOperatorSets on AllocationManager");
    let set_appointee_call = service_manager.setAppointee(
        deployer_address,
        allocation_manager_address,
        AllocationManager::createOperatorSetsCall::SELECTOR.into(),
    );
    let set_appointee_receipt = get_receipt(set_appointee_call).await?;
    if !set_appointee_receipt.status() {
        error!("Failed to set deployer as appointee for createOperatorSets on AllocationManager");
        return Err(color_eyre::eyre::eyre!("Failed to set deployer as appointee for createOperatorSets on AllocationManager").into());
    }

    // Set Registry Coordinator as appointee for createOperatorSets on AllocationManager
    info!("Setting Registry Coordinator as appointee for createOperatorSets on AllocationManager");
    let set_appointee_call = service_manager
        .setAppointee(
            registry_coordinator_address,
            allocation_manager_address,
            AllocationManager::createOperatorSetsCall::SELECTOR.into(),
        );
    let set_appointee_receipt = get_receipt(set_appointee_call).await?;
    if !set_appointee_receipt.status() {
        error!("Failed to set registry coordinator as appointee for createOperatorSets on AllocationManager");
        return Err(color_eyre::eyre::eyre!("Failed to set registry coordinator as appointee for createOperatorSets on AllocationManager").into());
    }

    // Set Instant Slasher as appointee for slashOperator on AllocationManager
    info!("Setting Instant Slasher as appointee for slashOperator on AllocationManager");
    let set_appointee_call = service_manager
        .setAppointee(
            slasher_address,
            allocation_manager_address,
            AllocationManager::slashOperatorCall::SELECTOR.into(),
        );
    let set_appointee_receipt = get_receipt(set_appointee_call).await?;
    if !set_appointee_receipt.status() {
        error!("Failed to set instant slasher as appointee for slashOperator on AllocationManager");
        return Err(color_eyre::eyre::eyre!("Failed to set instant slasher as appointee for slashOperator on AllocationManager").into());
    }

    // Set Deployer Account as appointee for updateAVSMetadataURI on AllocationManager
    info!("Setting Deployer Account as appointee for updateAVSMetadataURI on AllocationManager");
    let set_appointee_call = service_manager
        .setAppointee(
            deployer_address,
            allocation_manager_address,
            AllocationManager::updateAVSMetadataURICall::SELECTOR.into(),
        );
    let set_appointee_receipt = get_receipt(set_appointee_call).await?;
    if !set_appointee_receipt.status() {
        error!("Failed to set deployer as appointee for updateAVSMetadataURI on AllocationManager");
        return Err(color_eyre::eyre::eyre!("Failed to set deployer as appointee for updateAVSMetadataURI on AllocationManager").into());
    }

    // Update AVS Metadata URI
    info!("Updating AVS Metadata URI");
    let update_avs_metadata_uri_call = allocation_manager
        .updateAVSMetadataURI(service_manager_address, avs_metadata_uri);
    let update_avs_metadata_uri_receipt = get_receipt(update_avs_metadata_uri_call).await?;
    if !update_avs_metadata_uri_receipt.status() {
        error!("Failed to update AVS metadata URI");
        return Err(color_eyre::eyre::eyre!("Failed to update AVS metadata URI").into());
    }

    Ok(())
}
