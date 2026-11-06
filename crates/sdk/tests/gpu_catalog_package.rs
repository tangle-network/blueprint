//! The catalog is a published SDK asset consumed by other repositories.
use serde_json::Value;
use std::{collections::BTreeSet, path::Path, process::Command};

#[test]
fn published_core_contains_one_hardware_catalog() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("../core");
    let package = Command::new("cargo")
        .args([
            "package",
            "--list",
            "--locked",
            "--offline",
            "--allow-dirty",
            "--manifest-path",
        ])
        .arg(directory.join("Cargo.toml"))
        .output()
        .unwrap();
    assert!(
        package.status.success(),
        "{}",
        String::from_utf8_lossy(&package.stderr)
    );
    assert!(
        String::from_utf8(package.stdout)
            .unwrap()
            .lines()
            .any(|line| line == "gpu-catalog.json"),
        "published blueprint-core has no GPU catalog"
    );
    let source = std::fs::read_to_string(directory.join("gpu-catalog.json")).unwrap();
    assert_eq!(source, blueprint_sdk::core::gpu::CATALOG_JSON);
    let data: Value = serde_json::from_str(&source).unwrap();
    assert_eq!(data["schemaVersion"], 1);
    assert_eq!(data["units"]["vramGb"], "GB (decimal)");
    assert_eq!(data["units"]["memoryBandwidthGbps"], "GB/s");
    let classes = data["gpus"].as_array().unwrap();
    let ids: BTreeSet<_> = classes
        .iter()
        .map(|class| class["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids.len(), classes.len(), "duplicate GPU identifiers");
    for (id, vram, cap) in [
        ("nvidia-h200", 141, 8),
        ("nvidia-b200", 180, 8),
        ("nvidia-b300", 288, 8),
        ("nvidia-gb300", 288, 4),
    ] {
        let class = classes.iter().find(|class| class["id"] == id).unwrap();
        assert_eq!(class["vramGb"], vram);
        assert_eq!(class["maxPerNode"], cap);
        assert!(class["fp8"].is_boolean() && class["fp4"].is_boolean());
        assert!(class["memoryBandwidthGbps"].as_u64().unwrap() > 0);
        assert!(class["vendor"].is_string() && class["generation"].is_string());
        assert!(class["interconnect"].is_string() && class["providerSkus"].is_object());
    }
    let typed = blueprint_sdk::core::gpu::find("nvidia-b200").unwrap();
    assert_eq!(typed.sku("lambda-labs", 8), Some("gpu_8x_b200_sxm6"));
    assert!(typed.sku("lambda-labs", 7).is_none());
    assert!(blueprint_sdk::core::gpu::find("unknown-gpu").is_none());
    assert!(
        blueprint_sdk::core::gpu::find("nvidia-gb300")
            .unwrap()
            .sku("prime-intellect", 5)
            .is_none()
    );
    for (id, provider, count, sku) in [
        ("nvidia-rtx-a4000", "fluidstack", 1, "rtx_a4000"),
        ("nvidia-v100-16gb", "aws", 4, "p3.8xlarge"),
    ] {
        assert_eq!(
            blueprint_sdk::core::gpu::find(id)
                .unwrap()
                .sku(provider, count),
            Some(sku)
        );
    }
    let b200 = classes
        .iter()
        .find(|class| class["id"] == "nvidia-b200")
        .unwrap();
    assert_eq!(b200["providerSkus"]["lambda-labs"]["8"], "gpu_8x_b200_sxm6");
    assert!(
        data.get("providers").is_none(),
        "provider capabilities belong to generated descriptors"
    );
    for class in classes {
        assert!(
            class
                .as_object()
                .unwrap()
                .keys()
                .all(|key| !key.to_lowercase().contains("price"))
        );
    }
}
