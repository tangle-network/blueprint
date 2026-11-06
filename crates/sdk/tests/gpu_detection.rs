#![cfg(all(feature = "std", unix))]

use blueprint_sdk::core::gpu;
use std::{fs, os::unix::fs::PermissionsExt, process::Command};

#[test]
fn operator_inventory_uses_driver_memory_for_each_device() {
    const CHILD: &str = "BLUEPRINT_GPU_DETECTION_CHILD";
    if let Ok(scenario) = std::env::var(CHILD) {
        if scenario != "known" {
            assert!(gpu::detect().is_err());
            return;
        }
        let devices = gpu::detect().unwrap();
        assert_eq!(devices.len(), 4);
        assert_eq!(devices[0].class.as_deref(), Some("nvidia-b200"));
        assert_eq!(devices[1].class.as_deref(), Some("nvidia-rtx-3090"));
        assert!(devices[0].vram_gb() >= 180);
        assert!(devices[1].vram_gb() < 32);
        assert_eq!(devices[2].class.as_deref(), Some("nvidia-a100-40gb"));
        assert!(devices[3].class.is_none());
        assert_eq!(devices[3].memory_mib, 8192);
        return;
    }
    let directory = std::env::temp_dir().join(format!("gpu-driver-{}", std::process::id()));
    fs::create_dir(&directory).unwrap();
    let executable = directory.join("nvidia-smi");
    fs::write(
        &executable,
        r#"#!/bin/sh
[ "$#" = 2 ] || exit 2
[ "$1" = '--query-gpu=name,memory.total' ] || exit 2
[ "$2" = '--format=csv,noheader,nounits' ] || exit 2
case "$BLUEPRINT_GPU_DETECTION_CHILD" in
  known) printf 'NVIDIA B200, 183359\nNVIDIA GeForce RTX 3090, 24576\nNVIDIA A100-PCIE-40GB, 40536\nUnknown GPU, 8192\n';;
  malformed) printf 'NVIDIA B200, invalid\n';;
  failed) exit 1;;
esac
"#,
    )
    .unwrap();
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
    for scenario in ["known", "malformed", "failed"] {
        let status = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "operator_inventory_uses_driver_memory_for_each_device",
                "--nocapture",
            ])
            .env(CHILD, scenario)
            .env("PATH", &directory)
            .status()
            .unwrap();
        assert!(status.success(), "GPU detection failed scenario {scenario}");
    }
    fs::remove_file(executable).unwrap();
    fs::remove_dir(directory).unwrap();
}
