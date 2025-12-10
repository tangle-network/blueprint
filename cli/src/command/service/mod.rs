use alloy_primitives::{Address, Bytes, U256};
use blueprint_client_tangle_evm::{
    TangleEvmClient, TransactionResult, contracts::ITangleTypes, services::ServiceRequestParams,
};
use color_eyre::eyre::Result;

/// Submit a service request.
pub async fn request_service(
    client: &TangleEvmClient,
    params: ServiceRequestParams,
) -> Result<(TransactionResult, u64)> {
    client.request_service(params).await.map_err(Into::into)
}

/// Approve a pending service request.
pub async fn approve_service(
    client: &TangleEvmClient,
    request_id: u64,
    restaking_percent: u8,
) -> Result<TransactionResult> {
    client
        .approve_service(request_id, restaking_percent)
        .await
        .map_err(Into::into)
}

/// Approve a pending request with explicit security commitments.
pub async fn approve_service_with_commitments(
    client: &TangleEvmClient,
    request_id: u64,
    commitments: Vec<ITangleTypes::AssetSecurityCommitment>,
) -> Result<TransactionResult> {
    client
        .approve_service_with_commitments(request_id, commitments)
        .await
        .map_err(Into::into)
}

/// Reject a pending service request.
pub async fn reject_service(
    client: &TangleEvmClient,
    request_id: u64,
) -> Result<TransactionResult> {
    client.reject_service(request_id).await.map_err(Into::into)
}

/// Join a dynamic service.
pub async fn join_service(
    client: &TangleEvmClient,
    service_id: u64,
    exposure_bps: u16,
) -> Result<TransactionResult> {
    client
        .join_service(service_id, exposure_bps)
        .await
        .map_err(Into::into)
}

/// Leave a dynamic service (legacy immediate exit path).
pub async fn leave_service(client: &TangleEvmClient, service_id: u64) -> Result<TransactionResult> {
    client.leave_service(service_id).await.map_err(Into::into)
}

/// Convenience helper to build request parameters.
pub fn build_request_params(
    blueprint_id: u64,
    operators: Vec<Address>,
    operator_exposures: Option<Vec<u16>>,
    permitted_callers: Vec<Address>,
    ttl: u64,
    payment_token: Address,
    payment_amount: U256,
    config: Bytes,
) -> ServiceRequestParams {
    let mut params = ServiceRequestParams::new(
        blueprint_id,
        operators,
        permitted_callers,
        config,
        ttl,
        payment_token,
        payment_amount,
    );
    params.operator_exposures = operator_exposures;
    params
}

/// Attach security requirements to a request parameter set.
pub fn with_security_requirements(
    mut params: ServiceRequestParams,
    requirements: Vec<ITangleTypes::AssetSecurityRequirement>,
) -> ServiceRequestParams {
    params.security_requirements = requirements;
    params
}
