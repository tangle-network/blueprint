# middleware

## Purpose
Tower middleware for injecting TEE attestation metadata into the blueprint job pipeline. Provides a layer that stamps job results with attestation digests and a context extractor for job handlers.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports `TeeContext` and `TeeLayer`
- `tee_context.rs` - `TeeContext` extractor carrying verified attestation, provider, and deployment ID; usable as an `Extension` in job handlers to make TEE-aware decisions
- `tee_layer.rs` - `TeeLayer` (Tower `Layer`) and `TeeService` that inject `tee.attestation.digest`, `tee.provider`, and `tee.measurement` metadata keys into successful `JobResult::Ok` responses; uses `Arc<Mutex<Option<AttestationReport>>>` for live attestation updates

## Key APIs
- `TeeLayer::new()` / `TeeLayer::with_attestation(report)` - create the layer with optional initial attestation
- `TeeLayer::set_attestation(report)` - update the attestation report at runtime
- `TeeLayer::attestation_handle()` - returns `Arc<Mutex<Option<AttestationReport>>>` for external updates
- `TeeContext::with_attestation(verified)` - create context with a verified attestation
- `TeeContext::is_attested()` / `TeeContext::is_tee_active()` - convenience checks
- Metadata keys: `TEE_ATTESTATION_DIGEST_KEY`, `TEE_PROVIDER_KEY`, `TEE_MEASUREMENT_KEY`

## Relationships
- Depends on `crate::attestation::report::AttestationReport` and `crate::attestation::verifier::VerifiedAttestation`
- Integrates with `blueprint_core::JobCall`/`JobResult` and Tower `Service`/`Layer` traits
- Follows the same pattern as `TangleLayer` in the tangle-extra crate
- Applied via `Router::layer(TeeLayer::new())` in the blueprint pipeline
