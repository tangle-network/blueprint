# producers-extra

## Purpose
Protocol-agnostic job producers (`blueprint-producers-extra`). Catch-all crate for extra producer implementations that are not tied to a specific blockchain protocol. Currently provides a cron-scheduled job producer.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Crate source: `cron.rs` (cron-scheduled job producer using `tokio-cron-scheduler`, gated on `cron` feature), `lib.rs` (conditional module exports)

### Files
- `CHANGELOG.md` - Release history
- `Cargo.toml` - Crate manifest (`blueprint-producers-extra`); depends on `blueprint-core`, `document-features`, `futures`; optional deps for cron: `chrono`, `tokio-cron-scheduler`, `tokio`
- `README.md` - Overview of available producers and feature flags

## Key APIs (no snippets)
- `cron` module (feature `cron`) -- cron-based job producer for scheduling recurring job executions via cron expressions

## Relationships
- Depends on `blueprint-core`
- Used by blueprint services that need time-based job scheduling independent of on-chain events
- Complements protocol-specific producers in `blueprint-tangle-extra` and `blueprint-evm-extra`
- Feature flags: `std`, `tracing`, `cron`
