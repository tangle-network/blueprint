/// Blueprint Job Profiling System
///
/// This crate provides automated profiling for Blueprint jobs, inspired by Substrate's benchmarking system.
/// It measures execution time, memory usage, and other resources to determine FaaS compatibility.
///
/// ## Usage
///
/// ```rust,ignore
/// use blueprint_profiling::{ProfileRunner, ProfileConfig};
///
/// let config = ProfileConfig::default();
/// let profile = ProfileRunner::profile_job(job_fn, config).await?;
/// println!("Job profile: {:?}", profile);
/// ```
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProfilingError {
    #[error("Job execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Insufficient samples: expected at least {expected}, got {actual}")]
    InsufficientSamples { expected: u32, actual: u32 },
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
}

/// Configuration for profiling a job
#[derive(Debug, Clone)]
pub struct ProfileConfig {
    /// Number of times to execute the job for measurement
    pub sample_size: u32,
    /// Warm-up runs before measurement (not counted in stats)
    pub warmup_runs: u32,
    /// Maximum duration for a single job execution
    pub max_execution_time: Duration,
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            sample_size: 10,
            warmup_runs: 2,
            max_execution_time: Duration::from_secs(300),
        }
    }
}

/// Resource measurements for a single job execution
#[derive(Debug, Clone)]
pub struct ResourceMeasurement {
    /// Wall-clock execution time
    pub duration: Duration,
    /// Peak memory usage in bytes during execution
    pub peak_memory_bytes: u64,
    /// CPU time (user + system) in microseconds
    pub cpu_time_us: u64,
}

/// Complete profile of a job's resource usage
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobProfile {
    /// Average execution time in milliseconds
    pub avg_duration_ms: u64,
    /// 95th percentile duration in milliseconds
    pub p95_duration_ms: u64,
    /// 99th percentile duration in milliseconds
    pub p99_duration_ms: u64,
    /// Peak memory usage in megabytes
    pub peak_memory_mb: u32,
    /// Whether the job maintains state between invocations
    pub stateful: bool,
    /// Whether the job requires persistent connections
    pub persistent_connections: bool,
    /// Number of samples used to compute statistics
    pub sample_size: u32,
}

/// Complete blueprint profiling output
///
/// This structure is written to `target/blueprint-profiles.json` and read by
/// the Blueprint Manager to make deployment decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintProfiles {
    /// Name of the blueprint
    pub blueprint_name: String,
    /// Timestamp when profiles were generated (ISO 8601)
    pub profiled_at: String,
    /// Job profiles indexed by job ID
    pub jobs: std::collections::HashMap<u32, JobProfile>,
}

impl BlueprintProfiles {
    /// Create a new blueprint profiles output
    pub fn new(blueprint_name: impl Into<String>) -> Self {
        Self {
            blueprint_name: blueprint_name.into(),
            profiled_at: chrono::Utc::now().to_rfc3339(),
            jobs: std::collections::HashMap::new(),
        }
    }

    /// Add a job profile
    pub fn add_job(&mut self, job_id: u32, profile: JobProfile) {
        self.jobs.insert(job_id, profile);
    }

    /// Save profiles to a JSON file
    ///
    /// Typically saved to `target/blueprint-profiles.json` in the blueprint workspace.
    pub fn save_to_file(&self, path: impl AsRef<std::path::Path>) -> Result<(), ProfilingError> {
        let json = serde_json::to_string_pretty(self).map_err(|e| {
            ProfilingError::InvalidConfiguration(format!("JSON serialization failed: {}", e))
        })?;

        std::fs::write(path.as_ref(), json).map_err(|e| {
            ProfilingError::InvalidConfiguration(format!("Failed to write file: {}", e))
        })?;

        Ok(())
    }

    /// Load profiles from a JSON file
    pub fn load_from_file(path: impl AsRef<std::path::Path>) -> Result<Self, ProfilingError> {
        let content = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            ProfilingError::InvalidConfiguration(format!("Failed to read file: {}", e))
        })?;

        serde_json::from_str(&content).map_err(|e| {
            ProfilingError::InvalidConfiguration(format!("JSON deserialization failed: {}", e))
        })
    }

    /// Serialize and compress profiles to bytes (for on-chain storage)
    ///
    /// Uses gzip compression to minimize on-chain storage costs.
    /// Typical sizes: 1 job ~80 bytes, 10 jobs ~577 bytes, 50 jobs ~2.7KB
    pub fn to_compressed_bytes(&self) -> Result<Vec<u8>, ProfilingError> {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        // Serialize to JSON (without pretty printing to save space)
        let json = serde_json::to_string(self).map_err(|e| {
            ProfilingError::InvalidConfiguration(format!("JSON serialization failed: {}", e))
        })?;

        // Compress with gzip
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(json.as_bytes()).map_err(|e| {
            ProfilingError::InvalidConfiguration(format!("Compression failed: {}", e))
        })?;

        encoder.finish().map_err(|e| {
            ProfilingError::InvalidConfiguration(format!("Compression finalization failed: {}", e))
        })
    }

    /// Deserialize and decompress profiles from bytes (for on-chain retrieval)
    pub fn from_compressed_bytes(compressed: &[u8]) -> Result<Self, ProfilingError> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        // Decompress
        let mut decoder = GzDecoder::new(compressed);
        let mut json = String::new();
        decoder.read_to_string(&mut json).map_err(|e| {
            ProfilingError::InvalidConfiguration(format!("Decompression failed: {}", e))
        })?;

        // Deserialize
        serde_json::from_str(&json).map_err(|e| {
            ProfilingError::InvalidConfiguration(format!("JSON deserialization failed: {}", e))
        })
    }

    /// Encode profiles as base64-encoded compressed data for on-chain storage
    ///
    /// This is the format used in `ServiceMetadata.profiling_data` field.
    /// Format: base64(gzip(JSON))
    pub fn to_base64_string(&self) -> Result<String, ProfilingError> {
        use base64::Engine;
        let compressed = self.to_compressed_bytes()?;
        Ok(base64::engine::general_purpose::STANDARD.encode(&compressed))
    }

    /// Decode profiles from base64-encoded compressed data
    ///
    /// This is the format used in `ServiceMetadata.profiling_data` field.
    /// Format: base64(gzip(JSON))
    pub fn from_base64_string(encoded: &str) -> Result<Self, ProfilingError> {
        use base64::Engine;
        let compressed = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .map_err(|e| {
                ProfilingError::InvalidConfiguration(format!("Base64 decode failed: {}", e))
            })?;
        Self::from_compressed_bytes(&compressed)
    }

    /// Encode profiles for storage in description field (temporary solution)
    ///
    /// Uses a special marker prefix so we can distinguish profiling data
    /// from regular descriptions. This is a temporary approach until the
    /// dedicated `profiling_data` field is added to the chain.
    ///
    /// Format: `[PROFILING_DATA_V1]base64(gzip(JSON))`
    ///
    /// Total size: marker (20 bytes) + base64 data (~260-468 bytes for 1-10 jobs)
    pub fn to_description_field(&self) -> Result<String, ProfilingError> {
        let encoded = self.to_base64_string()?;
        Ok(format!("[PROFILING_DATA_V1]{}", encoded))
    }

    /// Extract profiles from description field if it contains profiling data
    ///
    /// Returns None if the description doesn't contain profiling data marker.
    /// Returns Some(Err) if the description has the marker but decoding fails.
    pub fn from_description_field(description: &str) -> Option<Result<Self, ProfilingError>> {
        description.strip_prefix("[PROFILING_DATA_V1]").map(|encoded| Self::from_base64_string(encoded))
    }
}

/// Helper to check if description contains profiling data
///
/// Useful for checking without parsing the full profile.
pub fn has_profiling_data(description: &str) -> bool {
    description.starts_with("[PROFILING_DATA_V1]")
}

/// Cross-platform memory measurement
#[cfg(unix)]
fn get_current_memory_bytes() -> u64 {
    use std::mem::MaybeUninit;

    unsafe {
        let mut usage = MaybeUninit::<libc::rusage>::uninit();
        let result = libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr());

        if result == 0 {
            let usage = usage.assume_init();

            // macOS reports in bytes, Linux reports in kilobytes
            #[cfg(target_os = "macos")]
            return usage.ru_maxrss as u64;

            #[cfg(target_os = "linux")]
            return (usage.ru_maxrss as u64) * 1024;
        }
    }

    0
}

#[cfg(not(unix))]
fn get_current_memory_bytes() -> u64 {
    0
}

/// Get CPU time (user + system) in microseconds
#[cfg(unix)]
fn get_cpu_time_us() -> u64 {
    use std::mem::MaybeUninit;

    unsafe {
        let mut usage = MaybeUninit::<libc::rusage>::uninit();
        let result = libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr());

        if result == 0 {
            let usage = usage.assume_init();
            let user_us =
                (usage.ru_utime.tv_sec as u64) * 1_000_000 + (usage.ru_utime.tv_usec as u64);
            let sys_us =
                (usage.ru_stime.tv_sec as u64) * 1_000_000 + (usage.ru_stime.tv_usec as u64);
            return user_us + sys_us;
        }
    }

    0
}

#[cfg(not(unix))]
fn get_cpu_time_us() -> u64 {
    0
}

/// Profile runner for executing and measuring jobs
pub struct ProfileRunner;

impl ProfileRunner {
    /// Profile a job by executing it multiple times and collecting statistics
    ///
    /// # Arguments
    /// * `job_fn` - The job function to profile (must be async)
    /// * `config` - Profiling configuration
    ///
    /// # Returns
    /// A `JobProfile` containing statistical analysis of the job's resource usage
    pub async fn profile_job<F, Fut>(
        job_fn: F,
        config: ProfileConfig,
    ) -> Result<JobProfile, ProfilingError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>,
    {
        if config.sample_size == 0 {
            return Err(ProfilingError::InvalidConfiguration(
                "sample_size must be greater than 0".to_string(),
            ));
        }

        // Warm-up runs
        for _ in 0..config.warmup_runs {
            let _ = tokio::time::timeout(config.max_execution_time, job_fn()).await;
        }

        // Measurement runs
        let mut measurements = Vec::with_capacity(config.sample_size as usize);

        for _ in 0..config.sample_size {
            let mem_before = get_current_memory_bytes();
            let cpu_before = get_cpu_time_us();
            let start = Instant::now();

            // Execute the job with timeout
            match tokio::time::timeout(config.max_execution_time, job_fn()).await {
                Ok(Ok(())) => {
                    let duration = start.elapsed();
                    let mem_after = get_current_memory_bytes();
                    let cpu_after = get_cpu_time_us();

                    measurements.push(ResourceMeasurement {
                        duration,
                        peak_memory_bytes: mem_after.saturating_sub(mem_before),
                        cpu_time_us: cpu_after.saturating_sub(cpu_before),
                    });
                }
                Ok(Err(e)) => {
                    return Err(ProfilingError::ExecutionFailed(e.to_string()));
                }
                Err(_) => {
                    return Err(ProfilingError::ExecutionFailed(format!(
                        "Job execution exceeded maximum time of {}s",
                        config.max_execution_time.as_secs()
                    )));
                }
            }
        }

        if measurements.is_empty() {
            return Err(ProfilingError::InsufficientSamples {
                expected: config.sample_size,
                actual: 0,
            });
        }

        Ok(Self::compute_statistics(measurements, config.sample_size))
    }

    /// Compute statistical summary from measurements
    fn compute_statistics(measurements: Vec<ResourceMeasurement>, sample_size: u32) -> JobProfile {
        let mut durations: Vec<u64> = measurements
            .iter()
            .map(|m| m.duration.as_millis() as u64)
            .collect();
        durations.sort_unstable();

        let mut memories: Vec<u64> = measurements.iter().map(|m| m.peak_memory_bytes).collect();
        memories.sort_unstable();

        let avg_duration_ms = if !durations.is_empty() {
            durations.iter().sum::<u64>() / durations.len() as u64
        } else {
            0
        };

        let p95_duration_ms = Self::percentile(&durations, 95);
        let p99_duration_ms = Self::percentile(&durations, 99);
        let peak_memory_mb = (memories.last().copied().unwrap_or(0) / (1024 * 1024)) as u32;

        JobProfile {
            avg_duration_ms,
            p95_duration_ms,
            p99_duration_ms,
            peak_memory_mb,
            stateful: false,
            persistent_connections: false,
            sample_size,
        }
    }

    /// Calculate percentile from sorted data
    fn percentile(sorted_data: &[u64], percentile: u8) -> u64 {
        if sorted_data.is_empty() {
            return 0;
        }

        let index = ((sorted_data.len() as f64) * (percentile as f64 / 100.0)).ceil() as usize;
        let index = index.saturating_sub(1).min(sorted_data.len() - 1);
        sorted_data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_profile_simple_job() {
        let config = ProfileConfig {
            sample_size: 5,
            warmup_runs: 1,
            max_execution_time: Duration::from_secs(10),
        };

        let result = ProfileRunner::profile_job(
            || async {
                tokio::time::sleep(Duration::from_millis(10)).await;
                Ok(())
            },
            config,
        )
        .await;

        assert!(result.is_ok());
        let profile = result.unwrap();
        assert_eq!(profile.sample_size, 5);
        assert!(profile.avg_duration_ms >= 10);
    }

    #[tokio::test]
    async fn test_profile_failing_job() {
        let config = ProfileConfig {
            sample_size: 3,
            warmup_runs: 0,
            max_execution_time: Duration::from_secs(10),
        };

        let result =
            ProfileRunner::profile_job(|| async { Err::<(), _>("test error".into()) }, config)
                .await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ProfilingError::ExecutionFailed(_)
        ));
    }

    #[tokio::test]
    async fn test_profile_timeout() {
        let config = ProfileConfig {
            sample_size: 2,
            warmup_runs: 0,
            max_execution_time: Duration::from_millis(50),
        };

        let result = ProfileRunner::profile_job(
            || async {
                tokio::time::sleep(Duration::from_secs(10)).await;
                Ok(())
            },
            config,
        )
        .await;

        assert!(result.is_err());
    }

    #[test]
    fn test_percentile_calculation() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(ProfileRunner::percentile(&data, 50), 5);
        assert_eq!(ProfileRunner::percentile(&data, 95), 10);
        assert_eq!(ProfileRunner::percentile(&data, 99), 10);
    }

    #[test]
    fn test_memory_measurement() {
        let mem = get_current_memory_bytes();
        // Should return a non-zero value on Unix systems
        #[cfg(unix)]
        assert!(mem > 0);
    }

    #[test]
    fn test_compression_single_job() {
        let mut profiles = BlueprintProfiles::new("test");
        profiles.add_job(
            0,
            JobProfile {
                avg_duration_ms: 100,
                p95_duration_ms: 150,
                p99_duration_ms: 200,
                peak_memory_mb: 256,
                stateful: false,
                persistent_connections: false,
                sample_size: 10,
            },
        );

        // Compress
        let compressed = profiles.to_compressed_bytes().unwrap();
        println!("Compressed size (1 job): {} bytes", compressed.len());

        // Verify compression is effective (should be < 250 bytes for 1 job)
        assert!(
            compressed.len() < 250,
            "Compression too large: {} bytes",
            compressed.len()
        );

        // Decompress and verify
        let decompressed = BlueprintProfiles::from_compressed_bytes(&compressed).unwrap();
        assert_eq!(decompressed.blueprint_name, profiles.blueprint_name);
        assert_eq!(decompressed.jobs.len(), 1);
        assert_eq!(decompressed.jobs.get(&0).unwrap().avg_duration_ms, 100);
    }

    #[test]
    fn test_compression_multiple_jobs() {
        let mut profiles = BlueprintProfiles::new("complex-blueprint");

        // Add 10 jobs
        for i in 0..10 {
            profiles.add_job(
                i,
                JobProfile {
                    avg_duration_ms: 100 + i as u64 * 50,
                    p95_duration_ms: 150 + i as u64 * 60,
                    p99_duration_ms: 200 + i as u64 * 70,
                    peak_memory_mb: 256 + i * 64,
                    stateful: i % 5 == 0,
                    persistent_connections: i % 7 == 0,
                    sample_size: 10,
                },
            );
        }

        // Compress
        let compressed = profiles.to_compressed_bytes().unwrap();
        println!("Compressed size (10 jobs): {} bytes", compressed.len());

        // Should be under 700 bytes for 10 jobs
        assert!(
            compressed.len() < 700,
            "Compression too large: {} bytes",
            compressed.len()
        );

        // Decompress and verify
        let decompressed = BlueprintProfiles::from_compressed_bytes(&compressed).unwrap();
        assert_eq!(decompressed.jobs.len(), 10);

        // Verify a few jobs
        assert_eq!(decompressed.jobs.get(&0).unwrap().peak_memory_mb, 256);
        assert_eq!(decompressed.jobs.get(&5).unwrap().stateful, true);
        assert_eq!(
            decompressed.jobs.get(&7).unwrap().persistent_connections,
            true
        );
    }

    #[test]
    fn test_compression_large_blueprint() {
        let mut profiles = BlueprintProfiles::new("massive-blueprint");

        // Add 50 jobs
        for i in 0..50 {
            profiles.add_job(
                i,
                JobProfile {
                    avg_duration_ms: 100 + i as u64 * 20,
                    p95_duration_ms: 150 + i as u64 * 25,
                    p99_duration_ms: 200 + i as u64 * 30,
                    peak_memory_mb: 256 + i * 32,
                    stateful: i % 5 == 0,
                    persistent_connections: i % 7 == 0,
                    sample_size: 10,
                },
            );
        }

        // Compress
        let compressed = profiles.to_compressed_bytes().unwrap();
        println!("Compressed size (50 jobs): {} bytes", compressed.len());

        // Should be under 3KB for 50 jobs
        assert!(
            compressed.len() < 3000,
            "Compression too large: {} bytes",
            compressed.len()
        );

        // Decompress and verify integrity
        let decompressed = BlueprintProfiles::from_compressed_bytes(&compressed).unwrap();
        assert_eq!(decompressed.jobs.len(), 50);
        assert_eq!(decompressed.blueprint_name, "massive-blueprint");
    }

    #[test]
    fn test_compression_roundtrip_preserves_data() {
        let mut profiles = BlueprintProfiles::new("test");
        profiles.add_job(
            42,
            JobProfile {
                avg_duration_ms: 12345,
                p95_duration_ms: 23456,
                p99_duration_ms: 34567,
                peak_memory_mb: 4096,
                stateful: true,
                persistent_connections: true,
                sample_size: 100,
            },
        );

        let compressed = profiles.to_compressed_bytes().unwrap();
        let decompressed = BlueprintProfiles::from_compressed_bytes(&compressed).unwrap();

        let original_job = profiles.jobs.get(&42).unwrap();
        let decompressed_job = decompressed.jobs.get(&42).unwrap();

        assert_eq!(
            original_job.avg_duration_ms,
            decompressed_job.avg_duration_ms
        );
        assert_eq!(
            original_job.p95_duration_ms,
            decompressed_job.p95_duration_ms
        );
        assert_eq!(
            original_job.p99_duration_ms,
            decompressed_job.p99_duration_ms
        );
        assert_eq!(original_job.peak_memory_mb, decompressed_job.peak_memory_mb);
        assert_eq!(original_job.stateful, decompressed_job.stateful);
        assert_eq!(
            original_job.persistent_connections,
            decompressed_job.persistent_connections
        );
        assert_eq!(original_job.sample_size, decompressed_job.sample_size);
    }

    #[test]
    fn test_base64_encoding_for_chain_storage() {
        let mut profiles = BlueprintProfiles::new("incredible-squaring");
        profiles.add_job(
            0,
            JobProfile {
                avg_duration_ms: 5,
                p95_duration_ms: 8,
                p99_duration_ms: 10,
                peak_memory_mb: 256,
                stateful: false,
                persistent_connections: false,
                sample_size: 10,
            },
        );

        // Encode as base64
        let encoded = profiles.to_base64_string().unwrap();
        println!("Base64 encoded size: {} bytes", encoded.len());

        // Should be reasonable size for on-chain storage
        assert!(
            encoded.len() < 400,
            "Base64 size should be < 400 bytes for 1 job"
        );

        // Decode and verify
        let decoded = BlueprintProfiles::from_base64_string(&encoded).unwrap();
        assert_eq!(decoded.blueprint_name, "incredible-squaring");
        assert_eq!(decoded.jobs.len(), 1);

        let job = decoded.jobs.get(&0).unwrap();
        assert_eq!(job.avg_duration_ms, 5);
        assert_eq!(job.peak_memory_mb, 256);
    }

    #[test]
    fn test_base64_encoding_multiple_jobs() {
        let mut profiles = BlueprintProfiles::new("complex-blueprint");

        // Add 10 jobs
        for i in 0..10 {
            profiles.add_job(
                i,
                JobProfile {
                    avg_duration_ms: 100 + i as u64 * 50,
                    p95_duration_ms: 150 + i as u64 * 60,
                    p99_duration_ms: 200 + i as u64 * 70,
                    peak_memory_mb: 256 + i * 64,
                    stateful: i % 5 == 0,
                    persistent_connections: i % 7 == 0,
                    sample_size: 10,
                },
            );
        }

        // Encode as base64
        let encoded = profiles.to_base64_string().unwrap();
        println!("Base64 encoded size (10 jobs): {} bytes", encoded.len());

        // Should still be reasonable for on-chain storage
        assert!(
            encoded.len() < 1000,
            "Base64 size should be < 1KB for 10 jobs"
        );

        // Roundtrip test
        let decoded = BlueprintProfiles::from_base64_string(&encoded).unwrap();
        assert_eq!(decoded.jobs.len(), 10);
        assert_eq!(decoded.jobs.get(&0).unwrap().peak_memory_mb, 256);
        assert_eq!(decoded.jobs.get(&5).unwrap().stateful, true);
    }
}
