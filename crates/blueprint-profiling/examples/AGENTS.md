# examples

## Purpose
Runnable examples demonstrating how to use the blueprint-profiling crate to measure job execution time, memory usage, and assess FaaS platform compatibility.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `basic_usage.rs` - Profiles three job types (fast computation, slow sleep, memory-intensive allocation) and performs FaaS compatibility analysis against AWS Lambda and GCP Cloud Functions limits.
- `memory_profiling.rs` - Demonstrates peak memory tracking by profiling a job with randomized allocation sizes using a seeded RNG for reproducibility.
- `simple_profiling.rs` - Minimal example profiling a simple square-sum computation, showing how to configure sample size and warmup runs.

## Key APIs (no snippets)
- All examples use `ProfileRunner::profile_job(job_fn, config)` to collect `JobProfile` statistics.
- `ProfileConfig` is used to set `sample_size`, `warmup_runs`, and `max_execution_time`.
- `basic_usage.rs` defines `analyze_faas_compatibility()` comparing `JobProfile` metrics against cloud provider limits.

## Relationships
- Depends on `blueprint-profiling` crate for `ProfileRunner`, `ProfileConfig`, and `JobProfile`.
- `memory_profiling.rs` additionally depends on `rand` for randomized workload generation.
