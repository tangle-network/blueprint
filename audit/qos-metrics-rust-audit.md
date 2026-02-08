# Security Audit Report: QoS Metrics Pipeline (Rust)

**Scope**: ABI encoding module, MetricsSource bridge, on-chain metric methods, Enhanced/Default/Remote provider implementations, heartbeat encoding
**Date**: 2026-02-08
**Auditor**: Claude Opus 4.6
**Repository**: blueprint-sdk

---

## FINDING 1: Critical Data Loss -- TOCTOU in `get_on_chain_metrics` Drain Pattern

**Severity**: HIGH
**Files**:
- `crates/qos/src/metrics/provider/enhanced.rs` lines 426-429
- `crates/qos/src/metrics/provider/default.rs` lines 177-183
- `crates/qos/src/heartbeat.rs` lines 206-215

**Description**: `get_on_chain_metrics()` uses `HashMap::drain()` -- a destructive read that empties the map. If the heartbeat transaction subsequently **fails**, the drained metrics are permanently lost. The error path only logs a warning with no re-enqueue.

Additionally, `DefaultMetricsProvider` uses `try_write()` on the drain: if the lock is contended, it silently returns an empty vector.

---

## FINDING 2: Silent Metric Drops in `DefaultMetricsProvider` via `try_write`/`try_read`

**Severity**: HIGH
**File**: `crates/qos/src/metrics/provider/default.rs` lines 171-183

`add_on_chain_metric` uses `try_write()` on a `tokio::sync::RwLock`. If contended, the metric is **silently discarded**. This is an `async` function -- there is no reason to use `try_write()` instead of `.write().await`. The error messages reference "lock poisoned" which is a `std::sync` concept; `tokio::sync::RwLock` does not poison.

---

## FINDING 3: Integer Truncation -- `status_code` u32 to u8 Cast

**Severity**: MEDIUM
**File**: `crates/qos/src/heartbeat.rs` line 237

`status_code` is `u32` in `HeartbeatStatus` but cast to `u8` with `as u8`, silently truncating the upper 24 bits. Status code 256 becomes 0.

---

## FINDING 4: Potential Overflow in Heartbeat Jitter Calculation

**Severity**: MEDIUM
**File**: `crates/qos/src/heartbeat.rs` lines 366-368

`interval_secs * 1000` could panic in debug mode or wrap in release mode for extreme values. `initial_interval_ms * jitter_percent` could similarly overflow.

---

## FINDING 5: Signature Scheme Divergence Between CLI and QoS Heartbeat

**Severity**: HIGH
**File**: `crates/qos/src/heartbeat.rs` lines 206-221 vs `cli/src/command/operator.rs` lines 92-101

The CLI signs `block_number || timestamp || service_id || blueprint_id || status_code` (36 bytes). The QoS heartbeat signs `service_id || blueprint_id || metrics_bytes`. These are incompatible signature formats going to the same contract.

---

## FINDING 6: `unwrap()` Panics in `RemoteMetricsProvider`

**Severity**: MEDIUM
**File**: `crates/qos/src/remote.rs` lines 173, 200, 219

Three `unwrap()` calls on `SystemTime::now().duration_since(UNIX_EPOCH)`. Other files use `.unwrap_or_default()`.

---

## FINDING 7: Metric Value Overwrites -- HashMap::insert Semantics

**Severity**: MEDIUM
**Files**:
- `crates/qos/src/metrics/provider/enhanced.rs` lines 421-424
- `crates/qos/src/metrics/provider/default.rs` lines 171-174

`add_on_chain_metric` uses `HashMap::insert` which overwrites. For counter-style metrics, intermediate values are silently lost. The API name `add_` is misleading.

---

## FINDING 8: ABI Encoding Correctness

**Severity**: LOW
**File**: `crates/qos/src/metrics/abi.rs` lines 1-21

The encoding is correct via `alloy-sol-types`. The `u64` to `U256` conversion is safe. Minor documentation note: `abi.decode(data, (MetricPair[]))` vs `abi.encode(MetricPair[])` may differ in tuple wrapping depending on exact Solidity decoder call.

---

## FINDING 9: Missing Gas Estimation and Transaction Parameters

**Severity**: MEDIUM
**File**: `crates/qos/src/heartbeat.rs` lines 244-246

Transaction request has no explicit gas limit, gas price, nonce, or chain ID. Provider auto-fill may overpay, cause nonce races, or create replay risk.

---

## FINDING 10: Unbounded Metric Key Length

**Severity**: LOW
**File**: `crates/qos/src/metrics/abi.rs` line 12

No length validation on metric keys or count of metric pairs. Large payloads could exceed block gas limit.

---

## FINDING 11: Drop Handler May Fail to Abort Heartbeat Task

**Severity**: LOW
**File**: `crates/qos/src/heartbeat.rs` lines 438-446

`Drop` uses `try_lock()` -- if contended, the background task is **never aborted**.

---

## FINDING 12: Keystore Access on Every Heartbeat

**Severity**: LOW
**File**: `crates/qos/src/heartbeat.rs` lines 192-205

Keystore is re-opened and signing key loaded from disk on every heartbeat cycle. A filesystem failure during heartbeat loses the drained metrics (Finding 1).

---

## FINDING 13: Supply Chain Assessment

**Severity**: INFORMATIONAL

The alloy crate ecosystem is well-maintained. No supply chain concerns identified.

---

## FINDING 14: Timing Side-Channel in Heartbeat Interval

**Severity**: INFORMATIONAL

Jitter is additive only (300-330s with defaults). Heartbeat timing is observable on-chain. `rand::thread_rng()` is appropriate.

---

## FINDING 15: Dual `start_collection` Implementations

**Severity**: LOW
**File**: `crates/qos/src/metrics/provider/enhanced.rs` lines 144-207 and 461-524

Both inherent and trait methods start a Prometheus server. If both called, port conflict and doubled collection loops.

---

## Summary Table

| # | Finding | Severity | Type |
|---|---------|----------|------|
| 1 | TOCTOU: Drained metrics lost on tx failure | HIGH | Data Loss |
| 2 | Silent metric drops via `try_write` in DefaultProvider | HIGH | Data Loss |
| 3 | `status_code` u32-to-u8 truncation | MEDIUM | Integer Truncation |
| 4 | Potential overflow in jitter calculation | MEDIUM | Integer Overflow |
| 5 | Signature payload divergence between CLI and QoS | HIGH | Protocol Inconsistency |
| 6 | `unwrap()` panics in RemoteMetricsProvider | MEDIUM | Panic/Crash |
| 7 | HashMap::insert overwrites -- misleading `add_` semantics | MEDIUM | Data Loss |
| 8 | ABI encoding format / documentation mismatch | LOW | Correctness |
| 9 | Missing gas/nonce/chain parameters in TX | MEDIUM | Reliability |
| 10 | Unbounded metric key/count -- gas DoS | LOW | DoS |
| 11 | Drop handler may fail to abort heartbeat task | LOW | Resource Leak |
| 12 | Keystore re-opened on every heartbeat | LOW | Performance |
| 13 | Supply chain assessment | INFO | Supply Chain |
| 14 | Timing side-channel in heartbeat jitter | INFO | Privacy |
| 15 | Dual `start_collection` implementations | LOW | Logic Error |

---

## Recommendations (Priority Order)

1. **Finding 1 (HIGH)**: Implement two-phase commit: peek metrics before drain, drain only after transaction confirmation, or re-enqueue on failure.
2. **Finding 5 (HIGH)**: Reconcile signature payload format between CLI and QoS heartbeat service.
3. **Finding 2 (HIGH)**: Replace `try_write()`/`try_read()` with `.write().await`/`.read().await` in DefaultMetricsProvider.
4. **Finding 3 (MEDIUM)**: Validate `status_code` fits in `u8` before casting.
5. **Finding 6 (MEDIUM)**: Replace `.unwrap()` with `.unwrap_or_default()` in remote.rs.
6. **Findings 9, 10 (MEDIUM/LOW)**: Add explicit gas limits and validate metric payload size.
