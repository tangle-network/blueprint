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
            let user_us = (usage.ru_utime.tv_sec as u64) * 1_000_000 + (usage.ru_utime.tv_usec as u64);
            let sys_us = (usage.ru_stime.tv_sec as u64) * 1_000_000 + (usage.ru_stime.tv_usec as u64);
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

        let mut memories: Vec<u64> = measurements
            .iter()
            .map(|m| m.peak_memory_bytes)
            .collect();
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

        let result = ProfileRunner::profile_job(
            || async {
                Err::<(), _>("test error".into())
            },
            config,
        )
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
}
