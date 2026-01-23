//! Tangle EVM Client
//!
//! Provides connectivity to Tangle v2 EVM contracts for blueprint operators.

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloy_network::Ethereum;
use alloy_primitives::{Address, B256, Bytes, TxKind, U256, keccak256};
use alloy_provider::{DynProvider, Provider, ProviderBuilder};
use alloy_rpc_types::{
    Block, BlockNumberOrTag, Filter, Log, TransactionReceipt,
    transaction::{TransactionInput, TransactionRequest},
};
use alloy_sol_types::SolType;
use blueprint_client_core::{BlueprintServicesClient, OperatorSet};
use blueprint_crypto::k256::K256Ecdsa;
use blueprint_keystore::Keystore;
use blueprint_keystore::backends::Backend;
use blueprint_std::collections::BTreeMap;
use blueprint_std::sync::Arc;
use blueprint_std::vec::Vec;
use core::fmt;
use core::time::Duration;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use tokio::sync::Mutex;

use crate::config::TangleEvmClientConfig;
use crate::contracts::{
    IBlueprintServiceManager, IMultiAssetDelegation, IMultiAssetDelegationTypes,
    IOperatorStatusRegistry, ITangle, ITangleTypes,
};
use crate::error::{Error, Result};
use crate::services::ServiceRequestParams;
use IMultiAssetDelegation::IMultiAssetDelegationInstance;
use IOperatorStatusRegistry::IOperatorStatusRegistryInstance;
use ITangle::ITangleInstance;

#[allow(missing_docs)]
mod erc20 {
    alloy_sol_types::sol! {
        #[sol(rpc)]
        interface IERC20 {
            function approve(address spender, uint256 amount) external returns (bool);
            function allowance(address owner, address spender) external view returns (uint256);
            function balanceOf(address owner) external view returns (uint256);
        }
    }
}

use erc20::IERC20;

/// Type alias for the dynamic provider
pub type TangleProvider = DynProvider<Ethereum>;

/// Type alias for ECDSA public key (uncompressed, 65 bytes)
pub type EcdsaPublicKey = [u8; 65];

/// Type alias for compressed ECDSA public key (33 bytes)
pub type CompressedEcdsaPublicKey = [u8; 33];

/// Restaking-specific metadata for an operator.
#[derive(Debug, Clone)]
pub struct RestakingMetadata {
    /// Operator self-stake amount (in wei).
    pub stake: U256,
    /// Number of delegations attached to this operator.
    pub delegation_count: u32,
    /// Whether the operator is active inside MultiAssetDelegation.
    pub status: RestakingStatus,
    /// Round when the operator scheduled a voluntary exit.
    pub leaving_round: u64,
}

/// Restaking status reported by MultiAssetDelegation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestakingStatus {
    /// Operator is active.
    Active,
    /// Operator is inactive (e.g., kicked or never joined).
    Inactive,
    /// Operator scheduled a leave operation.
    Leaving,
    /// Unknown status value (future-proofing).
    Unknown(u8),
}

impl From<u8> for RestakingStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => RestakingStatus::Active,
            1 => RestakingStatus::Inactive,
            2 => RestakingStatus::Leaving,
            other => RestakingStatus::Unknown(other),
        }
    }
}

/// Asset kinds supported by MultiAssetDelegation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetKind {
    /// Native asset (e.g. ETH).
    Native,
    /// ERC-20 token.
    Erc20,
    /// Unknown asset kind value.
    Unknown(u8),
}

impl From<u8> for AssetKind {
    fn from(value: u8) -> Self {
        match value {
            0 => AssetKind::Native,
            1 => AssetKind::Erc20,
            other => AssetKind::Unknown(other),
        }
    }
}

impl fmt::Display for AssetKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssetKind::Native => write!(f, "native"),
            AssetKind::Erc20 => write!(f, "erc20"),
            AssetKind::Unknown(value) => write!(f, "unknown({value})"),
        }
    }
}

/// Blueprint selection mode for a delegation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlueprintSelectionMode {
    /// Delegate across all blueprints.
    All,
    /// Delegate to a fixed set of blueprints.
    Fixed,
    /// Unknown selection mode value.
    Unknown(u8),
}

impl From<u8> for BlueprintSelectionMode {
    fn from(value: u8) -> Self {
        match value {
            0 => BlueprintSelectionMode::All,
            1 => BlueprintSelectionMode::Fixed,
            other => BlueprintSelectionMode::Unknown(other),
        }
    }
}

impl fmt::Display for BlueprintSelectionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlueprintSelectionMode::All => write!(f, "all"),
            BlueprintSelectionMode::Fixed => write!(f, "fixed"),
            BlueprintSelectionMode::Unknown(value) => write!(f, "unknown({value})"),
        }
    }
}

/// Lock multiplier tier for a deposit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockMultiplier {
    /// No lock multiplier.
    None,
    /// One-month lock.
    OneMonth,
    /// Two-month lock.
    TwoMonths,
    /// Three-month lock.
    ThreeMonths,
    /// Six-month lock.
    SixMonths,
    /// Unknown multiplier value.
    Unknown(u8),
}

impl From<u8> for LockMultiplier {
    fn from(value: u8) -> Self {
        match value {
            0 => LockMultiplier::None,
            1 => LockMultiplier::OneMonth,
            2 => LockMultiplier::TwoMonths,
            3 => LockMultiplier::ThreeMonths,
            4 => LockMultiplier::SixMonths,
            other => LockMultiplier::Unknown(other),
        }
    }
}

impl fmt::Display for LockMultiplier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LockMultiplier::None => write!(f, "none"),
            LockMultiplier::OneMonth => write!(f, "one-month"),
            LockMultiplier::TwoMonths => write!(f, "two-months"),
            LockMultiplier::ThreeMonths => write!(f, "three-months"),
            LockMultiplier::SixMonths => write!(f, "six-months"),
            LockMultiplier::Unknown(value) => write!(f, "unknown({value})"),
        }
    }
}

/// Asset specification returned by MultiAssetDelegation.
#[derive(Debug, Clone)]
pub struct AssetInfo {
    /// Asset kind identifier.
    pub kind: AssetKind,
    /// Token contract address (zero for native).
    pub token: Address,
}

/// Deposit summary for a delegator and token.
#[derive(Debug, Clone)]
pub struct DepositInfo {
    /// Total deposited amount.
    pub amount: U256,
    /// Portion already delegated.
    pub delegated_amount: U256,
}

/// Lock details for a delegator and token.
#[derive(Debug, Clone)]
pub struct LockInfo {
    /// Locked amount.
    pub amount: U256,
    /// Multiplier tier.
    pub multiplier: LockMultiplier,
    /// Block when lock expires.
    pub expiry_block: u64,
}

/// Delegation info for a delegator.
#[derive(Debug, Clone)]
pub struct DelegationInfo {
    /// Operator address.
    pub operator: Address,
    /// Delegated shares.
    pub shares: U256,
    /// Asset metadata.
    pub asset: AssetInfo,
    /// Blueprint selection mode.
    pub selection_mode: BlueprintSelectionMode,
}

/// Delegation with optional blueprint selections.
#[derive(Debug, Clone)]
pub struct DelegationRecord {
    /// Delegation metadata.
    pub info: DelegationInfo,
    /// Selected blueprint IDs (fixed mode only).
    pub blueprint_ids: Vec<u64>,
}

/// Pending delegator unstake request.
#[derive(Debug, Clone)]
pub struct PendingUnstake {
    /// Operator address.
    pub operator: Address,
    /// Asset metadata.
    pub asset: AssetInfo,
    /// Shares scheduled to unstake.
    pub shares: U256,
    /// Round when the unstake was requested.
    pub requested_round: u64,
    /// Blueprint selection mode.
    pub selection_mode: BlueprintSelectionMode,
    /// Slash factor snapshot at request time.
    pub slash_factor_snapshot: U256,
}

/// Pending delegator withdrawal request.
#[derive(Debug, Clone)]
pub struct PendingWithdrawal {
    /// Asset metadata.
    pub asset: AssetInfo,
    /// Amount requested for withdrawal.
    pub amount: U256,
    /// Round when the withdrawal was requested.
    pub requested_round: u64,
}

/// Metadata associated with a registered operator.
#[derive(Debug, Clone)]
pub struct OperatorMetadata {
    /// Operator's uncompressed ECDSA public key used for gossip/aggregation.
    pub public_key: EcdsaPublicKey,
    /// Operator-provided RPC endpoint.
    pub rpc_endpoint: String,
    /// Restaking information pulled from MultiAssetDelegation.
    pub restaking: RestakingMetadata,
}

/// Snapshot of an operator's heartbeat/status entry.
#[derive(Debug, Clone)]
pub struct OperatorStatusSnapshot {
    /// Service being inspected.
    pub service_id: u64,
    /// Operator address.
    pub operator: Address,
    /// Raw status code recorded on-chain.
    pub status_code: u8,
    /// Last heartbeat timestamp (Unix seconds).
    pub last_heartbeat: u64,
    /// Whether the operator is currently marked online.
    pub online: bool,
}

/// Event from Tangle EVM contracts
#[derive(Clone, Debug)]
pub struct TangleEvmEvent {
    /// Block number
    pub block_number: u64,
    /// Block hash
    pub block_hash: B256,
    /// Block timestamp
    pub timestamp: u64,
    /// Logs from the block
    pub logs: Vec<Log>,
}

/// Tangle EVM Client for interacting with Tangle v2 contracts
#[derive(Clone)]
pub struct TangleEvmClient {
    /// RPC provider
    provider: Arc<TangleProvider>,
    /// Tangle contract address
    tangle_address: Address,
    /// MultiAssetDelegation contract address
    restaking_address: Address,
    /// Operator status registry contract address
    status_registry_address: Address,
    /// Operator's account address
    account: Address,
    /// Client configuration
    pub config: TangleEvmClientConfig,
    /// Keystore for signing
    keystore: Arc<Keystore>,
    /// Latest block tracking
    latest_block: Arc<Mutex<Option<TangleEvmEvent>>>,
    /// Current block subscription
    block_subscription: Arc<Mutex<Option<u64>>>,
}

impl fmt::Debug for TangleEvmClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TangleEvmClient")
            .field("tangle_address", &self.tangle_address)
            .field("restaking_address", &self.restaking_address)
            .field("status_registry_address", &self.status_registry_address)
            .field("account", &self.account)
            .finish()
    }
}

impl TangleEvmClient {
    /// Create a new Tangle EVM client from configuration
    ///
    /// # Arguments
    /// * `config` - Client configuration
    ///
    /// # Errors
    /// Returns error if keystore initialization fails or RPC connection fails
    pub async fn new(config: TangleEvmClientConfig) -> Result<Self> {
        let keystore = Keystore::new(config.keystore_config())?;
        Self::with_keystore(config, keystore).await
    }

    /// Create a new Tangle EVM client with an existing keystore
    ///
    /// # Arguments
    /// * `config` - Client configuration
    /// * `keystore` - Keystore instance
    ///
    /// # Errors
    /// Returns error if RPC connection fails
    pub async fn with_keystore(config: TangleEvmClientConfig, keystore: Keystore) -> Result<Self> {
        let rpc_url = config.http_rpc_endpoint.as_str();

        // Create provider and wrap in DynProvider for type erasure
        let provider = ProviderBuilder::new()
            .connect(rpc_url)
            .await
            .map_err(|e| Error::Config(e.to_string()))?;

        let dyn_provider = DynProvider::new(provider);

        // Get operator's address from keystore (using ECDSA key)
        let ecdsa_key = keystore
            .first_local::<K256Ecdsa>()
            .map_err(Error::Keystore)?;

        // Convert ECDSA public key to Ethereum address
        // The key.0 is a VerifyingKey - extract the bytes from it
        let pubkey_bytes = ecdsa_key.0.to_encoded_point(false);
        let account = ecdsa_public_key_to_address(pubkey_bytes.as_bytes())?;

        Ok(Self {
            provider: Arc::new(dyn_provider),
            tangle_address: config.settings.tangle_contract,
            restaking_address: config.settings.restaking_contract,
            status_registry_address: config.settings.status_registry_contract,
            account,
            config,
            keystore: Arc::new(keystore),
            latest_block: Arc::new(Mutex::new(None)),
            block_subscription: Arc::new(Mutex::new(None)),
        })
    }

    /// Get the Tangle contract instance
    pub fn tangle_contract(&self) -> ITangleInstance<Arc<TangleProvider>> {
        ITangleInstance::new(self.tangle_address, Arc::clone(&self.provider))
    }

    /// Get the MultiAssetDelegation contract instance
    pub fn restaking_contract(&self) -> IMultiAssetDelegationInstance<Arc<TangleProvider>> {
        IMultiAssetDelegation::new(self.restaking_address, Arc::clone(&self.provider))
    }

    /// Get the operator status registry contract instance
    pub fn status_registry_contract(&self) -> IOperatorStatusRegistryInstance<Arc<TangleProvider>> {
        IOperatorStatusRegistryInstance::new(
            self.status_registry_address,
            Arc::clone(&self.provider),
        )
    }

    /// Get the operator's account address
    #[must_use]
    pub fn account(&self) -> Address {
        self.account
    }

    /// Get the keystore
    #[must_use]
    pub fn keystore(&self) -> &Arc<Keystore> {
        &self.keystore
    }

    /// Get the provider
    #[must_use]
    pub fn provider(&self) -> &Arc<TangleProvider> {
        &self.provider
    }

    /// Get the Tangle contract address
    #[must_use]
    pub fn tangle_address(&self) -> Address {
        self.tangle_address
    }

    /// Get the ECDSA signing key from the keystore
    ///
    /// # Errors
    /// Returns error if the key is not found in the keystore
    pub fn ecdsa_signing_key(&self) -> Result<blueprint_crypto::k256::K256SigningKey> {
        let public = self
            .keystore
            .first_local::<K256Ecdsa>()
            .map_err(Error::Keystore)?;
        self.keystore
            .get_secret::<K256Ecdsa>(&public)
            .map_err(Error::Keystore)
    }

    /// Get an Ethereum wallet for signing transactions
    ///
    /// # Errors
    /// Returns error if the key is not found or wallet creation fails
    pub fn wallet(&self) -> Result<alloy_network::EthereumWallet> {
        let signing_key = self.ecdsa_signing_key()?;
        let local_signer = signing_key
            .alloy_key()
            .map_err(|e| Error::Keystore(blueprint_keystore::Error::Other(e.to_string())))?;
        Ok(alloy_network::EthereumWallet::from(local_signer))
    }

    /// Get the current block number
    pub async fn block_number(&self) -> Result<u64> {
        self.provider
            .get_block_number()
            .await
            .map_err(Error::Transport)
    }

    /// Get a block by number
    pub async fn get_block(&self, number: BlockNumberOrTag) -> Result<Option<Block>> {
        self.provider
            .get_block_by_number(number)
            .await
            .map_err(Error::Transport)
    }

    /// Get logs matching a filter
    pub async fn get_logs(&self, filter: &Filter) -> Result<Vec<Log>> {
        self.provider
            .get_logs(filter)
            .await
            .map_err(Error::Transport)
    }

    /// Get the next event (polls for new blocks)
    pub async fn next_event(&self) -> Option<TangleEvmEvent> {
        loop {
            let current_block = self.block_number().await.ok()?;

            let mut last_block = self.block_subscription.lock().await;
            let from_block = last_block.map(|b| b + 1).unwrap_or(current_block);

            if from_block > current_block {
                drop(last_block);
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }

            // Get block info
            let block = self
                .get_block(BlockNumberOrTag::Number(current_block))
                .await
                .ok()??;

            // Create filter for Tangle contract events
            let filter = Filter::new()
                .address(self.tangle_address)
                .from_block(from_block)
                .to_block(current_block);

            let logs = self.get_logs(&filter).await.ok()?;

            *last_block = Some(current_block);

            let event = TangleEvmEvent {
                block_number: current_block,
                block_hash: block.header.hash,
                timestamp: block.header.timestamp,
                logs,
            };

            // Update latest
            *self.latest_block.lock().await = Some(event.clone());

            return Some(event);
        }
    }

    /// Get the latest observed event
    pub async fn latest_event(&self) -> Option<TangleEvmEvent> {
        let latest = self.latest_block.lock().await;
        match &*latest {
            Some(event) => Some(event.clone()),
            None => {
                drop(latest);
                self.next_event().await
            }
        }
    }

    /// Get the current block hash
    pub async fn now(&self) -> Option<B256> {
        Some(self.latest_event().await?.block_hash)
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // BLUEPRINT QUERIES
    // ═══════════════════════════════════════════════════════════════════════════

    /// Get blueprint information
    pub async fn get_blueprint(&self, blueprint_id: u64) -> Result<ITangleTypes::Blueprint> {
        let contract = self.tangle_contract();
        let result = contract
            .getBlueprint(blueprint_id)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        Ok(result)
    }

    /// Fetch the raw ABI-encoded blueprint definition bytes.
    pub async fn get_raw_blueprint_definition(&self, blueprint_id: u64) -> Result<Vec<u8>> {
        let mut data = Vec::with_capacity(4 + 32);
        let method_hash = keccak256("getBlueprintDefinition(uint64)".as_bytes());
        data.extend_from_slice(&method_hash[..4]);
        let mut arg = [0u8; 32];
        arg[24..].copy_from_slice(&blueprint_id.to_be_bytes());
        data.extend_from_slice(&arg);

        let mut request = TransactionRequest::default();
        request.to = Some(TxKind::Call(self.tangle_address));
        request.input = TransactionInput::new(Bytes::from(data));

        let response = self
            .provider
            .call(request)
            .await
            .map_err(Error::Transport)?;

        Ok(response.to_vec())
    }

    /// Get blueprint configuration
    pub async fn get_blueprint_config(
        &self,
        blueprint_id: u64,
    ) -> Result<ITangleTypes::BlueprintConfig> {
        let contract = self.tangle_contract();
        let result = contract
            .getBlueprintConfig(blueprint_id)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        Ok(result)
    }

    /// Get the full blueprint definition.
    pub async fn get_blueprint_definition(
        &self,
        blueprint_id: u64,
    ) -> Result<ITangleTypes::BlueprintDefinition> {
        let contract = self.tangle_contract();
        let result = contract
            .getBlueprintDefinition(blueprint_id)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        Ok(result)
    }

    /// Create a new blueprint from an encoded definition.
    pub async fn create_blueprint(
        &self,
        encoded_definition: Vec<u8>,
    ) -> Result<(TransactionResult, u64)> {
        let definition = ITangleTypes::BlueprintDefinition::abi_decode(encoded_definition.as_ref())
            .map_err(|err| {
                Error::Contract(format!("failed to decode blueprint definition: {err}"))
            })?;

        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = ITangle::new(self.tangle_address, &provider);
        let pending_tx = contract
            .createBlueprint(definition)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        let receipt = pending_tx.get_receipt().await?;
        let blueprint_id = self.extract_blueprint_id(&receipt)?;

        Ok((transaction_result_from_receipt(&receipt), blueprint_id))
    }

    /// Check if operator is registered for blueprint
    pub async fn is_operator_registered(
        &self,
        blueprint_id: u64,
        operator: Address,
    ) -> Result<bool> {
        let contract = self.tangle_contract();
        contract
            .isOperatorRegistered(blueprint_id, operator)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SERVICE QUERIES
    // ═══════════════════════════════════════════════════════════════════════════

    /// Get service information
    pub async fn get_service(&self, service_id: u64) -> Result<ITangleTypes::Service> {
        let contract = self.tangle_contract();
        let result = contract
            .getService(service_id)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        Ok(result)
    }

    /// Get service operators
    pub async fn get_service_operators(&self, service_id: u64) -> Result<Vec<Address>> {
        let contract = self.tangle_contract();
        contract
            .getServiceOperators(service_id)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Check if address is a service operator
    pub async fn is_service_operator(&self, service_id: u64, operator: Address) -> Result<bool> {
        let contract = self.tangle_contract();
        contract
            .isServiceOperator(service_id, operator)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Get service operator info including exposure
    ///
    /// Returns the `ServiceOperator` struct which contains `exposureBps`.
    pub async fn get_service_operator(
        &self,
        service_id: u64,
        operator: Address,
    ) -> Result<ITangleTypes::ServiceOperator> {
        let contract = self.tangle_contract();
        let result = contract
            .getServiceOperator(service_id, operator)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        Ok(result)
    }

    /// Get total exposure for a service
    ///
    /// Returns the sum of all operator exposureBps values.
    pub async fn get_service_total_exposure(&self, service_id: u64) -> Result<U256> {
        let mut total = U256::ZERO;
        for operator in self.get_service_operators(service_id).await? {
            let op_info = self.get_service_operator(service_id, operator).await?;
            if op_info.active {
                total = total.saturating_add(U256::from(op_info.exposureBps));
            }
        }
        Ok(total)
    }

    /// Get operator weights (exposureBps) for all operators in a service
    ///
    /// Returns a map of operator address to their exposure in basis points.
    /// This is useful for stake-weighted BLS signature threshold calculations.
    pub async fn get_service_operator_weights(
        &self,
        service_id: u64,
    ) -> Result<BTreeMap<Address, u16>> {
        let operators = self.get_service_operators(service_id).await?;
        let mut weights = BTreeMap::new();

        for operator in operators {
            let op_info = self.get_service_operator(service_id, operator).await?;
            if op_info.active {
                weights.insert(operator, op_info.exposureBps);
            }
        }

        Ok(weights)
    }

    /// Register the current operator for a blueprint.
    pub async fn register_operator(
        &self,
        blueprint_id: u64,
        rpc_endpoint: impl Into<String>,
        registration_inputs: Option<Bytes>,
    ) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = ITangle::new(self.tangle_address, &provider);

        let signing_key = self.ecdsa_signing_key()?;
        let verifying = signing_key.verifying_key();
        // Use uncompressed SEC1 format (65 bytes starting with 0x04)
        // The contract expects uncompressed public keys, not compressed (33 bytes)
        let encoded_point = verifying.0.to_encoded_point(false);
        let ecdsa_bytes = Bytes::copy_from_slice(encoded_point.as_bytes());
        let rpc_endpoint = rpc_endpoint.into();

        let receipt = if let Some(inputs) = registration_inputs {
            contract
                .registerOperator_0(
                    blueprint_id,
                    ecdsa_bytes.clone(),
                    rpc_endpoint.clone(),
                    inputs,
                )
                .send()
                .await
                .map_err(|e| Error::Contract(e.to_string()))?
                .get_receipt()
                .await?
        } else {
            contract
                .registerOperator_1(blueprint_id, ecdsa_bytes.clone(), rpc_endpoint.clone())
                .send()
                .await
                .map_err(|e| Error::Contract(e.to_string()))?
                .get_receipt()
                .await?
        };

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Unregister the current operator from a blueprint.
    pub async fn unregister_operator(&self, blueprint_id: u64) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = ITangle::new(self.tangle_address, &provider);

        let receipt = contract
            .unregisterOperator(blueprint_id)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Get the number of registered blueprints.
    pub async fn blueprint_count(&self) -> Result<u64> {
        let contract = self.tangle_contract();
        contract
            .blueprintCount()
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Get the number of registered services.
    pub async fn service_count(&self) -> Result<u64> {
        let contract = self.tangle_contract();
        contract
            .serviceCount()
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Get a service request by ID.
    pub async fn get_service_request(
        &self,
        request_id: u64,
    ) -> Result<ITangleTypes::ServiceRequest> {
        let contract = self.tangle_contract();
        contract
            .getServiceRequest(request_id)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Get the total number of service requests ever created.
    pub async fn service_request_count(&self) -> Result<u64> {
        let mut data = Vec::with_capacity(4);
        let selector = keccak256("serviceRequestCount()".as_bytes());
        data.extend_from_slice(&selector[..4]);

        let mut request = TransactionRequest::default();
        request.to = Some(TxKind::Call(self.tangle_address));
        request.input = TransactionInput::new(Bytes::from(data));

        let response = self
            .provider
            .call(request)
            .await
            .map_err(Error::Transport)?;

        if response.len() < 32 {
            return Err(Error::Contract(
                "serviceRequestCount returned malformed data".into(),
            ));
        }

        let raw = response.as_ref();
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&raw[24..32]);
        Ok(u64::from_be_bytes(buf))
    }

    /// Fetch metadata recorded for a specific job call.
    pub async fn get_job_call(
        &self,
        service_id: u64,
        call_id: u64,
    ) -> Result<ITangleTypes::JobCall> {
        let contract = self.tangle_contract();
        contract
            .getJobCall(service_id, call_id)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Fetch operator metadata (ECDSA public key + RPC endpoint) for a blueprint.
    pub async fn get_operator_metadata(
        &self,
        blueprint_id: u64,
        operator: Address,
    ) -> Result<OperatorMetadata> {
        let contract = self.tangle_contract();
        let prefs = contract
            .getOperatorPreferences(blueprint_id, operator)
            .call()
            .await
            .map_err(|e| Error::Contract(format!("getOperatorPreferences failed: {e}")))?;
        let restaking_meta = self
            .restaking_contract()
            .getOperatorMetadata(operator)
            .call()
            .await
            .map_err(|e| Error::Contract(format!("getOperatorMetadata failed: {e}")))?;
        let public_key = normalize_public_key(&prefs.ecdsaPublicKey.0)?;
        Ok(OperatorMetadata {
            public_key,
            rpc_endpoint: prefs.rpcAddress.to_string(),
            restaking: RestakingMetadata {
                stake: restaking_meta.stake,
                delegation_count: restaking_meta.delegationCount,
                status: RestakingStatus::from(u8::from(restaking_meta.status)),
                leaving_round: restaking_meta.leavingRound,
            },
        })
    }

    /// Submit a service request.
    #[allow(clippy::too_many_arguments)]
    pub async fn request_service(
        &self,
        params: ServiceRequestParams,
    ) -> Result<(TransactionResult, u64)> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = ITangle::new(self.tangle_address, &provider);

        let ServiceRequestParams {
            blueprint_id,
            operators,
            operator_exposures,
            permitted_callers,
            config,
            ttl,
            payment_token,
            payment_amount,
            security_requirements,
        } = params;

        let is_native_payment = payment_token == Address::ZERO && payment_amount > U256::ZERO;
        let request_id_hint = if !security_requirements.is_empty() {
            let mut call = contract.requestServiceWithSecurity(
                blueprint_id,
                operators.clone(),
                security_requirements.clone(),
                config.clone(),
                permitted_callers.clone(),
                ttl,
                payment_token,
                payment_amount,
            );
            call = call.from(self.account());
            if is_native_payment {
                call = call.value(payment_amount);
            }
            call.call().await.ok()
        } else if let Some(ref exposures) = operator_exposures {
            let mut call = contract.requestServiceWithExposure(
                blueprint_id,
                operators.clone(),
                exposures.clone(),
                config.clone(),
                permitted_callers.clone(),
                ttl,
                payment_token,
                payment_amount,
            );
            call = call.from(self.account());
            if is_native_payment {
                call = call.value(payment_amount);
            }
            call.call().await.ok()
        } else {
            let mut call = contract.requestService(
                blueprint_id,
                operators.clone(),
                config.clone(),
                permitted_callers.clone(),
                ttl,
                payment_token,
                payment_amount,
            );
            call = call.from(self.account());
            if is_native_payment {
                call = call.value(payment_amount);
            }
            call.call().await.ok()
        };
        let pre_count = self.service_request_count().await.ok();

        let pending_tx = if !security_requirements.is_empty() {
            let mut call = contract.requestServiceWithSecurity(
                blueprint_id,
                operators.clone(),
                security_requirements.clone(),
                config.clone(),
                permitted_callers.clone(),
                ttl,
                payment_token,
                payment_amount,
            );
            if is_native_payment {
                call = call.value(payment_amount);
            }
            call.send().await
        } else if let Some(exposures) = operator_exposures {
            let mut call = contract.requestServiceWithExposure(
                blueprint_id,
                operators.clone(),
                exposures,
                config.clone(),
                permitted_callers.clone(),
                ttl,
                payment_token,
                payment_amount,
            );
            if is_native_payment {
                call = call.value(payment_amount);
            }
            call.send().await
        } else {
            let mut call = contract.requestService(
                blueprint_id,
                operators.clone(),
                config.clone(),
                permitted_callers.clone(),
                ttl,
                payment_token,
                payment_amount,
            );
            if is_native_payment {
                call = call.value(payment_amount);
            }
            call.send().await
        }
        .map_err(|e| Error::Contract(e.to_string()))?;

        let receipt = pending_tx.get_receipt().await?;
        if !receipt.status() {
            return Err(Error::Contract(
                "requestService transaction reverted".into(),
            ));
        }

        let request_id = match self.extract_request_id(&receipt, blueprint_id).await {
            Ok(id) => id,
            Err(err) => {
                if let Some(id) = request_id_hint {
                    return Ok((transaction_result_from_receipt(&receipt), id));
                }
                if let Some(count) = pre_count {
                    return Ok((transaction_result_from_receipt(&receipt), count));
                }
                return Err(err);
            }
        };

        Ok((transaction_result_from_receipt(&receipt), request_id))
    }

    /// Join a dynamic service with the requested exposure.
    pub async fn join_service(
        &self,
        service_id: u64,
        exposure_bps: u16,
    ) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = ITangle::new(self.tangle_address, &provider);

        let receipt = contract
            .joinService(service_id, exposure_bps)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Join a dynamic service with the requested exposure and explicit security commitments.
    ///
    /// Use this method when the service has security requirements that mandate operators
    /// provide asset commitments when joining.
    pub async fn join_service_with_commitments(
        &self,
        service_id: u64,
        exposure_bps: u16,
        commitments: Vec<ITangleTypes::AssetSecurityCommitment>,
    ) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = ITangle::new(self.tangle_address, &provider);

        let receipt = contract
            .joinServiceWithCommitments(service_id, exposure_bps, commitments)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Leave a dynamic service using the legacy immediate exit helper.
    ///
    /// Note: This only works when `exitQueueDuration == 0`. For services with
    /// an exit queue (default 7 days), use the exit queue workflow:
    /// 1. `schedule_exit()` - Enter the exit queue
    /// 2. Wait for exit queue duration
    /// 3. `execute_exit()` - Complete the exit
    pub async fn leave_service(&self, service_id: u64) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = ITangle::new(self.tangle_address, &provider);

        let receipt = contract
            .leaveService(service_id)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Schedule an exit from a dynamic service.
    ///
    /// This enters the operator into the exit queue. After the exit queue duration
    /// has passed (default 7 days), call `execute_exit()` to complete the exit.
    ///
    /// Requires that the operator has fulfilled the minimum commitment duration
    /// since joining the service.
    pub async fn schedule_exit(&self, service_id: u64) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = ITangle::new(self.tangle_address, &provider);

        let receipt = contract
            .scheduleExit(service_id)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Execute a previously scheduled exit from a dynamic service.
    ///
    /// This completes the exit after the exit queue duration has passed.
    /// Must be called after `schedule_exit()` and waiting for the queue duration.
    pub async fn execute_exit(&self, service_id: u64) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = ITangle::new(self.tangle_address, &provider);

        let receipt = contract
            .executeExit(service_id)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Cancel a previously scheduled exit from a dynamic service.
    ///
    /// This cancels the exit and keeps the operator in the service.
    /// Can only be called before `execute_exit()`.
    pub async fn cancel_exit(&self, service_id: u64) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = ITangle::new(self.tangle_address, &provider);

        let receipt = contract
            .cancelExit(service_id)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Approve a pending service request with a simple restaking percentage.
    pub async fn approve_service(
        &self,
        request_id: u64,
        restaking_percent: u8,
    ) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = ITangle::new(self.tangle_address, &provider);

        let receipt = contract
            .approveService(request_id, restaking_percent)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Approve a service request with explicit security commitments.
    pub async fn approve_service_with_commitments(
        &self,
        request_id: u64,
        commitments: Vec<ITangleTypes::AssetSecurityCommitment>,
    ) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = ITangle::new(self.tangle_address, &provider);

        let receipt = contract
            .approveServiceWithCommitments(request_id, commitments)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Reject a pending service request.
    pub async fn reject_service(&self, request_id: u64) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = ITangle::new(self.tangle_address, &provider);

        let receipt = contract
            .rejectService(request_id)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // OPERATOR QUERIES (Restaking)
    // ═══════════════════════════════════════════════════════════════════════════

    /// Check if address is a registered operator
    pub async fn is_operator(&self, operator: Address) -> Result<bool> {
        let contract = self.restaking_contract();
        contract
            .isOperator(operator)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Check if operator is active
    pub async fn is_operator_active(&self, operator: Address) -> Result<bool> {
        let contract = self.restaking_contract();
        contract
            .isOperatorActive(operator)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Get operator's total stake
    pub async fn get_operator_stake(&self, operator: Address) -> Result<U256> {
        let contract = self.restaking_contract();
        contract
            .getOperatorStake(operator)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Get minimum operator stake requirement
    pub async fn min_operator_stake(&self) -> Result<U256> {
        let contract = self.restaking_contract();
        contract
            .minOperatorStake()
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Fetch status registry metadata for an operator/service pair.
    pub async fn operator_status(
        &self,
        service_id: u64,
        operator: Address,
    ) -> Result<OperatorStatusSnapshot> {
        if self.status_registry_address.is_zero() {
            return Err(Error::MissingStatusRegistry);
        }
        let contract = self.status_registry_contract();

        let last_heartbeat = contract
            .getLastHeartbeat(service_id, operator)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        let status_code = contract
            .getOperatorStatus(service_id, operator)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        let online = contract
            .isOnline(service_id, operator)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;

        let last_heartbeat = u64::try_from(last_heartbeat).unwrap_or(u64::MAX);

        Ok(OperatorStatusSnapshot {
            service_id,
            operator,
            status_code,
            last_heartbeat,
            online,
        })
    }

    /// Fetch restaking metadata for an operator.
    pub async fn get_restaking_metadata(&self, operator: Address) -> Result<RestakingMetadata> {
        let restaking_meta = self
            .restaking_contract()
            .getOperatorMetadata(operator)
            .call()
            .await
            .map_err(|e| Error::Contract(format!("getOperatorMetadata failed: {e}")))?;
        Ok(RestakingMetadata {
            stake: restaking_meta.stake,
            delegation_count: restaking_meta.delegationCount,
            status: RestakingStatus::from(u8::from(restaking_meta.status)),
            leaving_round: restaking_meta.leavingRound,
        })
    }

    /// Get operator self stake from MultiAssetDelegation.
    pub async fn get_operator_self_stake(&self, operator: Address) -> Result<U256> {
        let contract = self.restaking_contract();
        contract
            .getOperatorSelfStake(operator)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Get operator delegated stake from MultiAssetDelegation.
    pub async fn get_operator_delegated_stake(&self, operator: Address) -> Result<U256> {
        let contract = self.restaking_contract();
        contract
            .getOperatorDelegatedStake(operator)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Get delegators for the operator.
    pub async fn get_operator_delegators(&self, operator: Address) -> Result<Vec<Address>> {
        let contract = self.restaking_contract();
        contract
            .getOperatorDelegators(operator)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Get delegator count for the operator.
    pub async fn get_operator_delegator_count(&self, operator: Address) -> Result<u64> {
        let contract = self.restaking_contract();
        let count = contract
            .getOperatorDelegatorCount(operator)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        Ok(u64::try_from(count).unwrap_or(u64::MAX))
    }

    /// Get current restaking round.
    pub async fn restaking_round(&self) -> Result<u64> {
        let contract = self.restaking_contract();
        contract
            .currentRound()
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Get operator commission (basis points).
    pub async fn operator_commission_bps(&self) -> Result<u16> {
        let contract = self.restaking_contract();
        contract
            .operatorCommissionBps()
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Fetch ERC20 allowance for an owner/spender pair.
    pub async fn erc20_allowance(
        &self,
        token: Address,
        owner: Address,
        spender: Address,
    ) -> Result<U256> {
        let contract = IERC20::new(token, Arc::clone(&self.provider));
        contract
            .allowance(owner, spender)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Fetch ERC20 balance for an owner.
    pub async fn erc20_balance(&self, token: Address, owner: Address) -> Result<U256> {
        let contract = IERC20::new(token, Arc::clone(&self.provider));
        contract
            .balanceOf(owner)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Approve ERC20 spending for the given spender.
    pub async fn erc20_approve(
        &self,
        token: Address,
        spender: Address,
        amount: U256,
    ) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = IERC20::new(token, &provider);

        let receipt = contract
            .approve(spender, amount)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Fetch delegator deposit info for a token.
    pub async fn get_deposit_info(
        &self,
        delegator: Address,
        token: Address,
    ) -> Result<DepositInfo> {
        let contract = self.restaking_contract();
        let deposit = contract
            .getDeposit(delegator, token)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        Ok(DepositInfo {
            amount: deposit.amount,
            delegated_amount: deposit.delegatedAmount,
        })
    }

    /// Fetch lock info for a token.
    pub async fn get_locks(&self, delegator: Address, token: Address) -> Result<Vec<LockInfo>> {
        let contract = self.restaking_contract();
        let locks = contract
            .getLocks(delegator, token)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        Ok(locks
            .into_iter()
            .map(|lock| LockInfo {
                amount: lock.amount,
                multiplier: LockMultiplier::from(lock.multiplier),
                expiry_block: lock.expiryBlock,
            })
            .collect())
    }

    /// Fetch delegations for a delegator.
    pub async fn get_delegations(&self, delegator: Address) -> Result<Vec<DelegationInfo>> {
        let contract = self.restaking_contract();
        let delegations = contract
            .getDelegations(delegator)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        Ok(delegations
            .into_iter()
            .map(|delegation| DelegationInfo {
                operator: delegation.operator,
                shares: delegation.shares,
                asset: asset_info_from_types(delegation.asset),
                selection_mode: BlueprintSelectionMode::from(delegation.selectionMode),
            })
            .collect())
    }

    /// Fetch delegations with blueprint selections attached.
    pub async fn get_delegations_with_blueprints(
        &self,
        delegator: Address,
    ) -> Result<Vec<DelegationRecord>> {
        let delegations = self.get_delegations(delegator).await?;
        let mut records = Vec::with_capacity(delegations.len());
        for (idx, info) in delegations.into_iter().enumerate() {
            let blueprint_ids = if matches!(info.selection_mode, BlueprintSelectionMode::Fixed) {
                self.get_delegation_blueprints(delegator, idx as u64)
                    .await?
            } else {
                Vec::new()
            };
            records.push(DelegationRecord {
                info,
                blueprint_ids,
            });
        }
        Ok(records)
    }

    /// Fetch blueprint IDs for a fixed delegation.
    pub async fn get_delegation_blueprints(
        &self,
        delegator: Address,
        index: u64,
    ) -> Result<Vec<u64>> {
        let contract = self.restaking_contract();
        let ids = contract
            .getDelegationBlueprints(delegator, U256::from(index))
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        Ok(ids)
    }

    /// Fetch pending delegator unstakes.
    pub async fn get_pending_unstakes(&self, delegator: Address) -> Result<Vec<PendingUnstake>> {
        let contract = self.restaking_contract();
        let unstakes = contract
            .getPendingUnstakes(delegator)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        Ok(unstakes
            .into_iter()
            .map(|request| PendingUnstake {
                operator: request.operator,
                asset: asset_info_from_types(request.asset),
                shares: request.shares,
                requested_round: request.requestedRound,
                selection_mode: BlueprintSelectionMode::from(request.selectionMode),
                slash_factor_snapshot: request.slashFactorSnapshot,
            })
            .collect())
    }

    /// Fetch pending delegator withdrawals.
    pub async fn get_pending_withdrawals(
        &self,
        delegator: Address,
    ) -> Result<Vec<PendingWithdrawal>> {
        let contract = self.restaking_contract();
        let withdrawals = contract
            .getPendingWithdrawals(delegator)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        Ok(withdrawals
            .into_iter()
            .map(|request| PendingWithdrawal {
                asset: asset_info_from_types(request.asset),
                amount: request.amount,
                requested_round: request.requestedRound,
            })
            .collect())
    }

    /// Deposit and delegate in a single call.
    pub async fn deposit_and_delegate_with_options(
        &self,
        operator: Address,
        token: Address,
        amount: U256,
        selection_mode: BlueprintSelectionMode,
        blueprint_ids: Vec<u64>,
    ) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = IMultiAssetDelegation::new(self.restaking_address, &provider);

        let mut call = contract.depositAndDelegateWithOptions(
            operator,
            token,
            amount,
            selection_mode_to_u8(selection_mode),
            blueprint_ids,
        );
        if token == Address::ZERO {
            call = call.value(amount);
        }

        let receipt = call
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Delegate existing deposits with explicit selection.
    pub async fn delegate_with_options(
        &self,
        operator: Address,
        token: Address,
        amount: U256,
        selection_mode: BlueprintSelectionMode,
        blueprint_ids: Vec<u64>,
    ) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = IMultiAssetDelegation::new(self.restaking_address, &provider);

        let receipt = contract
            .delegateWithOptions(
                operator,
                token,
                amount,
                selection_mode_to_u8(selection_mode),
                blueprint_ids,
            )
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Schedule a delegator unstake (bond-less).
    pub async fn schedule_delegator_unstake(
        &self,
        operator: Address,
        token: Address,
        amount: U256,
    ) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = IMultiAssetDelegation::new(self.restaking_address, &provider);

        let receipt = contract
            .scheduleDelegatorUnstake(operator, token, amount)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Execute any matured delegator unstake requests.
    pub async fn execute_delegator_unstake(&self) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = IMultiAssetDelegation::new(self.restaking_address, &provider);

        let receipt = contract
            .executeDelegatorUnstake()
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Execute a specific delegator unstake and withdraw.
    pub async fn execute_delegator_unstake_and_withdraw(
        &self,
        operator: Address,
        token: Address,
        shares: U256,
        requested_round: u64,
        receiver: Address,
    ) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = IMultiAssetDelegation::new(self.restaking_address, &provider);

        let receipt = contract
            .executeDelegatorUnstakeAndWithdraw(operator, token, shares, requested_round, receiver)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Schedule a withdrawal for a token.
    pub async fn schedule_withdraw(
        &self,
        token: Address,
        amount: U256,
    ) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = IMultiAssetDelegation::new(self.restaking_address, &provider);

        let receipt = contract
            .scheduleWithdraw(token, amount)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Execute any matured withdrawal requests.
    pub async fn execute_withdraw(&self) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = IMultiAssetDelegation::new(self.restaking_address, &provider);

        let receipt = contract
            .executeWithdraw()
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Schedule an operator unstake.
    pub async fn schedule_operator_unstake(&self, amount: U256) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = IMultiAssetDelegation::new(self.restaking_address, &provider);

        let receipt = contract
            .scheduleOperatorUnstake(amount)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Execute an operator unstake.
    pub async fn execute_operator_unstake(&self) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = IMultiAssetDelegation::new(self.restaking_address, &provider);

        let receipt = contract
            .executeOperatorUnstake()
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Start leaving the operator set.
    pub async fn start_leaving(&self) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = IMultiAssetDelegation::new(self.restaking_address, &provider);

        let receipt = contract
            .startLeaving()
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Complete leaving after delay.
    pub async fn complete_leaving(&self) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = IMultiAssetDelegation::new(self.restaking_address, &provider);

        let receipt = contract
            .completeLeaving()
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // OPERATOR RESTAKING REGISTRATION
    // ═══════════════════════════════════════════════════════════════════════════

    /// Get the operator bond token address.
    ///
    /// Returns `Address::ZERO` if native ETH is used for operator bonds,
    /// otherwise returns the ERC20 token address (e.g., TNT).
    pub async fn operator_bond_token(&self) -> Result<Address> {
        let contract = self.restaking_contract();
        contract
            .operatorBondToken()
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Register as an operator on the restaking layer.
    ///
    /// Automatically uses the configured bond token (native ETH or ERC20 like TNT).
    /// For ERC20 tokens, automatically approves the restaking contract first.
    pub async fn register_operator_restaking(
        &self,
        stake_amount: U256,
    ) -> Result<TransactionResult> {
        let bond_token = self.operator_bond_token().await?;

        // Auto-approve ERC20 bond token if needed
        if bond_token != Address::ZERO {
            self.erc20_approve(bond_token, self.restaking_address, stake_amount)
                .await?;
        }

        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = IMultiAssetDelegation::new(self.restaking_address, &provider);

        let receipt = if bond_token == Address::ZERO {
            // Native ETH bond
            contract
                .registerOperator()
                .value(stake_amount)
                .send()
                .await
                .map_err(|e| Error::Contract(e.to_string()))?
                .get_receipt()
                .await?
        } else {
            // ERC20 bond (e.g., TNT)
            contract
                .registerOperatorWithAsset(bond_token, stake_amount)
                .send()
                .await
                .map_err(|e| Error::Contract(e.to_string()))?
                .get_receipt()
                .await?
        };

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Increase operator stake.
    ///
    /// Automatically uses the configured bond token (native ETH or ERC20 like TNT).
    /// For ERC20 tokens, automatically approves the restaking contract first.
    pub async fn increase_stake(&self, amount: U256) -> Result<TransactionResult> {
        let bond_token = self.operator_bond_token().await?;

        // Auto-approve ERC20 bond token if needed
        if bond_token != Address::ZERO {
            self.erc20_approve(bond_token, self.restaking_address, amount)
                .await?;
        }

        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = IMultiAssetDelegation::new(self.restaking_address, &provider);

        let receipt = if bond_token == Address::ZERO {
            // Native ETH bond
            contract
                .increaseStake()
                .value(amount)
                .send()
                .await
                .map_err(|e| Error::Contract(e.to_string()))?
                .get_receipt()
                .await?
        } else {
            // ERC20 bond (e.g., TNT)
            contract
                .increaseStakeWithAsset(bond_token, amount)
                .send()
                .await
                .map_err(|e| Error::Contract(e.to_string()))?
                .get_receipt()
                .await?
        };

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Deposit native ETH without delegating.
    ///
    /// Use this to pre-fund your account before delegating.
    pub async fn deposit_native(&self, amount: U256) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = IMultiAssetDelegation::new(self.restaking_address, &provider);

        let receipt = contract
            .deposit()
            .value(amount)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    /// Deposit ERC20 tokens without delegating.
    ///
    /// Use this to pre-fund your account before delegating.
    pub async fn deposit_erc20(&self, token: Address, amount: U256) -> Result<TransactionResult> {
        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;
        let contract = IMultiAssetDelegation::new(self.restaking_address, &provider);

        let receipt = contract
            .depositERC20(token, amount)
            .send()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            .get_receipt()
            .await?;

        Ok(transaction_result_from_receipt(&receipt))
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // BLS AGGREGATION QUERIES
    // ═══════════════════════════════════════════════════════════════════════════

    /// Get the blueprint manager address for a service
    pub async fn get_blueprint_manager(&self, service_id: u64) -> Result<Option<Address>> {
        let service = self.get_service(service_id).await?;
        let blueprint = self.get_blueprint(service.blueprintId).await?;
        if blueprint.manager == Address::ZERO {
            Ok(None)
        } else {
            Ok(Some(blueprint.manager))
        }
    }

    /// Check if a job requires BLS aggregation
    ///
    /// Queries the blueprint's service manager contract to determine if the specified
    /// job index requires aggregated BLS signatures instead of individual results.
    pub async fn requires_aggregation(&self, service_id: u64, job_index: u8) -> Result<bool> {
        let manager = match self.get_blueprint_manager(service_id).await? {
            Some(m) => m,
            None => return Ok(false), // No manager means no aggregation required
        };

        let bsm = IBlueprintServiceManager::new(manager, Arc::clone(&self.provider));
        match bsm.requiresAggregation(service_id, job_index).call().await {
            Ok(required) => Ok(required),
            Err(_) => Ok(false), // If call fails, assume no aggregation required
        }
    }

    /// Get the aggregation threshold configuration for a job
    ///
    /// Returns (threshold_bps, threshold_type) where:
    /// - threshold_bps: Threshold in basis points (e.g., 6700 = 67%)
    /// - threshold_type: 0 = CountBased (% of operators), 1 = StakeWeighted (% of stake)
    pub async fn get_aggregation_threshold(
        &self,
        service_id: u64,
        job_index: u8,
    ) -> Result<(u16, u8)> {
        let manager = match self.get_blueprint_manager(service_id).await? {
            Some(m) => m,
            None => return Ok((6700, 0)), // Default: 67% count-based
        };

        let bsm = IBlueprintServiceManager::new(manager, Arc::clone(&self.provider));
        match bsm
            .getAggregationThreshold(service_id, job_index)
            .call()
            .await
        {
            Ok(result) => Ok((result.thresholdBps, result.thresholdType)),
            Err(_) => Ok((6700, 0)), // Default if call fails
        }
    }

    /// Get the aggregation configuration for a specific job
    ///
    /// Returns the full aggregation config including whether it's required and threshold settings
    pub async fn get_aggregation_config(
        &self,
        service_id: u64,
        job_index: u8,
    ) -> Result<AggregationConfig> {
        let requires_aggregation = self.requires_aggregation(service_id, job_index).await?;
        let (threshold_bps, threshold_type) = self
            .get_aggregation_threshold(service_id, job_index)
            .await?;

        Ok(AggregationConfig {
            required: requires_aggregation,
            threshold_bps,
            threshold_type: if threshold_type == 0 {
                ThresholdType::CountBased
            } else {
                ThresholdType::StakeWeighted
            },
        })
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TRANSACTION SUBMISSION
    // ═══════════════════════════════════════════════════════════════════════════

    /// Submit a job invocation to the Tangle contract.
    pub async fn submit_job(
        &self,
        service_id: u64,
        job_index: u8,
        inputs: Bytes,
    ) -> Result<JobSubmissionResult> {
        use crate::contracts::ITangle::submitJobCall;
        use alloy_sol_types::SolCall;

        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;

        let call = submitJobCall {
            serviceId: service_id,
            jobIndex: job_index,
            inputs,
        };
        let calldata = call.abi_encode();

        let tx_request = TransactionRequest::default()
            .to(self.tangle_address)
            .input(calldata.into());

        let pending_tx = provider
            .send_transaction(tx_request)
            .await
            .map_err(Error::Transport)?;

        let receipt = pending_tx
            .get_receipt()
            .await
            .map_err(Error::PendingTransaction)?;

        let tx = TransactionResult {
            tx_hash: receipt.transaction_hash,
            block_number: receipt.block_number,
            gas_used: receipt.gas_used,
            success: receipt.status(),
        };

        let job_submitted_sig = keccak256("JobSubmitted(uint64,uint64,uint8,address,bytes)");
        let call_id = receipt
            .logs()
            .iter()
            .find_map(|log| {
                let topics = log.topics();
                if log.address() != self.tangle_address || topics.len() < 3 {
                    return None;
                }
                if topics[0].0 != job_submitted_sig {
                    return None;
                }
                let mut buf = [0u8; 32];
                buf.copy_from_slice(topics[2].as_slice());
                Some(U256::from_be_bytes(buf).to::<u64>())
            })
            .ok_or_else(|| {
                let status = receipt.status();
                let log_count = receipt.logs().len();
                let topics: Vec<String> = receipt
                    .logs()
                    .iter()
                    .map(|log| {
                        log.topics()
                            .iter()
                            .map(|topic| format!("{topic:#x}"))
                            .collect::<Vec<_>>()
                            .join(",")
                    })
                    .collect();
                Error::Contract(format!(
                    "submitJob receipt missing JobSubmitted event (status={status:?}, logs={log_count}, topics={topics:?})"
                ))
            })?;

        Ok(JobSubmissionResult { tx, call_id })
    }

    /// Submit a job result to the Tangle contract
    ///
    /// This sends a signed transaction to submit a single operator's result.
    ///
    /// # Arguments
    /// * `service_id` - The service ID
    /// * `call_id` - The call/job ID
    /// * `output` - The encoded result output
    ///
    /// # Returns
    /// The transaction hash and receipt on success
    pub async fn submit_result(
        &self,
        service_id: u64,
        call_id: u64,
        output: Bytes,
    ) -> Result<TransactionResult> {
        use crate::contracts::ITangle::submitResultCall;
        use alloy_sol_types::SolCall;

        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;

        let call = submitResultCall {
            serviceId: service_id,
            callId: call_id,
            result: output,
        };
        let calldata = call.abi_encode();

        let tx_request = TransactionRequest::default()
            .to(self.tangle_address)
            .input(calldata.into());

        let pending_tx = provider
            .send_transaction(tx_request)
            .await
            .map_err(Error::Transport)?;

        let receipt = pending_tx
            .get_receipt()
            .await
            .map_err(Error::PendingTransaction)?;

        Ok(TransactionResult {
            tx_hash: receipt.transaction_hash,
            block_number: receipt.block_number,
            gas_used: receipt.gas_used,
            success: receipt.status(),
        })
    }

    /// Submit an aggregated BLS signature result to the Tangle contract
    ///
    /// This sends a signed transaction to submit an aggregated result with BLS signature.
    ///
    /// # Arguments
    /// * `service_id` - The service ID
    /// * `call_id` - The call/job ID
    /// * `output` - The encoded result output
    /// * `signer_bitmap` - Bitmap indicating which operators signed
    /// * `aggregated_signature` - The aggregated BLS signature [2]
    /// * `aggregated_pubkey` - The aggregated BLS public key [4]
    ///
    /// # Returns
    /// The transaction hash and receipt on success
    pub async fn submit_aggregated_result(
        &self,
        service_id: u64,
        call_id: u64,
        output: Bytes,
        signer_bitmap: U256,
        aggregated_signature: [U256; 2],
        aggregated_pubkey: [U256; 4],
    ) -> Result<TransactionResult> {
        use crate::contracts::ITangle::submitAggregatedResultCall;
        use alloy_sol_types::SolCall;

        let wallet = self.wallet()?;
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect(self.config.http_rpc_endpoint.as_str())
            .await
            .map_err(Error::Transport)?;

        let call = submitAggregatedResultCall {
            serviceId: service_id,
            callId: call_id,
            output,
            signerBitmap: signer_bitmap,
            aggregatedSignature: aggregated_signature,
            aggregatedPubkey: aggregated_pubkey,
        };
        let calldata = call.abi_encode();

        let tx_request = TransactionRequest::default()
            .to(self.tangle_address)
            .input(calldata.into());

        let pending_tx = provider
            .send_transaction(tx_request)
            .await
            .map_err(Error::Transport)?;

        let receipt = pending_tx
            .get_receipt()
            .await
            .map_err(Error::PendingTransaction)?;

        Ok(TransactionResult {
            tx_hash: receipt.transaction_hash,
            block_number: receipt.block_number,
            gas_used: receipt.gas_used,
            success: receipt.status(),
        })
    }

    async fn extract_request_id(
        &self,
        receipt: &TransactionReceipt,
        blueprint_id: u64,
    ) -> Result<u64> {
        if let Some(event) = receipt.decoded_log::<ITangle::ServiceRequested>() {
            return Ok(event.data.requestId);
        }
        if let Some(event) = receipt.decoded_log::<ITangle::ServiceRequestedWithSecurity>() {
            return Ok(event.data.requestId);
        }

        let requested_sig = keccak256("ServiceRequested(uint64,uint64,address)".as_bytes());
        let requested_with_security_sig = keccak256(
            "ServiceRequestedWithSecurity(uint64,uint64,address,address[],((uint8,address),uint16,uint16)[])"
                .as_bytes(),
        );

        for log in receipt.logs() {
            let topics = log.topics();
            if topics.is_empty() {
                continue;
            }
            let sig = topics[0].0;
            if sig != requested_sig && sig != requested_with_security_sig {
                continue;
            }
            if topics.len() < 2 {
                continue;
            }

            let mut buf = [0u8; 32];
            buf.copy_from_slice(topics[1].as_slice());
            let id = U256::from_be_bytes(buf).to::<u64>();
            return Ok(id);
        }

        if let Some(block_number) = receipt.block_number {
            let filter = Filter::new()
                .select(block_number)
                .address(self.tangle_address)
                .event_signature(vec![requested_sig, requested_with_security_sig]);
            if let Ok(logs) = self.get_logs(&filter).await {
                for log in logs {
                    let topics = log.topics();
                    if topics.len() < 2 {
                        continue;
                    }
                    let mut buf = [0u8; 32];
                    buf.copy_from_slice(topics[1].as_slice());
                    let id = U256::from_be_bytes(buf).to::<u64>();
                    return Ok(id);
                }
            }
        }

        let count = self.service_request_count().await?;
        if count == 0 {
            return Err(Error::Contract(
                "requestService receipt missing ServiceRequested event".into(),
            ));
        }

        let account = self.account();
        let start = count.saturating_sub(5);
        for candidate in (start..count).rev() {
            if let Ok(request) = self.get_service_request(candidate).await {
                if request.blueprintId == blueprint_id && request.requester == account {
                    return Ok(candidate);
                }
            }
        }

        Ok(count - 1)
    }

    fn extract_blueprint_id(&self, receipt: &TransactionReceipt) -> Result<u64> {
        for log in receipt.logs() {
            if let Ok(event) = log.log_decode::<ITangle::BlueprintCreated>() {
                return Ok(event.inner.blueprintId);
            }
        }

        Err(Error::Contract(
            "createBlueprint receipt missing BlueprintCreated event".into(),
        ))
    }
}

/// Result of a submitted transaction
#[derive(Debug, Clone)]
pub struct TransactionResult {
    /// Transaction hash
    pub tx_hash: B256,
    /// Block number the transaction was included in
    pub block_number: Option<u64>,
    /// Gas used by the transaction
    pub gas_used: u64,
    /// Whether the transaction succeeded
    pub success: bool,
}

/// Result of submitting a job via `submitJob`.
#[derive(Debug, Clone)]
pub struct JobSubmissionResult {
    /// Transaction metadata.
    pub tx: TransactionResult,
    /// Call identifier assigned by the contract.
    pub call_id: u64,
}

/// Threshold type for BLS aggregation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdType {
    /// Threshold based on number of operators (e.g., 67% of operators must sign)
    CountBased,
    /// Threshold based on stake weight (e.g., 67% of total stake must sign)
    StakeWeighted,
}

/// Configuration for BLS signature aggregation
#[derive(Debug, Clone)]
pub struct AggregationConfig {
    /// Whether aggregation is required for this job
    pub required: bool,
    /// Threshold in basis points (e.g., 6700 = 67%)
    pub threshold_bps: u16,
    /// Type of threshold calculation
    pub threshold_type: ThresholdType,
}

/// Convert ECDSA public key to Ethereum address
fn ecdsa_public_key_to_address(pubkey: &[u8]) -> Result<Address> {
    use alloy_primitives::keccak256;

    // Handle both compressed (33 bytes) and uncompressed (65 bytes) keys
    let uncompressed = if pubkey.len() == 33 {
        // Decompress the key using k256
        use k256::EncodedPoint;
        use k256::elliptic_curve::sec1::FromEncodedPoint;

        let point = EncodedPoint::from_bytes(pubkey)
            .map_err(|e| Error::InvalidAddress(format!("Invalid compressed key: {e}")))?;

        let pubkey: k256::PublicKey = Option::from(k256::PublicKey::from_encoded_point(&point))
            .ok_or_else(|| Error::InvalidAddress("Failed to decompress public key".into()))?;

        pubkey.to_encoded_point(false).as_bytes().to_vec()
    } else if pubkey.len() == 65 {
        pubkey.to_vec()
    } else if pubkey.len() == 64 {
        // Already without prefix
        let mut full = vec![0x04];
        full.extend_from_slice(pubkey);
        full
    } else {
        return Err(Error::InvalidAddress(format!(
            "Invalid public key length: {}",
            pubkey.len()
        )));
    };

    // Skip the 0x04 prefix and hash the rest
    let hash = keccak256(&uncompressed[1..]);

    // Take the last 20 bytes as the address
    Ok(Address::from_slice(&hash[12..]))
}

fn normalize_public_key(raw: &[u8]) -> Result<EcdsaPublicKey> {
    match raw.len() {
        65 => {
            let mut key = [0u8; 65];
            key.copy_from_slice(raw);
            Ok(key)
        }
        64 => {
            let mut key = [0u8; 65];
            key[0] = 0x04;
            key[1..].copy_from_slice(raw);
            Ok(key)
        }
        33 => {
            use k256::EncodedPoint;
            use k256::elliptic_curve::sec1::FromEncodedPoint;

            let point = EncodedPoint::from_bytes(raw)
                .map_err(|e| Error::InvalidAddress(format!("Invalid compressed key: {e}")))?;
            let public_key: k256::PublicKey =
                Option::from(k256::PublicKey::from_encoded_point(&point)).ok_or_else(|| {
                    Error::InvalidAddress("Failed to decompress public key".into())
                })?;
            let encoded = public_key.to_encoded_point(false);
            let bytes = encoded.as_bytes();
            let mut key = [0u8; 65];
            key.copy_from_slice(bytes);
            Ok(key)
        }
        0 => Err(Error::Other(
            "Operator has not published an ECDSA public key".into(),
        )),
        len => Err(Error::InvalidAddress(format!(
            "Unexpected operator key length: {len}"
        ))),
    }
}

fn asset_info_from_types(asset: IMultiAssetDelegationTypes::Asset) -> AssetInfo {
    AssetInfo {
        kind: AssetKind::from(asset.kind),
        token: asset.token,
    }
}

fn selection_mode_to_u8(mode: BlueprintSelectionMode) -> u8 {
    match mode {
        BlueprintSelectionMode::All => 0,
        BlueprintSelectionMode::Fixed => 1,
        BlueprintSelectionMode::Unknown(value) => value,
    }
}

fn transaction_result_from_receipt(receipt: &TransactionReceipt) -> TransactionResult {
    TransactionResult {
        tx_hash: receipt.transaction_hash,
        block_number: receipt.block_number,
        gas_used: receipt.gas_used,
        success: receipt.status(),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// BLUEPRINT SERVICES CLIENT IMPLEMENTATION
// ═══════════════════════════════════════════════════════════════════════════════

impl BlueprintServicesClient for TangleEvmClient {
    type PublicApplicationIdentity = EcdsaPublicKey;
    type PublicAccountIdentity = Address;
    type Id = u64;
    type Error = Error;

    /// Get all operators for the current service with their ECDSA keys
    async fn get_operators(
        &self,
    ) -> core::result::Result<
        OperatorSet<Self::PublicAccountIdentity, Self::PublicApplicationIdentity>,
        Self::Error,
    > {
        let service_id = self
            .config
            .settings
            .service_id
            .ok_or_else(|| Error::Other("No service ID configured".into()))?;

        // Get service operators
        let operators = self.get_service_operators(service_id).await?;

        let mut map = BTreeMap::new();

        for operator in operators {
            let metadata = self
                .get_operator_metadata(self.config.settings.blueprint_id, operator)
                .await?;
            map.insert(operator, metadata.public_key);
        }

        Ok(map)
    }

    /// Get the current operator's ECDSA public key
    async fn operator_id(
        &self,
    ) -> core::result::Result<Self::PublicApplicationIdentity, Self::Error> {
        let key = self
            .keystore
            .first_local::<K256Ecdsa>()
            .map_err(Error::Keystore)?;

        // Convert VerifyingKey to 65-byte uncompressed format
        let encoded = key.0.to_encoded_point(false);
        let bytes = encoded.as_bytes();

        let mut uncompressed = [0u8; 65];
        uncompressed.copy_from_slice(bytes);

        Ok(uncompressed)
    }

    /// Get the current blueprint ID
    async fn blueprint_id(&self) -> core::result::Result<Self::Id, Self::Error> {
        Ok(self.config.settings.blueprint_id)
    }
}
