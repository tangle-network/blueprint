# RFC-002: Cloud-Key TEE Autopilot for Blueprint Remote Deployments

## Summary

This RFC defines a practical path where operators can run TEE-targeted deployments with cloud credentials and a small manager policy surface, without per-deployment manual provider tuning.

In this phase we focus on:
- explicit `tee_required` deployment intent in remote manager policy,
- fail-closed provider selection,
- confidential-capable instance family selection,
- deterministic `TEE_BACKEND` injection for runtime bootstrapping.

## Problem

Remote deployment flows currently select providers and instance types by CPU/memory heuristics only. They do not model confidential-compute intent, and selected instance types are not consistently honored by provider adapters.

This makes "cloud API keys are sufficient" unreliable for TEE deployment outcomes.

## Goals

1. Operators can set one manager-level TEE policy (`tee_required=true`) and avoid accidental non-TEE placement.
2. Manager/provider orchestration only chooses TEE-capable providers when TEE is required.
3. Provisioning requests use confidential-capable machine families for AWS/GCP/Azure.
4. Runtime receives explicit TEE backend intent (`TEE_BACKEND`) without manual per-service wiring.

## Non-Goals (This Phase)

- Cryptographic attestation verification at cloud-provisioning boundary.
- On-chain attestation commitments.
- Provider-specific evidence policy validation and measurement allowlists.

## Design

### 1) Manager Policy

`RemoteDeploymentPolicy` gets:
- `tee_required: bool`
- `tee_backend: Option<String>`

Defaults:
- `tee_required` from `BLUEPRINT_REMOTE_TEE_REQUIRED`
- `tee_backend` from `TEE_BACKEND` (first non-empty entry in comma-separated list)

### 2) Fail-Closed Provider Selection

`ProviderSelector` now models `tee_required` in `ResourceSpec` and a `tee_capable` preference list.

When `tee_required=true`, candidate providers are restricted to:
- AWS
- GCP
- Azure

No fallback to non-TEE providers is allowed.

### 3) TEE-Aware Instance Mapping

`blueprint-remote-providers` gains `map_to_instance_type_with_requirements(..., require_tee)`.

For `require_tee=true`, mapper chooses confidential-capable families:
- AWS: `m6i/c6i/g5` family selection by size class
- GCP: `n2d-standard-*`
- Azure: `Standard_DC*as_v5`

### 4) Provisioning API

`CloudProvisioner` adds:
- `provision_with_requirements(provider, spec, region, require_tee)`

Behavior:
- rejects non-TEE-capable providers when `require_tee=true`,
- uses TEE-aware instance mapping,
- preserves existing `provision(...)` as non-TEE default.

### 5) Adapter Honor-Instance-Type Fix

AWS/GCP/Azure adapters and provisioners are updated so the selected instance type is actually applied during provisioning.

Without this, TEE-aware mapping has no operational effect.

### 6) Runtime Backend Injection

For TEE-required deployments, manager injects:
- `TEE_REQUIRED=true`
- `TEE_BACKEND=<policy override or provider default>`

Provider defaults:
- AWS -> `aws-nitro`
- GCP -> `gcp-confidential`
- Azure -> `azure-skr`

## Operator Experience (ELI5)

1. Export cloud credentials for one TEE-capable provider (AWS/GCP/Azure).
2. Set `BLUEPRINT_REMOTE_TEE_REQUIRED=true` on manager.
3. Optionally set `TEE_BACKEND` globally.
4. Deploy normally.

Manager will fail instead of silently picking non-TEE providers.

## Remaining Gaps

To make "cloud keys are fully sufficient" a complete security claim, we still need:
- cryptographic attestation verification in-manager,
- policy engine for expected measurements/signers,
- attestation proof recording (on-chain or verifiable audit log),
- operator preflight checks that verify provider prerequisites before deployment.

## Rollout

- Land changes behind existing remote-provider flow (no protocol-specific branching).
- Keep defaults backward-compatible (`tee_required=false`).
- Add operator documentation and test coverage for TEE provider/instance selection.
