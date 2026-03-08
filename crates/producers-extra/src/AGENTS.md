# src

## Purpose
Provides non-protocol-specific event producers for the Blueprint SDK. Currently contains a cron-based producer that emits `JobCall`s on a configurable schedule.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `cron.rs` - Implements `CronJob`, a `Stream<Item = Result<JobCall, BoxError>>` that fires on a cron schedule (with seconds granularity). Wraps `tokio-cron-scheduler` and supports timezone-aware scheduling via `new_tz`. Includes an inline test verifying per-second scheduling.
- `lib.rs` - Crate root with feature-gated `cron` module declaration. Uses `document_features` for auto-generated feature docs.

## Key APIs (no snippets)
- `CronJob::new(job_id, schedule)` -- creates a cron producer using UTC timezone.
- `CronJob::new_tz(job_id, schedule, timezone)` -- creates a cron producer with a custom timezone.
- `CronJob` implements `Stream` yielding `Result<JobCall, BoxError>` -- each tick produces a `JobCall` with empty body and the configured job ID.

## Relationships
- Depends on `blueprint-core` for `JobCall`, `JobId`, `Bytes`, and error types.
- Depends on `tokio-cron-scheduler` for cron scheduling and `chrono` for timezone support.
- Used by blueprint authors who need time-triggered job execution without an external event source.
- Consumed via `blueprint-sdk` re-exports.
