use blueprint_client_tangle::TangleClient;
use blueprint_client_tangle::resolve_tee_deployment_profile;
use blueprint_client_tangle::services::BlueprintInfo;
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use dialoguer::console::style;
use futures::stream::{self, StreamExt};

const MAX_CONCURRENT_BLUEPRINT_FETCHES: usize = 16;

#[derive(Debug, Clone)]
pub struct BlueprintListEntry {
    /// On-chain blueprint identifier.
    pub blueprint_id: u64,
    /// Basic blueprint info from listing endpoint.
    pub info: BlueprintInfo,
    /// Whether TEE placement is mandatory.
    pub tee_required: Option<bool>,
    /// Whether blueprint supports TEE placement.
    pub tee_supported: Option<bool>,
    /// Metadata parsing/fetching issue, if any.
    pub tee_metadata_error: Option<String>,
}

/// Fetch all registered blueprints.
pub async fn list_blueprints(client: &TangleClient) -> Result<Vec<BlueprintListEntry>> {
    let blueprints = client
        .list_blueprints()
        .await
        .wrap_err("failed to list blueprints from tangle")?;

    let mut entries = stream::iter(blueprints)
        .map(|(blueprint_id, info)| async move {
            let (tee_required, tee_supported, tee_metadata_error) =
                match client.get_blueprint_definition(blueprint_id).await {
                    Ok(definition) => match resolve_tee_deployment_profile(&definition.metadata) {
                        Ok(profile) => (
                            profile.map(|value| value.tee_required),
                            profile.map(|value| value.supports_tee),
                            None,
                        ),
                        Err(err) => (None, None, Some(err.to_string())),
                    },
                    Err(err) => (
                        None,
                        None,
                        Some(format!("failed to fetch definition: {err}")),
                    ),
                };

            BlueprintListEntry {
                blueprint_id,
                info,
                tee_required,
                tee_supported,
                tee_metadata_error,
            }
        })
        .buffer_unordered(MAX_CONCURRENT_BLUEPRINT_FETCHES)
        .collect::<Vec<_>>()
        .await;

    entries.sort_by_key(|entry| entry.blueprint_id);

    Ok(entries)
}

/// Print blueprint details.
pub fn print_blueprints(blueprints: &[BlueprintListEntry]) {
    if blueprints.is_empty() {
        println!("{}", style("No blueprints registered").yellow());
        return;
    }

    println!("\n{}", style("Blueprints").cyan().bold());
    println!(
        "{}",
        style("=============================================").dim()
    );

    for entry in blueprints {
        let BlueprintListEntry {
            blueprint_id,
            info,
            tee_required,
            tee_supported,
            tee_metadata_error,
        } = entry;
        println!(
            "{}: {}",
            style("Blueprint ID").green().bold(),
            style(blueprint_id).green()
        );
        println!("{}: {}", style("Owner").green(), info.owner);
        println!("{}: {}", style("Manager").green(), info.manager);
        println!("{}: {}", style("Created At").green(), info.created_at);
        println!(
            "{}: {}",
            style("Operator Count").green(),
            info.operator_count
        );
        println!(
            "{}: {:?}",
            style("Membership Model").green(),
            info.membership
        );
        println!("{}: {:?}", style("Pricing Model").green(), info.pricing);
        let compatibility_label = match tee_supported {
            Some(true) => "yes",
            Some(false) => "no",
            None => "unspecified",
        };
        println!(
            "{}: {}",
            style("TEE Compatible").green(),
            compatibility_label
        );
        let policy_label = match tee_required {
            Some(true) => "required (fail closed)",
            Some(false) => "optional",
            None => "unspecified",
        };
        println!("{}: {}", style("TEE Policy").green(), policy_label);
        if let Some(err) = tee_metadata_error {
            println!("{}: {}", style("TEE Metadata Error").red(), err);
        }
        println!("{}: {}", style("Active").green(), info.active);
        println!(
            "{}",
            style("=============================================").dim()
        );
    }
}
