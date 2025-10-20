use super::types::*;
use crate::error::{EigenlayerExtraError, Result};
use reqwest::Client;
use std::time::Duration;

/// Sidecar API client for EigenLayer rewards data.
///
/// Communicates with EigenLayer's Sidecar API to fetch rewards proofs and summaries.
/// See: https://github.com/Layr-Labs/sidecar
#[derive(Clone)]
pub struct SidecarClient {
    base_url: String,
    client: Client,
}

impl SidecarClient {
    /// Create a new Sidecar client
    ///
    /// # Arguments
    /// * `base_url` - Base URL for the Sidecar API (e.g., https://sidecar-rpc.eigenlayer.xyz/mainnet)
    pub fn new(base_url: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .map_err(|e| {
                EigenlayerExtraError::Other(format!("Failed to create HTTP client: {}", e))
            })?;

        Ok(Self { base_url, client })
    }

    /// Generate a claim proof for the given earner and tokens
    ///
    /// # Arguments
    /// * `earner_address` - Address of the earner
    /// * `tokens` - List of token addresses to claim (empty vec claims all tokens)
    /// * `root_index` - Optional root index (uses current active root if None)
    pub async fn generate_claim_proof(
        &self,
        earner_address: &str,
        tokens: Vec<String>,
        root_index: Option<i64>,
    ) -> Result<Proof> {
        let url = format!("{}/rewards/v1/claim-proof", self.base_url);
        let req = GenerateClaimProofRequest {
            earner_address: earner_address.to_string(),
            tokens,
            root_index,
        };

        let response = self
            .client
            .post(&url)
            .header("x-sidecar-source", "tangle-cli")
            .json(&req)
            .send()
            .await
            .map_err(|e| EigenlayerExtraError::Other(format!("Sidecar request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(EigenlayerExtraError::Other(format!(
                "Sidecar API error {}: {}",
                status, body
            )));
        }

        let resp: GenerateClaimProofResponse = response
            .json()
            .await
            .map_err(|e| EigenlayerExtraError::Other(format!("Failed to parse response: {}", e)))?;

        Ok(resp.proof)
    }

    /// Get summarized rewards for an earner
    ///
    /// Returns earned, active, claimed, and claimable amounts per token
    ///
    /// # Arguments
    /// * `earner_address` - Address of the earner
    /// * `block_height` - Optional block height (uses latest if None)
    pub async fn get_summarized_rewards(
        &self,
        earner_address: &str,
        block_height: Option<u64>,
    ) -> Result<Vec<SummarizedEarnerReward>> {
        let mut url = format!(
            "{}/rewards/v1/earners/{}/summarized-rewards",
            self.base_url, earner_address
        );

        if let Some(height) = block_height {
            url.push_str(&format!("?blockHeight={}", height));
        }

        let response = self
            .client
            .get(&url)
            .header("x-sidecar-source", "tangle-cli")
            .send()
            .await
            .map_err(|e| EigenlayerExtraError::Other(format!("Sidecar request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(EigenlayerExtraError::Other(format!(
                "Sidecar API error {}: {}",
                status, body
            )));
        }

        let resp: GetSummarizedRewardsResponse = response
            .json()
            .await
            .map_err(|e| EigenlayerExtraError::Other(format!("Failed to parse response: {}", e)))?;

        Ok(resp.rewards)
    }

    /// List distribution roots
    ///
    /// # Arguments
    /// * `block_height` - Optional block height filter
    pub async fn list_distribution_roots(
        &self,
        block_height: Option<u64>,
    ) -> Result<Vec<DistributionRoot>> {
        let mut url = format!("{}/rewards/v1/distribution-roots", self.base_url);

        if let Some(height) = block_height {
            url.push_str(&format!("?blockHeight={}", height));
        }

        let response = self
            .client
            .get(&url)
            .header("x-sidecar-source", "tangle-cli")
            .send()
            .await
            .map_err(|e| EigenlayerExtraError::Other(format!("Sidecar request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(EigenlayerExtraError::Other(format!(
                "Sidecar API error {}: {}",
                status, body
            )));
        }

        let resp: ListDistributionRootsResponse = response
            .json()
            .await
            .map_err(|e| EigenlayerExtraError::Other(format!("Failed to parse response: {}", e)))?;

        Ok(resp.distribution_roots)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Requires Sidecar API endpoint"]
    async fn test_sidecar_client_creation() {
        let client = SidecarClient::new("https://sidecar-rpc.eigenlayer.xyz/holesky".to_string());
        assert!(client.is_ok());
    }

    #[tokio::test]
    #[ignore = "Requires Sidecar API endpoint and valid earner address"]
    async fn test_get_summarized_rewards() {
        let client =
            SidecarClient::new("https://sidecar-rpc.eigenlayer.xyz/holesky".to_string()).unwrap();

        // Use a test address - replace with actual earner address for real testing
        let result = client
            .get_summarized_rewards("0x0000000000000000000000000000000000000000", None)
            .await;

        // This test will fail with invalid address, but verifies the client works
        println!("Result: {:?}", result);
    }
}
