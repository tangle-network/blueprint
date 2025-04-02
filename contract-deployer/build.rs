use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let contract_dirs = vec!["./contracts", "./dependencies/eigenlayer-middleware-0.5.4"];
    blueprint_build_utils::soldeer_install();
    blueprint_build_utils::soldeer_update();
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

    if !status.success() {
        panic!("Failed to generate bindings for contracts");
    }

    // Generate bindings for middleware directory
    println!("Generating bindings for middleware...");

    // Build the command with all the select flags
    let mut cmd = Command::new("forge");
    cmd.args([
        "bind",
        "--alloy",
        "--skip-build",
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

    if !status.success() {
        panic!("Failed to generate bindings for middleware");
    }

    // Create the mod.rs in the bindings directory
    let mut contents = String::new();
    contents.push_str("pub mod core;\n");
    contents.push_str("pub mod deploy;\n");
    contents.push_str("\n");

    for contract in contracts_contracts.iter() {
        let lower_contract = contract.to_lowercase();
        contents.push_str(&format!(
            "pub use deploy::{}::{};\n",
            lower_contract, contract
        ));
    }
    for contract in middleware_contracts.iter() {
        let lower_contract = contract.to_lowercase();
        contents.push_str(&format!(
            "pub use core::{}::{};\n",
            lower_contract, contract
        ));
    }

    let path = Path::new("src/bindings/mod.rs");
    fs::write(path, contents).expect("Failed to write to mod.rs");
}
