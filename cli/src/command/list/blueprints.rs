use blueprint_client_tangle::TangleClient;
use blueprint_client_tangle::services::BlueprintInfo;
use blueprint_client_tangle::{ConfidentialityPolicy, resolve_execution_profile};
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
    /// Optional confidentiality execution policy declared by the blueprint.
    pub confidentiality_policy: Option<ConfidentialityPolicy>,
    /// Metadata parsing/fetching issue, if any.
    pub execution_metadata_error: Option<String>,
}

/// Fetch all registered blueprints.
pub async fn list_blueprints(client: &TangleClient) -> Result<Vec<BlueprintListEntry>> {
    let blueprints = client
        .list_blueprints()
        .await
        .wrap_err("failed to list blueprints from tangle")?;

    let mut entries = stream::iter(blueprints)
        .map(|(blueprint_id, info)| async move {
            let (confidentiality_policy, execution_metadata_error) =
                match client.get_blueprint_definition(blueprint_id).await {
                    Ok(definition) => match resolve_execution_profile(&definition.metadata) {
                        Ok(profile) => (profile.map(|value| value.confidentiality), None),
                        Err(err) => (None, Some(err.to_string())),
                    },
                    Err(err) => (None, Some(format!("failed to fetch definition: {err}"))),
                };

            BlueprintListEntry {
                blueprint_id,
                info,
                confidentiality_policy,
                execution_metadata_error,
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
            confidentiality_policy,
            execution_metadata_error,
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
        let policy_label = match confidentiality_policy {
            Some(ConfidentialityPolicy::Any) => "any",
            Some(ConfidentialityPolicy::TeeRequired) => "tee_required",
            Some(ConfidentialityPolicy::StandardRequired) => "standard_required",
            Some(ConfidentialityPolicy::TeePreferred) => "tee_preferred",
            None => "unspecified",
        };
        println!(
            "{}: {}",
            style("Execution Confidentiality").green(),
            policy_label
        );
        if let Some(err) = execution_metadata_error {
            println!("{}: {}", style("Execution Metadata Error").red(), err);
        }
        println!("{}: {}", style("Active").green(), info.active);
        println!(
            "{}",
            style("=============================================").dim()
        );
    }
}
