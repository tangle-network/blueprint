use blueprint_sdk::build;
use blueprint_sdk::tangle::blueprint;
use incredible_squaring_blueprint_lib::{square, square_faas};
use std::path::Path;
use std::process;

fn main() {
    let contract_dirs: Vec<&str> = vec!["./contracts"];
    build::soldeer_install();
    build::soldeer_update();
    build::build_contracts(contract_dirs);

    println!("cargo::rerun-if-changed=../incredible-squaring-lib");
    println!("cargo::rerun-if-changed=../../target/blueprint-profiles.json");

    let blueprint = blueprint! {
        name: "experiment",
        master_manager_revision: "Latest",
        manager: { Evm = "ExperimentalBlueprint" },
        jobs: [square, square_faas]
    };

    match blueprint {
        Ok(mut blueprint) => {
            // Load profiling data if available and add to description field (temporary)
            let workspace_dir = Path::new(env!("CARGO_WORKSPACE_DIR"));
            let profile_path = workspace_dir.join("target/blueprint-profiles.json");

            if profile_path.exists() {
                match blueprint_profiling::BlueprintProfiles::load_from_file(&profile_path) {
                    Ok(profiles) => {
                        match profiles.to_description_field() {
                            Ok(description_with_profiling) => {
                                // Prepend profiling data to existing description or replace
                                if let Some(existing_desc) = &blueprint.metadata.description {
                                    blueprint.metadata.description = Some(
                                        format!("{}\n\n{}", description_with_profiling, existing_desc).into()
                                    );
                                } else {
                                    blueprint.metadata.description = Some(description_with_profiling.into());
                                }
                                println!("cargo::warning=✓ Profiling data added to blueprint metadata");
                            }
                            Err(e) => {
                                println!("cargo::warning=Failed to encode profiling data: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("cargo::warning=Failed to load profiling data: {}", e);
                    }
                }
            } else {
                println!("cargo::warning=No profiling data found at {}. Run `cargo test --test profiling_test` to generate profiles.", profile_path.display());
            }

            // TODO: Should be a helper function probably
            let json = blueprint_sdk::tangle::metadata::macros::ext::serde_json::to_string_pretty(
                &blueprint,
            )
            .unwrap();
            std::fs::write(
                Path::new(env!("CARGO_WORKSPACE_DIR")).join("blueprint.json"),
                json.as_bytes(),
            )
            .unwrap();
        }
        Err(e) => {
            println!("cargo::error={e:?}");
            process::exit(1);
        }
    }
}
