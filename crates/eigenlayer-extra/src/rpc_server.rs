use crate::generic_task_aggregation::{
    AggregationError, EigenTask, ResponseSender, SignedTaskResponse, TaskAggregator, TaskResponse,
};
use blueprint_core::{debug, error, info};
use jsonrpc_core::{Error as RpcError, IoHandler, Params, Value};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::oneshot;

/// Generic JSON-RPC server for handling task processing
pub struct TaskAggregatorServer<T, R, S, E>
where
    T: EigenTask,
    R: TaskResponse,
    S: ResponseSender<T, R> + Clone + Send + Sync + 'static,
    E: DeserializeOwned + Debug + Send + 'static,
{
    /// The task aggregator instance
    aggregator: Arc<TaskAggregator<T, R, S>>,

    /// Converter function from external task response to generic task response
    response_converter: Arc<dyn Fn(E) -> SignedTaskResponse<R> + Send + Sync>,

    /// The server address
    address: SocketAddr,

    /// Server shutdown sender
    shutdown_sender: Option<oneshot::Sender<()>>,
}

impl<T, R, S, E> TaskAggregatorServer<T, R, S, E>
where
    T: EigenTask,
    R: TaskResponse,
    S: ResponseSender<T, R> + Clone + Send + Sync + 'static,
    E: DeserializeOwned + Debug + Send + 'static,
{
    /// Create a new task aggregator server
    ///
    /// # Errors
    /// - [`AggregationError::ContractError`] - If the address is invalid
    pub fn new<F>(
        aggregator: Arc<TaskAggregator<T, R, S>>,
        address: impl AsRef<str>,
        response_converter: F,
    ) -> Result<Self, AggregationError>
    where
        F: Fn(E) -> SignedTaskResponse<R> + Send + Sync + 'static,
    {
        // Parse the socket address
        let address = address.as_ref().parse().map_err(|e| {
            AggregationError::ContractError(format!("Failed to parse address: {}", e))
        })?;

        Ok(Self {
            aggregator,
            response_converter: Arc::new(response_converter),
            address,
            shutdown_sender: None,
        })
    }

    /// Start the server
    ///
    /// # Errors
    /// - [`AggregationError::ServiceInitError`] - If the server is already running
    pub fn start(&mut self) -> Result<(), AggregationError> {
        if self.shutdown_sender.is_some() {
            return Err(AggregationError::ServiceInitError(
                "Server is already running".to_string(),
            ));
        }

        // Create a new JSON-RPC handler
        let mut io = IoHandler::new();

        // Clone the values needed for the handler
        let aggregator = Arc::clone(&self.aggregator);
        let converter = Arc::clone(&self.response_converter);

        // Add method for processing signed responses
        io.add_method("process_signed_task_response", move |params: Params| {
            let aggregator = Arc::clone(&aggregator);
            let converter = Arc::clone(&converter);

            async move {
                // Parse the outer structure first
                let outer_params: Value = params.parse()?;

                // Extract the inner "params" object
                let inner_params = outer_params
                    .get("params")
                    .ok_or_else(|| RpcError::invalid_params("Missing 'params' field"))?;

                // Parse the response
                let external_response: E =
                    serde_json::from_value(inner_params.clone()).map_err(|e| {
                        error!("Failed to parse response: {}", e);
                        RpcError::invalid_params(format!("Invalid response format: {}", e))
                    })?;

                // Convert to generic format
                let generic_response = converter(external_response);
                debug!(
                    "Received task response for task {}",
                    generic_response.response.reference_task_index()
                );

                // Process the response through the aggregator
                aggregator.process_signed_response(generic_response).await;

                Ok(Value::Bool(true))
            }
        });

        // Create a channel to signal server shutdown
        let (shutdown_sender, shutdown_receiver) = oneshot::channel();

        // Start the server
        let server = ServerBuilder::new(io)
            .cors(DomainsValidation::AllowOnly(vec![
                AccessControlAllowOrigin::Any,
            ]))
            .start_http(&self.address)
            .map_err(|e| {
                AggregationError::ServiceInitError(format!("Failed to start server: {}", e))
            })?;

        info!("Task aggregator RPC server running at {}", self.address);

        // Create a close handle before we move the server
        let close_handle = server.close_handle();

        // Spawn a task to handle the server
        tokio::spawn(async move {
            tokio::select! {
                // Wait for shutdown signal
                _ = shutdown_receiver => {
                    info!("Received shutdown signal, stopping server");
                    close_handle.close();
                }
                // Keep server running
                () = async { server.wait(); } => {
                    info!("Server stopped unexpectedly");
                }
            }

            info!("Server shutdown complete");
        });

        self.shutdown_sender = Some(shutdown_sender);

        Ok(())
    }

    /// Stop the server
    ///
    /// # Errors
    /// - [`AggregationError::ServiceInitError`] - If the server is not running
    pub fn stop(&mut self) -> Result<(), AggregationError> {
        if let Some(sender) = self.shutdown_sender.take() {
            let _ = sender.send(());
            info!("Sent shutdown signal to RPC server");
            Ok(())
        } else {
            Err(AggregationError::ServiceInitError(
                "Server is not running".to_string(),
            ))
        }
    }
}
