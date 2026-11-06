use alloy_primitives::Address;
use blueprint_client_tangle::TransactionResult;
use color_eyre::eyre::Result;
use serde_json::json;
use std::path::PathBuf;

/// Build a `TxContext` from the shared `TangleClientArgs` resolver.
pub(super) fn upgrade_tx_ctx(
    network: &cargo_tangle::command::tangle::TangleClientArgs,
) -> Result<cargo_tangle::command::upgrade::TxContext> {
    let cfg = network.client_config(0, None)?;
    Ok(cargo_tangle::command::upgrade::TxContext {
        http_rpc_url: cfg.http_rpc_endpoint.clone(),
        tangle_contract: cfg.settings.tangle_contract,
        keystore_path: PathBuf::from(cfg.keystore_uri.clone()),
    })
}

pub(super) fn upgrade_view_ctx(
    network: &cargo_tangle::command::tangle::TangleClientArgs,
    auditors_contract: Option<Address>,
) -> Result<cargo_tangle::command::upgrade::ViewContext> {
    let cfg = network.client_config(0, None)?;
    Ok(cargo_tangle::command::upgrade::ViewContext {
        http_rpc_url: cfg.http_rpc_endpoint.clone(),
        tangle_contract: cfg.settings.tangle_contract,
        auditors_contract,
    })
}

pub(super) fn log_tx(prefix: &str, tx: &TransactionResult, json: bool) {
    if json {
        println!(
            "{}",
            json!({
                "event": "tx_submitted",
                "action": prefix,
                "tx_hash": format!("{:#x}", tx.tx_hash),
            })
        );
        println!(
            "{}",
            json!({
                "event": "tx_confirmed",
                "action": prefix,
                "tx_hash": format!("{:#x}", tx.tx_hash),
                "block": tx.block_number,
                "gas_used": tx.gas_used,
                "success": tx.success,
            })
        );
        return;
    }

    println!("{prefix}: submitted tx_hash={:#x}", tx.tx_hash);
    if tx.success {
        println!(
            "{prefix}: confirmed block={:?} gas_used={}",
            tx.block_number, tx.gas_used
        );
    } else {
        println!(
            "{prefix} failed: tx_hash={:#x} block={:?}",
            tx.tx_hash, tx.block_number
        );
    }
}
