use blueprint_sdk::core::{gpu, resources::ResourceSpec};

#[test]
fn published_catalog_and_shared_resources_describe_hardware_without_prices() {
    let catalog: serde_json::Value = serde_json::from_str(gpu::CATALOG_JSON).unwrap();
    assert_eq!(catalog["schemaVersion"], 1);
    for class in catalog["gpus"].as_array().unwrap() {
        let id = class["id"].as_str().unwrap();
        let description = gpu::find(id).unwrap();
        assert!(description.vram_gb > 0 && description.max_per_node > 0);
        assert!(
            class
                .as_object()
                .unwrap()
                .keys()
                .all(|key| !key.to_ascii_lowercase().contains("price"))
        );
        let spec = ResourceSpec {
            gpu_count: Some(description.max_per_node),
            gpu_class: Some(id.into()),
            gpu_vram_gb: Some(description.vram_gb),
            ..Default::default()
        };
        assert!(spec.validate().is_ok());
        assert!(
            ResourceSpec {
                gpu_count: Some(description.max_per_node + 1),
                ..spec
            }
            .validate()
            .is_err()
        );
    }
    for id in ["nvidia-h200", "nvidia-b200", "nvidia-b300", "nvidia-gb300"] {
        assert!(gpu::find(id).is_some());
    }
    assert!(gpu::find("nvidia-unknown").is_none());
    let b200 = gpu::find("nvidia-b200").unwrap();
    assert_eq!(b200.sku("runpod", 8), Some("NVIDIA B200"));
    assert_eq!(b200.sku("lambda-labs", 8), Some("gpu_8x_b200"));
    assert_eq!(gpu::find("nvidia-gb300").unwrap().max_per_node, 4);
}

#[test]
#[cfg(all(feature = "std", unix))]
fn operator_gpu_detection_uses_device_memory_and_preserves_each_device() {
    use std::{fs, os::unix::fs::PermissionsExt, process::Command};
    const CHILD: &str = "BLUEPRINT_GPU_DETECTION_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let devices = gpu::detect().unwrap();
        assert_eq!(devices.len(), 5);
        assert_eq!(devices[0].class.as_deref(), Some("nvidia-b200"));
        assert_eq!(devices[1].class.as_deref(), Some("nvidia-rtx-3090"));
        assert!(devices[0].vram_gb() >= 180);
        assert!(devices[1].vram_gb() < 32);
        assert_eq!(devices[2].class.as_deref(), Some("nvidia-a100-40gb"));
        assert_eq!(devices[3].class.as_deref(), Some("nvidia-a100-80gb"));
        assert_eq!(devices[4].class.as_deref(), Some("nvidia-h100"));
        return;
    }
    let directory =
        std::env::temp_dir().join(format!("blueprint-gpu-detector-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let executable = directory.join("nvidia-smi");
    fs::write(&executable,"#!/bin/sh\n[ \"$1\" = '--query-gpu=name,memory.total' ] || exit 2\nprintf 'NVIDIA B200, 183359\\nNVIDIA GeForce RTX 3090, 24576\\nNVIDIA A100-PCIE-40GB, 40536\\nNVIDIA A100-SXM4-80GB, 81920\\nNVIDIA H100 PCIe, 81920\\n'\n").unwrap();
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
    let status = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "operator_gpu_detection_uses_device_memory_and_preserves_each_device",
            "--nocapture",
        ])
        .env(CHILD, "1")
        .env("PATH", &directory)
        .status()
        .unwrap();
    fs::remove_file(executable).unwrap();
    fs::remove_dir(directory).unwrap();
    assert!(status.success());
}
