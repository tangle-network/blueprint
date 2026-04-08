# RFC-003: Zero-Code TEE Activation

## Problem

Today, enabling TEE on a blueprint requires code changes:

```rust
// Operator must add this to their runner builder
BlueprintRunner::builder(config, env)
    .tee(TeeConfig { ... })  // ← code change required
    .router(my_router)
    .run()
```

And per-handler TEE layering:
```rust
Router::new()
    .route(0, my_handler.layer(TeeLayer::new()))  // ← code change per handler
```

This means:
- Same binary cannot run with and without TEE
- Operator must recompile to enable/disable TEE
- Blueprint authors must explicitly wire TEE into every handler
- No way to "just deploy to a TEE machine and have it work"

## Proposal

TEE activation should be **config-driven and environment-detected**, not code-driven.

### 1. Runner auto-detects TEE environment

```rust
// This is ALL the operator writes. No .tee() call, no TeeLayer.
BlueprintRunner::builder(config, env)
    .router(my_router)
    .run()
```

At startup, the runner:
1. Reads `tee.mode` from config (default: `"auto"`)
2. If `auto`: probes for TEE hardware (`/dev/attestation`, `/dev/sev-guest`, Nitro NSM, etc.)
3. If TEE detected: wraps ALL router handlers with `TeeLayer` automatically, generates attestation, registers as TEE-capable on-chain
4. If no TEE detected: runs normally, registers as non-TEE
5. If `required`: refuses to start without TEE hardware
6. If `disabled`: never activates TEE even if hardware is present

### 2. Config-only TEE control

```toml
# operator-config.toml

[tee]
mode = "auto"           # "auto" | "required" | "disabled"
# requirement = "preferred"  # "preferred" | "required" (for incoming requests)
# provider = "any"           # "any" | "aws_nitro" | "azure_snp" | "intel_tdx" | "amd_sev_snp"
# attestation_refresh_secs = 3600
```

Or via environment variable:
```bash
TEE_MODE=auto ./my-operator
```

### 3. Runner wraps all handlers automatically

When TEE is active, the runner applies `TeeLayer` to every route in the router at startup — the blueprint author never touches it:

```rust
// Inside BlueprintRunner::run()
let router = if tee_active {
    // Wrap the entire router, not individual handlers
    self.router.layer(TeeLayer::with_attestation(report))
} else {
    self.router
};
```

This means:
- `TeeLayer::new()` in handler code becomes unnecessary (deprecated for direct use)
- All jobs get TEE attestation metadata when running in TEE
- Zero handler-level changes for blueprint authors
- Existing blueprints get TEE support by deploying to TEE hardware + setting config

### 4. On-chain TEE registration

When TEE is detected at startup, the runner:
1. Generates an attestation report from the hardware
2. Includes the attestation in the operator's on-chain registration
3. Periodically refreshes the attestation (configurable interval)
4. The BSM contract or Tangle Core stores the TEE status

Consumers (routers, users) can verify:
- Is this operator TEE-attested? (on-chain flag)
- When was the attestation last refreshed? (on-chain timestamp)
- What provider? (AWS Nitro, Intel TDX, etc.)

### 5. Router-side TEE filtering

The Tangle Router adds a request header:
```
X-Tangle-Require-TEE: true
```

`selectOperatorFromDb` adds a filter:
```typescript
if (requireTee) {
  where.teeAttested = true
}
```

Users who want TEE get routed to TEE operators. Users who don't care get routed to the cheapest/fastest operator. Both use the same router, same API, same blueprint binary.

### 6. Pricing differentiation

TEE operators cost more (hardware premium). The BSM contract already supports per-operator pricing. TEE operators set higher prices. The router's scoring algorithm considers price — users who require TEE pay more, users who don't get cheaper non-TEE operators.

No code change needed. The market handles differentiation.

## Detection Strategy

### Probe order (for `mode = "auto"`):

1. **AWS Nitro**: check for `/dev/nsm` (Nitro Security Module)
2. **Intel TDX**: check for `/dev/tdx-guest` or `/sys/firmware/acpi/tables/TDEL`
3. **AMD SEV-SNP**: check for `/dev/sev-guest`
4. **Azure CVM**: check for Azure IMDS attestation endpoint
5. **GCP Confidential Space**: check for `CONFIDENTIAL_SPACE_VERSION` env var

If none detected → TEE is not available. If `mode = "auto"`, run without TEE. If `mode = "required"`, exit with error.

## Migration Path

### For existing blueprints:

**Before (code-dependent TEE):**
```rust
Router::new()
    .route(0, my_handler.layer(TeeLayer::new()))

BlueprintRunner::builder(config, env)
    .tee(TeeConfig { ... })
    .router(router)
    .run()
```

**After (zero-code TEE):**
```rust
Router::new()
    .route(0, my_handler)  // no TeeLayer

BlueprintRunner::builder(config, env)
    .router(router)        // no .tee() call
    .run()
```

Config: `TEE_MODE=auto` (or in config file). Runner handles the rest.

**Backward compatibility**: `.tee(TeeConfig)` and `.layer(TeeLayer::new())` continue to work. They override auto-detection. Deprecation warning if both are used (explicit wins over auto).

### For the 7 inference blueprints:

Remove all `.layer(TeeLayer::new())` calls from handler code. Add `[tee]` section to config templates. The runner does the rest. One PR per blueprint, pure deletion.

## What This Enables

1. **Same binary, TEE or not** — deploy to a normal VM → runs without TEE. Deploy to a Confidential VM → runs with TEE. No recompile.

2. **Operator chooses at deploy time** — set `TEE_MODE=auto` and pick your infrastructure. The binary adapts.

3. **Users choose at request time** — `X-Tangle-Require-TEE: true` routes to TEE operators. No header → any operator.

4. **Market-driven privacy** — TEE operators charge more. Users pay for privacy when they want it. The platform doesn't force it.

5. **Blueprint authors don't think about TEE** — they write inference logic. The runner handles attestation. TEE is infrastructure, not application concern.

## Implementation Order

1. **`BlueprintRunner`: add TEE auto-detection** — probe hardware at startup, wrap router if detected
2. **`TeeConfig`: add `mode = "auto"` as default** — backward-compatible, existing configs still work
3. **On-chain TEE flag** — BSM contract records attestation status on registration
4. **Router: `X-Tangle-Require-TEE` filter** — one-line addition to `selectOperatorFromDb`
5. **Remove `TeeLayer` from inference blueprints** — 7 PRs, pure deletion
6. **Deprecate direct `TeeLayer` use** — add warning, keep functional

## Non-Goals

- This RFC does NOT change the attestation format or verification logic (RFC-001 covers that)
- This RFC does NOT add new TEE providers (use the existing 5)
- This RFC does NOT change billing/payment flows (x402 works the same)
- This RFC does NOT require changes to the Tangle Router beyond the `X-Tangle-Require-TEE` header filter
