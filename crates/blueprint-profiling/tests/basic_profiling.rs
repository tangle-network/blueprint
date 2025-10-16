//! Basic profiling tests verifying core functionality.

use blueprint_profiling::{ProfileConfig, ProfileRunner};

#[tokio::test]
async fn profile_fast_job() {
    let config = ProfileConfig {
        sample_size: 5,
        warmup_runs: 1,
        ..Default::default()
    };

    let profile = ProfileRunner::profile_job(
        || async {
            // Perform a lightweight computation
            let mut total = 0u64;
            for value in 0..1_000 {
                total = total.wrapping_add(value * value);
            }
            if total == 0 {
                Err("unexpected zero result".into())
            } else {
                Ok(())
            }
        },
        config,
    )
    .await
    .expect("profiling should succeed");

    assert_eq!(profile.sample_size, 5);
    assert!(profile.avg_duration_ms <= profile.p95_duration_ms);
    assert!(profile.p95_duration_ms <= profile.p99_duration_ms);
    assert!(
        profile.peak_memory_mb <= 64,
        "peak memory unexpectedly high: {} MB",
        profile.peak_memory_mb
    );
    assert!(!profile.stateful);
    assert!(!profile.persistent_connections);
}

#[tokio::test]
async fn profile_slow_job_detects_delay() {
    use std::time::Duration;

    let config = ProfileConfig {
        sample_size: 3,
        warmup_runs: 0,
        max_execution_time: Duration::from_secs(2),
    };

    let profile = ProfileRunner::profile_job(
        || async {
            tokio::time::sleep(Duration::from_millis(40)).await;
            Ok(())
        },
        config,
    )
    .await
    .expect("profiling should succeed");

    assert!(
        profile.avg_duration_ms >= 40,
        "expected >= 40ms, got {}ms",
        profile.avg_duration_ms
    );
}
