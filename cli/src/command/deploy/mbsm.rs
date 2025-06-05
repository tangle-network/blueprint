use alloy_signer_local::PrivateKeySigner;
use blueprint_chain_setup::tangle::transactions;
use blueprint_clients::tangle::client::{TangleClient, TangleConfig};
use blueprint_contexts::tangle::TangleClientContext;
use blueprint_crypto::sp_core::{SpEcdsa, SpSr25519};
use blueprint_crypto::tangle_pair_signer::TanglePairSigner;
use blueprint_keystore::backends::Backend;
use blueprint_testing_utils::tangle::harness::{ENDOWED_TEST_NAMES, generate_env_from_node_id};
use dialoguer::console::style;
use tangle_subxt::subxt::tx::Signer;
use tempfile::TempDir;
use url::Url;

/// Deploy the MBSM contract to the chain
///
/// # Arguments
/// * `http_rpc_url` - The URL of the HTTP RPC server
/// * `force` - Whether to force the deployment
///
/// # Errors
///
/// Returns an error if the deployment fails
pub async fn deploy_mbsm(http_rpc_url: Url, force: bool) -> color_eyre::Result<()> {
    let temp_dir = TempDir::new()?;
    let mut ws_endpoint = http_rpc_url.clone();
    ws_endpoint.set_scheme("ws").map_err(|()| {
        color_eyre::Report::msg(format!(
            "Failed to set the scheme of the URL to 'ws': {}",
            ws_endpoint
        ))
    })?;

    let temp_path = temp_dir.path().to_path_buf();
    println!(
        "{}",
        style("Checking if the deployment can proceed...")
            .cyan()
            .bold()
    );

    // Set up Alice's environment for MBSM deployment
    let alice_env = generate_env_from_node_id(
        ENDOWED_TEST_NAMES[0],
        http_rpc_url.clone(),
        ws_endpoint.clone(),
        temp_path.as_path(),
    )
    .await?;

    let alice_keystore = alice_env.keystore();
    let alice_client = alice_env.tangle_client().await?;

    let alice_sr25519_public = alice_keystore.first_local::<SpSr25519>()?;
    let alice_sr25519_pair = alice_keystore.get_secret::<SpSr25519>(&alice_sr25519_public)?;
    let alice_sr25519_signer = TanglePairSigner::new(alice_sr25519_pair.0);

    let alice_ecdsa_public = alice_keystore.first_local::<SpEcdsa>()?;
    let alice_ecdsa_pair = alice_keystore.get_secret::<SpEcdsa>(&alice_ecdsa_public)?;
    let alice_ecdsa_signer = TanglePairSigner::new(alice_ecdsa_pair.0);
    let alice_alloy_key = alice_ecdsa_signer
        .alloy_key()
        .map_err(|e| color_eyre::Report::msg(format!("Failed to get Alice's Alloy key: {}", e)))?;

    // Check if MBSM is already deployed
    let latest_revision = transactions::get_latest_mbsm_revision(&alice_client)
        .await
        .map_err(|e| {
            color_eyre::Report::msg(format!("Failed to get latest MBSM revision: {}", e))
        })?;

    match latest_revision {
        Some((rev, addr)) if !force => {
            println!(
                "{}",
                style(format!(
                    "MBSM is already deployed at revision #{} at address {}",
                    rev, addr
                ))
                .green()
            );
            return Ok(());
        }
        Some((rev, addr)) => {
            println!(
                "{}",
                style(format!(
                    "MBSM is already deployed at revision #{} at address {}",
                    rev, addr
                ))
                .yellow()
            );
        }
        None => {
            println!("{}", style("MBSM is not deployed").yellow());
        }
    }

    println!(
        "{}",
        style("MBSM is not deployed, deploying now with Alice's account...").cyan()
    );

    let bytecode = tnt_core_bytecode::bytecode::MASTER_BLUEPRINT_SERVICE_MANAGER;
    transactions::deploy_new_mbsm_revision(
        ws_endpoint.as_str(),
        &alice_client,
        &alice_sr25519_signer,
        alice_alloy_key.clone(),
        bytecode,
        alloy_primitives::address!("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"),
    )
    .await
    .map_err(|e| color_eyre::Report::msg(format!("Failed to deploy MBSM: {}", e)))?;

    println!("{}", style("MBSM deployed successfully").green());
    Ok(())
}

/// Check if the MBSM is deployed, and if not, deploy it
///
/// # Errors
///
/// * Unable to check for existing MBSM revision
/// * Unable to deploy new MBSM revision
pub async fn deploy_mbsm_if_needed<T: Signer<TangleConfig>>(
    ws_endpoint: Url,
    client: &TangleClient,
    account: &T,
    evm_signer: PrivateKeySigner,
) -> color_eyre::Result<()> {
    // Check if MBSM is already deployed
    let latest_revision = transactions::get_latest_mbsm_revision(client)
        .await
        .map_err(|e| {
            color_eyre::Report::msg(format!("Failed to get latest MBSM revision: {}", e))
        })?;

    if let Some((rev, addr)) = latest_revision {
        println!(
            "{}",
            style(format!(
                "MBSM is already deployed at revision #{} at address {}",
                rev, addr
            ))
            .green()
        );

        return Ok(());
    }

    println!(
        "{}",
        style("MBSM is not deployed, deploying now with Alice's account...").cyan()
    );

    let bytecode = tnt_core_bytecode::bytecode::MASTER_BLUEPRINT_SERVICE_MANAGER;
    transactions::deploy_new_mbsm_revision(
        ws_endpoint.as_str(),
        client,
        account,
        evm_signer,
        bytecode,
        alloy_primitives::address!("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"),
    )
    .await
    .map_err(|e| color_eyre::Report::msg(format!("Failed to deploy MBSM: {}", e)))?;

    println!("{}", style("MBSM deployed successfully").green());

    Ok(())
}
