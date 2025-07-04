use blueprint_clients::tangle::client::OnlineClient;
use blueprint_core::{debug, info};
use blueprint_crypto::sp_core::SpSr25519;
use blueprint_crypto::tangle_pair_signer::TanglePairSigner;
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use blueprint_runner::tangle::config::decompress_pubkey;
use blueprint_tangle_extra::serde::new_bounded_string;
use color_eyre::Result;
use dialoguer::console::style;
use tangle_subxt::subxt;
use tangle_subxt::subxt::error::DispatchError;
use tangle_subxt::subxt::tx::Signer;
use tangle_subxt::tangle_testnet_runtime::api;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::pallet_multi_asset_delegation as mad;

/// Registers a blueprint.
///
/// # Arguments
///
/// * `ws_rpc_url` - WebSocket RPC URL for the Tangle Network
/// * `blueprint_id` - ID of the blueprint to register
/// * `keystore_uri` - URI for the keystore
///
/// # Errors
///
/// Returns an error if:
/// * Failed to connect to the Tangle Network
/// * Failed to sign or submit the transaction
/// * Transaction failed
/// * Missing ECDSA key
///
/// # Panics
///
/// Panics if:
/// * Failed to create keystore
/// * Failed to get keys from keystore
pub async fn register(
    ws_rpc_url: impl AsRef<str>,
    blueprint_id: u64,
    keystore_uri: String,
    rpc_address: impl AsRef<str>,
    // keystore_password: Option<String>, // TODO: Add keystore password support
) -> Result<()> {
    let client = OnlineClient::from_url(ws_rpc_url.as_ref()).await?;

    let config = KeystoreConfig::new().fs_root(keystore_uri.clone());
    let keystore = Keystore::new(config).expect("Failed to create keystore");
    let public = keystore
        .first_local::<SpSr25519>()
        .map_err(|e| color_eyre::eyre::eyre!("Failed to get public key: {}", e))?;
    let pair = keystore
        .get_secret::<SpSr25519>(&public)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to get secret key: {}", e))?;
    let signer = TanglePairSigner::new(pair.0);

    // Get the account ID from the signer for display
    let account_id = signer.account_id();
    println!(
        "{}",
        style(format!(
            "Starting registration process for Operator ID: {}",
            account_id
        ))
        .cyan()
    );

    let ecdsa_public = keystore
        .first_local::<blueprint_crypto::sp_core::SpEcdsa>()
        .map_err(|e| color_eyre::eyre::eyre!("Missing ECDSA key: {}", e))?;

    let preferences =
        tangle_subxt::tangle_testnet_runtime::api::services::calls::types::register::Preferences {
            key: decompress_pubkey(&ecdsa_public.0.0).unwrap(),
            rpc_address: new_bounded_string(rpc_address.as_ref()),
        };

    info!("Joining operators...");
    let join_call = api::tx()
        .multi_asset_delegation()
        .join_operators(1_000_000_000_000_000);
    let join_res = client
        .tx()
        .sign_and_submit_then_watch_default(&join_call, &signer)
        .await?;

    // Wait for finalization instead of just in-block
    match join_res.wait_for_finalized_success().await {
        Ok(events) => {
            info!("Successfully joined operators with events: {:?}", events);
        }
        Err(e) => {
            match e {
                subxt::Error::Runtime(DispatchError::Module(module))
                    if module.as_root_error::<api::Error>().is_ok_and(|e| {
                        matches!(
                            e,
                            api::Error::MultiAssetDelegation(mad::pallet::Error::AlreadyOperator)
                        )
                    }) =>
                {
                    println!(
                        "{}",
                        style("Account is already an operator, skipping join step...").yellow()
                    );
                    info!(
                        "Account {} is already an operator, continuing with registration",
                        account_id
                    );
                }

                _ => {
                    // Re-throw any other error
                    return Err(e.into());
                }
            }
        }
    }

    println!(
        "{}",
        style(format!(
            "PreRegistering for blueprint with ID: {}...",
            blueprint_id
        ))
        .cyan()
    );

    let preregister_call = api::tx().services().pre_register(blueprint_id);
    let preregister_res = client
        .tx()
        .sign_and_submit_then_watch_default(&preregister_call, &signer)
        .await?;

    // Wait for finalization instead of just in-block
    let events = preregister_res.wait_for_finalized_success().await?;
    info!(
        "Successfully preregistered for blueprint with ID: {} with events: {:?}",
        blueprint_id, events
    );

    println!(
        "{}",
        style(format!("Registering for blueprint ID: {}...", blueprint_id)).cyan()
    );
    let registration_args = tangle_subxt::tangle_testnet_runtime::api::services::calls::types::register::RegistrationArgs::new();
    let register_call =
        api::tx()
            .services()
            .register(blueprint_id, preferences, registration_args, 0);
    let register_res = client
        .tx()
        .sign_and_submit_then_watch_default(&register_call, &signer)
        .await?;

    // Wait for finalization instead of just in-block
    let events = register_res.wait_for_finalized_success().await?;
    info!(
        "Successfully registered for blueprint with ID: {} with events: {:?}",
        blueprint_id, events
    );

    // Verify registration by querying the latest block
    println!("{}", style("Verifying registration...").cyan());
    let latest_block = client.blocks().at_latest().await?;
    let latest_block_hash = latest_block.hash();
    debug!("Latest block: {:?}", latest_block.number());

    // Create a TangleServicesClient to query operator blueprints
    let services_client =
        blueprint_clients::tangle::services::TangleServicesClient::new(client.clone());

    debug!("Querying blueprints for account: {:?}", account_id);

    // Query operator blueprints at the latest block
    let block_hash = latest_block_hash.0;
    let blueprints = services_client
        .query_operator_blueprints(block_hash, account_id.clone())
        .await?;

    info!("Found {} blueprints for operator", blueprints.len());
    for (i, blueprint) in blueprints.iter().enumerate() {
        info!("Blueprint {}: {:?}", i, blueprint);
    }

    println!("{}", style("Registration process completed").green());
    Ok(())
}
