use blueprint_client_tangle::TangleClient;
use blueprint_client_tangle::resolve_tee_deployment_profile;
use blueprint_client_tangle::services::BlueprintInfo;
use color_eyre::Result;
use dialoguer::console::style;

#[derive(Debug, Clone)]
pub struct BlueprintListEntry {
    pub blueprint_id: u64,
    pub info: BlueprintInfo,
    pub tee_required: Option<bool>,
    pub tee_supported: Option<bool>,
}

/// Fetch all registered blueprints.
pub async fn list_blueprints(client: &TangleClient) -> Result<Vec<BlueprintListEntry>> {
    let blueprints = client
        .list_blueprints()
        .await
        .map_err(|e| color_eyre::eyre::eyre!(e.to_string()))?;

    let mut entries = Vec::with_capacity(blueprints.len());
    for (blueprint_id, info) in blueprints {
        let tee_profile = client
            .get_blueprint_definition(blueprint_id)
            .await
            .ok()
            .and_then(|definition| resolve_tee_deployment_profile(&definition.metadata));
        entries.push(BlueprintListEntry {
            blueprint_id,
            info,
            tee_required: tee_profile.map(|profile| profile.tee_required),
            tee_supported: tee_profile.map(|profile| profile.supports_tee),
        });
    }

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
        println!("{}: {}", style("Active").green(), info.active);
        println!(
            "{}",
            style("=============================================").dim()
        );
    }
}
