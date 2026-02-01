use blueprint_client_tangle::TangleClient;
use blueprint_client_tangle::services::ServiceInfo;
use color_eyre::Result;
use dialoguer::console::style;
use serde_json::json;

/// Fetch all services registered on-chain.
pub async fn list_services(client: &TangleClient) -> Result<Vec<(u64, ServiceInfo)>> {
    client
        .list_services()
        .await
        .map_err(|e| color_eyre::eyre::eyre!(e.to_string()))
}

/// Print service metadata.
pub fn print_services(services: &[(u64, ServiceInfo)], json_output: bool) {
    if services.is_empty() {
        println!("{}", style("No services found").yellow());
        return;
    }

    if json_output {
        let payload: Vec<_> = services
            .iter()
            .map(|(service_id, info)| {
                json!({
                    "service_id": service_id,
                    "blueprint_id": info.blueprint_id,
                    "owner": format!("{:#x}", info.owner),
                    "created_at": info.created_at,
                    "ttl": info.ttl,
                    "terminated_at": info.terminated_at,
                    "last_payment_at": info.last_payment_at,
                    "operator_count": info.operator_count,
                    "min_operators": info.min_operators,
                    "max_operators": info.max_operators,
                    "membership": format!("{:?}", info.membership),
                    "pricing": format!("{:?}", info.pricing),
                    "status": format!("{:?}", info.status),
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&payload).expect("serialize services to json")
        );
        return;
    }

    println!("\n{}", style("Services").cyan().bold());
    println!(
        "{}",
        style("=============================================").dim()
    );

    for (service_id, info) in services {
        println!(
            "{}: {}",
            style("Service ID").green().bold(),
            style(service_id).green()
        );
        println!("{}: {}", style("Blueprint ID").green(), info.blueprint_id);
        println!("{}: {}", style("Owner").green(), info.owner);
        println!("{}: {}", style("Created At").green(), info.created_at);
        println!("{}: {}", style("TTL").green(), info.ttl);
        println!(
            "{}: {} - {}",
            style("Operator Bounds").green(),
            info.min_operators,
            info.max_operators
        );
        println!("{}: {:?}", style("Membership").green(), info.membership);
        println!("{}: {:?}", style("Pricing").green(), info.pricing);
        println!("{}: {:?}", style("Status").green(), info.status);
        println!(
            "{}",
            style("=============================================").dim()
        );
    }
}
