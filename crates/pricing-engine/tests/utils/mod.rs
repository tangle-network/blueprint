use blueprint_runner::BackgroundService;
use blueprint_runner::error::RunnerError;
use blueprint_tangle_extra::extract::TangleArg;
use blueprint_tangle_extra::extract::TangleResult;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;

// Square job ID
pub const XSQUARE_JOB_ID: u8 = 0;

/// A copy of the `square` function from the `incredible-squaring` crate used for testing
pub async fn square(TangleArg(x): TangleArg<u64>) -> TangleResult<u64> {
    let result = x * x;

    // The result is then converted into a `JobResult` to be sent back to the caller.
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
