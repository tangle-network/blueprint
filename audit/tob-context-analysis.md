# Trail of Bits Context Analysis: blueprint-sdk QoS Crate

**Scope:** `crates/qos/src/` -- heartbeat, metrics ABI, metrics providers, service wiring
**Date:** 2026-02-08
**Methodology:** Ultra-granular line-by-line static analysis

---

## 1. heartbeat.rs -- Signature Construction, Metric Encoding, Transaction Submission

### 1.1 Signature Construction (`sign_heartbeat_payload`, lines 448-478)

**What it does:**
Constructs an EIP-191 personal-sign style signature over `keccak256(service_id || blueprint_id || metrics)`.

**Line-by-line breakdown:**

```
L454: let mut payload = Vec::with_capacity(16 + metrics.len());
L455: payload.extend_from_slice(&service_id.to_be_bytes());   // 8 bytes
L456: payload.extend_from_slice(&blueprint_id.to_be_bytes()); // 8 bytes
L457: payload.extend_from_slice(metrics);                      // variable
```

The inner payload is `service_id_be(8) || blueprint_id_be(8) || metrics_bytes`.

```
L459: let message_hash = keccak256(&payload);
```

First hash: `H1 = keccak256(payload)`.

```
L461-463: prefixed = "\x19Ethereum Signed Message:\n32" || H1
```

This manually constructs the EIP-191 prefix. The `\n32` is hardcoded and correct because `H1` is always 32 bytes (keccak256 output).

```
L465-467: prefixed_hash = keccak256(prefixed); digest = prefixed_hash
```

Second hash: `H2 = keccak256(prefix || H1)`. This is the value actually signed.

```
L469-472: sign_prehash_recoverable(&digest)
```

Signs `H2` using k256 ECDSA with recovery.

```
L474-476: signature_bytes = r(32) || s(32) || v(1)
          recovery_id.to_byte() + 27
```

Standard Ethereum signature format: 65 bytes, `v = recovery_id + 27`.

**FINDING QOS-HB-01 (Medium): Double-hashing divergence from standard `eth_sign`**

The standard `eth_sign` / `personal_sign` flow is:
`sign(keccak256("\x19Ethereum Signed Message:\n" + len(message) + message))`

This code instead does:
`sign(keccak256("\x19Ethereum Signed Message:\n32" + keccak256(payload)))`

This is a double-keccak construction. The first hash reduces the payload to 32 bytes, then the prefix is applied, then it is hashed again. This is not the standard `personal_sign` flow where the *raw message* (not its hash) is prefixed. If the on-chain contract uses Solidity's `ecrecover` with `ECDSA.toEthSignedMessageHash(keccak256(payload))`, it will match. But if the contract uses `ECDSA.toEthSignedMessageHash(payload)` (i.e., hashing the raw payload bytes with the prefix), it will NOT match. This must be verified against the deployed contract. There is an identical copy of this function in `cli/src/command/operator.rs` (lines 168-198), so the two call-sites are at least consistent with each other.

**FINDING QOS-HB-02 (Low): `signing_key` taken as `&mut` unnecessarily**

`sign_heartbeat_payload` takes `signing_key: &mut K256SigningKey` (line 449), but the k256 `SigningKey::sign_prehash_recoverable` method only requires `&self`. The `&mut` borrow is unnecessary and could cause borrow-checker friction in calling code. Additionally, on line 205, `let mut signing_key = operator_ecdsa_secret.clone();` clones the secret key into a mutable local, which is only needed because of this `&mut` signature.

### 1.2 Metric Encoding in Heartbeat (lines 206-215)

```
L206-210: let custom_metrics = if let Some(ref source) = metrics_source {
              source.get_custom_metrics().await
          } else { vec![] };
L211-215: let metrics_bytes = if custom_metrics.is_empty() {
              vec![]
          } else {
              crate::metrics::abi::encode_metric_pairs(&custom_metrics)
          };
```

**FINDING QOS-HB-03 (Informational): Empty metrics bypass ABI encoding**

When `custom_metrics` is empty, `metrics_bytes` is set to `vec![]` (empty). However, `encode_metric_pairs(&[])` produces a valid ABI encoding of an empty array (non-empty bytes -- the test at `abi.rs:50` confirms this). This means the signed payload differs depending on which path is taken: empty `Vec<u8>` vs. ABI-encoded empty array. If the contract expects ABI-decoded metrics, an empty `vec![]` is NOT a valid ABI encoding and would fail to decode on-chain. However, since empty metrics_bytes just means "no metrics", the contract may skip decoding when the bytes are empty. This should be verified against the contract.

### 1.3 Transaction Submission (lines 216-274)

```
L216-221: signature = sign_heartbeat_payload(signing_key, service_id, blueprint_id, &metrics_bytes)
```

The signature covers `service_id || blueprint_id || metrics_bytes`. The `status_code` is NOT included in the signed payload. This means a relayer or frontrunner could change `statusCode` in the transaction calldata without invalidating the signature.

**FINDING QOS-HB-04 (Medium): `statusCode` not covered by signature**

The `submitHeartbeatCall` struct (lines 234-240) includes `statusCode: status.status_code as u8`, but the signed payload (lines 454-457) only contains `service_id`, `blueprint_id`, and `metrics`. The `status_code` value is not part of the signed data. This means the on-chain contract cannot cryptographically verify that the operator intended a particular status code. A transaction relayer or mempool observer could theoretically resubmit the same signature with a different `statusCode`. The severity depends on whether the contract validates the signature against a payload that includes `statusCode`.

**FINDING QOS-HB-05 (Low): `status.status_code as u8` truncation**

`status.status_code` is `u32` (line 72). At line 237, it is cast to `u8` via `as u8`, which silently truncates. If the status code exceeds 255, the submitted value will wrap without any warning or error. The CLI version (`operator.rs`) receives `status_code` as `u8` directly, so there is no truncation risk there -- only in the QoS heartbeat service.

### 1.4 Heartbeat Loop and Timing (lines 356-416)

```
L365-372: initial jitter calculation
L373:     tokio::time::sleep(Duration::from_millis(initial_jitter)).await;
```

The initial jitter delays the first heartbeat. However, `start_heartbeat` holds `self.running.lock().await` at line 358, sets it to `true`, then drops the lock before sleeping on line 373. This means `is_running()` will return `true` before the first heartbeat is actually sent.

```
L390-412: spawn loop
L392: loop {
L393:     if !*running_status.lock().await { break; }
L398:     do_send_heartbeat(context.clone()).await
L410:     tokio::time::sleep(sleep_duration).await;
}
```

**FINDING QOS-HB-06 (Medium): First heartbeat sent immediately, then sleep**

The loop checks the running flag, sends a heartbeat, then sleeps. This means the first heartbeat fires immediately after the initial jitter (no interval wait before the first heartbeat). Whether this is intentional depends on the protocol; some heartbeat protocols expect the first beat after one full interval.

**FINDING QOS-HB-07 (Low): Jitter is additive-only, never subtractive**

`sleep_duration = base_interval + Duration::from_millis(current_jitter)` (line 405). The jitter is always positive, meaning heartbeats are always delayed by `[interval, interval + max_jitter]`, never `[interval - max_jitter, interval + max_jitter]`. This creates a systematic bias toward longer intervals, which may cause the operator to appear slower than expected from the contract's perspective.

### 1.5 Lifecycle and Drop (lines 420-446)

```
L421: let mut running = self.running.lock().await;
L422: if !*running { return Err(...) }
L426: *running = false;
L427: drop(running);
L429-432: abort task handle
```

`stop_heartbeat` sets `running = false` and then aborts the task. The abort is redundant if the loop would check `running` on its next iteration, but since the loop may be sleeping for up to `interval + max_jitter` duration, the abort ensures prompt termination.

```
L438-446: Drop impl uses try_lock()
```

**FINDING QOS-HB-08 (Low): Drop uses `try_lock` which can silently fail**

If the `task_handle` Mutex is held by another task during drop, `try_lock()` fails silently and the spawned task is never aborted. This can leak a background task that continues sending heartbeats after the `HeartbeatService` is dropped. Since this is a `tokio::sync::Mutex`, `try_lock` returns `Err` under contention, not poisoning.

### 1.6 Keystore access per heartbeat (lines 192-203)

**FINDING QOS-HB-09 (Low): Keystore re-opened on every heartbeat**

Every call to `do_send_heartbeat` constructs a new `Keystore` instance (line 192) and queries the signing key. This is filesystem I/O on every heartbeat cycle (default: every 300 seconds). While not a performance issue at that cadence, it means the keystore path is resolved repeatedly and key material is loaded from disk each time rather than cached. If the keystore file is temporarily unavailable (e.g., during a volume remount), a single heartbeat will fail and be silently logged as a warning.

### 1.7 `block_number` hardcoded to 0 (line 161)

```
L161: block_number: 0,
```

`HeartbeatStatus.block_number` is always set to `0`. This field is never populated with the actual current block number from the chain. If the contract or any downstream consumer relies on this field, it will always see `0`. The CLI version (`operator.rs:93`) also hardcodes this to `0`.

### 1.8 `instance_service_id` vs `config_service_id` (lines 107-116)

The `HeartbeatTaskContext` carries both `config_service_id` / `config_blueprint_id` (from `HeartbeatConfig`) and `instance_service_id` / `instance_blueprint_id` (from the `HeartbeatService` constructor). The `config_*` values are used for the actual heartbeat payload (line 166-167) and transaction. The `instance_*` values are only logged (line 187-188). If these differ, the logged values will not match the submitted values.

---

## 2. metrics/abi.rs -- ABI Encoding Correctness

### 2.1 `encode_metric_pairs` (lines 1-21)

```rust
sol! {
    struct MetricPair {
        string name;
        uint256 value;
    }
}

pub fn encode_metric_pairs(metrics: &[(String, u64)]) -> Vec<u8> {
    let pairs: Vec<MetricPair> = metrics.iter().map(|(name, value)| MetricPair {
        name: name.clone(),
        value: alloy_primitives::U256::from(*value),
    }).collect();
    pairs.abi_encode()
}
```

**Analysis:**

The `sol!` macro generates a Solidity-compatible struct. `pairs.abi_encode()` produces `abi.encode(MetricPair[])`, which is the ABI encoding of a dynamic array of structs containing a dynamic `string` and a `uint256`. This is the correct encoding for `abi.decode(data, (MetricPair[]))` on the Solidity side.

**FINDING QOS-ABI-01 (Informational): u64 to U256 widening is safe but lossy in reverse**

`U256::from(*value)` widens a `u64` to `uint256`. This is lossless. However, if the contract writes back values larger than `u64::MAX`, the Rust side has no way to represent them. This is acceptable for metrics that originate from this Rust code.

**FINDING QOS-ABI-02 (Informational): No metric name validation**

Metric names are arbitrary `String` values. There is no validation for:
- Empty strings
- Strings containing null bytes
- Extremely long strings that could make the ABI encoding exceed gas limits
- Duplicate metric names

If a user calls `add_on_chain_metric("".to_string(), 42)`, an empty-named metric will be ABI-encoded and submitted on-chain.

### 2.2 Test coverage (lines 24-66)

The tests verify:
1. Round-trip encode/decode with 2 metrics
2. Empty array encoding produces non-empty bytes that decode back to empty
3. Single metric round-trip

The tests are sound. They confirm the ABI encoding is self-consistent. They do NOT verify interop with a Solidity contract (no cross-language test).

---

## 3. metrics/provider/enhanced.rs -- `on_chain_metrics` drain pattern, lock usage

### 3.1 `on_chain_metrics` data structure (line 41)

```rust
on_chain_metrics: Arc<RwLock<std::collections::HashMap<String, u64>>>,
```

This is a `tokio::sync::RwLock<HashMap<String, u64>>`. The `RwLock` is appropriate for the read-heavy collection loop, but the drain pattern deserves scrutiny.

### 3.2 `add_on_chain_metric` (lines 421-424)

```rust
async fn add_on_chain_metric(&self, key: String, value: u64) {
    let mut metrics = self.on_chain_metrics.write().await;
    metrics.insert(key, value);
}
```

This uses `.write().await` (blocking until the write lock is acquired). It replaces any existing metric with the same key. If two tasks call `add_on_chain_metric("cpu", 50)` and `add_on_chain_metric("cpu", 75)` concurrently, one will win non-deterministically.

### 3.3 `get_on_chain_metrics` (lines 426-430)

```rust
async fn get_on_chain_metrics(&self) -> Vec<(String, u64)> {
    let mut metrics = self.on_chain_metrics.write().await;
    let result: Vec<(String, u64)> = metrics.drain().collect();
    result
}
```

**FINDING QOS-METRIC-01 (Medium): Drain pattern creates a race window for metric loss**

The `drain()` call atomically empties the HashMap while holding the write lock. Any metrics added between two `get_on_chain_metrics` calls will be included in exactly one heartbeat submission. However, there is a race condition:

1. Task A calls `add_on_chain_metric("x", 1)` -- acquires write lock, inserts, releases
2. Heartbeat calls `get_on_chain_metrics()` -- acquires write lock, drains all, releases
3. Heartbeat signing/submission fails (e.g., RPC down)
4. The metric `("x", 1)` is now lost -- it was drained but never submitted on-chain

There is no mechanism to re-queue metrics that were drained but failed to submit. The calling code in `heartbeat.rs` (lines 206-215) obtains metrics, encodes them, and then signs and submits. If the submission fails (lines 248-256), the metrics are gone.

**FINDING QOS-METRIC-02 (Low): `get_on_chain_metrics` takes write lock unnecessarily**

The `drain()` method requires `&mut self` on the HashMap, which necessitates a write lock. An alternative pattern (swap with an empty HashMap) would also require a write lock, so the current approach is acceptable. However, the write lock contention means that `add_on_chain_metric` calls will block during the drain operation.

### 3.4 `MetricsSource` impl (lines 527-533)

```rust
impl MetricsSource for EnhancedMetricsProvider {
    fn get_custom_metrics(&self)
        -> Pin<Box<dyn Future<Output = Vec<(String, u64)>> + Send + '_>>
    {
        Box::pin(async { self.get_on_chain_metrics().await })
    }
}
```

This bridges the RPITIT-based `MetricsProvider::get_on_chain_metrics` to the dyn-compatible `MetricsSource` trait. The bridging is correct. Note that calling `get_custom_metrics` triggers the drain -- so calling it multiple times without intervening `add_on_chain_metric` calls returns an empty Vec the second time.

### 3.5 `start_collection` duplication (inherent method vs trait impl)

**FINDING QOS-METRIC-03 (Medium): `start_collection` is implemented twice**

The `EnhancedMetricsProvider` has two `start_collection` methods:

1. **Inherent method** at line 144: `pub async fn start_collection(self: Arc<Self>) -> Result<()>` -- takes `Arc<Self>`, called from `unified_service.rs:118`.
2. **Trait impl** at line 461: `async fn start_collection(&self) -> crate::error::Result<()>` -- takes `&self`, part of `MetricsProvider` trait.

Both methods contain nearly identical code that:
- Creates a PrometheusServer
- Starts the server
- Spawns a background collection loop

If both are called, two Prometheus servers will be started and two background collection loops will run, doubling metric collection and potentially binding the same port twice (which would fail). The inherent method is what is actually called in `unified_service.rs:118`, but the trait method exists and could be called by any code that holds a `&EnhancedMetricsProvider`.

### 3.6 History truncation uses `Vec::remove(0)` (lines 178-179, 188-189)

```rust
metrics.push(sys_metrics);
if metrics.len() > config.max_history {
    metrics.remove(0);
}
```

**FINDING QOS-METRIC-04 (Informational): O(n) history truncation**

`Vec::remove(0)` is O(n) because it shifts all remaining elements. With `max_history = 100` (default), this is negligible. If `max_history` were set to a large value, this would become a performance issue. A `VecDeque` would provide O(1) removal from the front.

---

## 4. metrics/provider/default.rs -- `on_chain_metrics` drain pattern, lock usage

### 4.1 `try_write` / `try_read` pattern throughout

**FINDING QOS-DEFAULT-01 (Medium): Silently dropping operations on lock contention**

The `DefaultMetricsProvider` uses `try_write()` and `try_read()` throughout, silently discarding operations when the lock is contended:

```rust
// Line 172-174
async fn add_on_chain_metric(&self, key: String, value: u64) {
    if let Ok(mut metrics) = self.on_chain_metrics.try_write() {
        metrics.insert(key, value);
    }
    // SILENT DROP if lock contended
}
```

```rust
// Line 177-183
async fn get_on_chain_metrics(&self) -> Vec<(String, u64)> {
    if let Ok(mut metrics) = self.on_chain_metrics.try_write() {
        metrics.drain().collect()
    } else {
        Vec::new()  // Returns empty, metrics NOT drained
    }
}
```

This is especially dangerous for `get_on_chain_metrics`: if the lock is contended, it returns an empty Vec, causing the heartbeat to submit with no metrics. The actual metrics remain in the HashMap and will be submitted on a later heartbeat (assuming the lock is not perpetually contended).

For `add_on_chain_metric`, a contended lock causes the metric to be silently dropped -- the caller has no indication the metric was not recorded.

This is inconsistent with `EnhancedMetricsProvider`, which uses `.write().await` (blocking until lock available). The `DefaultMetricsProvider` uses `tokio::sync::RwLock` which cannot be poisoned, so the error messages about "lock poisoned" (lines 112, 134, etc.) are misleading -- `try_read` / `try_write` on `tokio::sync::RwLock` only fail due to contention, never poisoning.

**FINDING QOS-DEFAULT-02 (Low): Misleading error messages about lock poisoning**

Multiple error messages reference "lock poisoned or unavailable" (lines 112, 125, 134, 145, 155, 166, 192, 201, etc.). `tokio::sync::RwLock` does not have the poisoning semantics of `std::sync::RwLock`. The `try_read()` and `try_write()` methods on `tokio::sync::RwLock` return `Err` only when the lock is currently held by another task in a conflicting mode. The error messages should say "lock contended" not "lock poisoned".

### 4.2 Collection loop also uses `try_write` (lines 220-270)

The background collection loop uses `try_write` for all updates:
- `system_metrics.try_write()` (line 226)
- `custom_metrics.try_read()` (line 238)
- `blueprint_metrics.try_write()` (line 246)
- `blueprint_status.try_write()` (line 257)

If any of these fail, the collection cycle skips that update and moves on. Under normal load this is fine since contention on these locks should be rare. But during metric-heavy periods, skipped updates could create gaps in the metrics history.

### 4.3 No `MetricsSource` implementation

**FINDING QOS-DEFAULT-03 (Medium): `DefaultMetricsProvider` does not implement `MetricsSource`**

Unlike `EnhancedMetricsProvider`, `DefaultMetricsProvider` does not implement the `MetricsSource` trait. This means it cannot be used as a `metrics_source` for the heartbeat service. If code attempted to use `DefaultMetricsProvider` with heartbeats, on-chain metrics would never be submitted. However, since the `unified_service.rs` always creates an `EnhancedMetricsProvider` via `MetricsService`, and `DefaultMetricsProvider` appears to be for testing/simple use cases, this may be intentional.

---

## 5. unified_service.rs -- `MetricsSource` wiring

### 5.1 Metrics-to-heartbeat wiring (lines 80-106)

```rust
// Line 81-85: Create metrics service
let metrics_service = match (config.metrics.clone(), otel_config) {
    (Some(mc), Some(oc)) => Some(Arc::new(MetricsService::with_otel_config(mc, &oc)?)),
    (Some(mc), None) => Some(Arc::new(MetricsService::new(mc)?)),
    (None, _) => None,
};

// Line 87-106: Create heartbeat service
let heartbeat_service = match (config.heartbeat.clone(), heartbeat_ctx) {
    (Some(hc), Some(mut ctx)) => {
        // Line 91-94: Auto-wire metrics source
        if ctx.metrics_source.is_none() {
            if let Some(ms) = &metrics_service {
                ctx.metrics_source = Some(ms.provider() as Arc<dyn MetricsSource>);
            }
        }
        Some(Arc::new(HeartbeatService::with_metrics_source(...)?))
    }
    ...
};
```

**Analysis:**

The auto-wiring logic at lines 91-94 is correct: if the user did not explicitly provide a `metrics_source` in the `HeartbeatContext`, the `EnhancedMetricsProvider` from the `MetricsService` is used. The `ms.provider()` returns `Arc<EnhancedMetricsProvider>`, which is then cast to `Arc<dyn MetricsSource>`. This works because `EnhancedMetricsProvider` implements `MetricsSource`.

**FINDING QOS-WIRE-01 (Informational): Metrics source is the same provider that collects metrics**

The `EnhancedMetricsProvider` that collects metrics is the same instance used as the `MetricsSource` for heartbeats. When the heartbeat service calls `get_custom_metrics()`, it drains `on_chain_metrics` from the same provider. This is correct and intentional -- metrics flow from `add_on_chain_metric()` -> HashMap -> drain via heartbeat -> ABI encode -> submit.

### 5.2 `start_collection` invocation (lines 116-119)

```rust
if let Some(ms) = &metrics_service {
    info!("Metrics service is Some, attempting to start collection.");
    ms.provider().clone().start_collection().await?;
}
```

This calls the **inherent** `start_collection(self: Arc<Self>)` method (because `ms.provider()` returns `Arc<EnhancedMetricsProvider>` and `.clone()` gives another `Arc`). This is the method at `enhanced.rs:144`, NOT the trait impl at `enhanced.rs:461`.

**FINDING QOS-WIRE-02 (Low): Prometheus server potentially started twice**

At line 118, `start_collection` is called, which creates and starts a `PrometheusServer` (enhanced.rs:147-152). Then at line 143 of `unified_service.rs`, if `config.manage_servers` is true, another `PrometheusServer` is created and started (lines 143-156). Both servers use the same shared registry, but they will attempt to bind to different ports (one from `MetricsConfig.prometheus_server`, one from `QoSConfig.prometheus_server`). If the ports differ, two servers expose the same metrics. If the ports are the same, the second bind will fail.

### 5.3 Shutdown does nothing (lines 511-515)

```rust
pub fn shutdown(&self) -> Result<()> {
    info!("QoSService shutting down...");
    info!("QoSService shutdown complete.");
    Ok(())
}
```

**FINDING QOS-WIRE-03 (Medium): Shutdown does not stop heartbeat or metrics collection**

`QoSService::shutdown()` only logs. It does not:
1. Call `heartbeat_service.stop_heartbeat()` -- heartbeats continue
2. Stop the metrics collection background task -- collection continues
3. Stop any managed servers (Grafana, Loki, Prometheus)

The `Drop` impl (lines 518-543) attempts to flush OTel metrics and send the completion signal, but it also does not stop heartbeats or collection tasks. The heartbeat service has its own `Drop` impl that attempts to abort the task via `try_lock`, but as noted in QOS-HB-08, this can silently fail.

### 5.4 Completion channel semantics (lines 214-226, 483-501, 518-543)

The `completion_tx` / `completion_rx` oneshot channel is created during initialization. The receiver is consumed by `wait_for_completion()` (can only be called once). The sender is fired during `Drop`. This means `wait_for_completion` blocks until the `QoSService` is dropped.

**FINDING QOS-WIRE-04 (Low): `wait_for_completion` only resolves on drop**

Since `shutdown()` does not send the completion signal, the only way `wait_for_completion` resolves is when the `QoSService` is dropped. If the caller holds the `QoSService` and calls `wait_for_completion`, they deadlock -- the `QoSService` will not be dropped while `wait_for_completion` is `.await`-ing on it.

---

## 6. service_builder.rs -- `MetricsSource` wiring

### 6.1 `build()` method (lines 281-324)

```rust
pub async fn build(self) -> Result<QoSService<C>> {
    let heartbeat_context = if self.config.heartbeat.is_some() {
        // ... validate consumer, http, keystore, registry ...
        Some(HeartbeatContext {
            consumer,
            http_rpc_endpoint: http,
            keystore_uri: keystore,
            status_registry_address: status_registry,
            dry_run: self.dry_run,
            metrics_source: None,   // <-- always None
        })
    } else { None };

    if let Some(otel_config) = self.otel_config {
        QoSService::with_otel_config(self.config, heartbeat_context, otel_config).await
    } else {
        QoSService::new(self.config, heartbeat_context).await
    }
}
```

**FINDING QOS-BUILD-01 (Informational): Builder always sets `metrics_source: None`**

The builder always constructs `HeartbeatContext` with `metrics_source: None` (line 313). This is correct because `unified_service.rs:91-94` will auto-wire the metrics source from the MetricsService if available. However, there is no builder method to explicitly provide a custom `MetricsSource`. If a user wants to use a custom metrics source (not the built-in `EnhancedMetricsProvider`), they must bypass the builder and construct `HeartbeatContext` directly.

### 6.2 Builder consumes `self.heartbeat_consumer.clone()` (line 283)

```rust
let consumer = self.heartbeat_consumer.clone().ok_or_else(|| { ... })?;
```

The `.clone()` is on `Option<Arc<C>>`, which clones the `Arc` (cheap reference count increment). This is correct.

### 6.3 No method to set `metrics_source` on builder

The builder has `with_heartbeat_consumer`, `with_heartbeat_config`, `with_http_rpc_endpoint`, `with_keystore_uri`, `with_status_registry_address`, and `with_dry_run`, but no `with_metrics_source`. This means external `MetricsSource` implementations cannot be wired through the builder.

---

## 7. Cross-cutting findings

### 7.1 CLI vs QoS heartbeat divergence

**FINDING QOS-CROSS-01 (Medium): Semantic difference in `metrics` field between CLI and QoS**

The CLI (`operator.rs:100`) passes `payload.encode()` (which is `block_number || timestamp || service_id || blueprint_id || status_code` -- 40 bytes total) as the `metrics` parameter to `submitHeartbeatCall`. This is the payload itself, not actual metrics data.

The QoS heartbeat service (`heartbeat.rs:214`) passes ABI-encoded `MetricPair[]` as the `metrics` parameter, or empty bytes if there are no metrics.

The `signature` in both cases is computed over `service_id || blueprint_id || metrics_bytes` where `metrics_bytes` differs between the two implementations. This means the two implementations produce incompatible signatures for the same `(service_id, blueprint_id)` pair, because the third component of the signed payload is different data.

The contract presumably validates the signature against the submitted `metrics` bytes. Since the CLI and QoS send different data in the `metrics` field, they sign different payloads. Both should be individually valid as long as the contract simply verifies `ecrecover(hash(serviceId, blueprintId, metrics), signature) == msg.sender`.

### 7.2 Duplicate `sign_heartbeat_payload` function

The `sign_heartbeat_payload` function exists in two places:
1. `crates/qos/src/heartbeat.rs:448-478`
2. `cli/src/command/operator.rs:168-198`

These are exact duplicates. Any bug fix or security update must be applied to both.

---

## Summary of Findings by Severity

| ID | Severity | Component | Description |
|----|----------|-----------|-------------|
| QOS-HB-01 | Medium | heartbeat.rs | Double-hashing divergence from standard `eth_sign` -- must match contract |
| QOS-HB-04 | Medium | heartbeat.rs | `statusCode` not covered by signature, can be tampered |
| QOS-HB-06 | Medium | heartbeat.rs | First heartbeat fires immediately (no initial interval wait) |
| QOS-METRIC-01 | Medium | enhanced.rs | Drain pattern loses metrics on submission failure |
| QOS-METRIC-03 | Medium | enhanced.rs | `start_collection` implemented twice (inherent + trait) |
| QOS-DEFAULT-01 | Medium | default.rs | Silent operation dropping on lock contention via `try_write` |
| QOS-DEFAULT-03 | Medium | default.rs | No `MetricsSource` impl, cannot feed heartbeats |
| QOS-WIRE-03 | Medium | unified_service.rs | `shutdown()` does not actually stop anything |
| QOS-CROSS-01 | Medium | cross-cutting | CLI and QoS send semantically different `metrics` data |
| QOS-HB-02 | Low | heartbeat.rs | Unnecessary `&mut` on signing key |
| QOS-HB-05 | Low | heartbeat.rs | `status_code` `u32`-to-`u8` silent truncation |
| QOS-HB-07 | Low | heartbeat.rs | Jitter is additive-only, biases toward longer intervals |
| QOS-HB-08 | Low | heartbeat.rs | Drop uses `try_lock`, can leak background task |
| QOS-HB-09 | Low | heartbeat.rs | Keystore re-opened on every heartbeat |
| QOS-METRIC-02 | Low | enhanced.rs | `get_on_chain_metrics` write lock blocks `add_on_chain_metric` |
| QOS-DEFAULT-02 | Low | default.rs | Error messages incorrectly reference lock poisoning |
| QOS-WIRE-02 | Low | unified_service.rs | Prometheus server potentially started twice |
| QOS-WIRE-04 | Low | unified_service.rs | `wait_for_completion` only resolves on drop (deadlock risk) |
| QOS-HB-03 | Info | heartbeat.rs | Empty metrics bypass ABI encoding (may mismatch contract) |
| QOS-ABI-01 | Info | abi.rs | u64 to U256 widening safe but not reversible |
| QOS-ABI-02 | Info | abi.rs | No metric name validation (empty, null bytes, length) |
| QOS-METRIC-04 | Info | enhanced.rs | O(n) history truncation via `Vec::remove(0)` |
| QOS-WIRE-01 | Info | unified_service.rs | Metrics source is same instance as collector (by design) |
| QOS-BUILD-01 | Info | service_builder.rs | Builder always sets `metrics_source: None` (auto-wired later) |
