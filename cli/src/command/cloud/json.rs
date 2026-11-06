//! The CLI transports the SDK's generated request and response contract.

use blueprint_remote_providers::managed::{
    BoundaryResponse, execute_boundary, read_boundary_request,
};
use color_eyre::Result;

pub(super) async fn execute(operation: &str) -> Result<()> {
    let response = match read_boundary_request(operation, std::io::stdin().lock()) {
        Some(request) => execute_boundary(request).await,
        None => BoundaryResponse::invalid(),
    };
    println!("{}", serde_json::to_string(&response)?);
    if !response.is_success() {
        std::process::exit(2);
    }
    Ok(())
}
