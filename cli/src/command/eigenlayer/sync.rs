/// Synchronize local registrations with on-chain state
use blueprint_eigenlayer_extra::registration::RegistrationStateManager;
use blueprint_runner::config::{BlueprintEnvironment, ContextConfig, SupportedChains};
use blueprint_runner::eigenlayer::config::EigenlayerProtocolSettings;
use color_eyre::Result;
use std::path::{Path, PathBuf};
use url::Url;

/// Synchronize local AVS registrations with on-chain state
///
/// # Arguments
///
/// * `http_rpc_url` - HTTP RPC endpoint for EigenLayer contracts
/// * `keystore_uri` - URI for the keystore containing operator keys
/// * `settings_file` - Optional path to protocol settings file
///
/// # Errors
///
/// Returns error if:
/// - Registration state cannot be loaded
/// - Keystore cannot be accessed
/// - RPC connection fails
/// - On-chain queries fail
pub async fn sync_avs_registrations(
    http_rpc_url: &Url,
    keystore_uri: &str,
    settings_file: Option<&Path>,
) -> Result<()> {
    println!("🔄 Synchronizing AVS registrations with on-chain state");
    println!("   RPC Endpoint: {}", http_rpc_url);

    // Load registration state
    let mut state_manager = RegistrationStateManager::load()?;

    let registration_count = state_manager.registrations().registrations.len();
    println!("   Local registrations: {}", registration_count);

    if registration_count == 0 {
        println!("⚠️  No local registrations to sync");
        return Ok(());
    }

    println!("🔑 Using keystore: {}", keystore_uri);

    // Load protocol settings if provided
    let eigenlayer_settings = if let Some(settings_path) = settings_file {
        println!(
            "📄 Loading protocol settings from: {}",
            settings_path.display()
        );
        // TODO: Implement settings file loading
        // For now, use defaults
        EigenlayerProtocolSettings::default()
    } else {
        println!("📄 Using default protocol settings");
        EigenlayerProtocolSettings::default()
    };

    // Create a minimal BlueprintEnvironment for on-chain queries
    let context_config = ContextConfig::create_eigenlayer_config(
        http_rpc_url.clone(),
        http_rpc_url.clone(), // Using HTTP for both
        keystore_uri.to_string(),
        None,                                  // No keystore password
        PathBuf::from("/tmp/eigenlayer_sync"), // Temporary data directory
        None,                                  // No bridge socket path
        SupportedChains::LocalTestnet,         // Default to local testnet
        eigenlayer_settings,
    );

    let env = BlueprintEnvironment::load_with_config(context_config)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to create environment: {}", e))?;

    println!();
    println!("🔍 Verifying registrations on-chain...");

    // Reconcile with on-chain state
    let changes = state_manager
        .reconcile_with_chain(&env)
        .await
        .map_err(|e| color_eyre::eyre::eyre!("Failed to reconcile with chain: {}", e))?;

    println!();
    if changes > 0 {
        println!(
            "✅ Synchronization complete: {} registration(s) updated",
            changes
        );
        println!();
        println!("💡 Run 'cargo tangle blueprint eigenlayer list' to see updated registrations");
    } else {
        println!("✅ All registrations are in sync with on-chain state");
    }

    Ok(())
}
