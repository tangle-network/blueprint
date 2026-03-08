# src

## Purpose
HTTP microservice for BLS signature aggregation on Tangle v2. Collects signatures from operators via REST endpoints, aggregates them when threshold is met (count-based or stake-weighted), and returns aggregated signature/pubkey ready for on-chain submission.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Module root and public API facade; `run()` entry point for starting the HTTP server.
  - **Key items**: `run()`, module re-exports, `AggregationService`, `AggregationServiceClient`
- `service.rs` - Core orchestration: configuration, signature verification, BLS aggregation logic.
  - **Key items**: `AggregationService`, `ServiceConfig` (verify_on_submit, validate_output, default_task_ttl), `init_task()`, `submit_signature()`, `get_aggregated_result()`, `mark_submitted()`, `start_cleanup_worker()`
  - **Interactions**: Calls `ArkBlsBn254::verify()` per submission, `ArkBlsBn254::aggregate()` for final result
- `state.rs` - Thread-safe in-memory state management with TaskState tracking and threshold logic.
  - **Key items**: `AggregationState`, `TaskState`, `ThresholdType` (Count, StakeWeighted), signer bitmap (U256, up to 256 operators), `init_task()`, `submit_signature()`, `cleanup()`
  - **Interactions**: Uses `parking_lot::RwLock<HashMap<TaskId, TaskState>>` for concurrent access
- `types.rs` - Request/response DTOs and wire types with hex serialization.
  - **Key items**: `TaskId`, `ThresholdConfig`, `InitTaskRequest/Response`, `SubmitSignatureRequest/Response`, `GetStatusResponse`, `AggregatedResultResponse`
- `api.rs` - Axum HTTP endpoint handlers and routing.
  - **Key items**: 7 endpoints: `GET /health`, `GET /v1/stats`, `POST /v1/tasks/{init,submit,status,aggregate,mark-submitted}`
  - **Interactions**: Uses `axum::State<Arc<AggregationService>>` extractor
- `client.rs` - Async HTTP client for operators (feature-gated on `client`).
  - **Key items**: `AggregationServiceClient`, `wait_for_threshold()` polling helper, `ClientError` enum
- `persistence.rs` - Optional data durability with trait-based backends.
  - **Key items**: `PersistenceBackend` trait (`save_task`, `load_task`, `delete_task`), `NoPersistence` (no-op), `FilePersistence` (JSON, atomic writes via temp+rename)

## Key APIs (no snippets)
- **Types**: `AggregationService` (main orchestrator), `AggregationState` (thread-safe state), `AggregationServiceClient` (HTTP client), `TaskState` (per-task tracking with bitmap)
- **Functions**: `init_task()`, `submit_signature()`, `get_status()`, `get_aggregated_result()`, `mark_submitted()`, `start_cleanup_worker()`
- **Threshold**: `ThresholdType::Count(u32)` or `ThresholdType::StakeWeighted(u16 bps)`

## Relationships
- **Depends on**: `blueprint-crypto-bn254` (BLS signatures: G1/G2 points, aggregation), `axum` (HTTP), `tokio` (async), `alloy-primitives::U256` (signer bitmap), `parking_lot` (RwLock), `reqwest` (client, optional)
- **Used by**: Tangle operators for off-chain signature collection before on-chain submission
- **Data/control flow**:
  - Init task -> operators submit signatures -> verify each -> track in bitmap
  - When threshold met -> aggregate BLS sigs/pubkeys -> return result
  - Mark submitted -> prevents re-aggregation -> cleanup worker removes expired tasks

## Notes
- Message format: `8B BE(service_id) || 8B BE(call_id) || keccak256(output)` for deterministic signing
- Signer bitmap: U256 supports up to 256 operators; bit N = operator N signed
- Stake-weighted: basis points (0-10000) for precision
- Persistence is trait-based; currently file-based or in-memory only
- 56+ unit tests across all modules
