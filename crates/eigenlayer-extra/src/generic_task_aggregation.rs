use alloy_primitives::{FixedBytes, hex, keccak256};
use blueprint_core::{info, error};
use eigensdk::client_avsregistry::reader::AvsRegistryChainReader;
use eigensdk::crypto_bls::Signature;
use eigensdk::services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
use eigensdk::services_blsaggregation::bls_agg::{
    AggregateReceiver, BlsAggregatorService, ServiceHandle, TaskMetadata, TaskSignature,
};
use eigensdk::services_blsaggregation::bls_aggregation_service_error::BlsAggregationServiceError;
use eigensdk::services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use eigensdk::services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use eigensdk::types::avs::{TaskIndex, TaskResponseDigest};
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::{Mutex, Notify, RwLock};
use tokio::task::JoinHandle;
use tokio::time::interval;

/// Error type for the generic task aggregation system
#[derive(Error, Debug)]
pub enum AggregationError {
    /// Task not found
    #[error("Task not found: {0}")]
    TaskNotFound(String),

    /// BLS aggregation service error
    #[error("BLS aggregation service error: {0}")]
    BlsAggregationError(#[from] BlsAggregationServiceError),

    /// Contract interaction error
    #[error("Contract interaction error: {0}")]
    ContractError(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON serialization error
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Aggregator already stopped
    #[error("Aggregator already stopped")]
    AlreadyStopped,

    /// Service initialization error
    #[error("Service initialization error: {0}")]
    ServiceInitError(String),

    /// Tokio task join error
    #[error("Tokio task join error: {0}")]
    TaskJoinError(#[from] tokio::task::JoinError),
}

/// Type alias for using Result with [`AggregationError`]
pub type Result<T> = std::result::Result<T, AggregationError>;

/// Convenience type alias for the BLS Aggregation Service used in EigenLayer
pub type BlsAggServiceInMemory = BlsAggregatorService<
    AvsRegistryServiceChainCaller<AvsRegistryChainReader, OperatorInfoServiceInMemory>,
>;

/// Trait for a generic EigenLayer task
pub trait EigenTask: Clone + Send + Sync + 'static {
    /// Get the task index/ID
    fn task_index(&self) -> TaskIndex;

    /// Get the block at which the task was created
    fn created_block(&self) -> u32;

    /// Get the quorum numbers this task is associated with
    fn quorum_numbers(&self) -> Vec<u8>;

    /// Get the quorum threshold percentage required for this task
    fn quorum_threshold_percentage(&self) -> u8;

    /// Encode the task to bytes for hashing
    fn encode(&self) -> Vec<u8>;

    /// Create a digest of the task
    fn digest(&self) -> FixedBytes<32> {
        keccak256(self.encode())
    }
}

/// Trait for a generic task response
pub trait TaskResponse: Clone + Send + Sync + 'static {
    /// Get the task index this response refers to
    fn reference_task_index(&self) -> TaskIndex;

    /// Encode the response to bytes for hashing
    fn encode(&self) -> Vec<u8>;

    /// Create a digest of the response
    fn digest(&self) -> FixedBytes<32> {
        keccak256(self.encode())
    }
}

/// A signed task response containing the response data, signature, and operator ID
#[derive(Clone, Debug)]
pub struct SignedTaskResponse<R: TaskResponse> {
    /// The task response data
    pub response: R,
    /// The BLS signature
    pub signature: Signature,
    /// The operator's ID that signed the response
    pub operator_id: FixedBytes<32>,
}

impl<R: TaskResponse> SignedTaskResponse<R> {
    /// Create a new signed task response
    pub fn new(response: R, signature: Signature, operator_id: FixedBytes<32>) -> Self {
        Self {
            response,
            signature,
            operator_id,
        }
    }

    /// Get the signature
    pub fn signature(&self) -> Signature {
        self.signature.clone()
    }

    /// Get the operator ID
    pub fn operator_id(&self) -> FixedBytes<32> {
        self.operator_id
    }
}

/// Trait for sending aggregated responses to EigenLayer contracts
pub trait ResponseSender<T: EigenTask, R: TaskResponse>: Send + Sync + Clone + 'static {
    /// Future type returned by `send_aggregated_response`
    type Future: std::future::Future<Output = Result<()>> + Send + 'static;

    /// Send an aggregated response to the contract
    fn send_aggregated_response(
        &self,
        task: &T,
        response: &R,
        aggregation_result: BlsAggregationServiceResponse,
    ) -> Self::Future;
}

/// Configuration for the generic task aggregator
#[derive(Clone)]
pub struct AggregatorConfig {
    /// How often to process the cached responses (in seconds)
    pub processing_interval: u64,
    /// Maximum number of responses to cache per task
    pub max_responses_per_task: usize,
    /// Maximum number of retries for sending responses
    pub send_retries: usize,
    /// Delay between retries (in seconds)
    pub retry_delay: u64,
}

impl Default for AggregatorConfig {
    fn default() -> Self {
        Self {
            processing_interval: 6,
            max_responses_per_task: 1000,
            send_retries: 3,
            retry_delay: 2,
        }
    }
}

/// Generic task aggregator for EigenLayer AVS
pub struct TaskAggregator<T, R, S>
where
    T: EigenTask,
    R: TaskResponse,
    S: ResponseSender<T, R>,
{
    /// Map of task index to task
    pub tasks: Arc<RwLock<HashMap<TaskIndex, T>>>,
    /// Map of task index to a map of response digest to response
    pub responses: Arc<RwLock<HashMap<TaskIndex, HashMap<TaskResponseDigest, R>>>>,
    /// Queue of signed responses waiting to be processed
    pub response_cache: Arc<Mutex<VecDeque<SignedTaskResponse<R>>>>,
    /// The BLS aggregation service handle
    pub bls_service: ServiceHandle,
    /// The receiver for aggregated results
    pub aggregate_receiver: Arc<Mutex<AggregateReceiver>>,
    /// The contract response sender
    pub response_sender: S,
    /// Configuration
    pub config: AggregatorConfig,
    /// Shutdown notification
    pub shutdown: Arc<(Notify, Mutex<bool>)>,
    /// Running task handles
    pub task_handles: Arc<Mutex<Vec<JoinHandle<()>>>>,
    _phantom: PhantomData<(T, R)>,
}

impl<T, R, S> TaskAggregator<T, R, S>
where
    T: EigenTask,
    R: TaskResponse,
    S: ResponseSender<T, R>,
{
    /// Create a new task aggregator
    pub fn new(
        bls_service: BlsAggServiceInMemory,
        response_sender: S,
        config: AggregatorConfig,
    ) -> Self {
        let (service_handle, aggregate_receiver) = bls_service.start();

        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            responses: Arc::new(RwLock::new(HashMap::new())),
            response_cache: Arc::new(Mutex::new(VecDeque::new())),
            bls_service: service_handle,
            aggregate_receiver: Arc::new(Mutex::new(aggregate_receiver)),
            response_sender,
            config,
            shutdown: Arc::new((Notify::new(), Mutex::new(false))),
            task_handles: Arc::new(Mutex::new(Vec::new())),
            _phantom: PhantomData,
        }
    }

    /// Register a new task with the aggregator
    ///
    /// # Errors
    /// - [`AggregationError::BlsAggregationError`] - If the task index already exists in the aggregator
    pub async fn register_task(&self, task: T) -> Result<()> {
        let task_index = task.task_index();
        let mut tasks = self.tasks.write().await;

        // Store the task by its index
        tasks.insert(task_index, task.clone());

        // Initialize empty response map for this task
        let mut responses = self.responses.write().await;
        responses.entry(task_index).or_insert_with(HashMap::new);

        // Initialize the task in the BLS service
        let task_metadata = TaskMetadata::new(
            task_index,
            u64::from(task.created_block()),
            task.quorum_numbers(),
            vec![task.quorum_threshold_percentage(); task.quorum_numbers().len()],
            Duration::from_secs(self.config.processing_interval * 10), // Set a reasonable expiry time
        );

        self.bls_service
            .initialize_task(task_metadata)
            .await
            .map_err(AggregationError::BlsAggregationError)?;

        Ok(())
    }

    /// Process a signed task response
    pub async fn process_signed_response(&self, signed_response: SignedTaskResponse<R>) {
        info!("Caching signed response");
        // Add to cache for processing
        self.response_cache.lock().await.push_back(signed_response);
    }

    /// Start the aggregator service
    pub async fn start(&self) {
        // Start the processing loop
        let process_handle = self.start_processing_loop();

        // Start the aggregation result handling loop
        let aggregation_handle = self.start_aggregation_handling_loop();

        // Store the handles
        let mut handles = self.task_handles.lock().await;
        handles.push(process_handle);
        handles.push(aggregation_handle);
    }

    /// Stop the aggregator service
    ///
    /// # Errors
    /// - [`AggregationError::AlreadyStopped`] - If the service is already stopped
    pub async fn stop(&self) -> Result<()> {
        // Set shutdown flag
        let (notify, is_shutdown) = &*self.shutdown;
        let mut shutdown_lock = is_shutdown.lock().await;

        if *shutdown_lock {
            return Err(AggregationError::AlreadyStopped);
        }

        info!("Setting shutdown flag for task aggregator");
        *shutdown_lock = true;
        drop(shutdown_lock);
        
        // Notify all waiters to check the shutdown flag
        notify.notify_waiters();
        
        // Get handles but don't hold the lock
        let handles = {
            let mut handles_lock = self.task_handles.lock().await;
            std::mem::take(&mut *handles_lock)
        };
        
        info!("Waiting for {} task aggregator background tasks to complete", handles.len());
        
        // Use a timeout to wait for tasks to complete
        let timeout = Duration::from_secs(5);
        for (i, handle) in handles.into_iter().enumerate() {
            if !handle.is_finished() {
                match tokio::time::timeout(timeout, handle).await {
                    Ok(_) => info!("Task aggregator background task completed successfully"),
                    Err(_) => {
                        error!("Task aggregator background task did not complete within timeout, aborting");
                        // We can't abort the handle directly, but we've set the shutdown flag
                        // which should cause it to exit eventually
                    }
                }
            }
        }
        
        info!("Task aggregator shutdown complete");
        Ok(())
    }

    // Private helper methods

    fn start_processing_loop(&self) -> JoinHandle<()> {
        let tasks = self.tasks.clone();
        let responses = self.responses.clone();
        let response_cache = self.response_cache.clone();
        let bls_service = self.bls_service.clone();
        let shutdown = self.shutdown.clone();
        let interval_secs = self.config.processing_interval;

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(interval_secs));

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        // Check shutdown status first
                        if *shutdown.1.lock().await {
                            break;
                        }

                        // Get responses to process while holding the lock briefly
                        let responses_to_process = {
                            let mut cache = response_cache.lock().await;
                            std::mem::take(&mut *cache)
                        };

                        for signed_resp in responses_to_process {
                            let task_index = signed_resp.response.reference_task_index();

                            // Check if we have the task
                            let task_exists = tasks.read().await.contains_key(&task_index);

                            if !task_exists {
                                // Put it back in the cache for later processing
                                response_cache.lock().await.push_back(signed_resp);
                                continue;
                            }

                            // Process the response
                            match Self::process_response(
                                task_index,
                                signed_resp.clone(),
                                &responses,
                                &bls_service,
                            ).await {
                                Ok(()) => {
                                    // Successfully processed
                                }
                                Err(e) => {
                                    // Log error and put back in cache for retry
                                    blueprint_core::error!("Failed to process response: {:?}", e);
                                    response_cache.lock().await.push_back(signed_resp);
                                }
                            }
                        }
                    }
                    () = shutdown.0.notified() => {
                        if *shutdown.1.lock().await {
                            break;
                        }
                    }
                }
            }

            blueprint_core::info!("Processing loop shutdown complete");
        })
    }

    fn start_aggregation_handling_loop(&self) -> JoinHandle<()> {
        let tasks = self.tasks.clone();
        let responses = self.responses.clone();
        let aggregate_receiver = self.aggregate_receiver.clone();
        let response_sender = self.response_sender.clone();
        let shutdown = self.shutdown.clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Ok(aggregation_result) = async {
                        let mut receiver = aggregate_receiver.lock().await;
                        receiver.receive_aggregated_response().await
                    } => {
                        let task_index = aggregation_result.task_index;
                        let response_digest = aggregation_result.task_response_digest;

                        blueprint_core::info!(
                            "Received aggregation result for task {}, digest: {}",
                            task_index,
                            hex::encode(response_digest)
                        );

                        // Retrieve the task and response
                        let task_opt = tasks.read().await.get(&task_index).cloned();
                        let response_opt = responses.read().await
                            .get(&task_index)
                            .and_then(|r| r.get(&response_digest))
                            .cloned();

                        if let (Some(task), Some(response)) = (task_opt, response_opt) {
                            // Try to send the response to the contract
                            for i in 0..config.send_retries {
                                match response_sender
                                    .send_aggregated_response(&task, &response, aggregation_result.clone())
                                    .await
                                {
                                    Ok(()) => {
                                        blueprint_core::info!(
                                            "Successfully sent aggregated response for task {}",
                                            task_index
                                        );
                                        break;
                                    }
                                    Err(e) if i < config.send_retries - 1 => {
                                        blueprint_core::warn!(
                                            "Failed to send aggregated response (attempt {}/{}): {:?}",
                                            i + 1,
                                            config.send_retries,
                                            e
                                        );
                                        tokio::time::sleep(Duration::from_secs(config.retry_delay)).await;
                                    }
                                    Err(e) => {
                                        blueprint_core::error!(
                                            "Failed to send aggregated response after {} attempts: {:?}",
                                            config.send_retries,
                                            e
                                        );
                                    }
                                }
                            }
                        } else {
                            blueprint_core::error!(
                                "Missing task or response for aggregation result. Task index: {}, response digest: {}",
                                task_index,
                                hex::encode(response_digest)
                            );
                        }
                    }
                    () = shutdown.0.notified() => {
                        if *shutdown.1.lock().await {
                            break;
                        }
                    }
                }
            }

            blueprint_core::info!("Aggregation handling loop shutdown complete");
        })
    }

    async fn process_response(
        task_index: TaskIndex,
        signed_resp: SignedTaskResponse<R>,
        responses: &Arc<RwLock<HashMap<TaskIndex, HashMap<TaskResponseDigest, R>>>>,
        bls_service: &ServiceHandle,
    ) -> Result<()> {
        let response_digest = signed_resp.response.digest();

        // Check if we've already processed this response
        let response_exists = {
            let resp_map = responses.read().await;
            resp_map
                .get(&task_index)
                .is_some_and(|m| m.contains_key(&response_digest))
        };

        if response_exists {
            blueprint_core::info!(
                "Response for task {} with digest {} already processed",
                task_index,
                hex::encode(response_digest)
            );
            return Ok(());
        }

        // Create a task signature
        let task_signature = TaskSignature::new(
            task_index,
            response_digest,
            signed_resp.signature(),
            signed_resp.operator_id(),
        );

        // Process the signature through BLS service
        bls_service
            .process_signature(task_signature)
            .await
            .map_err(AggregationError::BlsAggregationError)?;

        // Store the response
        {
            let mut resp_map = responses.write().await;
            if let Some(task_responses) = resp_map.get_mut(&task_index) {
                task_responses.insert(response_digest, signed_resp.response);
            }
        }

        blueprint_core::debug!("Successfully processed signature for task {}", task_index);
        Ok(())
    }
}
