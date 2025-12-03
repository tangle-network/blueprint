//! Tangle EVM Client
//!
//! Provides connectivity to Tangle v2 EVM contracts for blueprint operators.

extern crate alloc;

use alloc::format;
use alloc::string::ToString;
use alloc::vec;
use alloy_network::Ethereum;
use alloy_primitives::{Address, B256, U256};
use alloy_provider::{DynProvider, Provider, ProviderBuilder};
use alloy_rpc_types::{Block, BlockNumberOrTag, Filter, Log};
use blueprint_client_core::{BlueprintServicesClient, OperatorSet};
use blueprint_crypto::k256::K256Ecdsa;
use blueprint_keystore::backends::Backend;
use blueprint_keystore::Keystore;
use blueprint_std::collections::BTreeMap;
use blueprint_std::sync::Arc;
use blueprint_std::vec::Vec;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use tokio::sync::Mutex;

use crate::config::TangleEvmClientConfig;
use crate::contracts::{IMultiAssetDelegation, ITangle};
use crate::error::{Error, Result};

/// Type alias for the dynamic provider
pub type TangleProvider = DynProvider<Ethereum>;

/// Type alias for ECDSA public key (uncompressed, 65 bytes)
pub type EcdsaPublicKey = [u8; 65];

/// Type alias for compressed ECDSA public key (33 bytes)
pub type CompressedEcdsaPublicKey = [u8; 33];

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

impl core::fmt::Debug for TangleEvmClient {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TangleEvmClient")
            .field("tangle_address", &self.tangle_address)
            .field("restaking_address", &self.restaking_address)
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
            account,
            config,
            keystore: Arc::new(keystore),
            latest_block: Arc::new(Mutex::new(None)),
            block_subscription: Arc::new(Mutex::new(None)),
        })
    }

    /// Get the Tangle contract instance
    pub fn tangle_contract(&self) -> ITangle::ITangleInstance<Arc<TangleProvider>> {
        ITangle::new(self.tangle_address, Arc::clone(&self.provider))
    }

    /// Get the MultiAssetDelegation contract instance
    pub fn restaking_contract(&self) -> IMultiAssetDelegation::IMultiAssetDelegationInstance<Arc<TangleProvider>> {
        IMultiAssetDelegation::new(self.restaking_address, Arc::clone(&self.provider))
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
        let current_block = self.block_number().await.ok()?;

        let mut last_block = self.block_subscription.lock().await;
        let from_block = last_block.map(|b| b + 1).unwrap_or(current_block);

        if from_block > current_block {
            return None;
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

        Some(event)
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
    pub async fn get_blueprint(&self, blueprint_id: u64) -> Result<ITangle::getBlueprintReturn> {
        let contract = self.tangle_contract();
        contract
            .getBlueprint(blueprint_id)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    /// Get blueprint configuration
    pub async fn get_blueprint_config(
        &self,
        blueprint_id: u64,
    ) -> Result<ITangle::getBlueprintConfigReturn> {
        let contract = self.tangle_contract();
        contract
            .getBlueprintConfig(blueprint_id)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
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

    /// Get all operators registered for a blueprint
    pub async fn get_blueprint_operators(&self, blueprint_id: u64) -> Result<Vec<Address>> {
        let contract = self.tangle_contract();
        contract
            .getBlueprintOperators(blueprint_id)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SERVICE QUERIES
    // ═══════════════════════════════════════════════════════════════════════════

    /// Get service information
    pub async fn get_service(&self, service_id: u64) -> Result<ITangle::getServiceReturn> {
        let contract = self.tangle_contract();
        contract
            .getService(service_id)
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))
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
}

/// Convert ECDSA public key to Ethereum address
fn ecdsa_public_key_to_address(pubkey: &[u8]) -> Result<Address> {
    use alloy_primitives::keccak256;

    // Handle both compressed (33 bytes) and uncompressed (65 bytes) keys
    let uncompressed = if pubkey.len() == 33 {
        // Decompress the key using k256
        use k256::elliptic_curve::sec1::FromEncodedPoint;
        use k256::EncodedPoint;

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
            // For now, we use the address directly
            // In a full implementation, we'd query operator preferences for their ECDSA key
            // The preferences contain the uncompressed ECDSA public key

            // Placeholder: convert address to a dummy 65-byte key
            // TODO: Query actual ECDSA keys from operator preferences
            let mut key = [0u8; 65];
            key[0] = 0x04; // Uncompressed prefix
            key[1..21].copy_from_slice(operator.as_slice());

            map.insert(operator, key);
        }

        Ok(map)
    }

    /// Get the current operator's ECDSA public key
    async fn operator_id(&self) -> core::result::Result<Self::PublicApplicationIdentity, Self::Error> {
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
