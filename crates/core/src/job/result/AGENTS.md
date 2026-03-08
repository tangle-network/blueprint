# result

## Purpose
Job result type system. Defines the `JobResult` enum returned from job handlers, plus conversion traits (`IntoJobResult`, `IntoJobResultParts`) that allow diverse return types from jobs.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - `JobResult<T, E>` enum (`Ok{head: Parts, body}` / `Err(E)`), `Parts` struct holding `MetadataMap`, `Void` marker for jobs that produce no result. Provides `map`, `map_err`, `into_parts`, body/metadata accessors.
- `into_job_result.rs` - `IntoJobResult` trait (`fn into_job_result(self) -> Option<JobResult>`). Blanket impls for `Void`, `()`, `Bytes`, `String`, `Vec<u8>`, `Result<T,E>`, `Option<T>`, tuples of `(IntoJobResultParts..., IntoJobResult)`, and many byte-like types. Uses `all_the_tuples_no_last_special_case!` macro for tuple impls up to arity 16.
- `into_job_result_parts.rs` - `IntoJobResultParts` trait for attaching metadata to results. `JobResultParts` wrapper struct. Impls for `MetadataMap`, `[(K,V); N]` arrays, `Option<T>`, tuples. `TryIntoMetadataError` for fallible key/value conversion.

## Key APIs
- `JobResult<T, E>` -- the core result enum consumed by the job execution pipeline
- `Void` -- sentinel type indicating a job intentionally produces no result
- `IntoJobResult` trait -- convert arbitrary handler return types into `Option<JobResult>`
- `IntoJobResultParts` trait -- attach metadata to a `JobResultParts` being built
- `JobResultParts` -- intermediate builder wrapping a `JobResult` during tuple assembly

## Relationships
- Depends on `crate::error::Error`, `crate::metadata::{MetadataMap, MetadataValue}`
- Used by `Job` trait, `JobService`, extract module, and the router/runner pipeline
- Re-exported through `blueprint-sdk`
