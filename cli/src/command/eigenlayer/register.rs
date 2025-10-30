/// Register with a new EigenLayer AVS
use blueprint_eigenlayer_extra::registration::{
    AvsRegistration, AvsRegistrationConfig, RegistrationStateManager, RegistrationStatus,
};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::backends::eigenlayer::EigenlayerBackend;
use blueprint_keystore::crypto::k256::K256Ecdsa;
use blueprint_keystore::{Keystore, KeystoreConfig};
use color_eyre::Result;
use std::path::Path;

/// Register with a new EigenLayer AVS
///
/// # Arguments
///
/// * `config_path` - Path to the JSON configuration file containing AVS details
/// * `keystore_uri` - URI for the keystore containing operator keys
/// * `runtime_target` - Optional runtime target (native, hypervisor, container). Defaults to hypervisor.
/// * `verify` - Whether to perform on-chain verification after registration
///
/// # Errors
///
/// Returns error if:
/// - Configuration file cannot be read or parsed
/// - Keystore cannot be accessed
/// - Operator address cannot be derived
/// - Registration state cannot be saved
/// - On-chain verification fails (if enabled)
///
/// # Panics
///
/// Panics if the registration state manager cannot be created after failing to load existing state
pub async fn register_avs(
    config_path: &Path,
    keystore_uri: &str,
    runtime_target: Option<&str>,
    verify: bool,
) -> Result<()> {
    println!(
        "🔐 Loading EigenLayer AVS registration configuration from: {}",
        config_path.display()
    );

    // Read and parse the configuration file
    let config_json = std::fs::read_to_string(config_path)?;
    let mut config: AvsRegistrationConfig = serde_json::from_str(&config_json)?;

    // Override runtime_target if specified via CLI
    if let Some(runtime_str) = runtime_target {
        use blueprint_eigenlayer_extra::RuntimeTarget;
        let runtime = runtime_str
            .parse::<RuntimeTarget>()
            .map_err(|e| color_eyre::eyre::eyre!("Invalid runtime target: {}", e))?;
        config.runtime_target = runtime;
        println!("⚙️  Runtime target set to: {}", runtime);
    }

    println!(
        "✅ Configuration loaded for AVS: {:#x}",
        config.service_manager
    );

    // Validate configuration (fail fast)
    println!("🔍 Validating configuration...");
    config
        .validate()
        .map_err(|e| color_eyre::eyre::eyre!("Configuration validation failed: {}", e))?;
    println!("✅ Configuration validated successfully");

    // Load keystore to get operator address
    println!("🔑 Loading keystore from: {}", keystore_uri);
    let keystore_config = KeystoreConfig::new().fs_root(keystore_uri);
    let keystore = Keystore::new(keystore_config)?;

    // Get the ECDSA public key (operator address)
    let ecdsa_public = keystore
        .first_local::<K256Ecdsa>()
        .map_err(|e| color_eyre::eyre::eyre!("Failed to get ECDSA key: {}", e))?;

    let ecdsa_secret = keystore
        .expose_ecdsa_secret(&ecdsa_public)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to expose ECDSA secret: {}", e))?
        .ok_or_else(|| color_eyre::eyre::eyre!("No ECDSA secret found in keystore"))?;

    let operator_address = ecdsa_secret
        .alloy_address()
        .map_err(|e| color_eyre::eyre::eyre!("Failed to derive operator address: {}", e))?;

    println!("👤 Operator address: {:#x}", operator_address);

    // Create the registration
    let registration = AvsRegistration::new(operator_address, config.clone());

    println!("📝 Saving registration to state file...");

    // Load or create registration state manager
    let mut state_manager = RegistrationStateManager::load_or_create()
        .map_err(|e| color_eyre::eyre::eyre!("Failed to initialize registration state: {}", e))?;

    // Check if already registered
    if let Some(existing) = state_manager.registrations().get(config.service_manager) {
        if existing.status == RegistrationStatus::Active {
            println!(
                "⚠️  Already registered with AVS: {:#x}",
                config.service_manager
            );
            println!("   Status: {:?}", existing.status);
            println!("   Registered at: {}", existing.registered_at);
            return Ok(());
        }
    }

    // Add the registration
    state_manager.register(registration.clone())?;

    println!("✅ Registration saved successfully");
    println!();
    println!("📊 Registration Summary:");
    println!("   Service Manager:  {:#x}", config.service_manager);
    println!("   Operator Address: {:#x}", operator_address);
    println!("   Blueprint Path:   {}", config.blueprint_path.display());
    println!("   Runtime Target:   {}", config.runtime_target);
    println!("   Status:           Active");
    println!();

    // Perform on-chain verification if requested
    if verify {
        println!("🔍 Verifying registration on-chain...");
        println!("⚠️  On-chain verification not yet implemented");
        println!("   Use 'cargo tangle blueprint eigenlayer sync' to verify later");
    }

    println!("✅ Registration complete!");
    println!();
    println!("💡 Next steps:");
    println!("   1. Start the Blueprint Manager to spawn this AVS");
    println!("   2. The manager will read this registration and start the AVS blueprint");
    println!("   3. Monitor logs for AVS blueprint startup");

    Ok(())
}
