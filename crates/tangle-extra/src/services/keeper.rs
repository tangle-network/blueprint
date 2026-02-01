//! Core keeper abstractions for background lifecycle automation
//!
//! Provides reusable components for building keepers that monitor and trigger
//! lifecycle operations on Tangle v2 contracts.

use alloy::network::EthereumWallet;
use alloy::primitives::Address;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use blueprint_keystore::Keystore;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

/// Result type for keeper operations
pub type KeeperResult<T> = Result<T, KeeperError>;

/// Errors that can occur during keeper operations
#[derive(Error, Debug)]
pub enum KeeperError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Keystore error
    #[error("Keystore error: {0}")]
    Keystore(#[from] blueprint_keystore::Error),

    /// Transaction error
    #[error("Transaction error: {0}")]
    Transaction(String),

    /// Contract interaction error
    #[error("Contract error: {0}")]
    Contract(String),

    /// Provider error
    #[error("Provider error: {0}")]
    Provider(String),
}

/// Handle to a running background keeper
pub struct KeeperHandle {
    /// Join handle for the background task
    pub handle: JoinHandle<KeeperResult<()>>,
    /// Name of the keeper for logging
    pub name: &'static str,
}

impl KeeperHandle {
    /// Wait for the keeper to complete
    pub async fn join(self) -> KeeperResult<()> {
        self.handle
            .await
            .map_err(|e| KeeperError::Config(format!("Keeper {} panicked: {}", self.name, e)))?
    }
}

/// Configuration for lifecycle keepers
#[derive(Clone)]
pub struct KeeperConfig {
    /// HTTP RPC endpoint for the chain
    pub http_rpc_endpoint: String,

    /// Keystore for signing transactions
    pub keystore: Arc<Keystore>,

    /// InflationPool contract address (optional)
    pub inflation_pool: Option<Address>,

    /// MultiAssetDelegation contract address (optional)
    pub multi_asset_delegation: Option<Address>,

    /// StreamingPaymentManager contract address (optional)
    pub streaming_payment_manager: Option<Address>,

    /// Check interval for the epoch keeper (default: 5 minutes)
    pub epoch_check_interval: Duration,

    /// Check interval for the round keeper (default: 1 minute)
    pub round_check_interval: Duration,

    /// Check interval for the stream keeper (default: 10 minutes)
    pub stream_check_interval: Duration,

    /// Operators to monitor for stream drips (optional, if empty monitors own operator)
    pub monitored_operators: Vec<Address>,
}

impl KeeperConfig {
    /// Create a new keeper config with required fields
    pub fn new(http_rpc_endpoint: String, keystore: Arc<Keystore>) -> Self {
        Self {
            http_rpc_endpoint,
            keystore,
            inflation_pool: None,
            multi_asset_delegation: None,
            streaming_payment_manager: None,
            epoch_check_interval: Duration::from_secs(300), // 5 minutes
            round_check_interval: Duration::from_secs(60),  // 1 minute
            stream_check_interval: Duration::from_secs(600), // 10 minutes
            monitored_operators: Vec::new(),
        }
    }

    /// Set the InflationPool contract address
    pub fn with_inflation_pool(mut self, address: Address) -> Self {
        self.inflation_pool = Some(address);
        self
    }

    /// Set the MultiAssetDelegation contract address
    pub fn with_multi_asset_delegation(mut self, address: Address) -> Self {
        self.multi_asset_delegation = Some(address);
        self
    }

    /// Set the StreamingPaymentManager contract address
    pub fn with_streaming_payment_manager(mut self, address: Address) -> Self {
        self.streaming_payment_manager = Some(address);
        self
    }

    /// Set the epoch check interval
    pub fn with_epoch_interval(mut self, interval: Duration) -> Self {
        self.epoch_check_interval = interval;
        self
    }

    /// Set the round check interval
    pub fn with_round_interval(mut self, interval: Duration) -> Self {
        self.round_check_interval = interval;
        self
    }

    /// Set the stream check interval
    pub fn with_stream_interval(mut self, interval: Duration) -> Self {
        self.stream_check_interval = interval;
        self
    }

    /// Add operators to monitor for stream drips
    pub fn with_monitored_operators(mut self, operators: Vec<Address>) -> Self {
        self.monitored_operators = operators;
        self
    }

    /// Get the signer from the keystore
    pub fn get_signer(&self) -> KeeperResult<PrivateKeySigner> {
        use blueprint_crypto::BytesEncoding;
        use blueprint_keystore::backends::Backend;
        use blueprint_keystore::crypto::k256::K256Ecdsa;

        let ecdsa_public = self
            .keystore
            .as_ref()
            .first_local::<K256Ecdsa>()
            .map_err(KeeperError::Keystore)?;

        let ecdsa_secret = self
            .keystore
            .as_ref()
            .get_secret::<K256Ecdsa>(&ecdsa_public)
            .map_err(KeeperError::Keystore)?;

        let private_key = alloy::primitives::hex::encode(ecdsa_secret.to_bytes());

        private_key
            .parse()
            .map_err(|e| KeeperError::Config(format!("Invalid private key: {}", e)))
    }

    /// Get the operator address from the keystore
    pub fn get_operator_address(&self) -> KeeperResult<Address> {
        let signer = self.get_signer()?;
        Ok(signer.address())
    }

    /// Create a provider with wallet for sending transactions
    pub async fn get_provider(&self) -> KeeperResult<impl alloy::providers::Provider + Clone> {
        let signer = self.get_signer()?;
        let wallet = EthereumWallet::from(signer);

        ProviderBuilder::new()
            .wallet(wallet)
            .connect(&self.http_rpc_endpoint)
            .await
            .map_err(|e| KeeperError::Provider(e.to_string()))
    }

    /// Create a read-only provider (no wallet needed)
    pub async fn get_read_provider(&self) -> KeeperResult<impl alloy::providers::Provider + Clone> {
        ProviderBuilder::new()
            .connect(&self.http_rpc_endpoint)
            .await
            .map_err(|e| KeeperError::Provider(e.to_string()))
    }
}

/// Trait for background keepers
pub trait BackgroundKeeper: Sized {
    /// The name of this keeper (for logging)
    const NAME: &'static str;

    /// Start the background keeper
    fn start(config: KeeperConfig, shutdown: broadcast::Receiver<()>) -> KeeperHandle;

    /// Run a single check iteration
    /// Returns Ok(true) if an action was taken, Ok(false) if no action needed
    fn check_and_execute(
        config: &KeeperConfig,
    ) -> impl std::future::Future<Output = KeeperResult<bool>> + Send;
}
