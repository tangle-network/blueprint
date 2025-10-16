//! Security Vulnerability Tests
//!
//! Consolidated security testing suite covering all identified vulnerabilities.
//! Replaces scattered security test files with organized, comprehensive coverage.
//!
//! These tests demonstrate actual security vulnerabilities that exist in the codebase
//! and must be addressed for production deployment. All tests are legitimate defensive
//! security testing to identify and prevent security issues.

/// Command injection vulnerabilities in SSH deployment system
pub mod command_injection;

/// Cloud provider API security vulnerabilities
pub mod cloud_api;

/// Container deployment and runtime security issues
pub mod container;

/// SSH and network communication security flaws
pub mod network;

use std::collections::HashMap;

/// Shared security test utilities
pub struct SecurityTestContext {
    pub temp_dir: tempfile::TempDir,
    pub test_id: String,
}

impl SecurityTestContext {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = tempfile::TempDir::new()?;
        let test_id = format!("security-test-{}", chrono::Utc::now().timestamp());

        Ok(Self { temp_dir, test_id })
    }
}

/// Security test result indicating vulnerability status
#[derive(Debug, PartialEq)]
pub enum VulnerabilityStatus {
    /// Vulnerability exists and is exploitable
    Vulnerable,
    /// Vulnerability is mitigated
    Mitigated,
    /// Test inconclusive
    Inconclusive,
}

/// Common security testing utilities
pub mod utils {
    use super::*;

    /// Test for command injection in shell command construction
    pub fn test_command_injection(command: &str, injection_payload: &str) -> super::VulnerabilityStatus {
        if command.contains(injection_payload) {
            super::VulnerabilityStatus::Vulnerable
        } else {
            super::VulnerabilityStatus::Mitigated
        }
    }

    /// Test for plaintext credential storage
    pub fn test_plaintext_credentials(data: &str) -> super::VulnerabilityStatus {
        let sensitive_patterns = [
            "AKIA",           // AWS access keys
            "secret_key",     // Generic secret keys
            "password",       // Passwords
            "api_key",        // API keys
            "private_key",    // Private keys
        ];

        for pattern in &sensitive_patterns {
            if data.to_lowercase().contains(&pattern.to_lowercase()) {
                return super::VulnerabilityStatus::Vulnerable;
            }
        }

        super::VulnerabilityStatus::Mitigated
    }

    /// Test for container security hardening
    pub fn test_container_security(docker_command: &str) -> super::VulnerabilityStatus {
        let security_flags = [
            "--user",                          // Non-root user
            "--read-only",                     // Read-only filesystem
            "--security-opt no-new-privileges", // Privilege escalation protection
            "--cap-drop ALL",                  // Capability restrictions
            "--tmpfs",                         // Tmpfs isolation
        ];

        for flag in &security_flags {
            if !docker_command.contains(flag) {
                return super::VulnerabilityStatus::Vulnerable;
            }
        }

        super::VulnerabilityStatus::Mitigated
    }

    /// Test for network exposure vulnerabilities
    pub fn test_network_exposure(command: &str) -> super::VulnerabilityStatus {
        if command.contains("0.0.0.0") || command.contains("*:") {
            super::VulnerabilityStatus::Vulnerable
        } else {
            super::VulnerabilityStatus::Mitigated
        }
    }
}