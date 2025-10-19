/// EigenLayer AVS registration state management
///
/// This module provides persistent storage and querying of EigenLayer AVS registrations.
/// It maintains a local state file and provides methods to reconcile with on-chain state.
use alloy_primitives::Address;
use blueprint_core::{error, info, warn};
use blueprint_keystore::backends::eigenlayer::EigenlayerBackend;
use blueprint_runner::config::BlueprintEnvironment;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Status of an AVS registration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegistrationStatus {
    /// AVS is active and should be running
    Active,
    /// AVS has been deregistered locally (not running)
    Deregistered,
    /// AVS registration is pending on-chain confirmation
    Pending,
}

/// Runtime target for AVS blueprint execution
///
/// Maps 1:1 to the manager's Runtime enum without any Tangle dependencies.
///
/// Supports three runtime modes:
/// - Native: Direct process execution (blueprint_path)
/// - Hypervisor: VM-based isolation (blueprint_path)
/// - Container: Docker/Kata containers (container_image)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeTarget {
    /// Native process (no sandbox) - for testing only
    /// WARNING: No isolation, fastest startup, use for local testing only
    Native,
    /// cloud-hypervisor VM sandbox (default, production-ready)
    /// Provides strong isolation via hardware virtualization (requires Linux/KVM)
    Hypervisor,
    /// Container runtime (Docker/Kata)
    /// Requires container_image field in config
    Container,
}

impl Default for RuntimeTarget {
    fn default() -> Self {
        // Default to hypervisor for production safety
        Self::Hypervisor
    }
}

impl std::fmt::Display for RuntimeTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native => write!(f, "native"),
            Self::Hypervisor => write!(f, "hypervisor"),
            Self::Container => write!(f, "container"),
        }
    }
}

impl std::str::FromStr for RuntimeTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "native" => Ok(Self::Native),
            "hypervisor" | "vm" => Ok(Self::Hypervisor),
            "container" | "docker" | "kata" => Ok(Self::Container),
            _ => Err(format!(
                "Invalid runtime target: '{}'. Valid options: 'native', 'hypervisor', 'container'",
                s
            )),
        }
    }
}

/// Configuration for an AVS registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvsRegistrationConfig {
    /// Service manager contract address (unique identifier for AVS)
    pub service_manager: Address,
    /// Registry coordinator contract address
    pub registry_coordinator: Address,
    /// Operator state retriever contract address
    pub operator_state_retriever: Address,
    /// Strategy manager contract address
    pub strategy_manager: Address,
    /// Delegation manager contract address
    pub delegation_manager: Address,
    /// AVS directory contract address
    pub avs_directory: Address,
    /// Rewards coordinator contract address
    pub rewards_coordinator: Address,
    /// Permission controller contract address (optional)
    pub permission_controller: Option<Address>,
    /// Allocation manager contract address (optional)
    pub allocation_manager: Option<Address>,
    /// Strategy address for staking
    pub strategy_address: Address,
    /// Stake registry address
    pub stake_registry: Address,

    /// Path to the blueprint binary or source (for Native/Hypervisor runtimes)
    pub blueprint_path: PathBuf,

    /// Container image (for Container runtime only)
    /// Format: "registry/image:tag" or "image:tag"
    /// Example: "ghcr.io/my-org/my-avs:latest"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_image: Option<String>,

    /// Runtime target for blueprint execution
    #[serde(default)]
    pub runtime_target: RuntimeTarget,

    /// Allocation delay (in blocks)
    #[serde(default = "default_allocation_delay")]
    pub allocation_delay: u32,
    /// Deposit amount (in wei)
    #[serde(default = "default_deposit_amount")]
    pub deposit_amount: u128,
    /// Stake amount (in wei)
    #[serde(default = "default_stake_amount")]
    pub stake_amount: u64,

    /// Operator sets to register for (default: [0])
    #[serde(default = "default_operator_sets")]
    pub operator_sets: Vec<u32>,
}

fn default_allocation_delay() -> u32 {
    0
}

fn default_deposit_amount() -> u128 {
    5_000_000_000_000_000_000_000
}

fn default_stake_amount() -> u64 {
    1_000_000_000_000_000_000
}

fn default_operator_sets() -> Vec<u32> {
    vec![0]
}

/// A registered AVS entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvsRegistration {
    /// Operator address that registered
    pub operator_address: Address,
    /// When this registration was created (ISO 8601)
    pub registered_at: String,
    /// Current status
    pub status: RegistrationStatus,
    /// AVS configuration
    pub config: AvsRegistrationConfig,
}

impl AvsRegistrationConfig {
    /// Validate the registration configuration
    ///
    /// Checks that:
    /// - Blueprint path exists and is accessible
    /// - Runtime target is supported on current platform
    /// - Required feature flags are enabled
    ///
    /// # Errors
    ///
    /// Returns error if configuration is invalid
    pub fn validate(&self) -> Result<(), String> {
        // Check blueprint path exists
        if !self.blueprint_path.exists() {
            return Err(format!(
                "Blueprint path does not exist: {}",
                self.blueprint_path.display()
            ));
        }

        // For native binaries, check if it's a file
        if !self.blueprint_path.is_dir() && !self.blueprint_path.is_file() {
            return Err(format!(
                "Blueprint path is neither a file nor directory: {}",
                self.blueprint_path.display()
            ));
        }

        // Check runtime target compatibility
        match self.runtime_target {
            RuntimeTarget::Native => {
                // Native always works but warn in production
                #[cfg(not(debug_assertions))]
                {
                    warn!(
                        "Native runtime selected - this provides NO ISOLATION and should only be used for testing!"
                    );
                }
            }
            RuntimeTarget::Hypervisor => {
                // Hypervisor requires Linux
                #[cfg(not(target_os = "linux"))]
                {
                    return Err(
                        "Hypervisor runtime requires Linux/KVM. Use 'native' for local testing on macOS/Windows."
                            .to_string(),
                    );
                }

                // Check if vm-sandbox feature is enabled (compile-time check happens at spawn)
                #[cfg(not(feature = "vm-sandbox"))]
                {
                    return Err(
                        "Hypervisor runtime requires recompiling with --features vm-sandbox. Use 'native' for testing."
                            .to_string(),
                    );
                }
            }
            RuntimeTarget::Container => {
                // Container runtime requires container_image field
                if self.container_image.is_none() {
                    return Err(
                        "Container runtime requires 'container_image' field in config. \
                        Example: \"ghcr.io/my-org/my-avs:latest\"".to_string()
                    );
                }

                // Validate image format (basic check)
                if let Some(ref image) = self.container_image {
                    if image.trim().is_empty() {
                        return Err("Container image cannot be empty".to_string());
                    }
                    // Should have at least image:tag format
                    if !image.contains(':') {
                        return Err(
                            "Container image must include tag. Example: \"my-image:latest\"".to_string()
                        );
                    }
                }
            }
        }

        Ok(())
    }
}

impl AvsRegistration {
    /// Create a new AVS registration
    pub fn new(operator_address: Address, config: AvsRegistrationConfig) -> Self {
        Self {
            operator_address,
            registered_at: Utc::now().to_rfc3339(),
            status: RegistrationStatus::Active,
            config,
        }
    }

    /// Get a unique identifier for this AVS (based on service manager address)
    pub fn avs_id(&self) -> Address {
        self.config.service_manager
    }

    /// Get the blueprint ID for manager tracking (hash of service manager)
    pub fn blueprint_id(&self) -> u64 {
        let bytes = self.config.service_manager.as_slice();
        u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }
}

/// Collection of AVS registrations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AvsRegistrations {
    /// Map of service_manager_address -> registration
    #[serde(default)]
    pub registrations: HashMap<String, AvsRegistration>,
}

impl AvsRegistrations {
    /// Add a new registration
    pub fn add(&mut self, registration: AvsRegistration) {
        let key = format!("{:#x}", registration.config.service_manager);
        self.registrations.insert(key, registration);
    }

    /// Remove a registration by service manager address
    pub fn remove(&mut self, service_manager: Address) -> Option<AvsRegistration> {
        let key = format!("{:#x}", service_manager);
        self.registrations.remove(&key)
    }

    /// Get a registration by service manager address
    pub fn get(&self, service_manager: Address) -> Option<&AvsRegistration> {
        let key = format!("{:#x}", service_manager);
        self.registrations.get(&key)
    }

    /// Get a mutable reference to a registration
    pub fn get_mut(&mut self, service_manager: Address) -> Option<&mut AvsRegistration> {
        let key = format!("{:#x}", service_manager);
        self.registrations.get_mut(&key)
    }

    /// Get all active registrations
    pub fn active(&self) -> impl Iterator<Item = &AvsRegistration> {
        self.registrations
            .values()
            .filter(|r| r.status == RegistrationStatus::Active)
    }

    /// Mark a registration as deregistered
    pub fn mark_deregistered(&mut self, service_manager: Address) -> bool {
        if let Some(reg) = self.get_mut(service_manager) {
            reg.status = RegistrationStatus::Deregistered;
            true
        } else {
            false
        }
    }
}

/// Manager for AVS registration state
pub struct RegistrationStateManager {
    state_file: PathBuf,
    registrations: AvsRegistrations,
}

impl RegistrationStateManager {
    /// Load registration state from the default location
    ///
    /// The state file is stored at `~/.tangle/eigenlayer_registrations.json`
    ///
    /// # Errors
    ///
    /// Returns error if home directory cannot be determined or file cannot be read
    pub fn load() -> Result<Self, crate::error::Error> {
        let state_file = Self::default_state_file()?;
        Self::load_from_file(&state_file)
    }

    /// Load registration state from a specific file
    pub fn load_from_file(path: &Path) -> Result<Self, crate::error::Error> {
        let registrations = if path.exists() {
            let contents = std::fs::read_to_string(path).map_err(|e| {
                crate::error::Error::Other(format!("Failed to read registration state: {}", e))
            })?;

            serde_json::from_str(&contents).map_err(|e| {
                crate::error::Error::Other(format!("Failed to parse registration state: {}", e))
            })?
        } else {
            info!(
                "No existing registration state found at {}, creating new",
                path.display()
            );
            AvsRegistrations::default()
        };

        Ok(Self {
            state_file: path.to_path_buf(),
            registrations,
        })
    }

    /// Get the default state file path
    fn default_state_file() -> Result<PathBuf, crate::error::Error> {
        let home = dirs::home_dir()
            .ok_or_else(|| crate::error::Error::Other("Cannot determine home directory".into()))?;

        let tangle_dir = home.join(".tangle");
        std::fs::create_dir_all(&tangle_dir).map_err(|e| {
            crate::error::Error::Other(format!("Failed to create .tangle directory: {}", e))
        })?;

        Ok(tangle_dir.join("eigenlayer_registrations.json"))
    }

    /// Save registration state to disk
    pub fn save(&self) -> Result<(), crate::error::Error> {
        let contents = serde_json::to_string_pretty(&self.registrations).map_err(|e| {
            crate::error::Error::Other(format!("Failed to serialize registrations: {}", e))
        })?;

        std::fs::write(&self.state_file, contents).map_err(|e| {
            crate::error::Error::Other(format!("Failed to write registration state: {}", e))
        })?;

        info!("Saved registration state to {}", self.state_file.display());
        Ok(())
    }

    /// Get all registrations
    pub fn registrations(&self) -> &AvsRegistrations {
        &self.registrations
    }

    /// Get mutable access to registrations
    pub fn registrations_mut(&mut self) -> &mut AvsRegistrations {
        &mut self.registrations
    }

    /// Add a new registration and save to disk
    pub fn register(&mut self, registration: AvsRegistration) -> Result<(), crate::error::Error> {
        info!(
            "Registering AVS {} for operator {:#x}",
            registration.config.service_manager, registration.operator_address
        );

        self.registrations.add(registration);
        self.save()
    }

    /// Mark an AVS as deregistered and save to disk
    pub fn deregister(&mut self, service_manager: Address) -> Result<(), crate::error::Error> {
        info!("Deregistering AVS {:#x}", service_manager);

        if self.registrations.mark_deregistered(service_manager) {
            self.save()
        } else {
            Err(crate::error::Error::Other(format!(
                "AVS {:#x} not found in registrations",
                service_manager
            )))
        }
    }

    /// Verify registration status on-chain
    ///
    /// Queries the EigenLayer contracts to check if the operator is still registered
    pub async fn verify_on_chain(
        &self,
        service_manager: Address,
        env: &BlueprintEnvironment,
    ) -> Result<bool, crate::error::Error> {
        let registration = self.registrations.get(service_manager).ok_or_else(|| {
            crate::error::Error::Other(format!(
                "AVS {:#x} not found in local registrations",
                service_manager
            ))
        })?;

        // Get operator address from keystore
        use blueprint_keystore::backends::Backend;
        use blueprint_keystore::crypto::k256::K256Ecdsa;

        let ecdsa_public = env
            .keystore()
            .first_local::<K256Ecdsa>()
            .map_err(|e| crate::error::Error::Other(format!("Keystore error: {}", e)))?;

        let ecdsa_secret = env
            .keystore()
            .expose_ecdsa_secret(&ecdsa_public)
            .map_err(|e| crate::error::Error::Other(format!("Keystore error: {}", e)))?
            .ok_or_else(|| crate::error::Error::Other("No ECDSA secret found".into()))?;

        let operator_address = ecdsa_secret.alloy_address().map_err(|e| {
            crate::error::Error::Other(format!("Failed to get operator address: {}", e))
        })?;

        // Create AVS registry reader
        let avs_registry_reader =
            eigensdk::client_avsregistry::reader::AvsRegistryChainReader::new(
                registration.config.registry_coordinator,
                registration.config.operator_state_retriever,
                env.http_rpc_endpoint.to_string(),
            )
            .await
            .map_err(|e| {
                crate::error::Error::Other(format!("Failed to create AVS registry reader: {}", e))
            })?;

        // Check if operator is registered
        avs_registry_reader
            .is_operator_registered(operator_address)
            .await
            .map_err(|e| {
                crate::error::Error::Other(format!("Failed to check registration status: {}", e))
            })
    }

    /// Reconcile local state with on-chain state
    ///
    /// For each locally registered AVS:
    /// - Queries on-chain to verify registration status
    /// - Marks as deregistered if not found on-chain
    ///
    /// Returns the number of reconciled entries
    pub async fn reconcile_with_chain(
        &mut self,
        env: &BlueprintEnvironment,
    ) -> Result<usize, crate::error::Error> {
        let mut reconciled = 0;
        let service_managers: Vec<Address> = self
            .registrations
            .active()
            .map(|r| r.config.service_manager)
            .collect();

        for service_manager in service_managers {
            match self.verify_on_chain(service_manager, env).await {
                Ok(is_registered) => {
                    if !is_registered {
                        warn!(
                            "AVS {:#x} is registered locally but not on-chain, marking as deregistered",
                            service_manager
                        );
                        self.registrations.mark_deregistered(service_manager);
                        reconciled += 1;
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to verify AVS {:#x} on-chain: {}",
                        service_manager, e
                    );
                }
            }
        }

        if reconciled > 0 {
            self.save()?;
        }

        Ok(reconciled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_validation_nonexistent_path() {
        let config = AvsRegistrationConfig {
            service_manager: Address::ZERO,
            registry_coordinator: Address::ZERO,
            operator_state_retriever: Address::ZERO,
            strategy_manager: Address::ZERO,
            delegation_manager: Address::ZERO,
            avs_directory: Address::ZERO,
            rewards_coordinator: Address::ZERO,
            permission_controller: None,
            allocation_manager: None,
            strategy_address: Address::ZERO,
            stake_registry: Address::ZERO,
            blueprint_path: PathBuf::from("/nonexistent/path/to/blueprint"),
            runtime_target: RuntimeTarget::Native,
            allocation_delay: 0,
            deposit_amount: 1000,
            stake_amount: 100,
            operator_sets: vec![0],
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_validation_valid_config() {
        let temp_dir = tempfile::tempdir().unwrap();
        let blueprint_path = temp_dir.path().join("test_blueprint");
        std::fs::File::create(&blueprint_path).unwrap();

        let config = AvsRegistrationConfig {
            service_manager: Address::ZERO,
            registry_coordinator: Address::ZERO,
            operator_state_retriever: Address::ZERO,
            strategy_manager: Address::ZERO,
            delegation_manager: Address::ZERO,
            avs_directory: Address::ZERO,
            rewards_coordinator: Address::ZERO,
            permission_controller: None,
            allocation_manager: None,
            strategy_address: Address::ZERO,
            stake_registry: Address::ZERO,
            blueprint_path,
            runtime_target: RuntimeTarget::Native,
            allocation_delay: 0,
            deposit_amount: 1000,
            stake_amount: 100,
            operator_sets: vec![0],
        };

        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    #[cfg(not(target_os = "linux"))]
    fn test_validation_hypervisor_on_non_linux() {
        let temp_dir = tempfile::tempdir().unwrap();
        let blueprint_path = temp_dir.path().join("test_blueprint");
        std::fs::File::create(&blueprint_path).unwrap();

        let config = AvsRegistrationConfig {
            service_manager: Address::ZERO,
            registry_coordinator: Address::ZERO,
            operator_state_retriever: Address::ZERO,
            strategy_manager: Address::ZERO,
            delegation_manager: Address::ZERO,
            avs_directory: Address::ZERO,
            rewards_coordinator: Address::ZERO,
            permission_controller: None,
            allocation_manager: None,
            strategy_address: Address::ZERO,
            stake_registry: Address::ZERO,
            blueprint_path,
            runtime_target: RuntimeTarget::Hypervisor,
            allocation_delay: 0,
            deposit_amount: 1000,
            stake_amount: 100,
            operator_sets: vec![0],
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires Linux"));
    }

    #[test]
    fn test_registration_serialization() {
        let config = AvsRegistrationConfig {
            service_manager: Address::from([1u8; 20]),
            registry_coordinator: Address::from([2u8; 20]),
            operator_state_retriever: Address::from([3u8; 20]),
            strategy_manager: Address::from([4u8; 20]),
            delegation_manager: Address::from([5u8; 20]),
            avs_directory: Address::from([6u8; 20]),
            rewards_coordinator: Address::from([7u8; 20]),
            permission_controller: Some(Address::from([8u8; 20])),
            allocation_manager: Some(Address::from([9u8; 20])),
            strategy_address: Address::from([10u8; 20]),
            stake_registry: Address::from([11u8; 20]),
            blueprint_path: PathBuf::from("/path/to/blueprint"),
            runtime_target: RuntimeTarget::Native,
            allocation_delay: 0,
            deposit_amount: 5000,
            stake_amount: 1000,
            operator_sets: vec![0],
        };

        let registration = AvsRegistration::new(Address::from([12u8; 20]), config);

        let serialized = serde_json::to_string(&registration).unwrap();
        let deserialized: AvsRegistration = serde_json::from_str(&serialized).unwrap();

        assert_eq!(registration.operator_address, deserialized.operator_address);
        assert_eq!(registration.status, deserialized.status);
    }

    #[test]
    fn test_registrations_management() {
        let mut registrations = AvsRegistrations::default();

        let config = AvsRegistrationConfig {
            service_manager: Address::from([1u8; 20]),
            registry_coordinator: Address::from([2u8; 20]),
            operator_state_retriever: Address::from([3u8; 20]),
            strategy_manager: Address::from([4u8; 20]),
            delegation_manager: Address::from([5u8; 20]),
            avs_directory: Address::from([6u8; 20]),
            rewards_coordinator: Address::from([7u8; 20]),
            permission_controller: None,
            allocation_manager: None,
            strategy_address: Address::from([10u8; 20]),
            stake_registry: Address::from([11u8; 20]),
            blueprint_path: PathBuf::from("/path/to/blueprint"),
            runtime_target: RuntimeTarget::Native,
            allocation_delay: 0,
            deposit_amount: 5000,
            stake_amount: 1000,
            operator_sets: vec![0],
        };

        let registration = AvsRegistration::new(Address::from([12u8; 20]), config);
        let service_manager = registration.config.service_manager;

        registrations.add(registration);
        assert!(registrations.get(service_manager).is_some());

        assert!(registrations.mark_deregistered(service_manager));
        assert_eq!(
            registrations.get(service_manager).unwrap().status,
            RegistrationStatus::Deregistered
        );

        assert_eq!(registrations.active().count(), 0);
    }
}
