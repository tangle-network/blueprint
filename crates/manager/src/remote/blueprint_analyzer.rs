//! Blueprint analysis for deployment strategy selection.
//!
//! This module provides pure functions to analyze blueprint metadata and
//! recommend optimal deployment strategies (serverless, hybrid, traditional).

use serde::{Deserialize, Serialize};

/// Deployment strategy recommendation based on blueprint analysis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeploymentStrategy {
    /// Pure serverless: all jobs can run on `FaaS`
    Serverless {
        /// All jobs that will be deployed to `FaaS`
        job_ids: Vec<u32>,
    },
    /// Hybrid: some jobs on `FaaS`, some local/VM
    Hybrid {
        /// Jobs that will run on `FaaS`
        faas_jobs: Vec<u32>,
        /// Jobs that will run locally/VM
        local_jobs: Vec<u32>,
    },
    /// Traditional VM or Kubernetes deployment
    Traditional {
        /// All jobs run locally
        job_ids: Vec<u32>,
    },
}

/// `FaaS` compatibility limits (provider-specific).
#[derive(Debug, Clone)]
pub struct FaasLimits {
    /// Maximum memory in MB
    pub max_memory_mb: u32,
    /// Maximum timeout in seconds
    pub max_timeout_secs: u32,
    /// Maximum payload size in MB
    pub max_payload_mb: u32,
}

impl FaasLimits {
    /// AWS Lambda limits
    #[must_use]
    pub fn aws_lambda() -> Self {
        Self {
            max_memory_mb: 10240,  // 10 GB
            max_timeout_secs: 900, // 15 minutes
            max_payload_mb: 6,     // 6 MB
        }
    }

    /// GCP Cloud Functions limits
    #[must_use]
    pub fn gcp_functions() -> Self {
        Self {
            max_memory_mb: 32768,   // 32 GB
            max_timeout_secs: 3600, // 60 minutes
            max_payload_mb: 10,     // 10 MB
        }
    }

    /// Azure Functions limits
    #[must_use]
    pub fn azure_functions() -> Self {
        Self {
            max_memory_mb: 14336,  // 14 GB
            max_timeout_secs: 600, // 10 minutes (consumption plan)
            max_payload_mb: 100,   // 100 MB
        }
    }

    /// `DigitalOcean` Functions limits
    #[must_use]
    pub fn digitalocean_functions() -> Self {
        Self {
            max_memory_mb: 8192,   // 8 GB (configurable: 128MB-8GB)
            max_timeout_secs: 900, // 15 minutes (configurable: 1-900s)
            max_payload_mb: 8,     // 8 MB (estimated)
        }
    }

    /// Custom `FaaS` (conservative defaults)
    #[must_use]
    pub fn custom() -> Self {
        Self {
            max_memory_mb: 2048,   // 2 GB
            max_timeout_secs: 300, // 5 minutes
            max_payload_mb: 5,     // 5 MB
        }
    }
}

/// Job analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobAnalysis {
    pub job_id: u32,
    pub faas_compatible: bool,
    pub reason: Option<String>,
}

/// Resource sizing recommendation for VM/K8s deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSizing {
    /// Recommended CPU cores
    pub cpu_cores: f32,
    /// Recommended memory in MB
    pub memory_mb: u32,
    /// Reasoning for the recommendation
    pub reasoning: String,
}

impl ResourceSizing {
    /// Calculate recommended sizing from job profiles
    #[must_use]
    pub fn from_profiles(profiles: &[Option<super::blueprint_fetcher::JobProfile>]) -> Self {
        let mut max_memory_mb = 512; // Minimum baseline
        let mut has_data = false;

        for profile in profiles.iter().flatten() {
            has_data = true;
            // Add 50% headroom for safety using integer arithmetic
            let job_memory = profile.peak_memory_mb.saturating_mul(3) / 2;
            max_memory_mb = max_memory_mb.max(job_memory);
        }

        // Estimate CPU based on memory (heuristic: 1 core per 2GB memory)
        let cpu_cores = (max_memory_mb as f32 / 2048.0).max(1.0).ceil();

        let reasoning = if has_data {
            format!(
                "Based on profiling data: {}MB peak memory with 50% headroom, {} CPU cores estimated",
                max_memory_mb, cpu_cores
            )
        } else {
            "No profiling data - using conservative defaults (1 CPU, 512MB)".to_string()
        };

        Self {
            cpu_cores,
            memory_mb: max_memory_mb,
            reasoning,
        }
    }
}

/// Blueprint analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintAnalysis {
    pub total_jobs: usize,
    pub faas_compatible_jobs: Vec<JobAnalysis>,
    pub incompatible_jobs: Vec<JobAnalysis>,
    pub recommended_strategy: DeploymentStrategy,
    /// Resource sizing for VM/K8s deployment (if needed)
    pub resource_sizing: ResourceSizing,
}

/// Analyzes a blueprint and recommends deployment strategy.
///
/// This is a pure function - no I/O, easy to test.
///
/// If job profiles are available (from `cargo tangle blueprint profile`),
/// uses actual benchmarking data. Otherwise, falls back to heuristics.
#[must_use]
pub fn analyze_blueprint(
    job_count: u32,
    job_profiles: &[Option<super::blueprint_fetcher::JobProfile>],
    faas_limits: &FaasLimits,
    serverless_enabled: bool,
) -> BlueprintAnalysis {
    let job_ids: Vec<u32> = (0..job_count).collect();

    // Analyze each job using profiles if available
    let mut faas_compatible: Vec<JobAnalysis> = vec![];
    let mut incompatible: Vec<JobAnalysis> = vec![];

    for (job_id, profile_opt) in job_ids.iter().zip(job_profiles.iter()) {
        let analysis = if let Some(profile) = profile_opt {
            // Use actual profiling data!
            analyze_job_with_profile(*job_id, profile, faas_limits)
        } else {
            // No profile: CONSERVATIVE DEFAULT - assume NOT FaaS-compatible
            // This prevents untested jobs from being deployed to FaaS in production.
            // Developer must run `cargo tangle blueprint profile` to generate profiles.
            JobAnalysis {
                job_id: *job_id,
                faas_compatible: false,
                reason: Some(
                    "No profiling data - run `cargo tangle blueprint profile` to analyze job"
                        .to_string(),
                ),
            }
        };

        if analysis.faas_compatible {
            faas_compatible.push(analysis);
        } else {
            incompatible.push(analysis);
        }
    }

    let recommended_strategy = if serverless_enabled && !faas_compatible.is_empty() {
        if incompatible.is_empty() {
            // All jobs compatible → pure serverless
            DeploymentStrategy::Serverless {
                job_ids: job_ids.clone(),
            }
        } else {
            // Mixed compatibility → hybrid
            DeploymentStrategy::Hybrid {
                faas_jobs: faas_compatible.iter().map(|j| j.job_id).collect(),
                local_jobs: incompatible.iter().map(|j| j.job_id).collect(),
            }
        }
    } else {
        // Serverless disabled or no compatible jobs → traditional
        DeploymentStrategy::Traditional {
            job_ids: job_ids.clone(),
        }
    };

    // Calculate resource sizing for VM/K8s deployment
    let resource_sizing = ResourceSizing::from_profiles(job_profiles);

    BlueprintAnalysis {
        total_jobs: job_ids.len(),
        faas_compatible_jobs: faas_compatible,
        incompatible_jobs: incompatible,
        recommended_strategy,
        resource_sizing,
    }
}

/// Analyze a job using its profiling data.
fn analyze_job_with_profile(
    job_id: u32,
    profile: &super::blueprint_fetcher::JobProfile,
    limits: &FaasLimits,
) -> JobAnalysis {
    // Check each compatibility criterion
    let mut incompatible_reasons = vec![];

    // 1. Execution time
    if profile.p95_duration_ms > (u64::from(limits.max_timeout_secs) * 1000) {
        incompatible_reasons.push(format!(
            "p95 duration {}ms exceeds FaaS timeout {}s",
            profile.p95_duration_ms, limits.max_timeout_secs
        ));
    }

    // 2. Memory usage
    if profile.peak_memory_mb > limits.max_memory_mb {
        incompatible_reasons.push(format!(
            "peak memory {}MB exceeds FaaS limit {}MB",
            profile.peak_memory_mb, limits.max_memory_mb
        ));
    }

    // 3. Stateful jobs can't use FaaS
    if profile.stateful {
        incompatible_reasons.push("job is stateful (requires persistent state)".to_string());
    }

    // 4. Persistent connections can't use FaaS
    if profile.persistent_connections {
        incompatible_reasons
            .push("job maintains persistent connections (websockets, long-lived TCP)".to_string());
    }

    if incompatible_reasons.is_empty() {
        JobAnalysis {
            job_id,
            faas_compatible: true,
            reason: Some(format!(
                "Compatible: {}ms avg, {}MB peak",
                profile.avg_duration_ms, profile.peak_memory_mb
            )),
        }
    } else {
        JobAnalysis {
            job_id,
            faas_compatible: false,
            reason: Some(incompatible_reasons.join("; ")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_jobs_faas_compatible_with_profiles() {
        use super::super::blueprint_fetcher::JobProfile;

        let limits = FaasLimits::aws_lambda();
        let profiles = vec![
            Some(JobProfile {
                avg_duration_ms: 100,
                peak_memory_mb: 256,
                p95_duration_ms: 200,
                stateful: false,
                persistent_connections: false,
            }),
            Some(JobProfile {
                avg_duration_ms: 50,
                peak_memory_mb: 128,
                p95_duration_ms: 100,
                stateful: false,
                persistent_connections: false,
            }),
        ];

        let analysis = analyze_blueprint(2, &profiles, &limits, true);

        assert_eq!(analysis.total_jobs, 2);
        assert_eq!(analysis.faas_compatible_jobs.len(), 2);
        assert_eq!(analysis.incompatible_jobs.len(), 0);

        match analysis.recommended_strategy {
            DeploymentStrategy::Serverless { job_ids } => {
                assert_eq!(job_ids, vec![0, 1]);
            }
            _ => panic!("Expected Serverless strategy"),
        }
    }

    #[test]
    fn test_hybrid_deployment_with_profiles() {
        use super::super::blueprint_fetcher::JobProfile;

        let limits = FaasLimits::aws_lambda();
        let profiles = vec![
            // Job 0: Fast and compatible
            Some(JobProfile {
                avg_duration_ms: 100,
                peak_memory_mb: 256,
                p95_duration_ms: 200,
                stateful: false,
                persistent_connections: false,
            }),
            // Job 1: Too slow for Lambda (but would work on GCP)
            Some(JobProfile {
                avg_duration_ms: 20 * 60 * 1000, // 20 minutes
                peak_memory_mb: 512,
                p95_duration_ms: 25 * 60 * 1000,
                stateful: false,
                persistent_connections: false,
            }),
            // Job 2: Stateful - can't use FaaS
            Some(JobProfile {
                avg_duration_ms: 100,
                peak_memory_mb: 128,
                p95_duration_ms: 200,
                stateful: true,
                persistent_connections: false,
            }),
        ];

        let analysis = analyze_blueprint(3, &profiles, &limits, true);

        assert_eq!(analysis.total_jobs, 3);
        assert_eq!(analysis.faas_compatible_jobs.len(), 1); // Only job 0
        assert_eq!(analysis.incompatible_jobs.len(), 2); // Jobs 1 and 2

        match analysis.recommended_strategy {
            DeploymentStrategy::Hybrid {
                faas_jobs,
                local_jobs,
            } => {
                assert_eq!(faas_jobs, vec![0]);
                assert_eq!(local_jobs, vec![1, 2]);
            }
            _ => panic!("Expected Hybrid strategy"),
        }
    }

    #[test]
    fn test_serverless_disabled() {
        let limits = FaasLimits::aws_lambda();
        let profiles = vec![None, None, None];
        let analysis = analyze_blueprint(3, &profiles, &limits, false);

        match analysis.recommended_strategy {
            DeploymentStrategy::Traditional { job_ids } => {
                assert_eq!(job_ids, vec![0, 1, 2]);
            }
            _ => panic!("Expected Traditional strategy"),
        }
    }

    #[test]
    fn test_no_profiles_conservative_default() {
        let limits = FaasLimits::aws_lambda();
        let profiles = vec![None, None]; // No profiling data
        let analysis = analyze_blueprint(2, &profiles, &limits, true);

        // CONSERVATIVE DEFAULT: Without profiles, assume NOT FaaS-compatible
        // This prevents untested code from running in production serverless
        assert_eq!(analysis.faas_compatible_jobs.len(), 0);
        assert_eq!(analysis.incompatible_jobs.len(), 2);

        // Should recommend traditional deployment since no jobs are profiled
        match analysis.recommended_strategy {
            DeploymentStrategy::Traditional { job_ids } => {
                assert_eq!(job_ids, vec![0, 1]);
            }
            _ => panic!(
                "Expected Traditional strategy when no profiles available (conservative default)"
            ),
        }

        // Verify reason includes guidance to run profiling
        assert!(
            analysis.incompatible_jobs[0]
                .reason
                .as_ref()
                .unwrap()
                .contains("cargo tangle blueprint profile")
        );
    }
}
