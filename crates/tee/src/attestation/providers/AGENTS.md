# providers

## Purpose
Platform-specific TEE attestation verifier implementations. Each module implements the `AttestationVerifier` trait for a particular TEE platform, validating attestation reports by checking measurements and debug mode. All modules are feature-gated.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Feature-gated module declarations. TDX and SEV-SNP share the `native` module; separate `tdx` and `sev_snp` modules provide backward-compatible convenience wrappers.
- `native.rs` - (features: `tdx`, `sev-snp`) Unified verifier for ioctl-based TEE platforms. `NativePlatform` enum dispatches between TDX (MRTD validation, `/dev/tdx_guest`) and SEV-SNP (launch measurement, `/dev/sev-guest`). `NativeVerifier` implements `AttestationVerifier` with configurable expected measurement and debug allowance.
- `tdx.rs` - (feature: `tdx`) `TdxVerifier` wrapping `NativeVerifier` with TDX defaults. Builder methods `with_expected_mrtd()` and `allow_debug()`. Delegates `verify()` to `NativeVerifier`.
- `sev_snp.rs` - (feature: `sev-snp`) `SevSnpVerifier` wrapping `NativeVerifier` with SEV-SNP defaults. Builder methods `with_expected_measurement()` and `allow_debug()`. Delegates `verify()` to `NativeVerifier`.
- `aws_nitro.rs` - (feature: `aws-nitro`) `NitroVerifier` for AWS Nitro Enclave attestation documents. Validates PCR0 measurement and debug mode. COSE signature and certificate chain verification not yet implemented.
- `azure_snp.rs` - (feature: `azure-snp`) `AzureSnpVerifier` for Azure SEV-SNP/SKR attestation. Validates measurement digest and debug mode. MAA token signature validation not yet implemented.
- `gcp_confidential.rs` - (feature: `gcp-confidential`) `GcpConfidentialVerifier` for GCP Confidential Space tokens. Validates measurement and debug mode. Token signature and workload identity validation not yet implemented.

## Key APIs (no snippets)
- **Trait**: `AttestationVerifier` (from parent `verifier` module) with `verify(report) -> Result<VerifiedAttestation>` and `supported_provider() -> TeeProvider`
- **Types**: `NativeVerifier`, `NativePlatform`, `TdxVerifier`, `SevSnpVerifier`, `NitroVerifier`, `AzureSnpVerifier`, `GcpConfidentialVerifier`
- **Common pattern**: Each verifier has `new()`, builder methods (`with_expected_*()`, `allow_debug()`), and implements `AttestationVerifier`

## Relationships
- **Depends on**: `crate::attestation::report::AttestationReport`, `crate::attestation::verifier::{AttestationVerifier, VerifiedAttestation}`, `crate::config::TeeProvider`, `crate::errors::TeeError`
- **Used by**: TEE attestation verification pipeline; callers select the appropriate verifier based on `TeeProvider` and call `verify()` on incoming attestation reports
