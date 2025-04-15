use blueprint_core::info;
use blueprint_core_testing_utils::Error;
use blueprint_std::sync::{Arc, Mutex};
use blueprint_std::time::Duration;

/// Waits for the given `successful_responses` Mutex to be greater than or equal to `task_response_count`.
///
/// # Errors
/// - Returns `Error::WaitResponse` if the `successful_responses` Mutex lock fails.
pub async fn wait_for_responses(
    successful_responses: Arc<Mutex<usize>>,
    task_response_count: usize,
    timeout_duration: Duration,
) -> Result<Result<(), Error>, tokio::time::error::Elapsed> {
    tokio::time::timeout(timeout_duration, async move {
        loop {
            let count = match successful_responses.lock() {
                Ok(guard) => *guard,
                Err(e) => {
                    return Err(Error::WaitResponse(e.to_string()));
                }
            };
            if count >= task_response_count {
                info!("Successfully received {} task responses", count);
                return Ok(());
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    })
    .await
}
