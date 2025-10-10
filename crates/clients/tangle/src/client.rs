use sp_core::ecdsa;
use crate::error::{Result, Error};
use blueprint_std::sync::Arc;
use blueprint_std::time::Duration;
use subxt::blocks::{Block, BlockRef};
use subxt::events::Events;
use subxt::utils::AccountId32;
use subxt::PolkadotConfig;
use tangle_subxt::subxt;
use tangle_subxt::tangle_testnet_runtime::api;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::pallet_multi_asset_delegation::types::operator::OperatorMetadata;
use blueprint_client_core::{BlueprintServicesClient, EventsClient, OperatorSet};
use blueprint_runner::config::BlueprintEnvironment;
use blueprint_crypto_sp_core::{SpEcdsa, SpSr25519};
use blueprint_keystore::{Keystore, KeystoreConfig};
use blueprint_keystore::backends::Backend;
use crate::services::TangleServicesClient;

/// The [Config](subxt::Config) providing the runtime types.
pub type TangleConfig = PolkadotConfig;
/// The client used to perform API calls, using the [`TangleConfig`].
pub type OnlineClient = subxt::OnlineClient<TangleConfig>;
type TangleBlock = Block<TangleConfig, OnlineClient>;
type TangleBlockStream = subxt::backend::StreamOfResults<TangleBlock>;

#[derive(Clone, Debug)]
pub struct TangleEvent {
    /// Finalized block number.
    pub number: u64,
    /// Finalized block header hash.
    pub hash: [u8; 32],
    /// Events
    pub events: Events<TangleConfig>,
}

#[derive(Clone)]
pub struct TangleClient {
    finality_notification_stream: Arc<tokio::sync::Mutex<Option<TangleBlockStream>>>,
    latest_finality_notification: Arc<tokio::sync::Mutex<Option<TangleEvent>>>,
    account_id: AccountId32,
    pub config: BlueprintEnvironment,
    keystore: Arc<Keystore>,
    services_client: TangleServicesClient<TangleConfig>,
}

impl TangleClient {
    /// Create a new Tangle runtime client from an existing [`BlueprintEnvironment`].
    ///
    /// # Errors
    ///
    /// See [`Keystore::new()`]
    /// See [`Self::with_keystore()`]
    pub async fn new(config: BlueprintEnvironment) -> std::result::Result<Self, Error> {
        let keystore_config =
            KeystoreConfig::new().fs_root(config.keystore_uri.replace("file://", ""));

        Self::with_keystore(config, Keystore::new(keystore_config)?).await
    }

    /// Create a new Tangle runtime client from an existing [`BlueprintEnvironment`] and a [`Keystore`].
    ///
    /// # Errors
    ///
    /// See [`subxt::OnlineClient::from_url()`]
    pub async fn with_keystore(
        config: BlueprintEnvironment,
        keystore: Keystore,
    ) -> std::result::Result<Self, Error> {
        let rpc_url = config.ws_rpc_endpoint.as_str();
        let client =
            TangleServicesClient::new(subxt::OnlineClient::from_insecure_url(rpc_url).await?);

        let account_id = keystore
            .first_local::<SpSr25519>()
            .map_err(Error::Keystore)?
            .0
            .0
            .into();

        Ok(Self {
            keystore: Arc::new(keystore),
            services_client: client,
            finality_notification_stream: Arc::new(tokio::sync::Mutex::new(None)),
            latest_finality_notification: Arc::new(tokio::sync::Mutex::new(None)),
            account_id,
            config,
        })
    }

    /// Get the associated [`TangleServicesClient`]
    #[must_use]
    pub fn services_client(&self) -> &TangleServicesClient<subxt::PolkadotConfig> {
        &self.services_client
    }

    #[must_use]
    pub fn subxt_client(&self) -> &OnlineClient {
        &self.services_client().rpc_client
    }

    /// Initialize the `TangleRuntime` instance by listening for finality notifications.
    ///
    /// This method must be called before using the instance.
    async fn initialize(&self) -> Result<()> {
        let finality_notification_stream = self
            .services_client()
            .rpc_client
            .blocks()
            .subscribe_finalized()
            .await?;
        *self.finality_notification_stream.lock().await = Some(finality_notification_stream);
        Ok(())
    }

    #[must_use]
    pub fn runtime_api(
        &self,
        at: [u8; 32],
    ) -> subxt::runtime_api::RuntimeApi<TangleConfig, OnlineClient> {
        let block_ref = BlockRef::from_hash(subxt::utils::H256::from_slice(&at));
        self.services_client.rpc_client.runtime_api().at(block_ref)
    }

    #[must_use]
    pub fn account_id(&self) -> &AccountId32 {
        &self.account_id
    }

    /// Get [`metadata`](OperatorMetadata) for an operator by [`Account ID`](AccountId32)
    #[allow(clippy::missing_errors_doc)]
    pub async fn operator_metadata(
        &self,
        operator: AccountId32,
    ) -> std::result::Result<
        Option<
            OperatorMetadata<
                AccountId32,
                api::assets::events::burned::Balance,
                api::assets::events::accounts_destroyed::AssetId,
                api::runtime_types::tangle_testnet_runtime::MaxDelegations,
                api::runtime_types::tangle_testnet_runtime::MaxOperatorBlueprints,
            >,
        >,
        Error,
    > {
        let storage = self
            .services_client
            .rpc_client
            .storage()
            .at_latest()
            .await?;
        let metadata_storage_key = api::storage().multi_asset_delegation().operators(operator);

        let ret = storage.fetch(&metadata_storage_key).await?;
        Ok(ret)
    }

    /// Retrieves the current party index and operator mapping
    ///
    /// # Errors
    /// Returns an error if:
    /// - Failed to retrieve operator keys
    /// - Current party is not found in the operator list
    pub async fn get_party_index_and_operators(
        &self,
    ) -> std::result::Result<
        (
            usize,
            std::collections::BTreeMap<AccountId32, ecdsa::Public>,
        ),
        Error,
    > {
        let parties = self.get_operators().await?;
        let my_id = self
            .keystore
            .first_local::<SpSr25519>()
            .map_err(Error::Keystore)?;

        blueprint_core::trace!(
            "Looking for {my_id} in parties: {:?}",
            parties.keys().collect::<Vec<_>>()
        );

        let index_of_my_id = parties
            .iter()
            .position(|(id, _key)| id.0 == my_id.0.to_raw())
            .ok_or(Error::PartyNotFound)?;

        Ok((index_of_my_id, parties))
    }

    pub async fn now(&self) -> Option<[u8; 32]> {
        Some(self.latest_event().await?.hash)
    }
}

impl blueprint_std::ops::Deref for TangleClient {
    type Target = TangleServicesClient<TangleConfig>;

    fn deref(&self) -> &Self::Target {
        &self.services_client
    }
}

impl EventsClient<TangleEvent> for TangleClient {
    async fn next_event(&self) -> Option<TangleEvent> {
        let mut finality_stream = tokio::time::timeout(
            Duration::from_millis(500),
            self.finality_notification_stream.lock(),
        )
        .await
        .ok()?;
        match finality_stream.as_mut() {
            Some(stream) => {
                let block = stream.next().await?.ok()?;
                let events = block.events().await.ok()?;
                let notification = TangleEvent {
                    number: block.number().into(),
                    hash: block.hash().into(),
                    events,
                };
                let mut latest_finality = tokio::time::timeout(
                    Duration::from_millis(500),
                    self.latest_finality_notification.lock(),
                )
                .await
                .ok()?;
                *latest_finality = Some(notification.clone());
                Some(notification)
            }
            None => {
                drop(finality_stream);
                self.initialize().await.ok()?;
                // Next time, the stream should be initialized.
                Box::pin(async { self.next_event().await }).await
            }
        }
    }

    async fn latest_event(&self) -> Option<TangleEvent> {
        let latest_finality = tokio::time::timeout(
            Duration::from_millis(500),
            self.latest_finality_notification.lock(),
        )
        .await
        .ok()?;
        match &*latest_finality {
            Some(notification) => Some(notification.clone()),
            None => {
                drop(latest_finality);
                self.next_event().await
            }
        }
    }
}

pub type BlueprintId = u64;

impl BlueprintServicesClient for TangleClient {
    type PublicApplicationIdentity = ecdsa::Public;
    type PublicAccountIdentity = AccountId32;
    type Id = BlueprintId;
    type Error = Error;

    /// Retrieves the ECDSA keys for all current service operators
    ///
    /// # Errors
    /// Returns an error if:
    /// - Failed to connect to the Tangle client
    /// - Failed to retrieve operator information
    /// - Missing ECDSA key for any operator
    async fn get_operators(
        &self,
    ) -> std::result::Result<
        OperatorSet<Self::PublicAccountIdentity, Self::PublicApplicationIdentity>,
        Self::Error,
    > {
        let client = &self.services_client;
        let current_blueprint = self.blueprint_id().await?;
        let service_id = self
            .config
            .protocol_settings
            .tangle()
            .map_err(|_| Error::NotTangle)?
            .service_id
            .ok_or_else(|| Error::Other("No service ID injected into config".into()))?;
        let now = self
            .now()
            .await
            .ok_or_else(|| Error::Other("no timestamp in latest".into()))?;
        let current_service_op = self
            .services_client
            .current_service_operators(now, service_id)
            .await?;
        let storage = client.rpc_client.storage().at_latest().await?;

        let mut map = std::collections::BTreeMap::new();
        for (operator, _) in current_service_op {
            let addr = api::storage()
                .services()
                .operators(current_blueprint, &operator);

            let maybe_pref = storage.fetch(&addr).await.map_err(|err| {
                Error::Other(format!(
                    "Failed to fetch operator storage for {operator}: {err}"
                ))
            })?;

            if let Some(pref) = maybe_pref {
                let public = ecdsa::Public::from_full(pref.key.as_slice()).map_err(|()| {
                    Error::Other(format!(
                        "Failed to convert the ECDSA public key for operator: {operator}"
                    ))
                })?;

                map.insert(operator, public);
            } else {
                return Err(Error::MissingEcdsa(operator));
            }
        }

        Ok(map)
    }

    async fn operator_id(
        &self,
    ) -> std::result::Result<Self::PublicApplicationIdentity, Self::Error> {
        Ok(self.keystore.first_local::<SpEcdsa>()?.0)
    }

    /// Retrieves the current blueprint ID from the configuration
    ///
    /// # Errors
    /// Returns an error if the blueprint ID is not found in the configuration
    async fn blueprint_id(&self) -> std::result::Result<Self::Id, Self::Error> {
        let c = self
            .config
            .protocol_settings
            .tangle()
            .map_err(|_| Error::NotTangle)?;
        Ok(c.blueprint_id)
    }
}
