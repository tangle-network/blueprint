use blueprint_client_tangle_evm::TangleEvmClient;
use blueprint_client_tangle_evm::services::ServiceRequestInfo;
use color_eyre::Result;
use dialoguer::console::style;
use serde_json::json;

/// Fetch all service requests currently recorded on-chain.
pub async fn list_requests(client: &TangleEvmClient) -> Result<Vec<ServiceRequestInfo>> {
    let requests = client
        .list_service_requests()
        .await
        .map_err(|e| color_eyre::eyre::eyre!(e.to_string()))?;
    Ok(requests)
}

/// Pretty-print service request information.
pub fn print_requests(requests: &[ServiceRequestInfo], json_output: bool) {
    if requests.is_empty() {
        println!("{}", style("No service requests found").yellow());
        return;
    }

    if json_output {
        let payload: Vec<_> = requests
            .iter()
            .map(|request| {
                json!({
                    "request_id": request.request_id,
                    "blueprint_id": request.blueprint_id,
                    "requester": format!("{:#x}", request.requester),
                    "created_at": request.created_at,
                    "ttl": request.ttl,
                    "operator_count": request.operator_count,
                    "approval_count": request.approval_count,
                    "payment_token": format!("{:#x}", request.payment_token),
                    "payment_amount": request.payment_amount.to_string(),
                    "membership": format!("{:?}", request.membership),
                    "min_operators": request.min_operators,
                    "max_operators": request.max_operators,
                    "rejected": request.rejected,
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&payload).expect("serialize requests to json")
        );
        return;
    }

    println!("\n{}", style("Service Requests").cyan().bold());
    println!(
        "{}",
        style("=============================================").dim()
    );

    for request in requests {
        print_request(request);
        println!(
            "{}",
            style("=============================================").dim()
        );
    }
}

/// Pretty-print a single service request.
pub fn print_request(request: &ServiceRequestInfo) {
    println!(
        "{}: {}",
        style("Request ID").green().bold(),
        style(request.request_id).green()
    );
    println!(
        "{}: {}",
        style("Blueprint ID").green(),
        request.blueprint_id
    );
    println!("{}: {}", style("Requester").green(), request.requester);
    println!("{}: {}", style("Created At").green(), request.created_at);
    println!("{}: {}", style("TTL").green(), request.ttl);
    println!(
        "{}: {}",
        style("Operator Count").green(),
        request.operator_count
    );
    println!(
        "{}: {}",
        style("Approval Count").green(),
        request.approval_count
    );
    println!(
        "{}: {}",
        style("Payment Token").green(),
        request.payment_token
    );
    println!(
        "{}: {}",
        style("Payment Amount").green(),
        request.payment_amount
    );
    println!("{}: {:?}", style("Membership").green(), request.membership);
    println!(
        "{}: {} - {}",
        style("Operator Bounds").green(),
        request.min_operators,
        request.max_operators
    );
    println!("{}: {}", style("Rejected").green(), request.rejected);
}
