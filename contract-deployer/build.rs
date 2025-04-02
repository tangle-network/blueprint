use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

#[allow(clippy::too_many_lines, clippy::format_push_string)]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let contract_dirs = vec![
        "./dependencies/eigenlayer-middleware-0.5.4/lib/eigenlayer-contracts",
        "./dependencies/eigenlayer-middleware-0.5.4",
        "./contracts",
    ];
    // blueprint_build_utils::soldeer_install();
    // blueprint_build_utils::soldeer_update();
    blueprint_build_utils::build_contracts(contract_dirs);

    // Create bindings directory
    let src_dir = Path::new("src");
    let bindings_dir = src_dir.join("bindings");

    // Remove existing bindings directory if it exists
    if bindings_dir.exists() {
        fs::remove_dir_all(&bindings_dir).unwrap();
    }
    fs::create_dir_all(&bindings_dir).unwrap();

    // Define contracts from contracts directory
    let contracts_contracts = ["SquaringTask", "SquaringServiceManager"];

    // Define contracts from eigenlayer-middleware directory
    let middleware_contracts = [
        "IAllocationManager",
        "AllocationManager",
        "AVSDirectory",
        "BLSApkRegistry",
        "DelegationManager",
        "EigenPod",
        "EigenPodManager",
        "EmptyContract",
        "ISlashingRegistryCoordinator",
        "IndexRegistry",
        "InstantSlasher",
        "OperatorStateRetriever",
        "PauserRegistry",
        "ProxyAdmin",
        "PermissionController",
        "RegistryCoordinator",
        "RewardsCoordinator",
        "IServiceManager",
        "SlashingRegistryCoordinator",
        "SocketRegistry",
        "StakeRegistry",
        "StrategyBase",
        "StrategyFactory",
        "StrategyManager",
        "TransparentUpgradeableProxy",
        "UpgradeableBeacon",
        "IStrategy",
    ];

    // Generate bindings for contracts directory
    println!("Generating bindings for contracts...");

    // Build the command with all the select flags
    let mut cmd = Command::new("forge");
    cmd.args([
        "bind",
        "--alloy",
        "--skip-build",
        "--evm-version",
        "shanghai",
        "--bindings-path",
        "src/bindings/deploy",
        "--overwrite",
        "--root",
        "./contracts",
        "--module",
    ]);

    // Add select flags for each contract
    for contract in &contracts_contracts {
        cmd.args(["--select", &format!("^{}$", contract)]);
    }

    let status = cmd
        .status()
        .expect("Failed to execute forge bind command for contracts");

    assert!(status.success());

    // Generate bindings for middleware directory
    println!("Generating bindings for middleware...");

    // Build the command with all the select flags
    let mut cmd = Command::new("forge");
    cmd.args([
        "bind",
        "--alloy",
        "--skip-build",
        "--evm-version",
        "shanghai",
        "--bindings-path",
        "src/bindings/core",
        "--overwrite",
        "--root",
        "./dependencies/eigenlayer-middleware-0.5.4",
        "--module",
    ]);

    // Add select flags for each contract
    for contract in &middleware_contracts {
        cmd.args(["--select", &format!("^{}$", contract)]);
    }

    let status = cmd
        .status()
        .expect("Failed to execute forge bind command for middleware");

    assert!(status.success());

    // Post-process the generated files to add the required imports
    println!("Post-processing generated files...");

    // Process deploy contracts
    for contract in &contracts_contracts {
        let lower_contract = contract.to_lowercase();
        let file_path = format!("src/bindings/deploy/{}.rs", lower_contract);
        add_imports_to_file(&file_path, contract);
    }

    // Process middleware contracts
    for contract in &middleware_contracts {
        let lower_contract = contract.to_lowercase();
        let file_path = format!("src/bindings/core/{}.rs", lower_contract);
        add_imports_to_file(&file_path, contract);
    }

    // Create the mod.rs in the bindings directory
    let mut contents = String::new();
    contents.push_str("pub mod core;\n");
    contents.push_str("pub mod deploy;\n");
    contents.push('\n');

    for contract in &contracts_contracts {
        let lower_contract = contract.to_lowercase();
        contents.push_str(&format!(
            "pub use deploy::{}::{};\n",
            lower_contract, contract
        ));
    }
    for contract in &middleware_contracts {
        let lower_contract = contract.to_lowercase();
        contents.push_str(&format!(
            "pub use core::{}::{};\n",
            lower_contract, contract
        ));
    }

    let path = Path::new("src/bindings/mod.rs");
    fs::write(path, contents).expect("Failed to write to mod.rs");

    // Create core/mod.rs to re-export OperatorSet
    let mut core_mod_contents = String::new();
    core_mod_contents.push_str("// This file is generated by the build script\n");
    core_mod_contents.push_str("// Do not edit manually\n\n");

    // Add all modules
    for contract in &middleware_contracts {
        let lower_contract = contract.to_lowercase();
        core_mod_contents.push_str(&format!("pub mod {};\n", lower_contract));
    }

    // Re-export OperatorSet from AllocationManager
    core_mod_contents.push_str("\n// Re-export OperatorSet for use across modules\n");
    core_mod_contents
        .push_str("pub use self::allocationmanager::AllocationManager::OperatorSet;\n");

    let core_mod_path = Path::new("src/bindings/core/mod.rs");
    fs::write(core_mod_path, core_mod_contents).expect("Failed to write to core/mod.rs");

    // Create deploy/mod.rs
    let mut deploy_mod_contents = String::new();
    deploy_mod_contents.push_str("// This file is generated by the build script\n");
    deploy_mod_contents.push_str("// Do not edit manually\n\n");

    // Add all modules
    for contract in &contracts_contracts {
        let lower_contract = contract.to_lowercase();
        deploy_mod_contents.push_str(&format!("pub mod {};\n", lower_contract));
    }

    // Import OperatorSet from core
    deploy_mod_contents.push_str("\n// Import OperatorSet from core\n");
    deploy_mod_contents.push_str("pub use crate::bindings::core::OperatorSet;\n");

    let deploy_mod_path = Path::new("src/bindings/deploy/mod.rs");
    fs::write(deploy_mod_path, deploy_mod_contents).expect("Failed to write to deploy/mod.rs");
}

fn add_imports_to_file(file_path: &str, contract: &str) {
    // Read the file
    let path = Path::new(file_path);
    if !path.exists() {
        println!("Warning: File {} does not exist", file_path);
        return;
    }

    let mut file = fs::File::open(path).unwrap_or_else(|_| panic!("Failed to open {}", file_path));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|_| panic!("Failed to read {}", file_path));

    // Add the imports at the top
    let new_contents = format!(
        "#![allow(clippy::all, clippy::pedantic, clippy::nursery, warnings, unknown_lints, rustdoc::all, elided_lifetimes_in_paths)]\nuse {}::*;\n\n{}\n",
        contract, contents
    );

    // Write back to the file
    let mut file =
        fs::File::create(path).unwrap_or_else(|_| panic!("Failed to create {}", file_path));
    file.write_all(new_contents.as_bytes())
        .unwrap_or_else(|_| panic!("Failed to write to {}", file_path));
}
