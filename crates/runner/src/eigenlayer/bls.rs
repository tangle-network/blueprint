use alloy_primitives::{Address, Bytes, FixedBytes, U256, address, hex};
use blueprint_evm_extra::util::get_provider_http;
use blueprint_std::time::{SystemTime, UNIX_EPOCH};
use eigensdk::client_avsregistry::writer::AvsRegistryChainWriter;
use eigensdk::client_elcontracts::{reader::ELChainReader, writer::ELChainWriter};
use eigensdk::crypto_bls::BlsKeyPair;
use eigensdk::logging::get_test_logger;
use eigensdk::testing_utils::transaction::wait_transaction;
use eigensdk::types::operator::Operator;
use eigensdk::utils::slashing::core::avsdirectory::AVSDirectory;
use eigensdk::utils::slashing::middleware::registrycoordinator::ISlashingRegistryCoordinatorTypes::OperatorSetParam;
use eigensdk::utils::slashing::middleware::registrycoordinator::RegistryCoordinator;

use super::error::EigenlayerError;
use crate::BlueprintConfig;
use crate::config::BlueprintEnvironment;
use crate::error::RunnerError;
use blueprint_core::{error, info, warn};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::backends::bn254::Bn254Backend;
use blueprint_keystore::backends::eigenlayer::EigenlayerBackend;
use blueprint_keystore::crypto::k256::K256Ecdsa;

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

async fn register_bls_impl(
    env: &BlueprintEnvironment,
    earnings_receiver_address: Address,
    delegation_approver_address: Address,
) -> Result<(), RunnerError> {
    info!("Eigenlayer BLS Registration: Fetching Contract Addresses");
    let contract_addresses = env.protocol_settings.eigenlayer()?;
    let allocation_manager_address = contract_addresses.allocation_manager_address;
    let registry_coordinator_address = contract_addresses.registry_coordinator_address;
    let delegation_manager_address = contract_addresses.delegation_manager_address;
    let strategy_manager_address = contract_addresses.strategy_manager_address;
    let rewards_coordinator_address = contract_addresses.rewards_coordinator_address;
    let avs_directory_address = contract_addresses.avs_directory_address;
    let permission_controller_address = contract_addresses.permission_controller_address;
    let service_manager_address = contract_addresses.service_manager_address;
    let registry_coordinator_address = contract_addresses.registry_coordinator_address;

    info!("Eigenlayer BLS Registration: Fetching ECDSA Keys");
    let ecdsa_public = env.keystore().first_local::<K256Ecdsa>()?;
    let ecdsa_secret = env
        .keystore()
        .expose_ecdsa_secret(&ecdsa_public)?
        .ok_or_else(|| RunnerError::Other("No ECDSA secret found".into()))?;
    let operator_address = ecdsa_secret
        .alloy_address()
        .map_err(|e| RunnerError::Eigenlayer(e.to_string()))?;

    let operator_private_key = hex::encode(ecdsa_secret.0.to_bytes());

    info!("Eigenlayer BLS Registration: Creating AVS Registry Writer");
    let logger = get_test_logger();
    let avs_registry_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
        logger.clone(),
        env.http_rpc_endpoint.clone(),
        operator_private_key.clone(),
        registry_coordinator_address,
        service_manager_address,
    )
    .await
    .map_err(EigenlayerError::AvsRegistry)?;

    info!("Eigenlayer BLS Registration: Fetching BLS BN254 Keys");
    let bn254_public = env.keystore().iter_bls_bn254().next().unwrap();
    let bn254_secret = env
        .keystore()
        .expose_bls_bn254_secret(&bn254_public)
        .map_err(|e| EigenlayerError::Keystore(e.to_string()))?
        .ok_or(EigenlayerError::Keystore(
            "Missing BLS BN254 key".to_string(),
        ))?;
    let operator_bls_key = BlsKeyPair::new(bn254_secret.0.to_string())
        .map_err(|e| EigenlayerError::Keystore(e.to_string()))?;

    let digest_hash: FixedBytes<32> = operator_address.into_word();

    let now = SystemTime::now();
    let mut expiry: U256 = U256::from(0);
    if let Ok(duration_since_epoch) = now.duration_since(UNIX_EPOCH) {
        let seconds = duration_since_epoch.as_secs();
        expiry = U256::from(seconds) + U256::from(10000);
    } else {
        warn!("System time seems to be before the UNIX epoch.");
    }

    let quorum_nums = Bytes::from(vec![0]);

    info!("Eigenlayer BLS Registration: Creating EL Chain Reader");
    let el_chain_reader = ELChainReader::new(
        logger,
        Some(allocation_manager_address),
        delegation_manager_address,
        rewards_coordinator_address,
        avs_directory_address,
        Some(permission_controller_address),
        env.http_rpc_endpoint.clone(),
    );

    let set_count = el_chain_reader
        .get_num_operator_sets_for_operator(operator_address)
        .await
        .unwrap();
    warn!("Set count: {:?}", set_count);

    let avs_directory = AVSDirectory::new(
        avs_directory_address,
        get_provider_http(&env.http_rpc_endpoint),
    );

    let owner = avs_directory.owner().call().await.unwrap();
    warn!("Owner: {:?}", owner._0);

    let hash = el_chain_reader
        .calculate_operator_avs_registration_digest_hash(
            operator_address,
            service_manager_address,
            digest_hash,
            expiry,
        )
        .await
        .unwrap();
    warn!("Hash: {:?}", hash);

    info!("Eigenlayer BLS Registration: Creating EL Chain Writer");
    let el_writer = ELChainWriter::new(
        strategy_manager_address,
        rewards_coordinator_address,
        Some(permission_controller_address),
        Some(allocation_manager_address),
        registry_coordinator_address,
        el_chain_reader,
        env.http_rpc_endpoint.clone(),
        operator_private_key,
    );

    let staker_opt_out_window_blocks = 50400u32;
    let operator_details = Operator {
        address: operator_address,
        delegation_approver_address,
        metadata_url: "https://github.com/tangle-network/blueprint".to_string(),
        allocation_delay: Some(30), // TODO: Make allocation delay configurable
        _deprecated_earnings_receiver_address: Some(earnings_receiver_address),
        staker_opt_out_window_blocks: Some(staker_opt_out_window_blocks),
    };

    let tx_hash = el_writer
        .register_as_operator_preslashing(operator_details)
        .await
        .map_err(EigenlayerError::ElContracts)?;
    let registration_receipt = wait_transaction(&env.http_rpc_endpoint, tx_hash)
        .await
        .map_err(|e| {
            EigenlayerError::Registration(format!("AVS registration error: {}", e.to_string()))
        })?;
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
        return Err(RunnerError::Other("Operator registration failed".into()));
    }

    let strategy_addr = address!("4b6ab5f819a515382b0deb6935d793817bb4af28");
    let amount = U256::from(100);
    let avs_deposit_hash = el_writer
        .deposit_erc20_into_strategy(strategy_addr, amount)
        .await
        .map_err(EigenlayerError::ElContracts)?;
    let avs_deposit_receipt = wait_transaction(&env.http_rpc_endpoint, avs_deposit_hash)
        .await
        .map_err(|e| {
            EigenlayerError::Registration(format!("AVS deposit error: {}", e.to_string()))
        })?;
    if avs_deposit_receipt.status() {
        info!("Deposited into strategy {} for Eigenlayer", strategy_addr);
    } else {
        error!("AVS deposit failed for strategy {}", strategy_addr);
        return Err(RunnerError::Other("AVS deposit failed".into()));
    }

    // info!("Registering to AVS");
    // let tx_hash = avs_registry_writer
    //     .register_operator_in_quorum_with_avs_registry_coordinator(
    //         operator_bls_key,
    //         digest_hash,
    //         expiry,
    //         quorum_nums,
    //         env.http_rpc_endpoint.clone(),
    //     )
    //     .await
    //     .unwrap();

    // info!("Waiting for AVS registration to complete");
    // let avs_registration_receipt = wait_transaction(&env.http_rpc_endpoint, tx_hash)
    //     .await
    //     .map_err(|e| {
    //         EigenlayerError::Registration(format!("AVS registration error: {}", e.to_string()))
    //     })?;
    // if avs_registration_receipt.status() {
    //     info!("Registered operator {} to AVS", operator_address);
    // } else {
    //     error!("AVS registration failed for operator {}", operator_address);
    //     return Err(RunnerError::Other("AVS registration failed".into()));
    // }

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
        .ok_or_else(|| RunnerError::Other("No ECDSA secret found".into()))?;
    let operator_address = ecdsa_secret
        .alloy_address()
        .map_err(|e| RunnerError::Eigenlayer(e.to_string()))?;

    let avs_registry_reader = eigensdk::client_avsregistry::reader::AvsRegistryChainReader::new(
        get_test_logger(),
        registry_coordinator_address,
        operator_state_retriever_address,
        env.http_rpc_endpoint.clone(),
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
