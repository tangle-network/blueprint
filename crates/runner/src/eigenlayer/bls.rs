use alloy_primitives::{Address, U256, hex};
use blueprint_evm_extra::util::{get_provider_http, wait_transaction};
use eigensdk::client_elcontracts::{reader::ELChainReader, writer::ELChainWriter};
use eigensdk::crypto_bls::BlsKeyPair;
use eigensdk::logging::get_test_logger;
use eigensdk::types::operator::Operator;
use eigensdk::utils::slashing::core::allocationmanager::AllocationManager::{self, OperatorSet};
use eigensdk::utils::slashing::core::allocationmanager::IAllocationManagerTypes::AllocateParams;

use super::error::EigenlayerError;
use crate::BlueprintConfig;
use crate::config::BlueprintEnvironment;
use crate::error::RunnerError;
use blueprint_core::{error, info};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::backends::bn254::Bn254Backend;
use blueprint_keystore::backends::eigenlayer::EigenlayerBackend;
use blueprint_keystore::crypto::k256::K256Ecdsa;

/// Eigenlayer protocol configuration for BLS-based contracts
#[derive(Clone, Copy)]
pub struct EigenlayerBLSConfig {
    earnings_receiver_address: Address,
    delegation_approver_address: Address,
    exit_after_register: bool,
}

impl EigenlayerBLSConfig {
    /// Creates a new [`EigenlayerBLSConfig`] with the given earnings receiver and delegation approver addresses.
    ///
    /// By default, a Runner created with this config will exit after registration (Pre-Registration). To change
    /// this, use [`EigenlayerBLSConfig::with_exit_after_register`] passing false.
    #[must_use]
    pub fn new(earnings_receiver_address: Address, delegation_approver_address: Address) -> Self {
        Self {
            earnings_receiver_address,
            delegation_approver_address,
            exit_after_register: true,
        }
    }

    /// Sets whether the Runner should exit after registration or continue with execution.
    #[must_use]
    pub fn with_exit_after_register(mut self, should_exit_after_registration: bool) -> Self {
        self.exit_after_register = should_exit_after_registration;
        self
    }
}

impl BlueprintConfig for EigenlayerBLSConfig {
    async fn register(&self, env: &BlueprintEnvironment) -> Result<(), RunnerError> {
        info!("Eigenlayer BLS Config: Registering");
        register_bls_impl(
            env,
            self.earnings_receiver_address,
            self.delegation_approver_address,
        )
        .await
    }

    async fn requires_registration(&self, env: &BlueprintEnvironment) -> Result<bool, RunnerError> {
        info!("Eigenlayer BLS Config: Checking if registration is required");
        requires_registration_bls_impl(env).await
    }

    fn should_exit_after_registration(&self) -> bool {
        info!(
            "Eigenlayer BLS Config: {} exit after registration",
            if self.exit_after_register {
                "Should"
            } else {
                "Should not"
            }
        );
        self.exit_after_register
    }
}

async fn requires_registration_bls_impl(env: &BlueprintEnvironment) -> Result<bool, RunnerError> {
    is_operator_registered(env).await
}

#[allow(clippy::too_many_lines)]
async fn register_bls_impl(
    env: &BlueprintEnvironment,
    earnings_receiver_address: Address,
    delegation_approver_address: Address,
) -> Result<(), RunnerError> {
    info!("Eigenlayer BLS Registration: Fetching Contract Addresses");
    let contract_addresses = env.protocol_settings.eigenlayer()?;
    let allocation_manager_address = contract_addresses.allocation_manager_address;
    let delegation_manager_address = contract_addresses.delegation_manager_address;
    let strategy_manager_address = contract_addresses.strategy_manager_address;
    let rewards_coordinator_address = contract_addresses.rewards_coordinator_address;
    let avs_directory_address = contract_addresses.avs_directory_address;
    let permission_controller_address = contract_addresses.permission_controller_address;
    let service_manager_address = contract_addresses.service_manager_address;
    let registry_coordinator_address = contract_addresses.registry_coordinator_address;
    let strategy_address = contract_addresses.strategy_address;

    info!("Eigenlayer BLS Registration: Fetching ECDSA Keys");
    let ecdsa_public = env.keystore().first_local::<K256Ecdsa>()?;
    let ecdsa_secret = env
        .keystore()
        .expose_ecdsa_secret(&ecdsa_public)?
        .ok_or_else(|| EigenlayerError::Other("No ECDSA secret found".into()))?;
    let operator_address = ecdsa_secret
        .alloy_address()
        .map_err(|e| EigenlayerError::Crypto(e.into()))?;

    let operator_private_key = hex::encode(ecdsa_secret.0.to_bytes());

    info!("Eigenlayer BLS Registration: Creating AVS Registry Writer");
    let logger = get_test_logger();

    info!("Eigenlayer BLS Registration: Fetching BLS BN254 Keys");
    let bn254_public = env.keystore().iter_bls_bn254().next().unwrap();
    let bn254_secret = env
        .keystore()
        .expose_bls_bn254_secret(&bn254_public)
        .map_err(EigenlayerError::Keystore)?
        // TODO: Add MissingKey variant to keystore error
        .ok_or(EigenlayerError::Other("Missing BLS BN254 key".into()))?;
    let operator_bls_key = BlsKeyPair::new(bn254_secret.0.to_string())
        .map_err(|e| EigenlayerError::Other(e.into()))?;

    info!("Eigenlayer BLS Registration: Creating EL Chain Reader");
    let el_chain_reader = ELChainReader::new(
        logger,
        Some(allocation_manager_address),
        delegation_manager_address,
        rewards_coordinator_address,
        avs_directory_address,
        Some(permission_controller_address),
        env.http_rpc_endpoint.to_string(),
    );

    info!("Eigenlayer BLS Registration: Creating EL Chain Writer");
    let el_writer = ELChainWriter::new(
        strategy_manager_address,
        rewards_coordinator_address,
        Some(permission_controller_address),
        Some(allocation_manager_address),
        registry_coordinator_address,
        el_chain_reader.clone(),
        env.http_rpc_endpoint.to_string(),
        operator_private_key.clone(),
    );

    let staker_opt_out_window_blocks = 50400u32;
    let operator_details = Operator {
        address: operator_address,
        delegation_approver_address,
        metadata_url: "https://github.com/tangle-network/blueprint".to_string(),
        allocation_delay: Some(0), // TODO: Make allocation delay configurable
        _deprecated_earnings_receiver_address: Some(earnings_receiver_address),
        staker_opt_out_window_blocks: Some(staker_opt_out_window_blocks),
    };

    let tx_hash = el_writer
        .register_as_operator(operator_details)
        .await
        .map_err(EigenlayerError::ElContracts)?;
    let registration_receipt = wait_transaction(&env.http_rpc_endpoint.to_string(), tx_hash)
        .await
        .map_err(|e| EigenlayerError::Registration(format!("AVS registration error: {}", e)))?;
    if registration_receipt.status() {
        info!("Registered as operator {} for Eigenlayer", operator_address);
    } else if is_operator_registered(env).await? {
        info!(
            "Operator {} is already registered for Eigenlayer",
            operator_address
        );
    } else {
        error!(
            "Operator registration failed for operator {}",
            operator_address
        );
        return Err(EigenlayerError::Registration("Operator registration failed".into()).into());
    }

    let amount = U256::from(5_000_000_000_000_000_000_000u128); // TODO: Make deposit amount configurable

    let avs_deposit_hash = el_writer
        .deposit_erc20_into_strategy(strategy_address, amount)
        .await
        .map_err(EigenlayerError::ElContracts)?;

    info!("Deposit hash: {:?}", avs_deposit_hash);
    let avs_deposit_receipt =
        wait_transaction(&env.http_rpc_endpoint.to_string(), avs_deposit_hash)
            .await
            .map_err(|e| EigenlayerError::Registration(format!("AVS deposit error: {}", e)))?;
    if avs_deposit_receipt.status() {
        info!(
            "Deposited into strategy {} for Eigenlayer",
            strategy_address
        );
    } else {
        error!("AVS deposit failed for strategy {}", strategy_address);
        return Err(EigenlayerError::Other("AVS deposit failed".into()).into());
    }

    let allocation_delay = 0u32; // TODO: User-defined allocation delay
    let provider = get_provider_http(&env.http_rpc_endpoint.to_string());
    let allocation_manager = AllocationManager::new(allocation_manager_address, provider);
    let allocation_delay_receipt = allocation_manager
        .setAllocationDelay(operator_address, allocation_delay)
        .send()
        .await
        .map_err(|e| EigenlayerError::Registration(format!("Allocation delay set error: {}", e)))?
        .get_receipt()
        .await
        .map_err(|e| EigenlayerError::Registration(format!("Allocation delay set error: {}", e)))?;
    if allocation_delay_receipt.status() {
        info!(
            "Successfully set allocation delay to {} for operator {}",
            allocation_delay, operator_address
        );
    } else {
        error!(
            "Failed to set allocation delay for operator {}",
            operator_address
        );
        return Err(EigenlayerError::Other(
            "Allocation Manager setAllocationDelay call failed".into(),
        )
        .into());
    }

    // Stake tokens to the quorum
    let stake_amount = 1_000_000_000_000_000_000u64;
    let operator_sets = vec![0u32];

    info!(
        "Staking {} tokens to quorums {:?}",
        stake_amount, operator_sets
    );
    let stake_hash = el_writer
        .modify_allocations(
            operator_address,
            vec![AllocateParams {
                operatorSet: OperatorSet {
                    avs: service_manager_address,
                    id: operator_sets[0],
                },
                strategies: vec![strategy_address],
                newMagnitudes: vec![stake_amount],
            }],
        )
        .await
        .map_err(|e| EigenlayerError::Registration(e.to_string()))?;

    let stake_receipt = wait_transaction(&env.http_rpc_endpoint.to_string(), stake_hash)
        .await
        .map_err(|e| EigenlayerError::Registration(format!("Quorum staking error: {}", e)))?;

    if stake_receipt.status() {
        info!("Successfully staked tokens to quorums {:?}", operator_sets);
    } else {
        error!("Failed to stake tokens to quorums");
        return Err(EigenlayerError::Other("Quorum staking failed".into()).into());
    }

    info!("Operator BLS key pair: {:?}", operator_bls_key);

    // Register to Operator Sets
    info!("Registering to operator sets");
    let registration_hash = el_writer
        .register_for_operator_sets(
            operator_address,
            service_manager_address,
            vec![0u32],
            operator_bls_key,
            "incredible",
        )
        .await
        .map_err(EigenlayerError::ElContracts)?;

    let registration_receipt =
        wait_transaction(&env.http_rpc_endpoint.to_string(), registration_hash)
            .await
            .map_err(|e| {
                EigenlayerError::Registration(format!("Operator sets registration error: {}", e))
            })?;
    if registration_receipt.status() {
        info!("Registered to operator sets for Eigenlayer");
    } else {
        error!("Registration failed for operator sets");
        return Err(EigenlayerError::Registration("Registration failed".into()).into());
    }

    info!("If the terminal exits, you should re-run the runner to continue execution.");
    Ok(())
}

async fn is_operator_registered(env: &BlueprintEnvironment) -> Result<bool, RunnerError> {
    let contract_addresses = env.protocol_settings.eigenlayer()?;
    let registry_coordinator_address = contract_addresses.registry_coordinator_address;
    let operator_state_retriever_address = contract_addresses.operator_state_retriever_address;

    let ecdsa_public = env.keystore().first_local::<K256Ecdsa>()?;
    let ecdsa_secret = env
        .keystore()
        .expose_ecdsa_secret(&ecdsa_public)?
        .ok_or_else(|| EigenlayerError::Other("No ECDSA secret found".into()))?;
    let operator_address = ecdsa_secret
        .alloy_address()
        .map_err(|e| EigenlayerError::Crypto(e.into()))?;

    let avs_registry_reader = eigensdk::client_avsregistry::reader::AvsRegistryChainReader::new(
        get_test_logger(),
        registry_coordinator_address,
        operator_state_retriever_address,
        env.http_rpc_endpoint.to_string(),
    )
    .await
    .map_err(EigenlayerError::AvsRegistry)?;

    // Check if the operator has already registered for the service
    match avs_registry_reader
        .is_operator_registered(operator_address)
        .await
    {
        Ok(is_registered) => Ok(!is_registered),
        Err(e) => Err(EigenlayerError::AvsRegistry(e).into()),
    }
}
