use std::time::{SystemTime, UNIX_EPOCH};

use alloy_network::EthereumWallet;
use alloy_primitives::{Address, keccak256};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types_eth::TransactionRequest;
use alloy_sol_types::SolCall;
use blueprint_client_tangle::{IOperatorStatusRegistry, OperatorStatusSnapshot};
use blueprint_crypto::k256::K256SigningKey;
use color_eyre::eyre::{Result, eyre};
use dialoguer::console::style;
use serde_json::json;

use IOperatorStatusRegistry::submitHeartbeatCall;

const ETH_MESSAGE_PREFIX: &[u8] = b"\x19Ethereum Signed Message:\n32";

/// Heartbeat status payload
#[derive(Clone, Debug)]
pub struct HeartbeatPayload {
    pub block_number: u64,
    pub timestamp: u64,
    pub service_id: u64,
    pub blueprint_id: u64,
    pub status_code: u32,
}

impl HeartbeatPayload {
    /// Encode the payload to bytes (simple big-endian serialization)
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(32);
        bytes.extend_from_slice(&self.block_number.to_be_bytes());
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());
        bytes.extend_from_slice(&self.service_id.to_be_bytes());
        bytes.extend_from_slice(&self.blueprint_id.to_be_bytes());
        bytes.extend_from_slice(&self.status_code.to_be_bytes());
        bytes
    }
}

/// Print operator status in human or JSON form.
pub fn print_status(status: &OperatorStatusSnapshot, json_output: bool) {
    if json_output {
        let payload = json!({
            "service_id": status.service_id,
            "operator": format!("{:#x}", status.operator),
            "status_code": status.status_code,
            "last_heartbeat": status.last_heartbeat,
            "online": status.online,
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&payload).expect("serialize operator status to json")
        );
        return;
    }

    println!(
        "{}: {}",
        style("Service ID").green().bold(),
        style(status.service_id).green()
    );
    println!("{}: {:#x}", style("Operator").green(), status.operator);
    println!("{}: {}", style("Status Code").green(), status.status_code);
    if status.last_heartbeat == 0 {
        println!("{}: {}", style("Last Heartbeat").green(), "never");
    } else {
        println!(
            "{}: {}",
            style("Last Heartbeat").green(),
            status.last_heartbeat
        );
    }
    println!("{}: {}", style("Online").green(), status.online);
}

/// Submit a heartbeat to the OperatorStatusRegistry contract.
pub async fn submit_heartbeat(
    http_rpc_endpoint: &str,
    status_registry_address: Address,
    signing_key: &mut K256SigningKey,
    service_id: u64,
    blueprint_id: u64,
    status_code: u8,
    json_output: bool,
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| eyre!("System time error: {e}"))?
        .as_secs();

    let payload = HeartbeatPayload {
        block_number: 0,
        timestamp,
        service_id,
        blueprint_id,
        status_code: u32::from(status_code),
    };

    let metrics_bytes = payload.encode();
    let signature = sign_heartbeat_payload(signing_key, service_id, blueprint_id, &metrics_bytes)?;

    let local_signer = signing_key
        .alloy_key()
        .map_err(|e| eyre!("Failed to prepare wallet signer: {e}"))?;
    let wallet = EthereumWallet::from(local_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(http_rpc_endpoint)
        .await
        .map_err(|e| eyre!("Failed to connect to RPC endpoint: {e}"))?;

    let heartbeat_call = submitHeartbeatCall {
        serviceId: service_id,
        blueprintId: blueprint_id,
        statusCode: status_code,
        metrics: metrics_bytes.into(),
        signature: signature.into(),
    };

    let calldata = heartbeat_call.abi_encode();

    let tx_request = TransactionRequest::default()
        .to(status_registry_address)
        .input(calldata.into());

    let pending_tx = provider
        .send_transaction(tx_request)
        .await
        .map_err(|e| eyre!("Failed to submit heartbeat transaction: {e}"))?;

    let receipt = pending_tx
        .get_receipt()
        .await
        .map_err(|e| eyre!("Failed to finalize heartbeat transaction: {e}"))?;

    if json_output {
        let output = json!({
            "service_id": service_id,
            "blueprint_id": blueprint_id,
            "status_code": status_code,
            "timestamp": timestamp,
            "tx_hash": format!("{:#x}", receipt.transaction_hash),
            "success": receipt.status(),
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        if receipt.status() {
            println!(
                "{} Heartbeat submitted successfully",
                style("✓").green().bold()
            );
            println!("  Transaction: {:#x}", receipt.transaction_hash);
            println!("  Service ID: {}", service_id);
            println!("  Blueprint ID: {}", blueprint_id);
            println!("  Status Code: {}", status_code);
            println!("  Timestamp: {}", timestamp);
        } else {
            println!("{} Heartbeat transaction reverted", style("✗").red().bold());
            println!("  Transaction: {:#x}", receipt.transaction_hash);
        }
    }

    Ok(())
}

fn sign_heartbeat_payload(
    signing_key: &mut K256SigningKey,
    service_id: u64,
    blueprint_id: u64,
    metrics: &[u8],
) -> Result<Vec<u8>> {
    let mut payload = Vec::with_capacity(16 + metrics.len());
    payload.extend_from_slice(&service_id.to_be_bytes());
    payload.extend_from_slice(&blueprint_id.to_be_bytes());
    payload.extend_from_slice(metrics);

    let message_hash = keccak256(&payload);

    let mut prefixed = Vec::with_capacity(ETH_MESSAGE_PREFIX.len() + message_hash.len());
    prefixed.extend_from_slice(ETH_MESSAGE_PREFIX);
    prefixed.extend_from_slice(message_hash.as_slice());

    let prefixed_hash = keccak256(&prefixed);
    let mut digest = [0u8; 32];
    digest.copy_from_slice(prefixed_hash.as_slice());

    let (signature, recovery_id) = signing_key
        .0
        .sign_prehash_recoverable(&digest)
        .map_err(|e| eyre!("Failed to sign heartbeat payload: {e}"))?;

    let mut signature_bytes = Vec::with_capacity(65);
    signature_bytes.extend_from_slice(&signature.to_bytes());
    signature_bytes.push(recovery_id.to_byte() + 27);
    Ok(signature_bytes)
}
