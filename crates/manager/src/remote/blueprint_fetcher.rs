//! Blueprint metadata fetcher from Tangle chain.
//!
//! Fetches blueprint information from Tangle to determine deployment strategy.

use crate::error::{Error, Result};

/// Blueprint metadata from chain.
#[derive(Debug, Clone)]
pub struct BlueprintMetadata {
    pub blueprint_id: u64,
    pub job_count: u32,
    /// Job profiles from benchmarking (if available)
    pub job_profiles: Vec<Option<JobProfile>>,
}

/// Job profile from benchmarking (simplified version for manager).
#[derive(Debug, Clone)]
pub struct JobProfile {
    pub avg_duration_ms: u64,
    pub peak_memory_mb: u32,
    pub p95_duration_ms: u64,
    pub stateful: bool,
    pub persistent_connections: bool,
}

/// Fetch blueprint metadata from Tangle chain.
pub async fn fetch_blueprint_metadata(
    blueprint_id: u64,
    rpc_url: Option<&str>,
) -> Result<BlueprintMetadata> {
    #[cfg(feature = "tangle-client")]
    {
        fetch_from_chain(blueprint_id, rpc_url).await
    }

    #[cfg(not(feature = "tangle-client"))]
    {
        fetch_mock(blueprint_id).await
    }
}

#[cfg(feature = "tangle-client")]
async fn fetch_from_chain(blueprint_id: u64, rpc_url: Option<&str>) -> Result<BlueprintMetadata> {
    use blueprint_tangle_client::ServicesClient;

    let url = rpc_url.unwrap_or("ws://localhost:9944");

    tracing::debug!(
        "Fetching blueprint {} metadata from Tangle at {}",
        blueprint_id,
        url
    );

    let client = ServicesClient::new(url)
        .await
        .map_err(|e| Error::Other(format!("Failed to connect to Tangle: {}", e)))?;

    // Get latest block hash
    let latest_block = client
        .rpc_client()
        .blocks()
        .at_latest()
        .await
        .map_err(|e| Error::Other(format!("Failed to get latest block: {}", e)))?;

    let block_hash = latest_block.hash();

    // Query blueprint
    let blueprint = client
        .get_blueprint_by_id(block_hash.into(), blueprint_id)
        .await
        .map_err(|e| Error::Other(format!("Failed to query blueprint: {}", e)))?
        .ok_or_else(|| Error::Other(format!("Blueprint {} not found", blueprint_id)))?;

    let job_count = blueprint.jobs.0.len() as u32;

    // Extract job profiles if available
    let job_profiles = blueprint
        .jobs
        .0
        .iter()
        .map(|job| {
            // Try to extract profile from job metadata
            // For now, profiles are stored as JSON in the description field
            // TODO: Add dedicated profile field to JobMetadata on chain
            None // Placeholder until chain supports profiles
        })
        .collect();

    tracing::info!(
        "Fetched blueprint {} with {} jobs from Tangle",
        blueprint_id,
        job_count
    );

    Ok(BlueprintMetadata {
        blueprint_id,
        job_count,
        job_profiles,
    })
}

#[cfg(not(feature = "tangle-client"))]
async fn fetch_mock(blueprint_id: u64) -> Result<BlueprintMetadata> {
    tracing::warn!(
        "Tangle client not enabled, using mock blueprint metadata (blueprint_id={}, job_count=2)",
        blueprint_id
    );

    Ok(BlueprintMetadata {
        blueprint_id,
        job_count: 2,
        job_profiles: vec![None, None], // No profiles in mock mode
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_blueprint_metadata() {
        let metadata = fetch_blueprint_metadata(42, None).await.unwrap();
        assert_eq!(metadata.blueprint_id, 42);
        assert_eq!(metadata.job_count, 2);
    }
}
