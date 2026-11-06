use super::{
    blueprint::BlueprintCommands,
    chain::{upgrade_tx_ctx, upgrade_view_ctx},
};
use cargo_tangle::command::tangle::parse_address;
use color_eyre::eyre::{Result, bail, eyre};

pub(super) async fn run(command: BlueprintCommands) -> Result<()> {
    match command {
        BlueprintCommands::Ship {
            network,
            yes,
            no_build,
            package,
            binary,
            binary_uri,
            pin_ipfs,
            attestation_bundle,
            attestation_hash,
            promote,
            no_promote,
            policy_services,
            dry_run,
            blueprint_id,
            json,
        } => {
            let json_out = json || yes;
            let args = cargo_tangle::command::ship::ShipArgs {
                network,
                yes,
                no_build,
                package,
                binary,
                binary_uri,
                pin_ipfs,
                attestation_bundle,
                attestation_hash,
                promote,
                no_promote,
                policy_services,
                dry_run,
                blueprint_id,
                json: json_out,
            };
            cargo_tangle::command::ship::run(args).await?;
        }
        BlueprintCommands::PublishVersion {
            network,
            blueprint_id,
            binary,
            binary_uri,
            pin_to_ipfs,
            attestation_bundle,
            attestation_hash,
            json,
        } => {
            let ctx = upgrade_tx_ctx(&network)?;
            let (sha256_hash, _len) = cargo_tangle::command::upgrade::hash_file(&binary)?;
            let resolved_uri = if let Some(uri) = binary_uri {
                uri
            } else if pin_to_ipfs {
                let pinned = cargo_tangle::command::upgrade::pin_file_to_ipfs(&binary).await?;
                pinned.uri
            } else {
                bail!(
                    "must supply --binary-uri or --pin-to-ipfs so the contract has a binaryUri to publish"
                );
            };
            let resolved_attestation = if let Some(hex) = attestation_hash {
                cargo_tangle::command::upgrade::parse_b256(&hex, "--attestation-hash")?
            } else if let Some(bundle_path) = attestation_bundle {
                cargo_tangle::command::upgrade::hash_file(&bundle_path)?.0
            } else {
                alloy_primitives::B256::ZERO
            };
            let result = cargo_tangle::command::upgrade::publish_version(
                &ctx,
                blueprint_id,
                sha256_hash,
                resolved_uri,
                resolved_attestation,
            )
            .await?;
            cargo_tangle::command::upgrade::print_publish_result(&result, json);
        }
        BlueprintCommands::SetActiveVersion {
            network,
            blueprint_id,
            version_id,
            json,
        } => {
            let ctx = upgrade_tx_ctx(&network)?;
            let tx =
                cargo_tangle::command::upgrade::set_active_version(&ctx, blueprint_id, version_id)
                    .await?;
            cargo_tangle::command::upgrade::print_simple_tx(
                &format!("set-active-version(blueprint={blueprint_id}, version={version_id})"),
                &tx,
                json,
            );
        }
        BlueprintCommands::DeprecateVersion {
            network,
            blueprint_id,
            version_id,
            json,
        } => {
            let ctx = upgrade_tx_ctx(&network)?;
            let tx =
                cargo_tangle::command::upgrade::deprecate_version(&ctx, blueprint_id, version_id)
                    .await?;
            cargo_tangle::command::upgrade::print_simple_tx(
                &format!("deprecate-version(blueprint={blueprint_id}, version={version_id})"),
                &tx,
                json,
            );
        }
        BlueprintCommands::ListVersions {
            network,
            blueprint_id,
            json,
        } => {
            let view = upgrade_view_ctx(&network, None)?;
            let versions =
                cargo_tangle::command::upgrade::list_versions(&view, blueprint_id).await?;
            let active =
                cargo_tangle::command::upgrade::get_active_version_id(&view, blueprint_id).await?;
            cargo_tangle::command::upgrade::print_versions_table(
                blueprint_id,
                &versions,
                active,
                json,
            );
        }
        BlueprintCommands::ShowVersion {
            network,
            blueprint_id,
            version_id,
            json,
        } => {
            let view = upgrade_view_ctx(&network, None)?;
            let v = cargo_tangle::command::upgrade::get_version(&view, blueprint_id, version_id)
                .await?;
            cargo_tangle::command::upgrade::print_version_detail(blueprint_id, &v, json);
        }
        BlueprintCommands::TrustScore {
            network,
            blueprint_id,
            version_id,
            auditors_contract,
            min_score,
            json,
        } => {
            let auditors = match auditors_contract {
                Some(s) => Some(parse_address(&s, "AUDITORS_CONTRACT")?),
                None => None,
            };
            let view = upgrade_view_ctx(&network, auditors)?;
            let score = cargo_tangle::command::upgrade::compute_trust_score(
                &view,
                blueprint_id,
                version_id,
            )
            .await?;
            cargo_tangle::command::upgrade::print_trust_score(&score, json);
            if let Some(min) = min_score {
                if score.score < min {
                    return Err(eyre!(
                        "trust score {} < required {} for blueprint {} version {}",
                        score.score,
                        min,
                        blueprint_id,
                        version_id
                    ));
                }
            }
        }
        _ => unreachable!("command dispatched by its owning module"),
    }
    Ok(())
}
