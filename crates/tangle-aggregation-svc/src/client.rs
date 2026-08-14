//! HTTP client for the aggregation service
//!
//! This module provides a client for interacting with the aggregation service
//! from operator code. It handles all the HTTP communication and provides
//! a simple async API.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use blueprint_tangle_aggregation_svc::client::AggregationServiceClient;
//!
//! let client = AggregationServiceClient::new("http://localhost:8080");
//!
//! // Initialize a task
//! client
//!     .init_task(
//!         service_id,
//!         call_id,
//!         &output,
//!         operator_count,
//!         ThresholdConfig::Count { required_signers: 2 },
//!     )
//!     .await?;
//!
//! // Submit a signature
//! let response = client.submit_signature(request).await?;
//!
//! // Check status
//! let status = client.get_status(service_id, call_id).await?;
//!
//! // Get aggregated result
//! if let Some(result) = client.get_aggregated(service_id, call_id).await? {
//!     // Submit to chain
//! }
//! ```

use crate::types::*;
use std::time::Duration;
use thiserror::Error;

/// Client error types
#[derive(Debug, Error)]
pub enum ClientError {
    /// HTTP request failed
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    /// Server returned an error
    #[error("Server error: {0}")]
    Server(String),
    /// Invalid response
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    /// Task not found
    #[error("Task not found")]
    NotFound,
    /// Threshold not met
    #[error("Threshold not met: {collected}/{required}")]
    ThresholdNotMet { collected: usize, required: usize },
    /// Another operator already submitted this task to the chain.
    #[error("Aggregation task was already submitted to the chain")]
    AlreadySubmitted,
}

/// Outcome of waiting for an aggregation task.
#[derive(Debug)]
pub enum ThresholdWaitResult {
    /// The caller can submit this aggregate to the chain.
    Aggregated(AggregatedResultResponse),
    /// Another operator already submitted the task.
    Submitted,
}

/// HTTP client for the aggregation service
#[derive(Debug, Clone)]
pub struct AggregationServiceClient {
    client: reqwest::Client,
    base_url: String,
}

impl AggregationServiceClient {
    /// Create a new client
    pub fn new(base_url: impl Into<String>) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    /// Create a client with custom reqwest client
    pub fn with_client(client: reqwest::Client, base_url: impl Into<String>) -> Self {
        Self {
            client,
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    /// Health check
    pub async fn health(&self) -> Result<bool, ClientError> {
        let url = format!("{}/health", self.base_url);
        let response = self.client.get(&url).send().await?;
        Ok(response.status().is_success())
    }

    /// Initialize an aggregation task
    pub async fn init_task(
        &self,
        service_id: u64,
        call_id: u64,
        output: &[u8],
        operator_count: u32,
        threshold: ThresholdConfig,
    ) -> Result<(), ClientError> {
        let message = crate::service::create_signing_message(service_id, call_id, output);
        self.init_task_with_message(
            service_id,
            call_id,
            output,
            &message,
            Bn254SignatureScheme::ArkworksSha256,
            operator_count,
            threshold,
        )
        .await
    }

    /// Initialize a task with an explicit signed message and algorithm.
    #[allow(clippy::too_many_arguments)]
    pub async fn init_task_with_message(
        &self,
        service_id: u64,
        call_id: u64,
        output: &[u8],
        message: &[u8],
        signature_scheme: Bn254SignatureScheme,
        operator_count: u32,
        threshold: ThresholdConfig,
    ) -> Result<(), ClientError> {
        let url = format!("{}/v1/tasks/init", self.base_url);
        let request = InitTaskRequest {
            service_id,
            call_id,
            operator_count,
            threshold,
            output: output.to_vec(),
            message: message.to_vec(),
            signature_scheme,
        };

        let response: InitTaskResponse = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        if response.success {
            Ok(())
        } else {
            Err(ClientError::Server(
                response
                    .error
                    .unwrap_or_else(|| "Unknown error".to_string()),
            ))
        }
    }

    /// Submit a signature for aggregation
    pub async fn submit_signature(
        &self,
        request: SubmitSignatureRequest,
    ) -> Result<SubmitSignatureResponse, ClientError> {
        let url = format!("{}/v1/tasks/submit", self.base_url);
        let response: SubmitSignatureResponse = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        if response.accepted {
            Ok(response)
        } else {
            Err(ClientError::Server(
                response
                    .error
                    .unwrap_or_else(|| "Signature rejected".to_string()),
            ))
        }
    }

    /// Get the status of an aggregation task
    pub async fn get_status(
        &self,
        service_id: u64,
        call_id: u64,
    ) -> Result<GetStatusResponse, ClientError> {
        let url = format!("{}/v1/tasks/status", self.base_url);
        let request = GetStatusRequest {
            service_id,
            call_id,
        };

        let response: GetStatusResponse = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        if !response.exists {
            return Err(ClientError::NotFound);
        }

        Ok(response)
    }

    /// Get the aggregated result if threshold is met
    pub async fn get_aggregated(
        &self,
        service_id: u64,
        call_id: u64,
    ) -> Result<Option<AggregatedResultResponse>, ClientError> {
        let url = format!("{}/v1/tasks/aggregate", self.base_url);
        let request = GetStatusRequest {
            service_id,
            call_id,
        };

        let response = self.client.post(&url).json(&request).send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        if !response.status().is_success() {
            return Err(ClientError::Server(format!(
                "Server returned {}",
                response.status()
            )));
        }

        let result: Option<AggregatedResultResponse> = response.json().await?;
        Ok(result)
    }

    /// Return an aggregate or learn that another operator submitted it.
    pub async fn get_aggregated_or_submitted(
        &self,
        service_id: u64,
        call_id: u64,
    ) -> Result<ThresholdWaitResult, ClientError> {
        if let Some(result) = self.get_aggregated(service_id, call_id).await? {
            return Ok(ThresholdWaitResult::Aggregated(result));
        }

        let status = self.get_status(service_id, call_id).await?;
        if status.submitted {
            return Ok(ThresholdWaitResult::Submitted);
        }

        Err(ClientError::ThresholdNotMet {
            collected: status.signatures_collected,
            required: status.threshold_required,
        })
    }

    /// Mark a task as submitted to the chain
    pub async fn mark_submitted(&self, service_id: u64, call_id: u64) -> Result<(), ClientError> {
        let url = format!("{}/v1/tasks/mark-submitted", self.base_url);
        let request = GetStatusRequest {
            service_id,
            call_id,
        };

        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            let error: serde_json::Value = response.json().await?;
            return Err(ClientError::Server(
                error["error"]
                    .as_str()
                    .unwrap_or("Unknown error")
                    .to_string(),
            ));
        }

        Ok(())
    }

    /// Get service statistics
    pub async fn get_stats(&self) -> Result<crate::ServiceStats, ClientError> {
        let url = format!("{}/v1/stats", self.base_url);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(ClientError::Server(format!(
                "Server returned {}",
                response.status()
            )));
        }

        let stats: crate::ServiceStats = response.json().await?;
        Ok(stats)
    }

    /// Wait for threshold to be met, with polling
    ///
    /// Returns the aggregated result once threshold is met, or an error if timeout is reached.
    pub async fn wait_for_threshold(
        &self,
        service_id: u64,
        call_id: u64,
        poll_interval: Duration,
        timeout: Duration,
    ) -> Result<AggregatedResultResponse, ClientError> {
        match self
            .wait_for_threshold_or_submitted(service_id, call_id, poll_interval, timeout)
            .await?
        {
            ThresholdWaitResult::Aggregated(result) => Ok(result),
            ThresholdWaitResult::Submitted => Err(ClientError::AlreadySubmitted),
        }
    }

    /// Wait for an aggregate or learn that another operator submitted it.
    ///
    /// A submitted task no longer exposes aggregate bytes. Waiting only on the
    /// aggregate endpoint would therefore wait until timeout after a successful
    /// submission by another operator.
    pub async fn wait_for_threshold_or_submitted(
        &self,
        service_id: u64,
        call_id: u64,
        poll_interval: Duration,
        timeout: Duration,
    ) -> Result<ThresholdWaitResult, ClientError> {
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > timeout {
                // Get current status for error message
                let status = self.get_status(service_id, call_id).await?;
                if status.submitted {
                    return Ok(ThresholdWaitResult::Submitted);
                }
                return Err(ClientError::ThresholdNotMet {
                    collected: status.signatures_collected,
                    required: status.threshold_required,
                });
            }

            // Check if aggregated result is available
            if let Some(result) = self.get_aggregated(service_id, call_id).await? {
                return Ok(ThresholdWaitResult::Aggregated(result));
            }

            let status = self.get_status(service_id, call_id).await?;
            if status.submitted {
                return Ok(ThresholdWaitResult::Submitted);
            }

            // Wait before next poll
            tokio::time::sleep(poll_interval).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{api, AggregationService, ServiceConfig};
    use std::sync::Arc;

    #[test]
    fn test_client_url_normalization() {
        let client = AggregationServiceClient::new("http://localhost:8080/");
        assert_eq!(client.base_url, "http://localhost:8080");

        let client = AggregationServiceClient::new("http://localhost:8080");
        assert_eq!(client.base_url, "http://localhost:8080");
    }

    #[tokio::test]
    async fn waiting_operator_observes_submitted_task_without_aggregate_replay() {
        let service = Arc::new(AggregationService::new(ServiceConfig::default()));
        service.init_task(1, 1, vec![1], 2, 2).unwrap();
        service.mark_submitted(1, 1).unwrap();

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(listener, api::router(service)).await.unwrap();
        });

        let client = AggregationServiceClient::new(format!("http://{address}"));
        assert!(matches!(
            client.get_aggregated_or_submitted(1, 1).await.unwrap(),
            ThresholdWaitResult::Submitted
        ));

        let result = tokio::time::timeout(
            Duration::from_millis(250),
            client.wait_for_threshold_or_submitted(
                1,
                1,
                Duration::from_millis(5),
                Duration::from_secs(30),
            ),
        )
        .await
        .expect("submitted state should complete promptly")
        .unwrap();

        assert!(matches!(result, ThresholdWaitResult::Submitted));
        server.abort();
        let _ = server.await;
    }
}
