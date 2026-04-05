# Pursuit: SDK Quality — Principal Eng Standard

## Generation 1 — SHIPPED (PR #1361)
- Security criticals: deterministic RNG, constant-time auth, key zeroing, job dispatch panic
- Lint discipline: removed 5 dangerous workspace-wide allows
- Panic elimination: keystore, eigenlayer registration, runner config
- Result: 5.5 → 6.5/10

---

## Generation 2 — ALL REMAINING HIGH/MEDIUM/STRATEGIC
Date: 2026-04-04
Status: building

### Thesis
**Seal every crack, harden every surface.** Gen 1 stopped the bleeding. Gen 2 closes every remaining HIGH and MEDIUM finding, fixes the structural bloat, and adds the missing production hardening. Target: 9/10 across all categories.

### Track A — Soundness & Unsafe
1. Add `// SAFETY:` comments to `unsafe impl Send/Sync` in runner + manager, or eliminate them
2. Fix `unsafe` env var mutation in remote-providers (use Mutex or scoped approach)
3. Fix TOCTOU on key file permissions (create with restricted permissions from the start)
4. Fix `transmute` endianness in JobId (use `from_le_bytes`/`to_le_bytes`)

### Track B — Network Hardening
5. Add per-peer rate limiting to P2P protocol (request + gossip)
6. Add size-bounded deserialization for bincode on network messages
7. Replace SSH command denylist with allowlist pattern

### Track C — Async & Concurrency
8. Replace QoS busy-wait polling with `tokio::sync::watch` channel
9. Add backpressure to producer/consumer pipeline (bounded channels)

### Track D — Clippy Deep Clean
10. Audit remaining ~100 suppressed clippy lints — remove unjustified, fix warnings
11. Target: only genuinely stylistic lints remain suppressed

### Track E — TODO/Dead Code Purge
12. Audit all TODO/FIXME in production code — resolve meaningful ones, delete stale ones
13. Remove dead code surfaced by lint changes
14. Clean up hardcoded values (service_id=1, blueprint_id=1 in QoS)

### Track F — Build & Structure
15. Populate workspace-hack with hakari output for compile-time dedup
16. Audit and document the meta-crate strategy (keep/merge decision with rationale)

### Strategic (deferred to Gen 3 — require protocol design decisions)
- Key rotation for operators
- Pre-submission slashing protection
- MEV-protected result submission
- `cargo tangle dev` zero-config mode
