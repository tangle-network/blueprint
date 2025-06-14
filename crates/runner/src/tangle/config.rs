use crate::BlueprintConfig;
use crate::config::{BlueprintEnvironment, BlueprintSettings, Protocol, ProtocolSettingsT};
use crate::error::{ConfigError, RunnerError};
use crate::tangle::error::TangleError;
use blueprint_keystore::backends::Backend;
use blueprint_keystore::crypto::sp_core::{SpEcdsa, SpSr25519};
use blueprint_keystore::crypto::tangle_pair_signer::pair_signer::PairSigner;
use blueprint_keystore::crypto::tangle_pair_signer::sp_core;
use blueprint_tangle_extra::serde::new_bounded_string;
use futures_util::future::select_ok;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tangle_subxt::subxt::{OnlineClient, PolkadotConfig};
use tangle_subxt::tangle_testnet_runtime::api;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services;
use tangle_subxt::tangle_testnet_runtime::api::services::calls::types::register::RegistrationArgs;

/// Protocol settings for [Tangle]
///
/// [Tangle]: https://tangle.tools
#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct TangleProtocolSettings {
    /// The blueprint ID for the Tangle blueprint
    pub blueprint_id: u64,
    /// The service ID for the Tangle blueprint instance
    ///
    /// Note: This will be `None` in case this blueprint is running in Registration Mode.
    pub service_id: Option<u64>,
}

impl ProtocolSettingsT for TangleProtocolSettings {
    fn load(settings: BlueprintSettings) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Self {
            blueprint_id: settings
                .blueprint_id
                .ok_or(ConfigError::MissingBlueprintId)?,
            service_id: Some(settings.service_id.ok_or(ConfigError::MissingServiceId)?),
        })
    }

    fn protocol_name(&self) -> &'static str {
        "tangle"
    }

    fn protocol(&self) -> Protocol {
        Protocol::Tangle
    }
}

#[derive(Clone, Debug, Default)]
pub struct TangleConfig {
    pub rpc_address: String,
    pub exit_after_register: bool,
}

impl TangleConfig {
    #[must_use]
    pub fn new(rpc_address: impl Into<String>) -> Self {
        Self {
            rpc_address: rpc_address.into(),
            exit_after_register: true,
        }
    }

    #[must_use]
    pub fn with_exit_after_register(mut self, should_exit_after_registration: bool) -> Self {
        self.exit_after_register = should_exit_after_registration;
        self
    }
}

impl BlueprintConfig for TangleConfig {
    async fn register(&self, env: &BlueprintEnvironment) -> Result<(), RunnerError> {
        register_impl(&self.rpc_address, vec![], env).await
    }

    async fn requires_registration(&self, env: &BlueprintEnvironment) -> Result<bool, RunnerError> {
        requires_registration_impl(env).await
    }

    fn should_exit_after_registration(&self) -> bool {
        self.exit_after_register
    }
}

async fn requires_registration_impl(env: &BlueprintEnvironment) -> Result<bool, RunnerError> {
    let settings = env.protocol_settings.tangle()?;

    // Check if the operator is already registered
    let client = get_client(env.ws_rpc_endpoint.as_str(), env.http_rpc_endpoint.as_str()).await?;

    // TODO: Improve key fetching logic
    let keystore = env.keystore();

    // TODO: Key IDs
    let sr25519_key = keystore
        .first_local::<SpSr25519>()
        .map_err(TangleError::from)?;
    let sr25519_pair = keystore.get_secret::<SpSr25519>(&sr25519_key)?;
    let signer: PairSigner<sp_core::sr25519::Pair> = PairSigner::new(sr25519_pair.0);

    let account_id = signer.account_id();

    let operator_profile_query = api::storage().services().operators_profile(account_id);
    let operator_profile = client
        .storage()
        .at_latest()
        .await
        .map_err(TangleError::Network)?
        .fetch(&operator_profile_query)
        .await
        .map_err(TangleError::Network)?;
    let is_registered =
        operator_profile.is_some_and(|p| p.blueprints.0.contains(&settings.blueprint_id));

    Ok(!is_registered)
}

async fn register_impl(
    rpc_address: &str,
    registration_args: RegistrationArgs,
    env: &BlueprintEnvironment,
) -> Result<(), RunnerError> {
    let settings = env.protocol_settings.tangle()?;

    let client = get_client(env.ws_rpc_endpoint.as_str(), env.http_rpc_endpoint.as_str()).await?;

    // TODO: Improve key fetching logic
    let keystore = env.keystore();

    // TODO: Key IDs
    let sr25519_key = keystore
        .first_local::<SpSr25519>()
        .map_err(TangleError::from)?;
    let sr25519_pair = keystore.get_secret::<SpSr25519>(&sr25519_key)?;
    let signer = PairSigner::new(sr25519_pair.0);

    let ecdsa_key = keystore
        .first_local::<SpEcdsa>()
        .map_err(TangleError::from)?;

    let account_id = signer.account_id();
    // Check if the operator is active operator.
    let operator_active_query = api::storage()
        .multi_asset_delegation()
        .operators(account_id);
    let operator_active = client
        .storage()
        .at_latest()
        .await
        .map_err(TangleError::Network)?
        .fetch(&operator_active_query)
        .await
        .map_err(TangleError::Network)?;
    if operator_active.is_none() {
        return Err(TangleError::NotActiveOperator.into());
    }

    let blueprint_id = settings.blueprint_id;

    let uncompressed_pk =
        decompress_pubkey(&ecdsa_key.0.0).ok_or(TangleError::DecompressEcdsaKey)?;
    let xt = api::tx().services().register(
        blueprint_id,
        services::types::OperatorPreferences {
            key: uncompressed_pk,
            rpc_address: new_bounded_string(rpc_address),
        },
        registration_args,
        0,
    );

    // send the tx to the tangle and exit.
    let result = blueprint_tangle_extra::util::send(client, Arc::new(signer), xt)
        .await
        .map_err(TangleError::Network)?;
    blueprint_core::info!("Registered operator with hash: {:?}", result);
    Ok(())
}

// TODO: Push this upstream: https://docs.rs/sp-core/latest/src/sp_core/ecdsa.rs.html#59-74
#[must_use]
pub fn decompress_pubkey(compressed: &[u8; 33]) -> Option<[u8; 65]> {
    // Uncompress the public key
    let pk = k256::PublicKey::from_sec1_bytes(compressed).ok()?;
    let uncompressed = pk.to_encoded_point(false);
    let uncompressed_bytes = uncompressed.as_bytes();

    // Ensure the key has the correct length
    if uncompressed_bytes.len() != 65 {
        return None;
    }

    let mut result = [0u8; 65];
    result.copy_from_slice(uncompressed_bytes);
    Some(result)
}

async fn get_client(
    ws_url: &str,
    http_url: &str,
) -> Result<Arc<OnlineClient<PolkadotConfig>>, TangleError> {
    let task0 = OnlineClient::<PolkadotConfig>::from_insecure_url(ws_url);
    let task1 = OnlineClient::<PolkadotConfig>::from_insecure_url(http_url);
    let client = select_ok([Box::pin(task0), Box::pin(task1)])
        .await
        .map_err(TangleError::Network)?
        .0;
    Ok(Arc::new(client))
}
