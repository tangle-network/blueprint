use crate::sources::Status;
use std::path::PathBuf;
use tokio::sync::mpsc::UnboundedReceiver;
use tracing::{info, warn};

#[auto_impl::auto_impl(Box)]
#[dynosaur::dynosaur(pub(crate) DynBinarySourceFetcher)]
pub trait BinarySourceFetcher {
    fn get_binary(&self) -> impl Future<Output = crate::error::Result<PathBuf>> + Send;
}

unsafe impl Send for DynBinarySourceFetcher<'_> {}
unsafe impl Sync for DynBinarySourceFetcher<'_> {}

#[must_use]
pub fn generate_running_process_status_handle(
    process: tokio::process::Child,
    service_name: &str,
) -> (UnboundedReceiver<Status>, tokio::sync::oneshot::Sender<()>) {
    let (stop_tx, stop_rx) = tokio::sync::oneshot::channel::<()>();
    let (status_tx, status_rx) = tokio::sync::mpsc::unbounded_channel::<Status>();
    let service_name = service_name.to_string();

    let task = async move {
        info!("Starting process execution for {service_name}");
        let _ = status_tx.send(Status::Running).ok();
        let output = process.wait_with_output().await;
        if output.is_ok() {
            let _ = status_tx.send(Status::Finished).ok();
        } else {
            let _ = status_tx.send(Status::Error).ok();
        }

        warn!("Process for {service_name} exited: {output:?}");
    };

    let task = async move {
        tokio::select! {
            _ = stop_rx => {},
            () = task => {},
        }
    };

    tokio::spawn(task);
    (status_rx, stop_tx)
}
