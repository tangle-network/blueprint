/// Deregister from an EigenLayer AVS
use alloy_primitives::Address;
use blueprint_eigenlayer_extra::registration::{RegistrationStateManager, RegistrationStatus};
use color_eyre::Result;
use std::str::FromStr;

/// Deregister from an EigenLayer AVS
///
/// # Arguments
///
/// * `service_manager_str` - Service manager contract address (hex string)
/// * `keystore_uri` - URI for the keystore (currently unused, for future on-chain deregistration)
///
/// # Errors
///
/// Returns error if:
/// - Service manager address is invalid
/// - Registration state cannot be loaded
/// - AVS is not registered
/// - State cannot be saved
pub async fn deregister_avs(service_manager_str: &str, _keystore_uri: &str) -> Result<()> {
    println!("🔓 Deregistering from EigenLayer AVS");

    // Parse service manager address
    let service_manager = Address::from_str(service_manager_str)
        .map_err(|e| color_eyre::eyre::eyre!("Invalid service manager address: {}", e))?;

    println!("   Service Manager: {:#x}", service_manager);

    // Load registration state
    let mut state_manager = RegistrationStateManager::load()?;

    // Check if registered
    let registration = state_manager
        .registrations()
        .get(service_manager)
        .ok_or_else(|| {
            color_eyre::eyre::eyre!("Not registered with AVS: {:#x}", service_manager)
        })?;

    if registration.status == RegistrationStatus::Deregistered {
        println!("⚠️  AVS already deregistered");
        return Ok(());
    }

    println!();
    println!("📊 AVS Details:");
    println!("   Operator:         {:#x}", registration.operator_address);
    println!("   Registered at:    {}", registration.registered_at);
    println!(
        "   Blueprint Path:   {}",
        registration.config.blueprint_path.display()
    );

    // Mark as deregistered
    state_manager.deregister(service_manager)?;

    println!();
    println!("✅ Successfully deregistered from AVS");
    println!();
    println!("💡 Note:");
    println!("   - The AVS blueprint will stop on next Blueprint Manager restart");
    println!("   - To remove completely, delete from state file manually");
    println!("   - For on-chain deregistration, use the operator lifecycle commands");

    Ok(())
}
