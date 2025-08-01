use color_eyre::Result;
use dialoguer::console::style;
use blueprint_clients::tangle::client::OnlineClient;
use blueprint_crypto::sp_core::SpSr25519;
use blueprint_crypto::tangle_pair_signer::TanglePairSigner;
use blueprint_keystore::{Keystore, KeystoreConfig};
use tangle_subxt::tangle_testnet_runtime::api;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::sp_arithmetic::per_things::Percent;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::types::{
    Asset, AssetSecurityRequirement, MembershipModel,
};
use blueprint_core::info;
use crate::command::jobs::helpers::{load_job_args_from_file, prompt_for_job_params};
use crate::wait_for_in_block_success;
use tangle_subxt::subxt::utils::AccountId32;
use blueprint_keystore::backends::Backend;

/// Requests a service from the Tangle Network.
///
/// # Arguments
///
/// * `ws_rpc_url` - WebSocket RPC URL for the Tangle Network
/// * `blueprint_id` - ID of the blueprint to request
/// * `min_exposure_percent` - Minimum exposure percentage
/// * `max_exposure_percent` - Maximum exposure percentage
/// * `target_operators` - List of target operators
/// * `value` - Value to stake
/// * `keystore_uri` - URI for the keystore
///
/// # Errors
///
/// Returns an error if:
/// * Failed to connect to the Tangle Network
/// * Failed to sign or submit the transaction
/// * Transaction failed
///
/// # Panics
///
/// Panics if:
/// * Failed to create keystore
/// * Failed to get keys from keystore
#[allow(clippy::too_many_arguments)]
pub async fn request_service(
    ws_rpc_url: impl AsRef<str>,
    blueprint_id: u64,
    min_exposure_percent: u8,
    max_exposure_percent: u8,
    target_operators: Vec<AccountId32>,
    value: u128,
    keystore_uri: String,
    params_file: Option<String>,
    // keystore_password: Option<String>, // TODO: Add keystore password support
) -> Result<()> {
    let client = OnlineClient::from_url(ws_rpc_url.as_ref()).await?;

    let config = KeystoreConfig::new().fs_root(keystore_uri.clone());
    let keystore = Keystore::new(config).expect("Failed to create keystore");
    let public = keystore.first_local::<SpSr25519>().unwrap();
    let pair = keystore.get_secret::<SpSr25519>(&public).unwrap();
    let signer = TanglePairSigner::new(pair.0);

    let blueprint_key = api::storage().services().blueprints(blueprint_id);
    let maybe_blueprint = client
        .storage()
        .at_latest()
        .await?
        .fetch(&blueprint_key)
        .await?;
    let Some((_blueprint_owner, blueprint)) = maybe_blueprint else {
        return Err(color_eyre::eyre::eyre!(
            "Blueprint ID {} not found",
            blueprint_id
        ));
    };

    let min_operators = u32::try_from(target_operators.len())
        .map_err(|_| color_eyre::eyre::eyre!("Too many operators"))?;
    let security_requirements = vec![AssetSecurityRequirement {
        asset: Asset::Custom(0),
        min_exposure_percent: Percent(min_exposure_percent),
        max_exposure_percent: Percent(max_exposure_percent),
    }];

    println!(
        "{}",
        style(format!(
            "Preparing service request for blueprint ID: {}",
            blueprint_id
        ))
        .cyan()
    );
    println!(
        "{}",
        style(format!(
            "Target operators: {} (min: {})",
            target_operators.len(),
            min_operators
        ))
        .dim()
    );
    println!(
        "{}",
        style(format!(
            "Exposure range: {}% - {}%",
            min_exposure_percent, max_exposure_percent
        ))
        .dim()
    );

    // Get request arguments either from file or prompt
    let request_args = if blueprint.request_params.0.is_empty() {
        // No request parameters, use default values
        Vec::new()
    } else if let Some(file_path) = params_file {
        info!(
            "Request params definition: {:?}",
            blueprint.request_params.0
        );

        // Load arguments from file based on job definition
        load_job_args_from_file(&file_path, &blueprint.request_params.0)?
    } else {
        info!(
            "Request params definition: {:?}",
            blueprint.request_params.0
        );
        // Prompt for each parameter based on its type
        prompt_for_job_params(&blueprint.request_params.0)?
    };

    let call = api::tx().services().request(
        None,
        blueprint_id,
        Vec::new(),
        target_operators,
        request_args,
        security_requirements,
        1000,
        Asset::Custom(0),
        value,
        MembershipModel::Fixed { min_operators },
    );

    println!("{}", style("Submitting Service Request...").cyan());
    let res = client
        .tx()
        .sign_and_submit_then_watch_default(&call, &signer)
        .await?;
    let events = wait_for_in_block_success(res).await;

    let service_request_event = events
        .find_first::<api::services::events::ServiceRequested>()
        .map_err(|e| color_eyre::eyre::eyre!("Service request failed: {e}"))?
        .ok_or_else(|| color_eyre::eyre::eyre!("Service request event not found"))?;

    println!(
        "{}",
        style("Service Request submitted successfully").green()
    );

    println!(
        "{}",
        style(format!(
            "Service Request ID: {}",
            service_request_event.request_id
        ))
        .green()
        .bold()
    );

    Ok(())
}
