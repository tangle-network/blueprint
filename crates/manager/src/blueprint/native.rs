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
    // Use the base OS name for simpler matching
    let os = std::env::consts::OS.to_lowercase();
    let arch = std::env::consts::ARCH.to_lowercase();
    for binary in blueprint_binaries {
        let mut binary_os = binary.os.to_lowercase();
        // Normalize OS names
        if binary_os == "darwin" || binary_os == "apple-darwin" {
            binary_os = "macos".to_string();
        }
        if binary_os.contains(&os) || os.contains(&binary_os) || binary_os == os {
            let mut arch_str = binary.arch.to_lowercase();

            // Normalize architecture names to match std::env::consts::ARCH
            if arch_str == "amd" {
                arch_str = "x86".to_string();
            } else if arch_str == "amd64" {
                arch_str = "x86_64".to_string();
            } else if arch_str == "arm64" {
                arch_str = "aarch64".to_string();
            }

            if arch_str == arch {
                return Some(binary);
            }
        }
    }

    None
}
