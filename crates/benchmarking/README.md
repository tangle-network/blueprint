# blueprint-benchmarking

Benchmark harness primitives for Blueprint jobs.

## When to use

- You want lightweight runtime benchmarking around async job execution.
- You need consistent benchmark summaries (duration, vCPU count, RAM usage).

## Key API surface

- `Runtime` trait + `TokioRuntime` adapter.
- `Bencher<R>` for starting/stopping benchmark runs.
- `BenchmarkSummary` with display formatting for logs/CI output.

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/benchmarking
