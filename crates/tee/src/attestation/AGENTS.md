# attestation

## Purpose
Core TEE attestation type system. Defines typed attestation reports, structured claims, hardware measurements, public key bindings, and a pluggable verifier trait for validating attestation evidence across providers.

## Contents (one hop)
### Subdirectories
- [x] `providers/` - Provider-specific attestation verifier implementations (AWS Nitro, Azure SNP, GCP Confidential, Intel TDX, AMD SEV-SNP, native)

### Files
- `mod.rs` - Re-exports all submodules and key types (`AttestationClaims`, `AttestationReport`, `AttestationFormat`, `Measurement`, `PublicKeyBinding`, `AttestationVerifier`, `VerificationLevel`, `VerifiedAttestation`)
- `claims.rs` - `AttestationClaims` struct with typed fields (platform version, debug mode, boot measurements, signer/product IDs, user data) plus extensible `custom` map
- `report.rs` - `Measurement` (algorithm + hex digest), `PublicKeyBinding` (public key + type + binding digest), `AttestationFormat` enum (NitroDocument, TdxQuote, SevSnpReport, AzureMaaToken, GcpConfidentialToken, Mock), `AttestationReport` with evidence digest and expiry checking
- `verifier.rs` - `AttestationVerifier` trait (`verify`, `supported_provider`, `verification_level`); `VerifiedAttestation` wrapper with `pub(crate)` constructor to enforce verification-before-access; `VerificationLevel` enum (Structural, Cryptographic)

## Key APIs
- `AttestationReport` - carries provider, format, timestamp, measurement, optional public key binding, typed claims, and raw evidence bytes
- `AttestationReport::evidence_digest()` - SHA-256 hex digest of the evidence
- `AttestationReport::is_expired(max_age_secs)` - freshness check
- `AttestationVerifier` trait - `fn verify(&self, report) -> Result<VerifiedAttestation, TeeError>`
- `VerifiedAttestation` - type-level proof that a report passed verification; `pub(crate) new()` prevents external bypass
- `Measurement::sha256(digest)` / `Measurement::sha384(digest)` - convenience constructors

## Relationships
- Used by `crates/tee/src/exchange/` (attestation binding in key exchange responses)
- Used by `crates/tee/src/middleware/` (`TeeLayer` injects attestation metadata into job results)
- Used by `crates/tee/src/runtime/` (backends produce `AttestationReport` from deployed workloads)
- `TeeProvider` and `TeeError` come from sibling modules in `crates/tee/src/`
