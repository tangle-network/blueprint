use blueprint_sdk::runner::BackgroundService;
use blueprint_sdk::runner::error::RunnerError;
use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;

// Job IDs
pub const XSQUARE_JOB_ID: u32 = 0;      // Local execution
pub const XSQUARE_FAAS_JOB_ID: u32 = 1; // FaaS execution

// Job 0: Local execution - runs on blueprint operator's machine
//
// The arguments are made up of "extractors", which take a portion of the `JobCall` to convert into the
// target type.
pub async fn square(TangleArg(x): TangleArg<u64>) -> TangleResult<u64> {
    let result = x * x;
    TangleResult(result)
}

// Job 1: FaaS execution - IDENTICAL logic but runs on Lambda/serverless
//
// This demonstrates the key property: same job logic, different execution location.
// The result MUST flow through the same consumer pipeline to reach onchain.
pub async fn square_faas(TangleArg(x): TangleArg<u64>) -> TangleResult<u64> {
    let result = x * x;
    TangleResult(result)
}

#[derive(Clone)]
pub struct FooBackgroundService;

impl BackgroundService for FooBackgroundService {
    async fn start(&self) -> Result<Receiver<Result<(), RunnerError>>, RunnerError> {
        let (tx, rx) = oneshot::channel();
        tokio::spawn(async move {
            let _ = tx.send(Ok(()));
        });
        Ok(rx)
    }
}
