use crate::sdk::utils::get_formatted_os_string;
use crate::sources::types::{BlueprintBinary, BlueprintSource};
use blueprint_runner::config::Protocol;

pub struct FilteredBlueprint {
    pub blueprint_id: u64,
    pub services: Vec<u64>,
    pub sources: Vec<BlueprintSource>,
    pub name: String,
    pub registration_mode: bool,
    pub registration_capture_only: bool,
    pub protocol: Protocol,
}

#[must_use]
pub fn get_blueprint_binary(blueprint_binaries: &[BlueprintBinary]) -> Option<&BlueprintBinary> {
    let os = normalize_os(&get_formatted_os_string());
    let arch = normalize_arch(std::env::consts::ARCH);
    for binary in blueprint_binaries {
        let binary_os = normalize_os(&binary.os);
        if binary_os == os {
            let arch_str = normalize_arch(&binary.arch);

            if arch_str == arch {
                return Some(binary);
            }
        }
    }

    None
}

/// Canonicalize an architecture identifier so values published by tooling
/// (`amd64`, `arm64`, …) compare equal to Rust's `std::env::consts::ARCH`
/// (`x86_64`, `aarch64`, …).
///
/// Shared by the initial-fetch path and the upgrade-swap manifest resolver so
/// the two never drift — a divergence would let one path match a binary the
/// other rejects.
pub(crate) fn normalize_arch(value: &str) -> String {
    match value.to_lowercase().as_str() {
        "amd" => "x86".to_string(),
        "amd64" => "x86_64".to_string(),
        "arm64" => "aarch64".to_string(),
        other => other.to_string(),
    }
}

/// Canonicalize an OS identifier (`darwin`/`osx` → `macos`, target-triple
/// fragments like `unknown-linux-gnu` → `linux`, …). See [`normalize_arch`]
/// for why this is shared.
pub(crate) fn normalize_os(value: &str) -> String {
    let lower = value.to_lowercase();
    if lower.contains("darwin") || lower.contains("macos") || lower == "mac" || lower == "osx" {
        "macos".to_string()
    } else if lower.contains("windows") {
        "windows".to_string()
    } else if lower.contains("linux") {
        "linux".to_string()
    } else if lower.contains("bsd") {
        "bsd".to_string()
    } else if lower.contains("unknown") {
        "unknown".to_string()
    } else {
        lower
    }
}
