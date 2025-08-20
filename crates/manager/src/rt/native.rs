use super::service::Status;
use tokio::sync::mpsc::UnboundedReceiver;

/// Handle for a natively (no sandbox) running service
pub struct ProcessHandle {
    status: UnboundedReceiver<Status>,
    cached_status: Status,
    abort_handle: tokio::sync::oneshot::Sender<()>,
}

impl ProcessHandle {
    #[must_use]
    pub fn new(
        mut status: UnboundedReceiver<Status>,
        abort_handle: tokio::sync::oneshot::Sender<()>,
    ) -> Self {
        let cached_status = status.try_recv().ok().unwrap_or(Status::Running);
        Self {
            status,
            cached_status,
            abort_handle,
        }
    }

    pub fn status(&mut self) -> Status {
        self.status.try_recv().ok().unwrap_or(self.cached_status)
    }

    pub async fn wait_for_status_change(&mut self) -> Option<Status> {
        self.status.recv().await
    }

    #[must_use]
    pub fn abort(self) -> bool {
        self.abort_handle.send(()).is_ok()
    }
}
