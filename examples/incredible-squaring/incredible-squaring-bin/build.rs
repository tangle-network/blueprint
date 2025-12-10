//! Build script for the Incredible Squaring Blueprint
//!
//! This generates blueprint metadata for Tangle EVM (v2) blueprints.

use std::path::PathBuf;

fn main() {
    println!("cargo::rerun-if-changed=../incredible-squaring-lib");

    // For Tangle EVM (v2) blueprints, we generate metadata manually
    // The metadata defines the blueprint's jobs and their ABI encoding
    let blueprint_metadata = serde_json::json!({
        "name": "incredible-squaring",
        "description": "A simple blueprint that squares a number",
        "version": env!("CARGO_PKG_VERSION"),
        "manager": {
            "Evm": "IncredibleSquaringBSM"
        },
        "master_revision": "Latest",
        "jobs": [
            {
                "name": "square",
                "job_index": 0,
                "description": "Square a u64 number (1 operator required)",
                "inputs": ["uint64"],
                "outputs": ["uint64"],
                "required_results": 1
            },
            {
                "name": "verified_square",
                "job_index": 1,
                "description": "Square a u64 number (2 operators required for verification)",
                "inputs": ["uint64"],
                "outputs": ["uint64"],
                "required_results": 2
            },
            {
                "name": "consensus_square",
                "job_index": 2,
                "description": "Square a u64 number (3 operators required for Byzantine fault tolerance)",
                "inputs": ["uint64"],
                "outputs": ["uint64"],
                "required_results": 3
            }
        ]
    });

    let json = serde_json::to_string_pretty(&blueprint_metadata).unwrap();

    // Use CARGO_MANIFEST_DIR and navigate to workspace root
    // Path: examples/incredible-squaring/incredible-squaring-bin -> workspace root (3 levels up)
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir
        .parent() // incredible-squaring
        .and_then(|p| p.parent()) // examples
        .and_then(|p| p.parent()) // workspace root
        .expect("Failed to find workspace root");

    std::fs::write(workspace_root.join("blueprint.json"), json.as_bytes()).unwrap();
}
