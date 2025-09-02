use apikey_blueprint_lib::{write_resource, purchase_api_key};
use blueprint_sdk::build;
use blueprint_sdk::tangle::blueprint;

fn main() {
    if std::env::var("BUILD_CONTRACTS").is_ok() {
        let contract_dirs: Vec<&str> = vec!["./contracts"];
        build::soldeer_install();
        build::soldeer_update();
        build::build_contracts(contract_dirs);
    }

    println!("cargo::rerun-if-changed=../apikey-blueprint-lib");

    // Re-run triggers
    println!("cargo:rerun-if-changed=contracts/src");
    println!("cargo:rerun-if-changed=remappings.txt");
    println!("cargo:rerun-if-changed=foundry.toml");
    println!("cargo:rerun-if-changed=../apikey-blueprint-lib");

    // Produce blueprint.json describing jobs in this blueprint
    let blueprint = blueprint! {
        name: "apikey-blueprint",
        master_manager_revision: "Latest",
        manager: { Evm = "ExperimentalBlueprint" },
        jobs: [write_resource, purchase_api_key]
    };

    if let Ok(blueprint) = blueprint {
        let json =
            blueprint_sdk::tangle::metadata::macros::ext::serde_json::to_string_pretty(&blueprint)
                .unwrap();
        let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        std::fs::write(
            std::path::Path::new(&root).join("blueprint.json"),
            json.as_bytes(),
        )
        .unwrap();
    }
}
