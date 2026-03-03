# TEE + Container Source Audit Gap Closure (2026-03-03)

This document maps security/TEE claims to concrete code paths and tests implemented in
`feat/tee-container-audit-gap-closure`.

## Claims to code + tests

| Claim | Code path | Test evidence |
| --- | --- | --- |
| Source fallback ordering is deterministic across source types. | `crates/manager/src/protocol/tangle/event_handler.rs` (`ordered_source_indices`, `source_priority`, `ensure_service_running`) | `crates/manager/src/protocol/tangle/event_handler.rs` tests: `deterministic_order_prefers_native_then_container_then_testing`, `deterministic_order_prefers_container_when_requested`, `deterministic_order_is_stable_for_wasm_preference` |
| Source launch decisions are observable at runtime. | `crates/manager/src/protocol/tangle/event_handler.rs` (logs for preferred source, ordered source list, per-attempt source kind/runtime path, selected source) and `crates/manager/src/protocol/eigenlayer/event_handler.rs` (runtime target + runtime path logs) | Covered by existing protocol execution tests plus compile-time checked logging callsites in the paths above |
| TEE runtime selection fails closed with structured prerequisite errors. | `crates/manager/src/error.rs` (`TeeRuntimeUnavailable`, `TeePrerequisiteMissing`), `crates/eigenlayer-extra/src/registration.rs` (`RuntimeTarget::Tee`), `crates/manager/src/protocol/eigenlayer/event_handler.rs` (TEE runtime gate + `TEE_BACKEND` prerequisite), `crates/manager/src/rt/container/mod.rs` (requires Kata when `require_tee=true`) | `crates/eigenlayer-extra/src/registration.rs` tests: `test_runtime_target_parses_tee`, `test_validation_tee_requires_container_image` |
| Malformed container source metadata is linted and rejected before launch. | `crates/manager/src/protocol/tangle/metadata.rs` (`convert_container_source` validation for missing fields, URL scheme, whitespace) | `crates/manager/src/protocol/tangle/metadata.rs` tests: `skips_malformed_container_source_missing_tag`, `skips_malformed_container_source_with_registry_scheme` |
| EigenLayer runtime supports explicit `tee` target selection. | `crates/eigenlayer-extra/src/registration.rs` (`RuntimeTarget::Tee`, display/parse/validation), `crates/manager/src/protocol/eigenlayer/event_handler.rs` (`RuntimeTarget::Tee` arm) | `crates/eigenlayer-extra/src/registration.rs` tests above |
