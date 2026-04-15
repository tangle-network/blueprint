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
