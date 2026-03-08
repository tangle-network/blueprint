# container

## Purpose
Container runtime module that manages blueprint service processes as Kubernetes pods. Handles pod creation, namespace/service/endpoint setup, TEE isolation via Kata Containers, and pod lifecycle.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - `ContainerInstance` struct managing a single Kubernetes pod. `new()` requires a valid kube client from `BlueprintManagerContext`. `start()` creates the `blueprint-manager` namespace, a headless `blueprint-service` Service with Endpoints pointing to the host IP, then creates/patches a Pod with the container image, environment variables, resource limits, keystore volume mount, and optional Kata runtime class. Translates loopback IPs for local testnets. `status()` maps pod phase to `Status` enum. `shutdown()` deletes the pod (idempotent). `detect_kata()` checks for `RuntimeClass` named "kata". Confidentiality policy handling: `TeeRequired` mandates Kata, `TeePreferred` uses Kata if available, others skip it.

## Key APIs
- `ContainerInstance::new(ctx, limits, service_name, image, env, args, confidentiality_policy, debug)` -- create instance with kube client
- `ContainerInstance::start()` -- provision namespace, service, endpoints, and pod
- `ContainerInstance::status()` -- query pod phase (Running/Pending/Failed/Succeeded)
- `ContainerInstance::shutdown()` -- delete pod from cluster

## Relationships
- Depends on `kube` and `k8s-openapi` crates for Kubernetes API interaction
- Depends on `crate::config::BlueprintManagerContext` for kube client and service port
- Depends on `crate::rt::ResourceLimits` for memory constraints
- Depends on `crate::sources::{BlueprintEnvVars, BlueprintArgs}` for container environment configuration
- Depends on `blueprint-client-tangle::ConfidentialityPolicy` for TEE runtime decisions
- Used by `crate::rt::service::Service::new_container()` as the container backend
