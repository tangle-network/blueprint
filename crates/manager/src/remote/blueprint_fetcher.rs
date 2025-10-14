//! Blueprint metadata fetcher from Tangle chain.
//!
//! Fetches blueprint information from Tangle to determine deployment strategy.

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Blueprint metadata from chain.
#[derive(Debug, Clone)]
pub struct BlueprintMetadata {
    pub blueprint_id: u64,
    pub job_count: u32,
    /// Job profiles from benchmarking (if available)
    pub job_profiles: Vec<Option<JobProfile>>,
}

/// Job profile from benchmarking (simplified version for manager).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobProfile {
    pub avg_duration_ms: u64,
    pub peak_memory_mb: u32,
    pub p95_duration_ms: u64,
    pub stateful: bool,
    pub persistent_connections: bool,
}

impl JobProfile {
    /// Convert to pricing-engine BenchmarkProfile for cost calculation
    ///
    /// This creates a simplified BenchmarkProfile that can be used with the
    /// existing pricing-engine infrastructure.
    ///
    /// This is a pure data transformation - no cloud access required.
    pub fn to_pricing_benchmark_profile(&self) -> blueprint_pricing_engine_lib::BenchmarkProfile {
        use blueprint_pricing_engine_lib::benchmark::*;

        // Estimate CPU cores from duration (heuristic)
        // Fast jobs (<100ms) likely use < 1 core, slower jobs use more
        let avg_cores = if self.avg_duration_ms < 100 {
            0.5
        } else if self.avg_duration_ms < 1000 {
            1.0
        } else {
            2.0
        };

        blueprint_pricing_engine_lib::BenchmarkProfile {
            job_id: "job".to_string(), // Will be overridden by caller
            execution_mode: "native".to_string(),
            duration_secs: (self.avg_duration_ms / 1000).max(1),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            success: true,
            cpu_details: Some(CpuBenchmarkResult {
                num_cores_detected: 4, // Default assumption
                avg_cores_used: avg_cores,
                avg_usage_percent: 50.0, // Conservative estimate
                peak_cores_used: avg_cores * 1.2, // 20% headroom
                peak_usage_percent: 75.0, // Conservative peak estimate
                benchmark_duration_ms: self.avg_duration_ms,
                primes_found: 0, // Not measured in job profiling
                max_prime: 0,
                primes_per_second: 0.0,
                cpu_model: "Unknown".to_string(),
                cpu_frequency_mhz: 0.0,
            }),
            memory_details: Some(MemoryBenchmarkResult {
                avg_memory_mb: (self.peak_memory_mb as f32 * 0.7), // Avg ~70% of peak
                peak_memory_mb: self.peak_memory_mb as f32,
                block_size_kb: 4,
                total_size_mb: self.peak_memory_mb as u64,
                operations_per_second: 1000.0,
                transfer_rate_mb_s: 100.0,
                access_mode: MemoryAccessMode::Sequential,
                operation_type: MemoryOperationType::None,
                latency_ns: 100.0,
                duration_ms: self.avg_duration_ms,
            }),
            storage_details: None, // Job profiling doesn't measure storage
            network_details: None, // Job profiling doesn't measure network
            gpu_details: None,     // Job profiling doesn't measure GPU
            io_details: None,      // Job profiling doesn't measure I/O
        }
    }
}

/// Fetch blueprint metadata from Tangle chain and filesystem.
///
/// This function:
/// 1. Fetches blueprint structure from Tangle chain (job count, etc.)
/// 2. Attempts to load profiling data from filesystem (`target/blueprint-profiles.json`)
/// 3. Returns combined metadata for deployment analysis
pub async fn fetch_blueprint_metadata(
    blueprint_id: u64,
    rpc_url: Option<&str>,
    binary_path: Option<&std::path::Path>,
) -> Result<BlueprintMetadata> {
    // Get blueprint structure from chain
    let mut metadata = {
        #[cfg(feature = "tangle-client")]
        {
            fetch_from_chain(blueprint_id, rpc_url).await?
        }

        #[cfg(not(feature = "tangle-client"))]
        {
            fetch_mock(blueprint_id).await?
        }
    };

    // Try to load profiling data from filesystem if binary path provided
    if let Some(bin_path) = binary_path {
        if let Some(profiles) = load_profiles_from_filesystem(bin_path) {
            tracing::info!(
                "Loaded {} job profiles from filesystem",
                profiles.len()
            );
            metadata.job_profiles = profiles;
        } else {
            tracing::warn!(
                "No profiling data found - deployment will use conservative defaults. \
                 Run `cargo test --test profiling_test` to generate profiles."
            );
        }
    }

    Ok(metadata)
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

    // Extract job profiles from ServiceMetadata
    // Priority: profiling_data field (after migration) > description field (temporary) > defaults
    let job_profiles = {
        // FUTURE: After chain migration adds profiling_data field, uncomment this:
        /*
        if let Some(profiling_data_bounded) = &blueprint.metadata.profiling_data {
            // Convert BoundedString to &str
            let profiling_data_str = std::str::from_utf8(&profiling_data_bounded.0)
                .ok()
                .unwrap_or("");

            if !profiling_data_str.is_empty() {
                // Decode base64-encoded compressed profiles
                match decode_profiles_from_chain(profiling_data_str) {
                    Ok(profiles) => {
                        tracing::info!(
                            "Loaded {} job profiles from chain metadata (profiling_data field)",
                            profiles.iter().filter(|p| p.is_some()).count()
                        );
                        profiles
                    }
                    Err(e) => {
                        tracing::warn!(
                            "Failed to decode profiling data from chain: {}. Trying description field.",
                            e
                        );
                        vec![None; job_count as usize]
                    }
                }
            } else {
                tracing::debug!("profiling_data field exists but is empty. Trying description field.");
                vec![None; job_count as usize]
            }
        } else */
        // TEMPORARY: Extract from description field until chain migration completes
        if let Some(description_bounded) = &blueprint.metadata.description {
            let description_str = std::str::from_utf8(&description_bounded.0)
                .ok()
                .unwrap_or("");

            if blueprint_profiling::has_profiling_data(description_str) {
                match blueprint_profiling::BlueprintProfiles::from_description_field(description_str) {
                    Some(Ok(profiles)) => {
                        // Convert BlueprintProfiles to Vec<Option<JobProfile>>
                        let max_job_id = profiles.jobs.keys().copied().max().unwrap_or(0);
                        let mut result = vec![None; (max_job_id + 1).max(job_count) as usize];

                        for (job_id, profile) in profiles.jobs {
                            if (job_id as usize) < result.len() {
                                result[job_id as usize] = Some(profile);
                            }
                        }

                        tracing::info!(
                            "Loaded {} job profiles from chain metadata (description field - temporary)",
                            profiles.jobs.len()
                        );
                        result
                    }
                    Some(Err(e)) => {
                        tracing::warn!(
                            "Failed to decode profiling data from description field: {}. Using defaults.",
                            e
                        );
                        vec![None; job_count as usize]
                    }
                    None => {
                        tracing::debug!("No profiling data marker in description field");
                        vec![None; job_count as usize]
                    }
                }
            } else {
                tracing::debug!("No profiling data in description field");
                vec![None; job_count as usize]
            }
        } else {
            tracing::debug!("No description field in chain metadata");
            vec![None; job_count as usize]
        }
    };

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

/// Decode profiling data from chain metadata
///
/// Decodes base64-encoded compressed profiling data from ServiceMetadata.
fn decode_profiles_from_chain(encoded: &str) -> Result<Vec<Option<JobProfile>>> {
    use base64::Engine;

    // Decode base64
    let compressed = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| crate::error::Error::Other(format!("Base64 decode failed: {}", e)))?;

    // Decompress gzip
    use flate2::read::GzDecoder;
    use std::io::Read;

    let mut decoder = GzDecoder::new(&compressed[..]);
    let mut json = String::new();
    decoder
        .read_to_string(&mut json)
        .map_err(|e| crate::error::Error::Other(format!("Decompression failed: {}", e)))?;

    // Parse JSON
    let profiles: serde_json::Value = serde_json::from_str(&json)
        .map_err(|e| crate::error::Error::Other(format!("JSON parse failed: {}", e)))?;

    // Extract jobs map
    let jobs = profiles
        .get("jobs")
        .and_then(|j| j.as_object())
        .ok_or_else(|| crate::error::Error::Other("Missing 'jobs' field in profile data".to_string()))?;

    // Convert to Vec<Option<JobProfile>>
    let max_job_id = jobs
        .keys()
        .filter_map(|k| k.parse::<u32>().ok())
        .max()
        .unwrap_or(0);

    let mut result = vec![None; (max_job_id + 1) as usize];

    for (job_id_str, profile_value) in jobs {
        if let Ok(job_id) = job_id_str.parse::<u32>() {
            if let Ok(profile) = serde_json::from_value::<JobProfile>(profile_value.clone()) {
                if (job_id as usize) < result.len() {
                    result[job_id as usize] = Some(profile);
                }
            }
        }
    }

    Ok(result)
}

/// Load profiling data from filesystem
///
/// Looks for `target/blueprint-profiles.json` relative to the binary path.
/// Returns None if file doesn't exist or can't be parsed.
fn load_profiles_from_filesystem(binary_path: &std::path::Path) -> Option<Vec<Option<JobProfile>>> {
    // Find target directory (binary is in target/release/ or target/debug/)
    let binary_dir = binary_path.parent()?;
    let target_dir = binary_dir.parent()?;
    let profile_path = target_dir.join("blueprint-profiles.json");

    if !profile_path.exists() {
        tracing::debug!(
            "No profiling data found at {}",
            profile_path.display()
        );
        return None;
    }

    // Read and parse the JSON file
    let content = std::fs::read_to_string(&profile_path).ok()?;
    let profiles: serde_json::Value = serde_json::from_str(&content).ok()?;

    // Extract jobs map
    let jobs = profiles.get("jobs")?.as_object()?;

    // Convert to Vec<Option<JobProfile>>
    // We need to handle job IDs that might not be sequential
    let max_job_id = jobs
        .keys()
        .filter_map(|k| k.parse::<u32>().ok())
        .max()
        .unwrap_or(0);

    let mut result = vec![None; (max_job_id + 1) as usize];

    for (job_id_str, profile_value) in jobs {
        if let Ok(job_id) = job_id_str.parse::<u32>() {
            if let Ok(profile) = serde_json::from_value::<JobProfile>(profile_value.clone()) {
                if (job_id as usize) < result.len() {
                    result[job_id as usize] = Some(profile);
                }
            }
        }
    }

    tracing::info!(
        "Loaded profiling data from {} ({} jobs profiled)",
        profile_path.display(),
        jobs.len()
    );

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_blueprint_metadata() {
        let metadata = fetch_blueprint_metadata(42, None, None).await.unwrap();
        assert_eq!(metadata.blueprint_id, 42);
        assert_eq!(metadata.job_count, 2);
    }
}
