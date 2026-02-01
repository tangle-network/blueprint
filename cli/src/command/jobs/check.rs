use std::time::Duration;

use blueprint_client_tangle::TangleClient;
use blueprint_client_tangle::contracts::ITangle;
use color_eyre::eyre::{Result, eyre};
use tokio::time::timeout;

/// Wait for a `JobResultSubmitted` event matching the provided identifiers.
pub async fn wait_for_job_result(
    client: &TangleClient,
    service_id: u64,
    call_id: u64,
    timeout_duration: Duration,
) -> Result<Vec<u8>> {
    let fut = async {
        loop {
            if let Some(event) = client.next_event().await {
                for log in event.logs {
                    if let Ok(decoded) = log.log_decode::<ITangle::JobResultSubmitted>() {
                        if decoded.inner.serviceId == service_id && decoded.inner.callId == call_id
                        {
                            let bytes: Vec<u8> = decoded.inner.result.clone().into();
                            return Ok(bytes);
                        }
                    }
                }
            }
        }
    };

    timeout(timeout_duration, fut)
        .await
        .map_err(|_| eyre!("timed out waiting for result for call {call_id}"))
        .and_then(|res| res)
}
