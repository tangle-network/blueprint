//! Helper for building Akash SDL (Stack Definition Language) manifests.
//!
//! This emits a minimal valid SDL document for a single-service blueprint
//! deployment. The relay service is responsible for translating any extra
//! provider-specific bid attributes (region, audited providers, …) before
//! submitting the on-chain transaction.

use crate::core::resources::ResourceSpec;

/// Build an SDL manifest YAML string for a blueprint container deployment.
///
/// The `profile` argument is one of the named GPU profiles produced by
/// [`crate::providers::akash::AkashInstanceMapper`]. The relay maps the profile
/// name to concrete `resources.gpu.attributes` (vendor, model, vram).
pub fn build_sdl_manifest(
    blueprint_image: &str,
    profile: &str,
    spec: &ResourceSpec,
    exposed_ports: &[u16],
) -> String {
    let cpu_units = ((spec.cpu * 1000.0).round() as u32).max(100);
    let memory_mi = ((spec.memory_gb * 1024.0).round() as u32).max(256);
    let storage_mi = ((spec.storage_gb * 1024.0).round() as u32).max(1024);
    let gpu_units = spec.gpu_count.unwrap_or(1).max(1);

    let mut expose_block = String::new();
    if exposed_ports.is_empty() {
        expose_block.push_str(
            "        - port: 80\n          as: 80\n          to:\n            - global: true\n",
        );
    } else {
        for port in exposed_ports {
            expose_block.push_str(&format!(
                "        - port: {port}\n          as: {port}\n          to:\n            - global: true\n"
            ));
        }
    }

    format!(
        r#"---
version: "2.0"
services:
  blueprint:
    image: {blueprint_image}
    expose:
{expose_block}profiles:
  compute:
    blueprint:
      resources:
        cpu:
          units: {cpu_units}m
        memory:
          size: {memory_mi}Mi
        storage:
          size: {storage_mi}Mi
        gpu:
          units: {gpu_units}
          attributes:
            vendor:
              nvidia:
                - model: {profile}
  placement:
    akash:
      pricing:
        blueprint:
          denom: uakt
          amount: 10000
deployment:
  blueprint:
    akash:
      profile: blueprint
      count: 1
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_spec() -> ResourceSpec {
        ResourceSpec {
            cpu: 4.0,
            memory_gb: 16.0,
            storage_gb: 100.0,
            gpu_count: Some(1),
            allow_spot: false,
            qos: Default::default(),
        }
    }

    #[test]
    fn manifest_contains_image_and_profile() {
        let sdl = build_sdl_manifest(
            "ghcr.io/foo/bar:latest",
            "gpu-a100-80gb",
            &sample_spec(),
            &[],
        );
        assert!(sdl.contains("ghcr.io/foo/bar:latest"));
        assert!(sdl.contains("gpu-a100-80gb"));
        assert!(sdl.contains("gpu:"));
        assert!(sdl.contains("denom: uakt"));
    }

    #[test]
    fn manifest_uses_default_port_when_empty() {
        let sdl = build_sdl_manifest("img", "gpu-t4", &sample_spec(), &[]);
        assert!(sdl.contains("port: 80"));
    }

    #[test]
    fn manifest_includes_explicit_ports() {
        let sdl = build_sdl_manifest("img", "gpu-t4", &sample_spec(), &[9615, 9944]);
        assert!(sdl.contains("port: 9615"));
        assert!(sdl.contains("port: 9944"));
    }
}
