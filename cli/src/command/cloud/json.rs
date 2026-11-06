//! The CLI transports the SDK's generated request and response contract.

use blueprint_remote_providers::managed::{BoundaryRequest, BoundaryResponse, execute_boundary};
use color_eyre::Result;
use serde_json::{Value, json};
use std::io::Read;
use zeroize::Zeroizing;

pub(super) async fn execute(operation: &str) -> Result<()> {
    let response = match read(operation) {
        Some(request) => execute_boundary(request).await,
        None => BoundaryResponse::invalid(),
    };
    println!("{}", serde_json::to_string(&response)?);
    if !response.is_success() {
        std::process::exit(2);
    }
    Ok(())
}

fn read(operation: &str) -> Option<BoundaryRequest> {
    let mut input = Zeroizing::new(Vec::new());
    std::io::stdin()
        .take(1_048_577)
        .read_to_end(&mut input)
        .ok()?;
    if input.len() > 1_048_576 {
        return None;
    }
    let mut value: Value = if input.is_empty() && operation == "catalog" {
        json!({})
    } else {
        serde_json::from_slice(&input).ok()?
    };
    let object = value.as_object_mut()?;
    if object
        .get("operation")
        .is_some_and(|value| value.as_str() != Some(operation))
    {
        return None;
    }
    object.insert("operation".into(), Value::String(operation.into()));
    serde_json::from_value(value).ok()
}
