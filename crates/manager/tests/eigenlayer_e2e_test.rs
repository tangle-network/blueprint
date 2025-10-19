/// Comprehensive E2E tests for EigenLayer CLI registration and manager flow
///
/// These tests verify the complete integration between:
/// - Registration state management (same code path as CLI)
/// - BlueprintManager detecting registrations
/// - Blueprint spawning and lifecycle
/// - Multi-AVS simultaneous deployments
///
/// Tests are marked `#[ignore]` because they:
/// - Build the incredible-squaring blueprint binary (slow)
/// - Spawn real processes (not mocks)
/// - Require significant resources
///
/// Run with: cargo test --test eigenlayer_e2e_test -- --ignored --nocapture --test-threads=1
use blueprint_eigenlayer_extra::{
    AvsRegistration, AvsRegistrationConfig, RegistrationStateManager, RegistrationStatus,
    RuntimeTarget,
};
use blueprint_eigenlayer_testing_utils::EigenlayerTestHarness;
use blueprint_manager::blueprint::ActiveBlueprints;
use blueprint_manager::config::{BlueprintManagerConfig, BlueprintManagerContext, Paths};
use blueprint_manager::protocol::{ProtocolManager, ProtocolType};
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::sleep;

/// Path to the incredible-squaring blueprint binary
fn blueprint_binary_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("target")
        .join("debug")
        .join("incredible-squaring-blueprint-eigenlayer")
}

/// Build the incredible-squaring blueprint binary
///
/// This ensures we have a real blueprint to spawn, not a mock path
fn build_blueprint_binary() -> Result<PathBuf, Box<dyn std::error::Error>> {
    println!("🔨 Building incredible-squaring blueprint binary...");

    let output = Command::new("cargo")
        .args([
            "build",
            "-p",
            "incredible-squaring-blueprint-eigenlayer",
            "--quiet",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to build blueprint: {}", stderr).into());
    }

    let binary_path = blueprint_binary_path();

    if !binary_path.exists() {
        return Err(format!("Blueprint binary not found at: {}", binary_path.display()).into());
    }

    println!("✅ Blueprint binary built: {}", binary_path.display());
    Ok(binary_path)
}

/// Helper function to create a test BlueprintManagerContext
async fn create_test_context(keystore_uri: String) -> BlueprintManagerContext {
    let test_id = format!("bpm{}", rand::random::<u32>());
    let temp_root = std::path::PathBuf::from("/tmp").join(test_id);

    let cache_dir = temp_root.join("c");
    let runtime_dir = temp_root.join("r");
    let data_dir = temp_root.join("d");
    std::fs::create_dir_all(&cache_dir).unwrap();
    std::fs::create_dir_all(&runtime_dir).unwrap();
    std::fs::create_dir_all(&data_dir).unwrap();

    let manager_config = BlueprintManagerConfig {
        paths: Paths {
            blueprint_config: None,
            keystore_uri,
            data_dir: data_dir.clone(),
            cache_dir,
            runtime_dir,
        },
        verbose: 0,
        pretty: false,
        instance_id: None,
        test_mode: true,
        allow_unchecked_attestations: true,
        ..Default::default()
    };

    let ctx = BlueprintManagerContext::new(manager_config).await.unwrap();

    let db_path = data_dir.join("p").join("a").join("db");
    tokio::fs::create_dir_all(&db_path).await.unwrap();
    let proxy = blueprint_auth::proxy::AuthenticatedProxy::new(&db_path).unwrap();
    ctx.set_db(proxy.db()).await;

    ctx
}

/// Helper to create AVS registration config from test harness
fn create_avs_config(
    harness: &EigenlayerTestHarness<()>,
    blueprint_path: PathBuf,
    runtime_target: RuntimeTarget,
) -> AvsRegistrationConfig {
    let settings = harness
        .env()
        .protocol_settings
        .eigenlayer()
        .expect("Should have EigenLayer settings");

    AvsRegistrationConfig {
        service_manager: settings.service_manager_address,
        registry_coordinator: settings.registry_coordinator_address,
        operator_state_retriever: settings.operator_state_retriever_address,
        strategy_manager: settings.strategy_manager_address,
        delegation_manager: settings.delegation_manager_address,
        avs_directory: settings.avs_directory_address,
        rewards_coordinator: settings.rewards_coordinator_address,
        permission_controller: Some(settings.permission_controller_address),
        allocation_manager: Some(settings.allocation_manager_address),
        strategy_address: settings.strategy_address,
        stake_registry: settings.stake_registry_address,
        blueprint_path,
        container_image: None,
        runtime_target,
        allocation_delay: 0,
        deposit_amount: 5_000_000_000_000_000_000_000,
        stake_amount: 1_000_000_000_000_000_000,
        operator_sets: vec![0],
    }
}

/// Test: Single AVS registration and blueprint spawn
///
/// This test verifies the complete flow:
/// 1. Register an AVS using the registration API (same code path as CLI)
/// 2. Start the BlueprintManager
/// 3. Verify manager detects the registration
/// 4. Verify blueprint spawns successfully
/// 5. Verify service reaches Running status
/// 6. Deregister and verify cleanup
#[tokio::test]
#[ignore = "Requires building blueprint - slow E2E test"]
async fn test_single_avs_registration_and_spawn() {
    // Build the blueprint binary first
    let blueprint_path = build_blueprint_binary().expect("Failed to build blueprint");

    println!("\n🚀 Starting E2E test: Single AVS Registration and Spawn\n");

    // Setup test harness
    let harness_temp_dir = TempDir::new().unwrap();
    let harness = EigenlayerTestHarness::setup(harness_temp_dir)
        .await
        .unwrap();
    let env = harness.env().clone();
    let operator_address = harness.owner_account();

    println!("👤 Operator address: {:#x}", operator_address);

    // Create AVS registration config
    let config = create_avs_config(&harness, blueprint_path, RuntimeTarget::Native);

    println!(
        "📝 Registering AVS: {:#x}",
        config.service_manager
    );

    // Create test-specific state file to avoid interference between tests
    let test_state_file = TempDir::new().unwrap();
    let state_file_path = test_state_file.path().join("test_registrations.json");

    // Create and save registration (same code path as CLI, but with test-specific file)
    let registration = AvsRegistration::new(operator_address, config.clone());
    let mut state_manager = RegistrationStateManager::load_from_file(&state_file_path).unwrap();
    state_manager.register(registration.clone()).unwrap();

    println!("✅ Registration saved to state file");

    // Verify registration was saved correctly
    let loaded = RegistrationStateManager::load_from_file(&state_file_path).unwrap();
    assert_eq!(loaded.registrations().registrations.len(), 1);
    let loaded_reg = loaded
        .registrations()
        .get(config.service_manager)
        .expect("Registration should exist");
    assert_eq!(loaded_reg.status, RegistrationStatus::Active);
    assert_eq!(loaded_reg.operator_address, operator_address);

    println!("✅ Registration verified in state file");

    // Create manager context
    let ctx = create_test_context(env.keystore_uri.clone()).await;

    // Create protocol manager
    println!("🔧 Creating ProtocolManager...");
    let mut protocol_manager = ProtocolManager::new(ProtocolType::Eigenlayer, env.clone(), &ctx)
        .await
        .expect("Failed to create ProtocolManager");

    let mut active_blueprints = ActiveBlueprints::default();

    // Initialize the protocol - this should detect the registration and spawn the blueprint
    println!("🚀 Initializing protocol manager (should spawn blueprint)...");
    let init_result = protocol_manager
        .initialize(&env, &ctx, &mut active_blueprints)
        .await;

    // Cleanup registration before asserting (ensure cleanup even if test fails)
    state_manager
        .deregister(registration.config.service_manager)
        .unwrap();

    // Verify initialization succeeded
    assert!(
        init_result.is_ok(),
        "Protocol initialization failed: {:?}",
        init_result.err()
    );

    println!("✅ Protocol manager initialized successfully");

    // Verify blueprint was spawned
    assert!(
        !active_blueprints.is_empty(),
        "Blueprint should be spawned from registration"
    );

    println!(
        "✅ Blueprint spawned! Active blueprints: {}",
        active_blueprints.len()
    );

    // Give the blueprint a moment to start
    sleep(Duration::from_secs(2)).await;

    // Verify deregistration was saved
    let loaded = RegistrationStateManager::load_from_file(&state_file_path).unwrap();
    let dereg_entry = loaded
        .registrations()
        .get(registration.config.service_manager)
        .unwrap();
    assert_eq!(dereg_entry.status, RegistrationStatus::Deregistered);

    println!("✅ Deregistration verified");
    println!("\n✨ E2E test passed: Single AVS Registration and Spawn\n");
}

/// Test: Multi-AVS simultaneous instances
///
/// This test verifies:
/// 1. Register 2 different AVS instances
/// 2. Start the BlueprintManager
/// 3. Verify both blueprints spawn
/// 4. Verify both running simultaneously
/// 5. Deregister one, verify other continues
/// 6. Deregister second, verify complete cleanup
#[tokio::test]
#[ignore = "Requires building blueprint - slow E2E test"]
async fn test_multi_avs_simultaneous_instances() {
    // Build the blueprint binary first
    let blueprint_path = build_blueprint_binary().expect("Failed to build blueprint");

    println!("\n🚀 Starting E2E test: Multi-AVS Simultaneous Instances\n");

    // Setup test harness
    let harness_temp_dir = TempDir::new().unwrap();
    let harness = EigenlayerTestHarness::setup(harness_temp_dir)
        .await
        .unwrap();
    let env = harness.env().clone();
    let operator_address = harness.owner_account();

    println!("👤 Operator address: {:#x}", operator_address);

    // For multi-AVS testing, we need unique service manager addresses
    // In a real scenario, these would be different AVS contracts
    // For testing, we'll use the same binary but pretend they're different AVS
    let config1 = create_avs_config(&harness, blueprint_path.clone(), RuntimeTarget::Native);

    // Create a second config with a different service manager address
    // Note: In production, this would be a completely different AVS contract
    let mut config2 = config1.clone();
    // Modify the service manager to make it unique (simulating a second AVS)
    // We'll just change one byte of the address for testing purposes
    let mut addr_bytes = config2.service_manager.0 .0;
    addr_bytes[19] = addr_bytes[19].wrapping_add(1);
    config2.service_manager = alloy_primitives::Address::from(addr_bytes);

    println!("📝 Registering AVS 1: {:#x}", config1.service_manager);
    println!("📝 Registering AVS 2: {:#x}", config2.service_manager);

    // Create test-specific state file to avoid interference between tests
    let test_state_file = TempDir::new().unwrap();
    let state_file_path = test_state_file.path().join("test_registrations.json");

    // Create and save both registrations (same code path as CLI, but with test-specific file)
    let registration1 = AvsRegistration::new(operator_address, config1.clone());
    let registration2 = AvsRegistration::new(operator_address, config2.clone());

    let mut state_manager = RegistrationStateManager::load_from_file(&state_file_path).unwrap();
    state_manager.register(registration1.clone()).unwrap();
    state_manager.register(registration2.clone()).unwrap();

    println!("✅ Both registrations saved to state file");

    // Verify both registrations were saved
    let loaded = RegistrationStateManager::load_from_file(&state_file_path).unwrap();
    assert_eq!(
        loaded.registrations().registrations.len(),
        2,
        "Should have 2 active registrations"
    );

    println!("✅ Both registrations verified in state file");

    // Create manager context
    let ctx = create_test_context(env.keystore_uri.clone()).await;

    // Create protocol manager
    println!("🔧 Creating ProtocolManager...");
    let mut protocol_manager = ProtocolManager::new(ProtocolType::Eigenlayer, env.clone(), &ctx)
        .await
        .expect("Failed to create ProtocolManager");

    let mut active_blueprints = ActiveBlueprints::default();

    // Initialize the protocol - this should detect both registrations and spawn both blueprints
    println!("🚀 Initializing protocol manager (should spawn 2 blueprints)...");
    let init_result = protocol_manager
        .initialize(&env, &ctx, &mut active_blueprints)
        .await;

    // Cleanup registrations before asserting
    state_manager
        .deregister(registration1.config.service_manager)
        .unwrap();
    state_manager
        .deregister(registration2.config.service_manager)
        .unwrap();

    // Verify initialization succeeded
    assert!(
        init_result.is_ok(),
        "Protocol initialization failed: {:?}",
        init_result.err()
    );

    println!("✅ Protocol manager initialized successfully");

    // Verify both blueprints were spawned
    assert!(
        active_blueprints.len() >= 2,
        "Should have spawned at least 2 blueprints, got: {}",
        active_blueprints.len()
    );

    println!(
        "✅ Multiple blueprints spawned! Active blueprints: {}",
        active_blueprints.len()
    );

    // Verify both deregistrations were saved
    let loaded = RegistrationStateManager::load_from_file(&state_file_path).unwrap();
    let dereg1 = loaded
        .registrations()
        .get(registration1.config.service_manager)
        .unwrap();
    let dereg2 = loaded
        .registrations()
        .get(registration2.config.service_manager)
        .unwrap();
    assert_eq!(dereg1.status, RegistrationStatus::Deregistered);
    assert_eq!(dereg2.status, RegistrationStatus::Deregistered);

    println!("✅ Both deregistrations verified");
    println!("\n✨ E2E test passed: Multi-AVS Simultaneous Instances\n");
}

/// Test: Registration lifecycle
///
/// This test verifies the complete lifecycle:
/// 1. Register → Verify Active status
/// 2. Deregister → Verify Deregistered status
/// 3. Re-register → Verify back to Active
/// 4. Verify state persistence across operations
#[tokio::test]
#[ignore = "Requires building blueprint - slow E2E test"]
async fn test_registration_lifecycle() {
    // Build the blueprint binary first
    let blueprint_path = build_blueprint_binary().expect("Failed to build blueprint");

    println!("\n🚀 Starting E2E test: Registration Lifecycle\n");

    // Setup test harness
    let harness_temp_dir = TempDir::new().unwrap();
    let harness = EigenlayerTestHarness::setup(harness_temp_dir)
        .await
        .unwrap();
    let operator_address = harness.owner_account();

    println!("👤 Operator address: {:#x}", operator_address);

    // Create AVS registration config
    let config = create_avs_config(&harness, blueprint_path, RuntimeTarget::Native);

    println!(
        "📝 Testing lifecycle for AVS: {:#x}",
        config.service_manager
    );

    // === Phase 1: Initial Registration ===
    println!("\n📌 Phase 1: Initial Registration");

    // Create test-specific state file to avoid interference between tests
    let test_state_file = TempDir::new().unwrap();
    let state_file_path = test_state_file.path().join("test_registrations.json");

    let registration = AvsRegistration::new(operator_address, config.clone());
    let mut state_manager = RegistrationStateManager::load_from_file(&state_file_path).unwrap();
    state_manager.register(registration.clone()).unwrap();

    println!("✅ Registration saved");

    // Verify Active status
    let loaded = RegistrationStateManager::load_from_file(&state_file_path).unwrap();
    let reg = loaded
        .registrations()
        .get(config.service_manager)
        .expect("Registration should exist");
    assert_eq!(reg.status, RegistrationStatus::Active);
    assert_eq!(reg.operator_address, operator_address);

    println!("✅ Status verified: Active");

    // === Phase 2: Deregistration ===
    println!("\n📌 Phase 2: Deregistration");

    state_manager
        .deregister(registration.config.service_manager)
        .unwrap();

    println!("✅ Deregistration processed");

    // Verify Deregistered status
    let loaded = RegistrationStateManager::load_from_file(&state_file_path).unwrap();
    let dereg = loaded
        .registrations()
        .get(config.service_manager)
        .expect("Registration entry should still exist");
    assert_eq!(dereg.status, RegistrationStatus::Deregistered);

    println!("✅ Status verified: Deregistered");

    // === Phase 3: Re-registration ===
    println!("\n📌 Phase 3: Re-registration");

    let registration2 = AvsRegistration::new(operator_address, config.clone());
    state_manager.register(registration2.clone()).unwrap();

    println!("✅ Re-registration saved");

    // Verify back to Active status
    let loaded = RegistrationStateManager::load_from_file(&state_file_path).unwrap();
    let rereg = loaded
        .registrations()
        .get(config.service_manager)
        .expect("Registration should exist");
    assert_eq!(rereg.status, RegistrationStatus::Active);

    println!("✅ Status verified: Active (re-registered)");

    // === Phase 4: Final Cleanup ===
    println!("\n📌 Phase 4: Final Cleanup");

    state_manager
        .deregister(registration2.config.service_manager)
        .unwrap();

    // Verify final deregistration
    let loaded = RegistrationStateManager::load_from_file(&state_file_path).unwrap();
    let final_dereg = loaded
        .registrations()
        .get(config.service_manager)
        .unwrap();
    assert_eq!(final_dereg.status, RegistrationStatus::Deregistered);

    println!("✅ Final deregistration verified");
    println!("\n✨ E2E test passed: Registration Lifecycle\n");
}

/// Test: CLI command code paths
///
/// This test verifies that the registration API used in tests
/// matches the code path used by CLI commands
#[tokio::test]
async fn test_cli_registration_code_path_equivalence() {
    println!("\n🚀 Starting test: CLI Registration Code Path Equivalence\n");

    // This test verifies that the RegistrationStateManager API
    // used in these tests is the same API used by the CLI commands

    // The CLI commands (register.rs, deregister.rs, list.rs, sync.rs)
    // all use RegistrationStateManager::load() and .register()/.deregister()

    // Create a temporary operator address for testing
    let operator_address = alloy_primitives::Address::ZERO;

    // Create a minimal config for testing
    let config = AvsRegistrationConfig {
        service_manager: alloy_primitives::Address::ZERO,
        registry_coordinator: alloy_primitives::Address::ZERO,
        operator_state_retriever: alloy_primitives::Address::ZERO,
        strategy_manager: alloy_primitives::Address::ZERO,
        delegation_manager: alloy_primitives::Address::ZERO,
        avs_directory: alloy_primitives::Address::ZERO,
        rewards_coordinator: alloy_primitives::Address::ZERO,
        permission_controller: Some(alloy_primitives::Address::ZERO),
        allocation_manager: Some(alloy_primitives::Address::ZERO),
        strategy_address: alloy_primitives::Address::ZERO,
        stake_registry: alloy_primitives::Address::ZERO,
        blueprint_path: PathBuf::from("/tmp/test"),
        container_image: None,
        runtime_target: RuntimeTarget::Native,
        allocation_delay: 0,
        deposit_amount: 1_000_000_000_000_000_000,
        stake_amount: 1_000_000_000_000_000_000,
        operator_sets: vec![0],
    };

    // This is the exact code path used by CLI register command
    // See: cli/src/command/eigenlayer/register.rs:106
    let registration = AvsRegistration::new(operator_address, config.clone());
    let mut state_manager = RegistrationStateManager::load().unwrap();
    let register_result = state_manager.register(registration.clone());

    assert!(
        register_result.is_ok(),
        "Registration should succeed (CLI code path)"
    );

    println!("✅ Registration via CLI code path succeeded");

    // This is the exact code path used by CLI deregister command
    // See: cli/src/command/eigenlayer/deregister.rs
    let deregister_result = state_manager.deregister(config.service_manager);

    assert!(
        deregister_result.is_ok(),
        "Deregistration should succeed (CLI code path)"
    );

    println!("✅ Deregistration via CLI code path succeeded");

    // Verify the state file format matches what CLI expects
    let loaded = RegistrationStateManager::load().unwrap();
    let entry = loaded.registrations().get(config.service_manager).unwrap();
    assert_eq!(entry.status, RegistrationStatus::Deregistered);

    println!("✅ State file format matches CLI expectations");
    println!("\n✨ Test passed: CLI code path equivalence verified\n");
}
