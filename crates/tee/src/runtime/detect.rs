//! TEE hardware detection.
//!
//! Probes the current environment for TEE hardware and returns the detected
//! provider, if any. Used by `BlueprintRunner` when `TeeMode::Auto` is set
//! (the default) to activate TEE transparently.
//!
//! All probes use filesystem checks (device nodes, sysfs, ACPI tables) — no
//! network calls or subprocess spawning. Device paths are validated as
//! character devices to prevent spoofing via `touch /dev/nsm`.

use crate::config::TeeProvider;

/// Probe the current environment for TEE hardware.
///
/// Returns the detected provider, or `None` if no TEE is available.
/// Probes are ordered by specificity: device files first, then env vars.
/// Device nodes are verified as character devices, not just checked for
/// existence, to prevent trivial spoofing.
pub fn detect_tee_provider() -> Option<TeeProvider> {
    // AWS Nitro: attempt to open /dev/nsm and verify it's a real char device.
    // A fake file created with `touch` won't be a character device.
    if let Ok(file) = std::fs::File::open("/dev/nsm") {
        use std::os::unix::fs::FileTypeExt;
        if let Ok(metadata) = file.metadata() {
            if metadata.file_type().is_char_device() {
                return Some(TeeProvider::AwsNitro);
            }
        }
    }

    // Intel TDX: character device check (kernel exposes two possible paths)
    if is_char_device("/dev/tdx-guest") {
        return Some(TeeProvider::IntelTdx);
    }
    if is_char_device("/dev/tdx_guest") {
        return Some(TeeProvider::IntelTdx);
    }
    // ACPI table fallback (read-only sysfs, can't be spoofed without root)
    if std::path::Path::new("/sys/firmware/acpi/tables/TDEL").exists()
        || std::path::Path::new("/sys/firmware/acpi/tables/CCEL").exists()
    {
        return Some(TeeProvider::IntelTdx);
    }

    // AMD SEV-SNP: character device check
    if is_char_device("/dev/sev-guest") {
        return Some(TeeProvider::AmdSevSnp);
    }

    // GCP Confidential Space: attestation token file (only exists in real
    // Confidential Space, more robust than env var alone)
    if std::path::Path::new(
        "/run/container_launcher/attestation_verifier_claims_token",
    )
    .exists()
    {
        return Some(TeeProvider::GcpConfidential);
    }
    // Fallback: env var (less secure but catches more configurations)
    if std::env::var("CONFIDENTIAL_SPACE_VERSION").is_ok() {
        return Some(TeeProvider::GcpConfidential);
    }

    // Azure CVM: sysfs probes (no network calls, no subprocess)
    if azure_cvm_detected() {
        return Some(TeeProvider::AzureSnp);
    }

    None
}

/// Check if a path is a character device (not a regular file or symlink).
fn is_char_device(path: &str) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::FileTypeExt;
        std::fs::metadata(path)
            .map(|m| m.file_type().is_char_device())
            .unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        false
    }
}

/// Azure CVM detection via sysfs — no subprocess, no network call.
/// Azure Confidential VMs expose SEV-SNP status in DMI/SMBIOS data.
fn azure_cvm_detected() -> bool {
    let is_azure = std::fs::read_to_string("/sys/class/dmi/id/board_vendor")
        .map(|v| v.trim().contains("Microsoft"))
        .unwrap_or(false);
    if !is_azure {
        return false;
    }
    // SEV-SNP capability (present on Azure DCasv5/ECasv5 series)
    std::path::Path::new("/sys/kernel/mm/sev").exists() || is_char_device("/dev/sev-guest")
}
