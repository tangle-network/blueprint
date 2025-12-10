use alloy_primitives::Bytes;
use blueprint_client_tangle_evm::{JobSubmissionResult, TangleEvmClient};
use color_eyre::Result;

/// Submit a job invocation to the configured service.
pub async fn submit_job(
    client: &TangleEvmClient,
    service_id: u64,
    job_index: u8,
    inputs: Bytes,
) -> Result<JobSubmissionResult> {
    let submission = client
        .submit_job(service_id, job_index, inputs)
        .await
        .map_err(|e| color_eyre::Report::msg(e.to_string()))?;
    Ok(submission)
}
