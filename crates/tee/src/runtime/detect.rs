//! TEE hardware detection.
//!
//! Probes the current environment for TEE hardware and returns the detected
//! provider, if any. Used by `BlueprintRunner` when `TeeMode::Auto` is set
//! (the default) to activate TEE transparently.

use crate::config::TeeProvider;

/// Probe the current environment for TEE hardware.
///
/// Returns the detected provider, or `None` if no TEE is available.
/// Probes are ordered by specificity: device files first, then env vars,
/// then network endpoints.
pub fn detect_tee_provider() -> Option<TeeProvider> {
    // AWS Nitro — Nitro Security Module device
    if std::path::Path::new("/dev/nsm").exists() {
        return Some(TeeProvider::AwsNitro);
    }

    // Intel TDX — guest device or ACPI table
    if std::path::Path::new("/dev/tdx-guest").exists()
        || std::path::Path::new("/sys/firmware/acpi/tables/TDEL").exists()
    {
        return Some(TeeProvider::IntelTdx);
    }

    // AMD SEV-SNP — guest device
    if std::path::Path::new("/dev/sev-guest").exists() {
        return Some(TeeProvider::AmdSevSnp);
    }

    // GCP Confidential Space — env var set by the runtime
    if std::env::var("CONFIDENTIAL_SPACE_VERSION").is_ok() {
        return Some(TeeProvider::GcpConfidential);
    }

    // Azure CVM — IMDS attestation endpoint
    if azure_imds_attestation_available() {
        return Some(TeeProvider::AzureSnp);
    }

    None
}

/// Quick synchronous check for Azure IMDS attestation endpoint.
/// Uses a 1-second connect timeout to avoid blocking startup on non-Azure hosts.
fn azure_imds_attestation_available() -> bool {
    std::process::Command::new("curl")
        .args([
            "-sf",
            "--connect-timeout",
            "1",
            "-H",
            "Metadata:true",
            "http://169.254.169.254/metadata/attested/document?api-version=2021-02-01",
        ])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
