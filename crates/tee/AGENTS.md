# Trusted execution

Read [README.md](README.md) and the affected verifier, exchange, or runtime implementation before changing TEE behavior.
Keep sealed secrets as the supported secret-injection path.
Preserve one-time session use, expiration, private-key erasure, and attestation binding.
Test failure and replay cases through the actual boundary.

A parsed or structurally checked report does not establish cryptographic verification.
Keep the implementation's verification level visible to callers and preserve its enforced trust requirements.
Test constructors and mock reports do not prove a hardware-backed deployment.
For cloud provisioning, also read [remote-provider instructions](../blueprint-remote-providers/AGENTS.md).
