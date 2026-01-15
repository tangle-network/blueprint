use alloy_primitives::Address;
use alloy_sol_types::sol;
use blueprint_core::Job;
use blueprint_router::Router;
use blueprint_tangle_evm_extra::extract::{Caller, TangleEvmArg, TangleEvmResult};
use blueprint_tangle_evm_extra::TangleEvmLayer;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub const CREATE_DOCUMENT_JOB: u8 = 0;

sol! {
    /// Input payload sent from the Tangle contract.
    struct DocumentRequest {
        string docId;
        string contents;
    }

    /// Output payload returned back to the caller.
    struct DocumentReceipt {
        string docId;
        string contents;
        string operator;
    }
}

type DocumentStore = Arc<RwLock<HashMap<String, String>>>;

fn store() -> &'static DocumentStore {
    static STORE: OnceCell<DocumentStore> = OnceCell::new();
    STORE.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

/// Minimal job that persists a document and returns a receipt.
pub async fn create_document(
    Caller(caller): Caller,
    TangleEvmArg(request): TangleEvmArg<DocumentRequest>,
) -> TangleEvmResult<DocumentReceipt> {
    let mut docs = store().write().await;
    docs.insert(request.docId.clone(), request.contents.clone());

    let caller_address = Address::from_slice(&caller);

    TangleEvmResult(DocumentReceipt {
        docId: request.docId,
        contents: request.contents,
        operator: format!("{caller_address:#x}"),
    })
}

/// Router used by the example blueprint runner.
#[must_use]
pub fn router() -> Router {
    Router::new().route(CREATE_DOCUMENT_JOB, create_document.layer(TangleEvmLayer))
}

/// Inspect the in-memory store (handy for doctests).
pub async fn get_document(id: &str) -> Option<String> {
    let docs = store().read().await;
    docs.get(id).cloned()
}

/// Reset the document store (tests only).
pub async fn clear_store() {
    let mut docs = store().write().await;
    docs.clear();
}
