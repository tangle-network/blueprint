use crate::sdk::utils::get_formatted_os_string;
use blueprint_runner::config::Protocol;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::{BlueprintBinary, BlueprintSource};

pub struct FilteredBlueprint {
    pub blueprint_id: u64,
    pub services: Vec<u64>,
    pub sources: Vec<BlueprintSource>,
    pub name: String,
    pub registration_mode: bool,
    pub protocol: Protocol,
}

#[must_use]
pub fn get_blueprint_binary(gadget_binaries: &[BlueprintBinary]) -> Option<&BlueprintBinary> {
    let os = get_formatted_os_string().to_lowercase();
    let arch = std::env::consts::ARCH.to_lowercase();
    for binary in gadget_binaries {
        let binary_str = format!("{:?}", binary.os).to_lowercase();
        if binary_str.contains(&os) || os.contains(&binary_str) || binary_str == os {
            let mut arch_str = format!("{:?}", binary.arch).to_lowercase();

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
