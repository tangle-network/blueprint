# Remote providers

Read [README.md](README.md) and the implementation of the provider you will change.
Cloud integration tests can create billable resources; use the authorized account and preserve cleanup on failure.
Some [security tests](tests/security/mod.rs) catalogue vulnerable behavior and deliberately assert `Vulnerable`.
A passing test count alone does not prove these vulnerabilities are fixed.

## TEE deployment and evidence

Read [attestation/](src/attestation/) before changing `require_tee` provisioning.
`gate_provisioned` fetches provider evidence and delegates cryptographic verification to `blueprint-tee`.
Keep failures closed: an unavailable or invalid attestation must not become a trusted deployment.

The workload must expose fresh, nonce-bound evidence through the provider's required interface.
This crate does not deploy the forwarding component for GCP Confidential Space or Azure MAA tokens.
Check the provider implementation for the required endpoint and verify the workload supplies it before claiming a live deployment works.

AWS Nitro evidence originates inside the enclave.
The out-of-enclave `AwsNitroGate::fetch` returns `Unsatisfiable`; enabling verification does not create an evidence-fetch path.
An enclave application must provide its document to `verify_document` through its own channel.
The Nitro verification feature must be enabled.

Preserve the distinction between hardware attestation and workload binding.
Read `AttestationPolicy::is_workload_bound` for the required audience and image constraints.
Only `tee_attested=true` represents workload-bound evidence; `hardware-only` and `tee_workload_bound=false` do not.

Freshness, signatures, nonce validation, and non-debug execution remain required.
Read the policy's enforced limits instead of copying them into configuration guidance.
Do not permit production `custom_config` to weaken them or enable test-only debug behavior.
