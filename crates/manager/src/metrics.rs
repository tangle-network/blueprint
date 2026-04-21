//! Manager lifecycle metrics — Prometheus histograms and counters for every
//! critical path in the blueprint manager.
//!
//! All metrics register on the default prometheus registry, which the QoS
//! Prometheus HTTP server already exposes.  No additional wiring needed.
//!
//! Histogram bucket design follows the principle that each bucket boundary
//! should represent a qualitatively different regime:
//!
//!   - **Sub-second** (0.05–0.5s): local operations, cache hits, health checks
//!   - **Seconds** (1–10s): binary downloads, container pulls, cargo builds
//!   - **Tens of seconds** (30–120s): cloud VM provisioning, cold builds
//!   - **Minutes** (300–600s): full GPU provisioning end-to-end
//!
//! Counters use label cardinality bounded by known enum values (source_kind,
//! result) — never user-supplied strings.

use once_cell::sync::Lazy;
use prometheus::{
    Histogram, HistogramVec, IntCounterVec, IntGauge, register_histogram, register_histogram_vec,
    register_int_counter_vec, register_int_gauge,
};

// ── Bucket definitions ──────────────────────────────────────────────────

/// Fast operations: health checks, metadata resolution, event decode.
const FAST_BUCKETS: &[f64] = &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5];

/// Service startup: fetch + build + spawn + health check.
const STARTUP_BUCKETS: &[f64] = &[0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0, 60.0, 120.0, 300.0];

/// Cloud provisioning: VM spin-up, SSH ready, binary deployed.
const PROVISION_BUCKETS: &[f64] = &[5.0, 15.0, 30.0, 60.0, 120.0, 300.0, 600.0];

/// Block processing: event decode + metadata resolution + service reconcile.
const BLOCK_BUCKETS: &[f64] = &[0.01, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0];

// ── Metrics ─────────────────────────────────────────────────────────────

/// Total manager initialization time (event replay + contract scan + all service starts).
pub static INIT_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        prometheus::histogram_opts!(
            "tangle_manager_init_seconds",
            "Total manager initialization time",
            STARTUP_BUCKETS.to_vec()
        ),
        &["result"]
    )
    .expect("tangle_manager_init_seconds")
});

/// Contract state scan time (enumerating all services for this operator).
pub static CONTRACT_SCAN_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        prometheus::histogram_opts!(
            "tangle_contract_scan_seconds",
            "Time to scan on-chain contract state for active services",
            FAST_BUCKETS.to_vec()
        ),
        &[]
    )
    .expect("tangle_contract_scan_seconds")
});

/// Per-service startup time (from discovery to healthy).
pub static SERVICE_STARTUP_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        prometheus::histogram_opts!(
            "tangle_service_startup_seconds",
            "Time to start a single blueprint service",
            STARTUP_BUCKETS.to_vec()
        ),
        &["blueprint_id", "source_kind", "result"]
    )
    .expect("tangle_service_startup_seconds")
});

/// Per-source-attempt time (fetch + build/pull + spawn + health check).
pub static SOURCE_ATTEMPT_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        prometheus::histogram_opts!(
            "tangle_source_attempt_seconds",
            "Time for a single source launch attempt",
            STARTUP_BUCKETS.to_vec()
        ),
        &["source_kind", "runtime_path", "result"]
    )
    .expect("tangle_source_attempt_seconds")
});

/// Block event processing time.
pub static BLOCK_PROCESSING_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        prometheus::histogram_opts!(
            "tangle_block_processing_seconds",
            "Time to process all events in a single block",
            BLOCK_BUCKETS.to_vec()
        ),
        &[]
    )
    .expect("tangle_block_processing_seconds")
});

/// Service discovery outcomes.
pub static SERVICE_DISCOVERY: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        prometheus::opts!(
            "tangle_service_discovery_total",
            "Service discovery outcomes during contract state scan"
        ),
        &["result"] // "started", "failed", "skipped", "metadata_unavailable"
    )
    .expect("tangle_service_discovery_total")
});

/// Source fallback attempts.
pub static SOURCE_ATTEMPTS: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        prometheus::opts!(
            "tangle_source_attempts_total",
            "Source launch attempts by kind and outcome"
        ),
        &["source_kind", "result"] // result: "success", "failed_health", "failed_spawn"
    )
    .expect("tangle_source_attempts_total")
});

/// Number of currently active services managed by this operator.
pub static ACTIVE_SERVICES: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "tangle_active_services",
        "Number of currently running blueprint services"
    )
    .expect("tangle_active_services")
});

/// Remote cloud provisioning time (GPU/CPU VM spin-up).
pub static REMOTE_PROVISION_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        prometheus::histogram_opts!(
            "tangle_remote_provision_seconds",
            "Cloud VM provisioning time",
            PROVISION_BUCKETS.to_vec()
        ),
        &["provider", "result"]
    )
    .expect("tangle_remote_provision_seconds")
});

// ── Job Execution Metrics ───────────────────────────────────────────

/// Job execution buckets: from fast lookups to LLM inference.
const JOB_BUCKETS: &[f64] = &[0.01, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0, 60.0];

/// Cost buckets in USD: from sub-cent to multi-dollar GPU inference.
const COST_BUCKETS: &[f64] = &[0.0001, 0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0, 10.0];

/// Per-job execution time (from call_id submission to result posted).
pub static JOB_EXECUTION_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        prometheus::histogram_opts!(
            "tangle_job_execution_seconds",
            "End-to-end job execution time",
            JOB_BUCKETS.to_vec()
        ),
        &["blueprint_id", "job_index", "result"]
    )
    .expect("tangle_job_execution_seconds")
});

/// Per-job cost in USD (tokens × rate or compute × rate).
pub static JOB_COST_USD: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        prometheus::histogram_opts!(
            "tangle_job_cost_usd",
            "Estimated cost per job in USD",
            COST_BUCKETS.to_vec()
        ),
        &["blueprint_id", "job_index"]
    )
    .expect("tangle_job_cost_usd")
});

/// Total jobs processed (success + failure).
pub static JOBS_TOTAL: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        prometheus::opts!("tangle_jobs_total", "Total jobs processed by outcome"),
        &["blueprint_id", "job_index", "result"]
    )
    .expect("tangle_jobs_total")
});

/// Cumulative cloud compute cost in USD (running total).
pub static COMPUTE_COST_USD: Lazy<Histogram> = Lazy::new(|| {
    register_histogram!(prometheus::histogram_opts!(
        "tangle_compute_cost_usd",
        "Cumulative cloud compute cost per provisioning event",
        COST_BUCKETS.to_vec()
    ))
    .expect("tangle_compute_cost_usd")
});

#[cfg(test)]
mod tests {
    use super::*;

    /// Find a MetricFamily by name from the default registry.
    fn find_family(name: &str) -> prometheus::proto::MetricFamily {
        prometheus::gather()
            .into_iter()
            .find(|f| f.name() == name)
            .unwrap_or_else(|| panic!("metric family '{name}' not found in registry"))
    }

    /// Find the first Metric whose labels contain `value`.
    fn find_metric_with_label_value<'a>(
        family: &'a prometheus::proto::MetricFamily,
        value: &str,
    ) -> &'a prometheus::proto::Metric {
        family
            .get_metric()
            .iter()
            .find(|m| m.get_label().iter().any(|l| l.value() == value))
            .unwrap_or_else(|| {
                panic!(
                    "no metric with label value '{value}' in family '{}'",
                    family.name()
                )
            })
    }

    // ── 1. Every metric registers on the default registry ──────────────

    #[test]
    fn all_metrics_register_on_default_registry() {
        // Force lazy init AND materialize a child for each Vec metric —
        // `prometheus::gather()` only emits a MetricFamily for a `*Vec` after
        // a labeled child has been observed. Scalar types (IntGauge, bare
        // Histogram) show up on registration alone.
        INIT_DURATION
            .with_label_values(&["register_check"])
            .observe(0.0);
        CONTRACT_SCAN_DURATION
            .with_label_values::<&str>(&[])
            .observe(0.0);
        SERVICE_STARTUP_DURATION
            .with_label_values(&["register_check", "github", "ok"])
            .observe(0.0);
        SOURCE_ATTEMPT_DURATION
            .with_label_values(&["github", "/register/check", "ok"])
            .observe(0.0);
        BLOCK_PROCESSING_DURATION
            .with_label_values::<&str>(&[])
            .observe(0.0);
        SERVICE_DISCOVERY
            .with_label_values(&["register_check"])
            .inc();
        SOURCE_ATTEMPTS
            .with_label_values(&["register_check", "success"])
            .inc();
        let _ = &*ACTIVE_SERVICES;
        REMOTE_PROVISION_DURATION
            .with_label_values(&["register_check", "ok"])
            .observe(0.0);
        JOB_EXECUTION_DURATION
            .with_label_values(&["register_check", "0", "ok"])
            .observe(0.0);
        JOB_COST_USD
            .with_label_values(&["register_check", "0"])
            .observe(0.0);
        JOBS_TOTAL
            .with_label_values(&["register_check", "0", "success"])
            .inc();
        let _ = &*COMPUTE_COST_USD;

        let families = prometheus::gather();
        let names: Vec<&str> = families.iter().map(|f| f.name()).collect();

        let expected = [
            "tangle_manager_init_seconds",
            "tangle_contract_scan_seconds",
            "tangle_service_startup_seconds",
            "tangle_source_attempt_seconds",
            "tangle_block_processing_seconds",
            "tangle_service_discovery_total",
            "tangle_source_attempts_total",
            "tangle_active_services",
            "tangle_remote_provision_seconds",
            "tangle_job_execution_seconds",
            "tangle_job_cost_usd",
            "tangle_jobs_total",
            "tangle_compute_cost_usd",
        ];
        for name in &expected {
            assert!(
                names.contains(name),
                "metric '{name}' missing from default registry; found: {names:?}",
            );
        }
    }

    // ── 2. Bucket definitions are exact ────────────────────────────────

    #[test]
    fn fast_buckets_values() {
        assert_eq!(
            FAST_BUCKETS,
            &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5]
        );
    }

    #[test]
    fn startup_buckets_values() {
        assert_eq!(
            STARTUP_BUCKETS,
            &[0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0, 60.0, 120.0, 300.0]
        );
    }

    #[test]
    fn provision_buckets_values() {
        assert_eq!(
            PROVISION_BUCKETS,
            &[5.0, 15.0, 30.0, 60.0, 120.0, 300.0, 600.0]
        );
    }

    #[test]
    fn block_buckets_values() {
        assert_eq!(
            BLOCK_BUCKETS,
            &[0.01, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
        );
    }

    #[test]
    fn job_buckets_values() {
        assert_eq!(
            JOB_BUCKETS,
            &[0.01, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0, 60.0]
        );
    }

    #[test]
    fn cost_buckets_values() {
        assert_eq!(
            COST_BUCKETS,
            &[0.0001, 0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0, 10.0]
        );
    }

    // ── 3. Histogram bucket counts match definitions in gathered output ─

    #[test]
    fn init_duration_has_startup_bucket_count_in_gathered() {
        INIT_DURATION
            .with_label_values(&["bucket_count_test"])
            .observe(0.001);

        let family = find_family("tangle_manager_init_seconds");
        let metric = find_metric_with_label_value(&family, "bucket_count_test");
        let buckets = metric.get_histogram().get_bucket();
        assert_eq!(buckets.len(), STARTUP_BUCKETS.len());

        for (i, &expected) in STARTUP_BUCKETS.iter().enumerate() {
            assert!(
                (buckets[i].upper_bound() - expected).abs() < 1e-9,
                "bucket {i}: got {}, expected {expected}",
                buckets[i].upper_bound(),
            );
        }
        // Last explicit bucket is the highest defined boundary (not +Inf in protobuf mode).
        assert!(
            (buckets.last().unwrap().upper_bound() - *STARTUP_BUCKETS.last().unwrap()).abs() < 1e-9
        );
    }

    #[test]
    fn block_processing_has_block_bucket_count_in_gathered() {
        BLOCK_PROCESSING_DURATION
            .with_label_values::<&str>(&[])
            .observe(0.001);

        let family = find_family("tangle_block_processing_seconds");
        let metric = &family.get_metric()[0];
        let buckets = metric.get_histogram().get_bucket();
        assert_eq!(buckets.len(), BLOCK_BUCKETS.len());

        for (i, &expected) in BLOCK_BUCKETS.iter().enumerate() {
            assert!(
                (buckets[i].upper_bound() - expected).abs() < 1e-9,
                "block bucket {i}: got {}, expected {expected}",
                buckets[i].upper_bound(),
            );
        }
    }

    #[test]
    fn remote_provision_has_provision_bucket_count_in_gathered() {
        REMOTE_PROVISION_DURATION
            .with_label_values(&["bucket_count_prov", "ok"])
            .observe(1.0);

        let family = find_family("tangle_remote_provision_seconds");
        let metric = find_metric_with_label_value(&family, "bucket_count_prov");
        let buckets = metric.get_histogram().get_bucket();
        assert_eq!(buckets.len(), PROVISION_BUCKETS.len());
    }

    // ── 4. Observations land in correct histogram buckets ──────────────

    #[test]
    fn service_startup_observation_lands_in_correct_bucket() {
        let tag = "bucket_place_bp";
        SERVICE_STARTUP_DURATION
            .with_label_values(&[tag, "github", "success"])
            .observe(2.0);

        let family = find_family("tangle_service_startup_seconds");
        let metric = find_metric_with_label_value(&family, tag);
        let hist = metric.get_histogram();

        assert!(hist.get_sample_count() >= 1);
        assert!(hist.get_sample_sum() >= 2.0);

        let buckets = hist.get_bucket();
        // 2.0s must NOT be in the 1.0s bucket.
        let bucket_1 = buckets
            .iter()
            .find(|b| (b.upper_bound() - 1.0).abs() < 1e-9)
            .unwrap();
        assert_eq!(bucket_1.cumulative_count(), 0);

        // 2.0s MUST be in the 2.5s bucket.
        let bucket_2_5 = buckets
            .iter()
            .find(|b| (b.upper_bound() - 2.5).abs() < 1e-9)
            .unwrap();
        assert!(bucket_2_5.cumulative_count() >= 1);
    }

    #[test]
    fn remote_provision_observation_lands_in_correct_bucket() {
        let tag = "prov_bucket_place";
        REMOTE_PROVISION_DURATION
            .with_label_values(&[tag, "success"])
            .observe(45.0);

        let family = find_family("tangle_remote_provision_seconds");
        let metric = find_metric_with_label_value(&family, tag);
        let hist = metric.get_histogram();

        assert!(hist.get_sample_count() >= 1);
        assert!(hist.get_sample_sum() >= 45.0);

        let buckets = hist.get_bucket();
        // 45.0s NOT in 30.0s bucket.
        let b30 = buckets
            .iter()
            .find(|b| (b.upper_bound() - 30.0).abs() < 1e-9)
            .unwrap();
        assert_eq!(b30.cumulative_count(), 0);

        // 45.0s IS in 60.0s bucket.
        let b60 = buckets
            .iter()
            .find(|b| (b.upper_bound() - 60.0).abs() < 1e-9)
            .unwrap();
        assert!(b60.cumulative_count() >= 1);
    }

    #[test]
    fn contract_scan_fast_bucket_observation() {
        CONTRACT_SCAN_DURATION
            .with_label_values::<&str>(&[])
            .observe(0.03);

        let family = find_family("tangle_contract_scan_seconds");
        let metric = &family.get_metric()[0];
        let hist = metric.get_histogram();
        let buckets = hist.get_bucket();
        assert_eq!(buckets.len(), FAST_BUCKETS.len());

        // 0.03s NOT in 0.025s bucket.
        let b025 = buckets
            .iter()
            .find(|b| (b.upper_bound() - 0.025).abs() < 1e-9)
            .unwrap();
        assert_eq!(b025.cumulative_count(), 0);

        // 0.03s IS in 0.05s bucket.
        let b05 = buckets
            .iter()
            .find(|b| (b.upper_bound() - 0.05).abs() < 1e-9)
            .unwrap();
        assert!(b05.cumulative_count() >= 1);
    }

    #[test]
    fn source_attempt_duration_records_in_correct_bucket() {
        let kind = "src_attempt_bucket";
        SOURCE_ATTEMPT_DURATION
            .with_label_values(&[kind, "/test/bin", "success"])
            .observe(7.5);

        let family = find_family("tangle_source_attempt_seconds");
        let metric = find_metric_with_label_value(&family, kind);
        let hist = metric.get_histogram();

        assert!(hist.get_sample_count() >= 1);
        assert!(hist.get_sample_sum() >= 7.5);

        let buckets = hist.get_bucket();
        // 7.5s NOT in 5.0s bucket.
        let b5 = buckets
            .iter()
            .find(|b| (b.upper_bound() - 5.0).abs() < 1e-9)
            .unwrap();
        assert_eq!(b5.cumulative_count(), 0);

        // 7.5s IS in 10.0s bucket.
        let b10 = buckets
            .iter()
            .find(|b| (b.upper_bound() - 10.0).abs() < 1e-9)
            .unwrap();
        assert!(b10.cumulative_count() >= 1);
    }

    #[test]
    fn job_execution_duration_lands_in_correct_bucket() {
        let tag = "job_exec_bucket";
        JOB_EXECUTION_DURATION
            .with_label_values(&[tag, "0", "success"])
            .observe(3.0);

        let family = find_family("tangle_job_execution_seconds");
        let metric = find_metric_with_label_value(&family, tag);
        let hist = metric.get_histogram();

        assert!(hist.get_sample_count() >= 1);
        assert!(hist.get_sample_sum() >= 3.0);

        let buckets = hist.get_bucket();
        // 3.0s NOT in 2.5s bucket.
        let b2_5 = buckets
            .iter()
            .find(|b| (b.upper_bound() - 2.5).abs() < 1e-9)
            .unwrap();
        assert_eq!(b2_5.cumulative_count(), 0);

        // 3.0s IS in 5.0s bucket.
        let b5 = buckets
            .iter()
            .find(|b| (b.upper_bound() - 5.0).abs() < 1e-9)
            .unwrap();
        assert!(b5.cumulative_count() >= 1);
    }

    #[test]
    fn job_cost_usd_lands_in_correct_bucket() {
        let tag = "job_cost_bucket";
        JOB_COST_USD.with_label_values(&[tag, "1"]).observe(0.07);

        let family = find_family("tangle_job_cost_usd");
        let metric = find_metric_with_label_value(&family, tag);
        let hist = metric.get_histogram();

        assert!(hist.get_sample_count() >= 1);
        assert!(hist.get_sample_sum() >= 0.07);

        let buckets = hist.get_bucket();
        // $0.07 NOT in $0.05 bucket.
        let b005 = buckets
            .iter()
            .find(|b| (b.upper_bound() - 0.05).abs() < 1e-9)
            .unwrap();
        assert_eq!(b005.cumulative_count(), 0);

        // $0.07 IS in $0.1 bucket.
        let b01 = buckets
            .iter()
            .find(|b| (b.upper_bound() - 0.1).abs() < 1e-9)
            .unwrap();
        assert!(b01.cumulative_count() >= 1);
    }

    #[test]
    fn compute_cost_usd_lands_in_correct_bucket() {
        COMPUTE_COST_USD.observe(0.3);

        let family = find_family("tangle_compute_cost_usd");
        let metric = &family.get_metric()[0];
        let hist = metric.get_histogram();

        assert!(hist.get_sample_count() >= 1);
        assert!(hist.get_sample_sum() >= 0.3);

        let buckets = hist.get_bucket();
        // $0.3 NOT in $0.1 bucket.
        let b01 = buckets
            .iter()
            .find(|b| (b.upper_bound() - 0.1).abs() < 1e-9)
            .unwrap();
        assert_eq!(b01.cumulative_count(), 0);

        // $0.3 IS in $0.5 bucket.
        let b05 = buckets
            .iter()
            .find(|b| (b.upper_bound() - 0.5).abs() < 1e-9)
            .unwrap();
        assert!(b05.cumulative_count() >= 1);
    }

    // ── 5. Label cardinality is bounded -- exact label names ───────────

    #[test]
    fn label_names_are_exact_for_every_metric() {
        // Observe with unique tags to create metric entries.
        INIT_DURATION
            .with_label_values(&["label_names_test"])
            .observe(0.1);
        SERVICE_STARTUP_DURATION
            .with_label_values(&["label_names_bp", "docker", "ok"])
            .observe(0.1);
        SOURCE_ATTEMPT_DURATION
            .with_label_values(&["docker", "/label/test", "ok"])
            .observe(0.1);
        SERVICE_DISCOVERY
            .with_label_values(&["label_names_started"])
            .inc();
        SOURCE_ATTEMPTS
            .with_label_values(&["label_names_docker", "success"])
            .inc();
        REMOTE_PROVISION_DURATION
            .with_label_values(&["label_names_aws", "ok"])
            .observe(0.1);
        JOB_EXECUTION_DURATION
            .with_label_values(&["label_names_bp2", "0", "ok"])
            .observe(0.1);
        JOB_COST_USD
            .with_label_values(&["label_names_bp3", "1"])
            .observe(0.01);
        JOBS_TOTAL
            .with_label_values(&["label_names_bp4", "0", "success"])
            .inc();

        let families = prometheus::gather();

        let assert_labels = |metric_name: &str, expected: &[&str]| {
            let fam = families
                .iter()
                .find(|f| f.name() == metric_name)
                .unwrap_or_else(|| panic!("missing {metric_name}"));
            if expected.is_empty() {
                let m = &fam.get_metric()[0];
                assert!(
                    m.get_label().is_empty(),
                    "{metric_name} should have zero labels, got {:?}",
                    m.get_label()
                );
            } else {
                let m = fam
                    .get_metric()
                    .iter()
                    .find(|m| !m.get_label().is_empty())
                    .unwrap_or_else(|| panic!("{metric_name}: no labeled metrics found"));
                let mut names: Vec<&str> = m.get_label().iter().map(|l| l.name()).collect();
                names.sort();
                let mut sorted_expected: Vec<&str> = expected.to_vec();
                sorted_expected.sort();
                assert_eq!(
                    names, sorted_expected,
                    "label names mismatch for {metric_name}"
                );
            }
        };

        assert_labels("tangle_manager_init_seconds", &["result"]);
        assert_labels("tangle_contract_scan_seconds", &[]);
        assert_labels(
            "tangle_service_startup_seconds",
            &["blueprint_id", "source_kind", "result"],
        );
        assert_labels(
            "tangle_source_attempt_seconds",
            &["source_kind", "runtime_path", "result"],
        );
        assert_labels("tangle_block_processing_seconds", &[]);
        assert_labels("tangle_service_discovery_total", &["result"]);
        assert_labels("tangle_source_attempts_total", &["source_kind", "result"]);
        assert_labels("tangle_active_services", &[]);
        assert_labels("tangle_remote_provision_seconds", &["provider", "result"]);
        assert_labels(
            "tangle_job_execution_seconds",
            &["blueprint_id", "job_index", "result"],
        );
        assert_labels("tangle_job_cost_usd", &["blueprint_id", "job_index"]);
        assert_labels(
            "tangle_jobs_total",
            &["blueprint_id", "job_index", "result"],
        );
        assert_labels("tangle_compute_cost_usd", &[]);
    }

    // ── 6. Counter increments reflected in gathered output ─────────────

    #[test]
    fn service_discovery_counter_increments() {
        let tag = "counter_inc_test";
        SERVICE_DISCOVERY.with_label_values(&[tag]).inc();
        SERVICE_DISCOVERY.with_label_values(&[tag]).inc();
        SERVICE_DISCOVERY.with_label_values(&[tag]).inc();

        let family = find_family("tangle_service_discovery_total");
        let metric = find_metric_with_label_value(&family, tag);
        let count = metric.get_counter().value() as u64;
        assert!(count >= 3, "expected >= 3 for '{tag}', got {count}");
    }

    #[test]
    fn source_attempts_counter_tracks_separate_outcomes() {
        let kind = "counter_outcome_test";
        SOURCE_ATTEMPTS.with_label_values(&[kind, "success"]).inc();
        SOURCE_ATTEMPTS
            .with_label_values(&[kind, "failed_spawn"])
            .inc();
        SOURCE_ATTEMPTS
            .with_label_values(&[kind, "failed_spawn"])
            .inc();

        let family = find_family("tangle_source_attempts_total");

        let success = family
            .get_metric()
            .iter()
            .find(|m| {
                m.get_label().iter().any(|l| l.value() == kind)
                    && m.get_label().iter().any(|l| l.value() == "success")
            })
            .expect("success metric not found");
        assert!(success.get_counter().value() >= 1.0);

        let fail = family
            .get_metric()
            .iter()
            .find(|m| {
                m.get_label().iter().any(|l| l.value() == kind)
                    && m.get_label().iter().any(|l| l.value() == "failed_spawn")
            })
            .expect("failed_spawn metric not found");
        assert!(fail.get_counter().value() >= 2.0);
    }

    #[test]
    fn jobs_total_counter_increments() {
        let tag = "jobs_counter_test";
        JOBS_TOTAL.with_label_values(&[tag, "0", "success"]).inc();
        JOBS_TOTAL.with_label_values(&[tag, "0", "success"]).inc();
        JOBS_TOTAL.with_label_values(&[tag, "0", "failed"]).inc();

        let family = find_family("tangle_jobs_total");

        let success = family
            .get_metric()
            .iter()
            .find(|m| {
                m.get_label().iter().any(|l| l.value() == tag)
                    && m.get_label().iter().any(|l| l.value() == "success")
            })
            .expect("jobs success metric");
        assert!(success.get_counter().value() >= 2.0);

        let failed = family
            .get_metric()
            .iter()
            .find(|m| {
                m.get_label().iter().any(|l| l.value() == tag)
                    && m.get_label().iter().any(|l| l.value() == "failed")
            })
            .expect("jobs failed metric");
        assert!(failed.get_counter().value() >= 1.0);
    }

    // ── 7. IntGauge set / inc / dec ────────────────────────────────────

    #[test]
    fn active_services_gauge_reflects_mutations() {
        ACTIVE_SERVICES.set(10);
        assert_eq!(ACTIVE_SERVICES.get(), 10);

        ACTIVE_SERVICES.inc();
        assert_eq!(ACTIVE_SERVICES.get(), 11);

        ACTIVE_SERVICES.dec();
        ACTIVE_SERVICES.dec();
        assert_eq!(ACTIVE_SERVICES.get(), 9);

        // Verify through gathered output.
        let family = find_family("tangle_active_services");
        let metric = &family.get_metric()[0];
        let val = metric.get_gauge().value() as i64;
        assert_eq!(val, 9, "gathered gauge value should be 9");
    }
}
