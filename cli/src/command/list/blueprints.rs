use blueprint_client_tangle::TangleClient;
use blueprint_client_tangle::services::BlueprintInfo;
use color_eyre::Result;
use dialoguer::console::style;

/// Fetch all registered blueprints.
pub async fn list_blueprints(client: &TangleClient) -> Result<Vec<(u64, BlueprintInfo)>> {
    client
        .list_blueprints()
        .await
        .map_err(|e| color_eyre::eyre::eyre!(e.to_string()))
}

/// Print blueprint details.
pub fn print_blueprints(blueprints: &[(u64, BlueprintInfo)]) {
    if blueprints.is_empty() {
        println!("{}", style("No blueprints registered").yellow());
        return;
    }

    println!("\n{}", style("Blueprints").cyan().bold());
    println!(
        "{}",
        style("=============================================").dim()
    );

    for (blueprint_id, info) in blueprints {
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
        println!("{}: {}", style("Active").green(), info.active);
        println!(
            "{}",
            style("=============================================").dim()
        );
    }
}
