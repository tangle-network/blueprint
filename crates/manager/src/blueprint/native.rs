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
    let os = get_formatted_os_string().to_lowercase();
    let arch = std::env::consts::ARCH.to_lowercase();
    for binary in blueprint_binaries {
        let binary_str = binary.os.to_lowercase();
        if binary_str.contains(&os) || os.contains(&binary_str) || binary_str == os {
            let mut arch_str = binary.arch.to_lowercase();

            if arch_str == "amd" {
                arch_str = "x86".to_string();
            } else if arch_str == "amd64" {
                arch_str = "x86_64".to_string();
            }

            if arch_str == arch {
                return Some(binary);
            }
        }
    }

    None
}
