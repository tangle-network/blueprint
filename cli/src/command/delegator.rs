use alloy_primitives::{Address, U256};
use blueprint_client_tangle_evm::{
    AssetInfo, AssetKind, BlueprintSelectionMode, DelegationRecord, DepositInfo, LockInfo,
    PendingUnstake, PendingWithdrawal, RestakingMetadata,
};
use dialoguer::console::style;
use serde_json::json;

pub fn print_delegations(delegator: Address, delegations: &[DelegationRecord], json_output: bool) {
    if json_output {
        let items: Vec<_> = delegations
            .iter()
            .map(|record| {
                let asset = &record.info.asset;
                json!({
                    "operator": format!("{:#x}", record.info.operator),
                    "shares": record.info.shares.to_string(),
                    "asset_kind": asset.kind.to_string(),
                    "asset_token": format!("{:#x}", asset.token),
                    "selection_mode": record.info.selection_mode.to_string(),
                    "blueprint_ids": record.blueprint_ids,
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "delegator": format!("{:#x}", delegator),
                "delegations": items
            }))
            .expect("serialize delegations")
        );
        return;
    }

    println!("{}: {:#x}", style("Delegator").green(), delegator);
    if delegations.is_empty() {
        println!("{}", style("No delegations found").yellow());
        return;
    }

    for (idx, record) in delegations.iter().enumerate() {
        let asset = &record.info.asset;
        println!("{}", style(format!("Delegation #{idx}")).green().bold());
        println!(
            "  {}: {:#x}",
            style("Operator").green(),
            record.info.operator
        );
        println!("  {}: {}", style("Shares").green(), record.info.shares);
        println!(
            "  {}: {} ({:#x})",
            style("Asset").green(),
            format_asset_kind(asset),
            asset.token
        );
        println!(
            "  {}: {}",
            style("Selection").green(),
            record.info.selection_mode
        );
        if matches!(record.info.selection_mode, BlueprintSelectionMode::Fixed) {
            println!(
                "  {}: {:?}",
                style("Blueprint IDs").green(),
                record.blueprint_ids
            );
        }
    }
}

pub fn print_pending_unstakes(delegator: Address, unstakes: &[PendingUnstake], json_output: bool) {
    if json_output {
        let items: Vec<_> = unstakes
            .iter()
            .map(|request| {
                let asset = &request.asset;
                json!({
                    "operator": format!("{:#x}", request.operator),
                    "shares": request.shares.to_string(),
                    "asset_kind": asset.kind.to_string(),
                    "asset_token": format!("{:#x}", asset.token),
                    "selection_mode": request.selection_mode.to_string(),
                    "requested_round": request.requested_round,
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "delegator": format!("{:#x}", delegator),
                "pending_unstakes": items
            }))
            .expect("serialize pending unstakes")
        );
        return;
    }

    println!("{}: {:#x}", style("Delegator").green(), delegator);
    if unstakes.is_empty() {
        println!("{}", style("No pending unstakes").yellow());
        return;
    }

    for (idx, request) in unstakes.iter().enumerate() {
        let asset = &request.asset;
        println!(
            "{}",
            style(format!("Pending Unstake #{idx}")).green().bold()
        );
        println!("  {}: {:#x}", style("Operator").green(), request.operator);
        println!("  {}: {}", style("Shares").green(), request.shares);
        println!(
            "  {}: {} ({:#x})",
            style("Asset").green(),
            format_asset_kind(asset),
            asset.token
        );
        println!(
            "  {}: {}",
            style("Selection").green(),
            request.selection_mode
        );
        println!(
            "  {}: {}",
            style("Requested Round").green(),
            request.requested_round
        );
    }
}

pub fn print_pending_withdrawals(
    delegator: Address,
    withdrawals: &[PendingWithdrawal],
    json_output: bool,
) {
    if json_output {
        let items: Vec<_> = withdrawals
            .iter()
            .map(|request| {
                let asset = &request.asset;
                json!({
                    "asset_kind": asset.kind.to_string(),
                    "asset_token": format!("{:#x}", asset.token),
                    "amount": request.amount.to_string(),
                    "requested_round": request.requested_round,
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "delegator": format!("{:#x}", delegator),
                "pending_withdrawals": items
            }))
            .expect("serialize pending withdrawals")
        );
        return;
    }

    println!("{}: {:#x}", style("Delegator").green(), delegator);
    if withdrawals.is_empty() {
        println!("{}", style("No pending withdrawals").yellow());
        return;
    }

    for (idx, request) in withdrawals.iter().enumerate() {
        let asset = &request.asset;
        println!(
            "{}",
            style(format!("Pending Withdrawal #{idx}")).green().bold()
        );
        println!(
            "  {}: {} ({:#x})",
            style("Asset").green(),
            format_asset_kind(asset),
            asset.token
        );
        println!("  {}: {}", style("Amount").green(), request.amount);
        println!(
            "  {}: {}",
            style("Requested Round").green(),
            request.requested_round
        );
    }
}

pub fn print_positions(
    delegator: Address,
    token: Address,
    deposit: &DepositInfo,
    locks: &[LockInfo],
    delegations: &[DelegationRecord],
    unstakes: &[PendingUnstake],
    withdrawals: &[PendingWithdrawal],
    json_output: bool,
) {
    if json_output {
        let lock_items: Vec<_> = locks
            .iter()
            .map(|lock| {
                json!({
                    "amount": lock.amount.to_string(),
                    "multiplier": lock.multiplier.to_string(),
                    "expiry_block": lock.expiry_block,
                })
            })
            .collect();
        let delegation_items: Vec<_> = delegations
            .iter()
            .map(|record| {
                let asset = &record.info.asset;
                json!({
                    "operator": format!("{:#x}", record.info.operator),
                    "shares": record.info.shares.to_string(),
                    "asset_kind": asset.kind.to_string(),
                    "asset_token": format!("{:#x}", asset.token),
                    "selection_mode": record.info.selection_mode.to_string(),
                    "blueprint_ids": record.blueprint_ids,
                })
            })
            .collect();
        let unstake_items: Vec<_> = unstakes
            .iter()
            .map(|request| {
                let asset = &request.asset;
                json!({
                    "operator": format!("{:#x}", request.operator),
                    "shares": request.shares.to_string(),
                    "asset_kind": asset.kind.to_string(),
                    "asset_token": format!("{:#x}", asset.token),
                    "selection_mode": request.selection_mode.to_string(),
                    "requested_round": request.requested_round,
                })
            })
            .collect();
        let withdrawal_items: Vec<_> = withdrawals
            .iter()
            .map(|request| {
                let asset = &request.asset;
                json!({
                    "asset_kind": asset.kind.to_string(),
                    "asset_token": format!("{:#x}", asset.token),
                    "amount": request.amount.to_string(),
                    "requested_round": request.requested_round,
                })
            })
            .collect();

        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "delegator": format!("{:#x}", delegator),
                "token": format!("{:#x}", token),
                "deposit": {
                    "amount": deposit.amount.to_string(),
                    "delegated_amount": deposit.delegated_amount.to_string(),
                },
                "locks": lock_items,
                "delegations": delegation_items,
                "pending_unstakes": unstake_items,
                "pending_withdrawals": withdrawal_items,
            }))
            .expect("serialize positions")
        );
        return;
    }

    println!("{}: {:#x}", style("Delegator").green(), delegator);
    println!("{}: {:#x}", style("Token").green(), token);
    println!("{}: {}", style("Deposit").green(), format_deposit(deposit));

    if locks.is_empty() {
        println!("{}: none", style("Locks").green());
    } else {
        println!("{}:", style("Locks").green());
        for (idx, lock) in locks.iter().enumerate() {
            println!(
                "  #{idx} amount={} multiplier={} expiry_block={}",
                lock.amount, lock.multiplier, lock.expiry_block
            );
        }
    }

    if delegations.is_empty() {
        println!("{}: none", style("Delegations").green());
    } else {
        println!("{}:", style("Delegations").green());
        for (idx, record) in delegations.iter().enumerate() {
            let asset = &record.info.asset;
            println!(
                "  #{idx} operator={:#x} shares={} asset={} token={:#x} selection={} blueprints={:?}",
                record.info.operator,
                record.info.shares,
                format_asset_kind(asset),
                asset.token,
                record.info.selection_mode,
                record.blueprint_ids
            );
        }
    }

    if unstakes.is_empty() {
        println!("{}: none", style("Pending Unstakes").green());
    } else {
        println!("{}:", style("Pending Unstakes").green());
        for (idx, request) in unstakes.iter().enumerate() {
            let asset = &request.asset;
            println!(
                "  #{idx} operator={:#x} shares={} asset={} token={:#x} selection={} requested_round={}",
                request.operator,
                request.shares,
                format_asset_kind(asset),
                asset.token,
                request.selection_mode,
                request.requested_round
            );
        }
    }

    if withdrawals.is_empty() {
        println!("{}: none", style("Pending Withdrawals").green());
    } else {
        println!("{}:", style("Pending Withdrawals").green());
        for (idx, request) in withdrawals.iter().enumerate() {
            let asset = &request.asset;
            println!(
                "  #{idx} amount={} asset={} token={:#x} requested_round={}",
                request.amount,
                format_asset_kind(asset),
                asset.token,
                request.requested_round
            );
        }
    }
}

pub fn print_operator_restaking(
    operator: Address,
    restaking: &RestakingMetadata,
    self_stake: U256,
    delegated_stake: U256,
    commission_bps: u16,
    current_round: u64,
    json_output: bool,
) {
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "operator": format!("{:#x}", operator),
                "stake": restaking.stake.to_string(),
                "self_stake": self_stake.to_string(),
                "delegated_stake": delegated_stake.to_string(),
                "delegation_count": restaking.delegation_count,
                "status": format!("{:?}", restaking.status),
                "leaving_round": restaking.leaving_round,
                "commission_bps": commission_bps,
                "current_round": current_round,
            }))
            .expect("serialize operator restaking")
        );
        return;
    }

    println!("{}: {:#x}", style("Operator").green(), operator);
    println!("{}: {}", style("Status").green(), format_status(restaking));
    println!("{}: {}", style("Stake").green(), restaking.stake);
    println!("{}: {}", style("Self Stake").green(), self_stake);
    println!("{}: {}", style("Delegated Stake").green(), delegated_stake);
    println!(
        "{}: {}",
        style("Delegation Count").green(),
        restaking.delegation_count
    );
    println!(
        "{}: {}",
        style("Leaving Round").green(),
        restaking.leaving_round
    );
    println!("{}: {}", style("Commission BPS").green(), commission_bps);
    println!("{}: {}", style("Current Round").green(), current_round);
}

pub fn print_operator_delegators(operator: Address, delegators: &[Address], json_output: bool) {
    if json_output {
        let items: Vec<_> = delegators
            .iter()
            .map(|address| format!("{:#x}", address))
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "operator": format!("{:#x}", operator),
                "delegators": items
            }))
            .expect("serialize operator delegators")
        );
        return;
    }

    println!("{}: {:#x}", style("Operator").green(), operator);
    if delegators.is_empty() {
        println!("{}", style("No delegators found").yellow());
        return;
    }

    for (idx, delegator) in delegators.iter().enumerate() {
        println!("  #{idx} {:#x}", delegator);
    }
}

pub fn print_erc20_allowance(
    owner: Address,
    spender: Address,
    token: Address,
    allowance: U256,
    json_output: bool,
) {
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "owner": format!("{:#x}", owner),
                "spender": format!("{:#x}", spender),
                "token": format!("{:#x}", token),
                "allowance": allowance.to_string(),
            }))
            .expect("serialize erc20 allowance")
        );
        return;
    }

    println!("{}: {:#x}", style("Owner").green(), owner);
    println!("{}: {:#x}", style("Spender").green(), spender);
    println!("{}: {:#x}", style("Token").green(), token);
    println!("{}: {}", style("Allowance").green(), allowance);
}

pub fn print_erc20_balance(owner: Address, token: Address, balance: U256, json_output: bool) {
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "owner": format!("{:#x}", owner),
                "token": format!("{:#x}", token),
                "balance": balance.to_string(),
            }))
            .expect("serialize erc20 balance")
        );
        return;
    }

    println!("{}: {:#x}", style("Owner").green(), owner);
    println!("{}: {:#x}", style("Token").green(), token);
    println!("{}: {}", style("Balance").green(), balance);
}

fn format_asset_kind(asset: &AssetInfo) -> String {
    match asset.kind {
        AssetKind::Native => "native".to_string(),
        AssetKind::Erc20 => "erc20".to_string(),
        AssetKind::Unknown(value) => format!("unknown({value})"),
    }
}

fn format_deposit(deposit: &DepositInfo) -> String {
    format!(
        "amount={} delegated={}",
        deposit.amount, deposit.delegated_amount
    )
}

fn format_status(restaking: &RestakingMetadata) -> String {
    format!("{:?}", restaking.status)
}
