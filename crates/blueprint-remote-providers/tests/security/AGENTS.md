# security

## Purpose
Defensive security vulnerability tests demonstrating and cataloguing attack surfaces in the remote providers system. Tests cover command injection, cloud credential handling, container hardening, and network security, acting as both regression tests and security audit documentation.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module declarations, shared `SecurityTestContext`, `VulnerabilityStatus` enum (Vulnerable/Mitigated/Inconclusive), and `utils` module with helpers for testing command injection, plaintext credentials, container security flags, and network exposure patterns.
- `command_injection.rs` - Tests demonstrating command injection vectors: environment variable injection in `docker create`, JSON config injection via `echo | tee`, container image name injection, container ID injection in log commands, SSH parameter injection, systemd template injection, and blueprint binary path injection.
- `cloud_api.rs` - Tests for cloud credential security issues: plaintext credential storage, credential logging exposure, insecure HTTP transmission, credential persistence in temp files, environment variable exposure, API response credential leakage, and session token mishandling.
- `container.rs` - Tests for container security gaps: missing security hardening (no user, no read-only, no cap-drop), privilege escalation (privileged mode, host filesystem mounts), network isolation (host networking, excessive port exposure), secrets in environment variables, writable filesystems, missing resource limits, vulnerable base images, and dangerous capabilities.
- `network.rs` - Tests for network security weaknesses: SSH without host key verification, weak SSH key generation (1024-bit RSA), unencrypted HTTP API calls, binary installation without integrity verification, permissive firewall rules, services on all interfaces, missing monitoring/IDS, insecure protocols (Telnet, FTP, SNMP v1/v2), and DNS without DNSSEC.

## Key APIs
- `VulnerabilityStatus` - enum representing test outcomes
- `utils::test_command_injection()` - checks if injection payload appears in constructed commands
- `utils::test_plaintext_credentials()` - scans for sensitive credential patterns
- `utils::test_container_security()` - verifies Docker security flags are present
- `utils::test_network_exposure()` - detects binding to 0.0.0.0 or wildcard interfaces

## Relationships
- Self-contained test module; does not import from `blueprint_remote_providers` production code
- Documents known vulnerability patterns in SSH deployment (`ssh.rs`), container commands (`secure_commands.rs`), and cloud credential handling (`discovery.rs`)
- Tests intentionally assert `Vulnerable` status to catalogue security debt
