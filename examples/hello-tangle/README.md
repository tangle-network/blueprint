# Hello Tangle Blueprint

This package contains a minimal Blueprint that stores short “documents” via the
Tangle EVM job system. It is intentionally tiny so it can double as the
reference implementation for the new Anvil harness.

## Running the example

```bash
# Execute the end-to-end test suite (spawns Anvil via testcontainers)
cargo test -p hello-tangle-blueprint --test anvil -- --nocapture
```

The test does the following:

1. Boots an Anvil container seeded with the latest `LocalTestnet.s.sol` state.
2. Seeds a temporary keystore with the default operator key from the LocalTestnet fixture.
3. Spins up the Blueprint runner with the router exported from `src/lib.rs`.
4. Submits an ABI-encoded `DocumentRequest` and waits for the on-chain
   `JobResultSubmitted` event, verifying the `DocumentReceipt` contents.

This flow mirrors how operator blueprints will be exercised in CI going forward.

## Extending the router

Add additional jobs just like any other Blueprint crate:

```rust
pub async fn delete_document(TangleEvmArg(doc_id): TangleEvmArg<String>) -> TangleEvmResult<()> {
    let mut docs = store().write().await;
    docs.remove(&doc_id);
    TangleEvmResult(())
}

pub fn router() -> Router {
    Router::new()
        .route(CREATE_DOCUMENT_JOB, create_document)
        .route(DELETE_DOCUMENT_JOB, delete_document)
}
```

Re-run `cargo test -p hello-tangle-blueprint --test anvil` to exercise the new
job end-to-end.
