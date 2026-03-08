# benchmark

## Purpose
System benchmarking suite that measures CPU, memory, disk I/O, network, GPU, and storage performance. Used by the pricing engine to profile hardware capabilities for cost estimation. Provides both a comprehensive benchmark suite runner and individual benchmark functions.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Defines result types (`BenchmarkProfile`, `MemoryBenchmarkResult`, `NetworkBenchmarkResult`, `GpuBenchmarkResult`, `StorageBenchmarkResult`, `MemoryAccessMode`, `MemoryOperationType`), `BenchmarkRunConfig` with configurable test selection, and entry points `run_benchmark_suite` (runs selected benchmarks and assembles a profile) and `run_benchmark` (monitors an external command's resource usage).
- `cpu.rs` - Multi-threaded CPU benchmark using prime number calculation (sysbench-style). Detects core count, measures CPU usage via sysinfo monitoring thread, reads CPU model/frequency from `/proc/cpuinfo`. Exports `CpuBenchmarkResult` and `run_cpu_benchmark`.
- `gpu.rs` - GPU detection and info gathering via multiple methods: nvidia-smi, rocm-smi, intel_gpu_top, glxinfo, and device file checks. Returns model, memory, and frequency. Exports `run_gpu_benchmark` and vendor-specific default memory constants.
- `io.rs` - Disk I/O benchmark with sequential and random read/write modes. Uses 128MB test files with 4KB blocks, embedded CRC32 checksums for data validation. Measures IOPS, latency (avg/max), and throughput. Exports `IoBenchmarkResult`, `IoTestMode`, and `run_io_benchmark`.
- `memory.rs` - Multi-threaded memory benchmark supporting sequential/random access with read/write/none operations. Monitors process memory usage via sysinfo. Measures operations/second, transfer rate, and latency. Exports `run_memory_benchmark`.
- `network.rs` - Network benchmark measuring download speed (100MB from Cloudflare), upload speed (10MB to httpbin), latency/jitter/packet-loss via ping to 8.8.8.8. Reads `/proc/net/dev` for byte counters. Exports `run_network_benchmark`.
- `types.rs` - Empty file (types defined in mod.rs instead).
- `utils.rs` - Utility functions: `get_io_stats` (reads `/proc/diskstats`), `get_network_stats` (reads `/proc/net/dev`), `run_and_monitor_command` (spawns a process and samples CPU/memory via sysinfo at configurable intervals).

## Key APIs (no snippets)
- `run_benchmark_suite` - Main entry point; runs selected benchmarks and returns a `BenchmarkProfile`.
- `run_cpu_benchmark` / `run_memory_benchmark` / `run_io_benchmark` / `run_network_benchmark` / `run_gpu_benchmark` - Individual benchmark functions.
- `run_and_monitor_command` - Monitors an external process and produces a `BenchmarkProfile` of its resource usage.

## Relationships
- Used by the pricing engine to measure hardware capabilities for cost calculation.
- Depends on `sysinfo` for system metrics, `num_cpus` for core detection, `rand` for random I/O patterns.
- Uses `crate::error::{PricingError, Result}` for error handling.
