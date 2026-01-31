//! Aggregation-Aware Job Consumer
//!
//! This consumer automatically detects whether a job requires BLS aggregation
//! and routes to the appropriate submission path:
//! - Jobs NOT requiring aggregation: Submit directly via `submitResult`
//! - Jobs requiring aggregation: Coordinate with other operators via the
//!   aggregation service, and submit via `submitAggregatedResult`
//!
//! ## Usage
//!
//! ```rust,ignore
//! use blueprint_tangle_extra::AggregatingConsumer;
//!
//! // Create the consumer with aggregation support
//! let consumer = AggregatingConsumer::new(client)
//!     .with_aggregation_service("http://localhost:8080", bls_keypair, operator_index);
//!
//! // Use it just like TangleConsumer - it automatically handles aggregation!
//! consumer.send(job_result).await?;
//! ```

use crate::aggregation::AggregationError;
use crate::extract;
use alloy_primitives::{Address, Bytes};
use blueprint_client_tangle::{AggregationConfig, OperatorMetadata, TangleClient, ThresholdType};
use blueprint_core::JobResult;
use blueprint_core::error::BoxError;
use blueprint_std::boxed::Box;
use blueprint_std::collections::{HashMap, VecDeque};
use blueprint_std::format;
use blueprint_std::string::{String, ToString};
use blueprint_std::sync::{Arc, Mutex};
#[cfg(any(feature = "aggregation", feature = "p2p-aggregation"))]
use blueprint_std::time::Duration;
use blueprint_std::vec::Vec;
#[cfg(feature = "aggregation")]
use blueprint_tangle_aggregation_svc::{OperatorStake, ThresholdConfig};
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_util::Sink;

/// Error type for the aggregating consumer
#[derive(Debug, thiserror::Error)]
pub enum AggregatingConsumerError {
    /// Client error
    #[error("Client error: {0}")]
    Client(String),
    /// Missing metadata
    #[error("Missing metadata: {0}")]
    MissingMetadata(&'static str),
    /// Invalid metadata
    #[error("Invalid metadata: {0}")]
    InvalidMetadata(&'static str),
    /// Transaction error
    #[error("Transaction error: {0}")]
    Transaction(String),
    /// Aggregation error
    #[error("Aggregation error: {0}")]
    Aggregation(#[from] AggregationError),
    /// Aggregation service error
    #[cfg(feature = "aggregation")]
    #[error("Aggregation service error: {0}")]
    AggregationService(#[from] blueprint_tangle_aggregation_svc::ClientError),
    /// BLS crypto error
    #[cfg(feature = "aggregation")]
    #[error("BLS error: {0}")]
    Bls(String),
    /// Aggregation not configured
    #[error("Aggregation required but not configured. Call with_aggregation_service() first.")]
    AggregationNotConfigured,
}

/// Job result with parsed metadata for submission
struct PendingJobResult {
    service_id: u64,
    call_id: u64,
    job_index: u8,
    output: Bytes,
}

enum State {
    WaitingForResult,
    ProcessingSubmission(
        Pin<Box<dyn core::future::Future<Output = Result<(), AggregatingConsumerError>> + Send>>,
    ),
}

impl State {
    fn is_waiting(&self) -> bool {
        matches!(self, State::WaitingForResult)
    }
}

/// Configuration for the aggregation service
#[cfg(feature = "aggregation")]
#[derive(Clone)]
pub struct AggregationServiceConfig {
    /// HTTP clients for aggregation services (supports multiple for redundancy)
    pub clients: Vec<blueprint_tangle_aggregation_svc::AggregationServiceClient>,
    /// BLS secret key for signing
    pub bls_secret: Arc<blueprint_crypto_bn254::ArkBlsBn254Secret>,
    /// BLS public key (derived from secret)
    pub bls_public: Arc<blueprint_crypto_bn254::ArkBlsBn254Public>,
    /// Operator index in the service
    pub operator_index: u32,
    /// Whether to wait for threshold to be met before returning
    pub wait_for_threshold: bool,
    /// Timeout for waiting for threshold (default: 60s)
    pub threshold_timeout: Duration,
    /// Poll interval when waiting for threshold (default: 1s)
    pub poll_interval: Duration,
    /// Whether to try to submit the aggregated result to chain (default: true)
    /// When true, all operators race to submit; first valid submission wins
    pub submit_to_chain: bool,
}

#[cfg(feature = "aggregation")]
impl AggregationServiceConfig {
    /// Create a new aggregation service config with a single service URL
    pub fn new(
        service_url: impl Into<String>,
        bls_secret: blueprint_crypto_bn254::ArkBlsBn254Secret,
        operator_index: u32,
    ) -> Self {
        use blueprint_crypto_bn254::ArkBlsBn254;
        use blueprint_crypto_core::KeyType;

        let bls_public = ArkBlsBn254::public_from_secret(&bls_secret);
        Self {
            clients: vec![
                blueprint_tangle_aggregation_svc::AggregationServiceClient::new(service_url),
            ],
            bls_secret: Arc::new(bls_secret),
            bls_public: Arc::new(bls_public),
            operator_index,
            wait_for_threshold: false,
            threshold_timeout: Duration::from_secs(60),
            poll_interval: Duration::from_secs(1),
            submit_to_chain: true, // Everyone tries to submit by default
        }
    }

    /// Create a new aggregation service config with multiple service URLs
    ///
    /// This allows submitting to multiple aggregation services for redundancy.
    /// Signatures will be sent to ALL services, and threshold polling will
    /// try each service until one succeeds.
    pub fn with_multiple_services(
        service_urls: impl IntoIterator<Item = impl Into<String>>,
        bls_secret: blueprint_crypto_bn254::ArkBlsBn254Secret,
        operator_index: u32,
    ) -> Self {
        use blueprint_crypto_bn254::ArkBlsBn254;
        use blueprint_crypto_core::KeyType;

        let bls_public = ArkBlsBn254::public_from_secret(&bls_secret);
        let clients = service_urls
            .into_iter()
            .map(|url| blueprint_tangle_aggregation_svc::AggregationServiceClient::new(url))
            .collect();

        Self {
            clients,
            bls_secret: Arc::new(bls_secret),
            bls_public: Arc::new(bls_public),
            operator_index,
            wait_for_threshold: false,
            threshold_timeout: Duration::from_secs(60),
            poll_interval: Duration::from_secs(1),
            submit_to_chain: true,
        }
    }

    /// Add an additional aggregation service URL
    pub fn add_service(mut self, service_url: impl Into<String>) -> Self {
        self.clients
            .push(blueprint_tangle_aggregation_svc::AggregationServiceClient::new(service_url));
        self
    }

    /// Set whether to wait for threshold to be met
    pub fn with_wait_for_threshold(mut self, wait: bool) -> Self {
        self.wait_for_threshold = wait;
        self
    }

    /// Set the timeout for waiting for threshold
    pub fn with_threshold_timeout(mut self, timeout: Duration) -> Self {
        self.threshold_timeout = timeout;
        self
    }

    /// Set whether to submit the aggregated result to chain
    ///
    /// When true (default), this operator will attempt to submit the aggregated
    /// result to chain once threshold is met. Multiple operators can race to submit;
    /// the contract ensures only the first valid submission succeeds.
    pub fn with_submit_to_chain(mut self, submit: bool) -> Self {
        self.submit_to_chain = submit;
        self
    }

    /// Get the first client (for backwards compatibility)
    pub fn client(&self) -> &blueprint_tangle_aggregation_svc::AggregationServiceClient {
        self.clients
            .first()
            .expect("At least one client must be configured")
    }

    /// Discover aggregation service URLs by reading operator metadata on chain.
    ///
    /// This queries the Tangle contract for registered operators, fetches their
    /// metadata (ECDSA key + RPC endpoint), and converts those RPC endpoints into
    /// aggregation service URLs using the provided path suffix.
    ///
    /// # Arguments
    /// * `client` - Tangle EVM client used to query on-chain metadata
    /// * `blueprint_id` - The blueprint ID to query operators for
    /// * `service_id` - Service whose operator set should be scanned
    /// * `aggregation_path` - Path suffix for aggregation service (e.g., "/aggregation" or ":8080")
    ///
    /// # Example
    /// ```rust,ignore
    /// // Discover operators and add their aggregation services
    /// let urls = AggregationServiceConfig::discover_operator_services(
    ///     &client,
    ///     blueprint_id,
    ///     service_id,
    ///     ":9090",  // Aggregation service port
    /// ).await?;
    ///
    /// let config = AggregationServiceConfig::with_multiple_services(
    ///     urls,
    ///     bls_secret,
    ///     operator_index,
    /// );
    /// ```
    pub async fn discover_operator_services(
        client: &TangleClient,
        blueprint_id: u64,
        service_id: u64,
        aggregation_path: &str,
    ) -> Result<Vec<String>, AggregatingConsumerError> {
        let operators = client
            .get_service_operators(service_id)
            .await
            .map_err(|e| AggregatingConsumerError::Client(e.to_string()))?;
        let mut rpc_addresses = Vec::with_capacity(operators.len());
        for operator in operators {
            let metadata = client
                .get_operator_metadata(blueprint_id, operator)
                .await
                .map_err(|e| AggregatingConsumerError::Client(e.to_string()))?;
            if !metadata.rpc_endpoint.is_empty() {
                rpc_addresses.push(metadata.rpc_endpoint);
            }
        }

        // Convert RPC addresses to aggregation service URLs
        let urls: Vec<String> = rpc_addresses
            .iter()
            .filter_map(|rpc| {
                // Parse the RPC address and append the aggregation path
                if rpc.is_empty() {
                    return None;
                }

                // If the path starts with ":", treat it as a port replacement
                if aggregation_path.starts_with(':') {
                    // Replace the port in the URL
                    if let Some(host_end) = rpc.rfind(':') {
                        // Check if there's already a port (not just protocol separator)
                        let before_port = &rpc[..host_end];
                        if before_port.contains("://") {
                            return Some(format!("{}{}", before_port, aggregation_path));
                        }
                    }
                    // No port found, just append
                    Some(format!("{}{}", rpc, aggregation_path))
                } else {
                    // Append as a path
                    let base = rpc.trim_end_matches('/');
                    Some(format!("{}{}", base, aggregation_path))
                }
            })
            .collect();

        blueprint_core::info!(
            target: "tangle-aggregating-consumer",
            "Discovered {} operator aggregation services for blueprint {}",
            urls.len(),
            blueprint_id
        );

        Ok(urls)
    }
}

/// An aggregation-aware consumer that automatically routes jobs to either
/// direct submission or aggregated submission based on BSM configuration.
///
/// For jobs that require aggregation, this consumer:
/// 1. Queries the BSM to check aggregation requirements
/// 2. Signs the job output with the operator's BLS key
/// 3. Coordinates aggregation via the configured strategy (HTTP or P2P)
/// 4. Submits the aggregated result to the contract
///
/// For jobs that don't require aggregation, it behaves identically to `TangleConsumer`.
///
/// ## Aggregation Strategies
///
/// The consumer supports two aggregation strategies:
///
/// - **HTTP Service** (recommended): Uses a centralized aggregation service
/// - **P2P Gossip**: Uses peer-to-peer gossip protocol for fully decentralized aggregation
///
/// ## Example
///
/// ```rust,ignore
/// use blueprint_tangle_extra::{AggregatingConsumer, AggregationStrategy, HttpServiceConfig};
///
/// // Create consumer with HTTP aggregation strategy
/// let consumer = AggregatingConsumer::new(client)
///     .with_aggregation_strategy(AggregationStrategy::HttpService(
///         HttpServiceConfig::new("http://localhost:8080", bls_secret, operator_index)
///     ));
/// ```
pub struct AggregatingConsumer {
    client: Arc<TangleClient>,
    buffer: Mutex<VecDeque<PendingJobResult>>,
    state: Mutex<State>,
    /// Shared cache for service configs (aggregation configs, operator weights)
    cache: crate::cache::SharedServiceConfigCache,
    /// Aggregation service configuration (legacy, when feature enabled)
    #[cfg(feature = "aggregation")]
    aggregation_config: Option<AggregationServiceConfig>,
    /// Aggregation strategy (new unified approach)
    #[cfg(any(feature = "aggregation", feature = "p2p-aggregation"))]
    aggregation_strategy: Option<crate::strategy::AggregationStrategy>,
}

impl AggregatingConsumer {
    /// Create a new aggregating consumer with default cache (5 minute TTL)
    pub fn new(client: TangleClient) -> Self {
        Self {
            client: Arc::new(client),
            buffer: Mutex::new(VecDeque::new()),
            state: Mutex::new(State::WaitingForResult),
            cache: crate::cache::shared_cache(),
            #[cfg(feature = "aggregation")]
            aggregation_config: None,
            #[cfg(any(feature = "aggregation", feature = "p2p-aggregation"))]
            aggregation_strategy: None,
        }
    }

    /// Create a new aggregating consumer with a custom cache
    ///
    /// This allows sharing a cache across multiple consumers or
    /// configuring a custom TTL.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// use blueprint_tangle_extra::{AggregatingConsumer, shared_cache_with_ttl};
    /// use blueprint_std::time::Duration;
    ///
    /// // Create a shared cache with 10 minute TTL
    /// let cache = shared_cache_with_ttl(Duration::from_secs(600));
    ///
    /// // Multiple consumers can share the same cache
    /// let consumer1 = AggregatingConsumer::with_cache(client1, cache.clone());
    /// let consumer2 = AggregatingConsumer::with_cache(client2, cache.clone());
    /// ```
    pub fn with_cache(client: TangleClient, cache: crate::cache::SharedServiceConfigCache) -> Self {
        Self {
            client: Arc::new(client),
            buffer: Mutex::new(VecDeque::new()),
            state: Mutex::new(State::WaitingForResult),
            cache,
            #[cfg(feature = "aggregation")]
            aggregation_config: None,
            #[cfg(any(feature = "aggregation", feature = "p2p-aggregation"))]
            aggregation_strategy: None,
        }
    }

    /// Configure the aggregation strategy
    ///
    /// This sets the strategy to use for BLS signature aggregation.
    /// Choose between HTTP service (simpler) or P2P gossip (decentralized).
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// // HTTP service strategy (recommended)
    /// let consumer = AggregatingConsumer::new(client)
    ///     .with_aggregation_strategy(AggregationStrategy::HttpService(
    ///         HttpServiceConfig::new("http://localhost:8080", bls_secret, 0)
    ///     ));
    ///
    /// // P2P gossip strategy (advanced)
    /// let consumer = AggregatingConsumer::new(client)
    ///     .with_aggregation_strategy(AggregationStrategy::P2PGossip(
    ///         P2PGossipConfig::new(network_handle, participant_keys)
    ///     ));
    /// ```
    #[cfg(any(feature = "aggregation", feature = "p2p-aggregation"))]
    pub fn with_aggregation_strategy(
        mut self,
        strategy: crate::strategy::AggregationStrategy,
    ) -> Self {
        self.aggregation_strategy = Some(strategy);
        self
    }

    /// Get the configured aggregation strategy
    #[cfg(any(feature = "aggregation", feature = "p2p-aggregation"))]
    pub fn aggregation_strategy(&self) -> Option<&crate::strategy::AggregationStrategy> {
        self.aggregation_strategy.as_ref()
    }

    /// Configure the aggregation service for BLS signature aggregation
    ///
    /// This enables automatic signing and submission to the aggregation service
    /// when jobs require BLS aggregation.
    #[cfg(feature = "aggregation")]
    pub fn with_aggregation_service(
        mut self,
        service_url: impl Into<String>,
        bls_secret: blueprint_crypto_bn254::ArkBlsBn254Secret,
        operator_index: u32,
    ) -> Self {
        self.aggregation_config = Some(AggregationServiceConfig::new(
            service_url,
            bls_secret,
            operator_index,
        ));
        self
    }

    /// Configure aggregation with full config options
    #[cfg(feature = "aggregation")]
    pub fn with_aggregation_config(mut self, config: AggregationServiceConfig) -> Self {
        self.aggregation_config = Some(config);
        self
    }

    /// Get the underlying client
    #[must_use]
    pub fn client(&self) -> &TangleClient {
        &self.client
    }

    /// Get the service config cache
    ///
    /// This can be used to:
    /// - Pre-populate the cache with known values
    /// - Invalidate cached entries when configs change
    /// - Share the cache with other components
    #[must_use]
    pub fn cache(&self) -> &crate::cache::SharedServiceConfigCache {
        &self.cache
    }

    /// Invalidate all cached data for a service
    ///
    /// Call this when you know a service's configuration has changed
    /// (e.g., operator joined/left, threshold changed).
    pub fn invalidate_service_cache(&self, service_id: u64) {
        self.cache.invalidate_service(service_id);
    }

    /// Get aggregation config, using cache when available
    async fn get_aggregation_config(
        cache: &crate::cache::SharedServiceConfigCache,
        client: &TangleClient,
        service_id: u64,
        job_index: u8,
    ) -> Result<AggregationConfig, AggregatingConsumerError> {
        cache
            .get_aggregation_config(client, service_id, job_index)
            .await
            .map_err(|e| AggregatingConsumerError::Client(e.to_string()))
    }

    /// Get operator weights for a service, using cache when available
    pub async fn get_operator_weights(
        &self,
        service_id: u64,
    ) -> Result<crate::cache::OperatorWeights, AggregatingConsumerError> {
        self.cache
            .get_operator_weights(&self.client, service_id)
            .await
            .map_err(|e| AggregatingConsumerError::Client(e.to_string()))
    }

    /// Get service operators list, using cache when available
    pub async fn get_service_operators(
        &self,
        service_id: u64,
    ) -> Result<crate::cache::ServiceOperators, AggregatingConsumerError> {
        self.cache
            .get_service_operators(&self.client, service_id)
            .await
            .map_err(|e| AggregatingConsumerError::Client(e.to_string()))
    }

    /// Get operator metadata for all operators in a service.
    pub async fn get_service_operator_metadata(
        &self,
        service_id: u64,
    ) -> Result<HashMap<Address, OperatorMetadata>, AggregatingConsumerError> {
        self.cache
            .get_service_operator_metadata(
                &self.client,
                self.client.config.settings.blueprint_id,
                service_id,
            )
            .await
            .map_err(|e| AggregatingConsumerError::Client(e.to_string()))
    }
}

impl Sink<JobResult> for AggregatingConsumer {
    type Error = BoxError;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: JobResult) -> Result<(), Self::Error> {
        let JobResult::Ok { head, body } = &item else {
            blueprint_core::trace!(target: "tangle-aggregating-consumer", "Discarding job result with error");
            return Ok(());
        };

        let (Some(call_id_raw), Some(service_id_raw)) = (
            head.metadata.get(extract::CallId::METADATA_KEY),
            head.metadata.get(extract::ServiceId::METADATA_KEY),
        ) else {
            blueprint_core::trace!(target: "tangle-aggregating-consumer", "Discarding job result with missing metadata");
            return Ok(());
        };

        // Get job index from metadata (defaults to 0 if not present)
        let job_index: u8 = head
            .metadata
            .get(extract::JobIndex::METADATA_KEY)
            .and_then(|v| {
                let val: u64 = v.try_into().ok()?;
                u8::try_from(val).ok()
            })
            .unwrap_or(0);

        blueprint_core::debug!(
            target: "tangle-aggregating-consumer",
            result = ?item,
            job_index = job_index,
            "Received job result, handling..."
        );

        let call_id: u64 = call_id_raw
            .try_into()
            .map_err(|_| AggregatingConsumerError::InvalidMetadata("call_id"))?;
        let service_id: u64 = service_id_raw
            .try_into()
            .map_err(|_| AggregatingConsumerError::InvalidMetadata("service_id"))?;

        self.get_mut()
            .buffer
            .lock()
            .unwrap()
            .push_back(PendingJobResult {
                service_id,
                call_id,
                job_index,
                output: Bytes::copy_from_slice(body),
            });
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let consumer = self.get_mut();
        let mut state = consumer.state.lock().unwrap();

        {
            let buffer = consumer.buffer.lock().unwrap();
            if buffer.is_empty() && state.is_waiting() {
                return Poll::Ready(Ok(()));
            }
        }

        loop {
            match &mut *state {
                State::WaitingForResult => {
                    let result = {
                        let mut buffer = consumer.buffer.lock().unwrap();
                        buffer.pop_front()
                    };

                    let Some(pending) = result else {
                        return Poll::Ready(Ok(()));
                    };

                    let client = Arc::clone(&consumer.client);
                    let cache = Arc::clone(&consumer.cache);

                    #[cfg(feature = "aggregation")]
                    let agg_config = consumer.aggregation_config.clone();

                    let fut = Box::pin(async move {
                        #[cfg(feature = "aggregation")]
                        {
                            submit_job_result(
                                cache,
                                client,
                                pending.service_id,
                                pending.call_id,
                                pending.job_index,
                                pending.output,
                                agg_config,
                            )
                            .await
                        }
                        #[cfg(not(feature = "aggregation"))]
                        {
                            submit_job_result(
                                cache,
                                client,
                                pending.service_id,
                                pending.call_id,
                                pending.job_index,
                                pending.output,
                            )
                            .await
                        }
                    });

                    *state = State::ProcessingSubmission(fut);
                }
                State::ProcessingSubmission(future) => match future.as_mut().poll(cx) {
                    Poll::Ready(Ok(())) => {
                        *state = State::WaitingForResult;
                    }
                    Poll::Ready(Err(e)) => return Poll::Ready(Err(e.into())),
                    Poll::Pending => return Poll::Pending,
                },
            }
        }
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let buffer = self.buffer.lock().unwrap();
        if buffer.is_empty() {
            Poll::Ready(Ok(()))
        } else {
            Poll::Pending
        }
    }
}

/// Submit a job result, automatically choosing aggregation if required
#[cfg(feature = "aggregation")]
async fn submit_job_result(
    cache: crate::cache::SharedServiceConfigCache,
    client: Arc<TangleClient>,
    service_id: u64,
    call_id: u64,
    job_index: u8,
    output: Bytes,
    agg_config: Option<AggregationServiceConfig>,
) -> Result<(), AggregatingConsumerError> {
    // Check if aggregation is required (uses cache)
    let config =
        AggregatingConsumer::get_aggregation_config(&cache, &client, service_id, job_index).await?;

    if config.required {
        blueprint_core::info!(
            target: "tangle-aggregating-consumer",
            "Job {} for service {} requires aggregation (threshold: {}bps, type: {:?})",
            call_id,
            service_id,
            config.threshold_bps,
            config.threshold_type
        );

        // Get aggregation config or error
        let agg = agg_config.ok_or(AggregatingConsumerError::AggregationNotConfigured)?;

        submit_aggregated_result(
            cache, client, service_id, call_id, job_index, output, config, agg,
        )
        .await
    } else {
        // No aggregation needed, submit directly
        submit_direct_result(client, service_id, call_id, output).await
    }
}

#[cfg(feature = "aggregation")]
struct AggregationTaskInit {
    operator_count: u32,
    threshold: ThresholdConfig,
}

#[cfg(feature = "aggregation")]
async fn prepare_aggregation_task(
    cache: &crate::cache::SharedServiceConfigCache,
    client: &TangleClient,
    service_id: u64,
    job_index: u8,
    config: &AggregationConfig,
) -> Result<AggregationTaskInit, AggregatingConsumerError> {
    let operators = cache
        .get_service_operators(client, service_id)
        .await
        .map_err(|e| AggregatingConsumerError::Client(e.to_string()))?;

    if operators.is_empty() {
        return Err(AggregatingConsumerError::Client(format!(
            "No operators registered for service {service_id}"
        )));
    }

    let operator_count = operators.len() as u32;

    let threshold = match config.threshold_type {
        ThresholdType::CountBased => {
            let required = integration::calculate_required_signers(
                operators.len(),
                config.threshold_bps,
                ThresholdType::CountBased,
                None,
            );
            ThresholdConfig::Count {
                required_signers: required as u32,
            }
        }
        ThresholdType::StakeWeighted => {
            let weights = cache
                .get_operator_weights(client, service_id)
                .await
                .map_err(|e| AggregatingConsumerError::Client(e.to_string()))?;

            if weights.is_empty() {
                blueprint_core::warn!(
                    target: "tangle-aggregating-consumer",
                    service_id,
                    job_index,
                    "No operator weights found for service {}; falling back to count-based threshold",
                    service_id
                );
                let required = integration::calculate_required_signers(
                    operators.len(),
                    config.threshold_bps,
                    ThresholdType::CountBased,
                    None,
                );
                ThresholdConfig::Count {
                    required_signers: required as u32,
                }
            } else {
                let mut stakes = Vec::with_capacity(operators.len());
                let mut numeric_stakes = Vec::with_capacity(operators.len());
                for (idx, operator) in operators.iter().enumerate() {
                    let weight = u64::from(*weights.weights.get(operator).unwrap_or(&0));
                    stakes.push(OperatorStake {
                        operator_index: idx as u32,
                        stake: weight,
                    });
                    numeric_stakes.push(weight);
                }

                if numeric_stakes.iter().all(|stake| *stake == 0) {
                    blueprint_core::warn!(
                        target: "tangle-aggregating-consumer",
                        service_id,
                        job_index,
                        "Operator weights for service {} are zero; falling back to count-based threshold",
                        service_id
                    );
                    let required = integration::calculate_required_signers(
                        operators.len(),
                        config.threshold_bps,
                        ThresholdType::CountBased,
                        None,
                    );
                    ThresholdConfig::Count {
                        required_signers: required as u32,
                    }
                } else {
                    blueprint_core::trace!(
                        target: "tangle-aggregating-consumer",
                        service_id,
                        job_index,
                        stakes = ?numeric_stakes,
                        "Prepared stake-weighted threshold"
                    );

                    ThresholdConfig::StakeWeighted {
                        threshold_bps: u32::from(config.threshold_bps),
                        operator_stakes: stakes,
                    }
                }
            }
        }
    };

    Ok(AggregationTaskInit {
        operator_count,
        threshold,
    })
}

/// Submit a job result without aggregation feature
#[cfg(not(feature = "aggregation"))]
async fn submit_job_result(
    cache: crate::cache::SharedServiceConfigCache,
    client: Arc<TangleClient>,
    service_id: u64,
    call_id: u64,
    job_index: u8,
    output: Bytes,
) -> Result<(), AggregatingConsumerError> {
    // Check if aggregation is required (uses cache)
    let config =
        AggregatingConsumer::get_aggregation_config(&cache, &client, service_id, job_index).await?;

    if config.required {
        blueprint_core::warn!(
            target: "tangle-aggregating-consumer",
            "Job {} for service {} requires aggregation but 'aggregation' feature not enabled. \
             Enable the feature and configure the aggregation service.",
            call_id,
            service_id,
        );
        Ok(())
    } else {
        submit_direct_result(client, service_id, call_id, output).await
    }
}

/// Submit using the aggregation service(s)
///
/// This function:
/// 1. Signs the output with the operator's BLS key
/// 2. Sends the signature to ALL configured aggregation services (for redundancy)
/// 3. Waits for threshold if configured
/// 4. Submits the aggregated result to chain if enabled (all operators can race to submit)
#[cfg(feature = "aggregation")]
async fn submit_aggregated_result(
    cache: crate::cache::SharedServiceConfigCache,
    client: Arc<TangleClient>,
    service_id: u64,
    call_id: u64,
    job_index: u8,
    output: Bytes,
    config: AggregationConfig,
    agg: AggregationServiceConfig,
) -> Result<(), AggregatingConsumerError> {
    use blueprint_crypto_bn254::ArkBlsBn254;
    use blueprint_crypto_core::{BytesEncoding, KeyType};
    use blueprint_tangle_aggregation_svc::{SubmitSignatureRequest, create_signing_message};

    let task_init =
        prepare_aggregation_task(&cache, &client, service_id, job_index, &config).await?;

    blueprint_core::debug!(
        target: "tangle-aggregating-consumer",
        service_id,
        call_id,
        job_index,
        operator_count = task_init.operator_count,
        threshold = ?task_init.threshold,
        "Prepared aggregation task initialization payload"
    );

    blueprint_core::debug!(
        target: "tangle-aggregating-consumer",
        "Submitting signature to {} aggregation service(s) for service {} call {}",
        agg.clients.len(),
        service_id,
        call_id
    );

    // Create the message to sign
    let message = create_signing_message(service_id, call_id, &output);

    // Sign with BLS key - we need a mutable clone since sign_with_secret takes &mut
    let mut secret_clone = (*agg.bls_secret).clone();
    let signature = ArkBlsBn254::sign_with_secret(&mut secret_clone, &message)
        .map_err(|e| AggregatingConsumerError::Bls(e.to_string()))?;

    // Get public key and signature bytes using BytesEncoding trait
    let pubkey_bytes = agg.bls_public.to_bytes();
    let sig_bytes = signature.to_bytes();

    // Create the submit request (same for all services)
    let submit_request = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: agg.operator_index,
        output: output.to_vec(),
        signature: sig_bytes.clone(),
        public_key: pubkey_bytes.clone(),
    };

    // Submit to ALL aggregation services
    let mut any_success = false;
    let mut last_response = None;

    for (idx, service_client) in agg.clients.iter().enumerate() {
        // Try to initialize the task (may already exist from another operator)
        let _ = service_client
            .init_task(
                service_id,
                call_id,
                output.as_ref(),
                task_init.operator_count,
                task_init.threshold.clone(),
            )
            .await;

        // Submit our signature
        match service_client
            .submit_signature(submit_request.clone())
            .await
        {
            Ok(response) => {
                blueprint_core::info!(
                    target: "tangle-aggregating-consumer",
                    "Submitted signature to aggregation service {}: {}/{} signatures (threshold met: {})",
                    idx,
                    response.signatures_collected,
                    response.threshold_required,
                    response.threshold_met
                );
                any_success = true;
                last_response = Some(response);
            }
            Err(e) => {
                blueprint_core::warn!(
                    target: "tangle-aggregating-consumer",
                    "Failed to submit to aggregation service {}: {}",
                    idx,
                    e
                );
            }
        }
    }

    if !any_success {
        return Err(AggregatingConsumerError::Client(
            "Failed to submit to any aggregation service".to_string(),
        ));
    }

    // Check if we should submit to chain
    if !agg.submit_to_chain {
        blueprint_core::debug!(
            target: "tangle-aggregating-consumer",
            "submit_to_chain is disabled, not submitting to chain"
        );
        return Ok(());
    }

    // Try to submit to chain
    let response = last_response.unwrap();

    if response.threshold_met {
        // Threshold already met, try to submit immediately
        if let Err(e) =
            try_submit_aggregated_to_chain(client.clone(), &agg, service_id, call_id).await
        {
            blueprint_core::debug!(
                target: "tangle-aggregating-consumer",
                "Failed to submit aggregated result (likely already submitted): {}",
                e
            );
        }
    } else if agg.wait_for_threshold {
        // Wait for threshold to be met, then submit
        blueprint_core::debug!(
            target: "tangle-aggregating-consumer",
            "Waiting for threshold to be met..."
        );

        // Try to get result from any service
        let result = wait_for_threshold_any_service(&agg, service_id, call_id).await?;

        // Try to submit to chain (race with other operators)
        if let Err(e) =
            submit_aggregated_to_chain_with_result(client, &agg, service_id, call_id, result).await
        {
            blueprint_core::debug!(
                target: "tangle-aggregating-consumer",
                "Failed to submit aggregated result (likely already submitted by another operator): {}",
                e
            );
        }
    }

    Ok(())
}

/// Wait for threshold to be met, trying all configured services
#[cfg(feature = "aggregation")]
async fn wait_for_threshold_any_service(
    agg: &AggregationServiceConfig,
    service_id: u64,
    call_id: u64,
) -> Result<blueprint_tangle_aggregation_svc::AggregatedResultResponse, AggregatingConsumerError> {
    use blueprint_std::time::Instant;

    let start = Instant::now();
    let timeout = agg.threshold_timeout;
    let poll_interval = agg.poll_interval;

    while start.elapsed() < timeout {
        // Try each service until one returns a result
        for client in &agg.clients {
            match client.get_aggregated(service_id, call_id).await {
                Ok(Some(result)) => {
                    return Ok(result);
                }
                Ok(None) => {
                    // Threshold not yet met on this service
                }
                Err(e) => {
                    blueprint_core::trace!(
                        target: "tangle-aggregating-consumer",
                        "Error polling aggregation service: {}",
                        e
                    );
                }
            }
        }

        tokio::time::sleep(poll_interval).await;
    }

    Err(AggregatingConsumerError::Client(
        "Timeout waiting for aggregation threshold".to_string(),
    ))
}

/// Try to submit the aggregated result to chain, handling "already submitted" gracefully
#[cfg(feature = "aggregation")]
async fn try_submit_aggregated_to_chain(
    client: Arc<TangleClient>,
    agg: &AggregationServiceConfig,
    service_id: u64,
    call_id: u64,
) -> Result<(), AggregatingConsumerError> {
    // Try to get result from any service
    for service_client in &agg.clients {
        if let Ok(Some(result)) = service_client.get_aggregated(service_id, call_id).await {
            return submit_aggregated_to_chain_with_result(
                client, agg, service_id, call_id, result,
            )
            .await;
        }
    }

    Err(AggregatingConsumerError::Client(
        "Aggregated result not available from any service".to_string(),
    ))
}

/// Submit the aggregated result to the blockchain with a pre-fetched result
#[cfg(feature = "aggregation")]
async fn submit_aggregated_to_chain_with_result(
    client: Arc<TangleClient>,
    agg: &AggregationServiceConfig,
    service_id: u64,
    call_id: u64,
    result: blueprint_tangle_aggregation_svc::AggregatedResultResponse,
) -> Result<(), AggregatingConsumerError> {
    use crate::aggregation::{AggregatedResult, G1Point, G2Point, SignerBitmap};

    if client.config.dry_run {
        blueprint_core::info!(
            target: "tangle-aggregating-consumer",
            "Dry run enabled; skipping aggregated result submission for service {} call {}",
            service_id,
            call_id
        );
        return Ok(());
    }

    blueprint_core::info!(
        target: "tangle-aggregating-consumer",
        "Submitting aggregated result to chain for service {} call {}",
        service_id,
        call_id
    );

    // Parse the signature and pubkey from the response
    let signature = G1Point::from_bytes(&result.aggregated_signature)
        .ok_or_else(|| AggregatingConsumerError::Bls("Invalid aggregated signature".to_string()))?;
    let pubkey = G2Point::from_bytes(&result.aggregated_pubkey)
        .ok_or_else(|| AggregatingConsumerError::Bls("Invalid aggregated pubkey".to_string()))?;

    let aggregated = AggregatedResult::new(
        service_id,
        call_id,
        Bytes::from(result.output),
        SignerBitmap(result.signer_bitmap),
        signature,
        pubkey,
    );

    // Submit to the contract
    aggregated
        .submit(&Arc::new(client.as_ref().clone()))
        .await?;

    // Mark as submitted in all aggregation services
    for client in &agg.clients {
        let _ = client.mark_submitted(service_id, call_id).await;
    }

    blueprint_core::info!(
        target: "tangle-aggregating-consumer",
        "Successfully submitted aggregated result for service {} call {}",
        service_id,
        call_id
    );

    Ok(())
}

/// Submit a result directly without aggregation
async fn submit_direct_result(
    client: Arc<TangleClient>,
    service_id: u64,
    call_id: u64,
    output: Bytes,
) -> Result<(), AggregatingConsumerError> {
    blueprint_core::debug!(
        target: "tangle-aggregating-consumer",
        "Submitting direct result for service {} call {}",
        service_id,
        call_id
    );

    if client.config.dry_run {
        blueprint_core::info!(
            target: "tangle-aggregating-consumer",
            "Dry run enabled; skipping direct result submission for service {} call {}",
            service_id,
            call_id
        );
        return Ok(());
    }

    let result = client
        .submit_result(service_id, call_id, output)
        .await
        .map_err(|e| {
            AggregatingConsumerError::Transaction(format!("Failed to submit result: {e}"))
        })?;

    if result.success {
        blueprint_core::info!(
            target: "tangle-aggregating-consumer",
            "Successfully submitted direct result for service {} call {}: tx_hash={:?}",
            service_id,
            call_id,
            result.tx_hash
        );
    } else {
        return Err(AggregatingConsumerError::Transaction(format!(
            "Transaction reverted for service {} call {}: tx_hash={:?}",
            service_id, call_id, result.tx_hash
        )));
    }

    Ok(())
}

/// Helper to integrate with the P2P aggregation protocol
///
/// This would be used by blueprint developers who want full control
/// over the aggregation process.
pub mod integration {
    use super::*;

    /// Create the message hash that needs to be signed for BLS aggregation
    ///
    /// This matches the contract's verification: keccak256(abi.encodePacked(serviceId, callId, keccak256(output)))
    pub fn create_signing_message(service_id: u64, call_id: u64, output: &[u8]) -> Vec<u8> {
        use alloy_primitives::keccak256;

        let output_hash = keccak256(output);
        let mut message = Vec::with_capacity(8 + 8 + 32);
        message.extend_from_slice(&service_id.to_be_bytes());
        message.extend_from_slice(&call_id.to_be_bytes());
        message.extend_from_slice(output_hash.as_slice());
        message
    }

    /// Calculate required signer count based on threshold config
    pub fn calculate_required_signers(
        total_operators: usize,
        threshold_bps: u16,
        threshold_type: ThresholdType,
        operator_stakes: Option<&[u64]>,
    ) -> usize {
        fn count_based(total: usize, threshold_bps: u16) -> usize {
            if total == 0 {
                return 1;
            }
            let mut required = (total as u64 * threshold_bps as u64) / 10000;
            if required == 0 {
                required = 1;
            }
            let required = required as usize;
            required.min(total).max(1)
        }

        match threshold_type {
            ThresholdType::CountBased => count_based(total_operators, threshold_bps),
            ThresholdType::StakeWeighted => {
                if let Some(stakes) = operator_stakes {
                    if stakes.is_empty() || stakes.iter().all(|stake| *stake == 0) {
                        return count_based(total_operators, threshold_bps);
                    }

                    let total_stake: u128 = stakes.iter().map(|s| *s as u128).sum();
                    if total_stake == 0 {
                        return count_based(total_operators, threshold_bps);
                    }

                    let mut required_stake = (total_stake * threshold_bps as u128) / 10000u128;
                    if required_stake == 0 {
                        required_stake = 1;
                    }

                    let mut sorted: Vec<u64> = stakes.to_vec();
                    sorted.sort_by(|a, b| b.cmp(a));

                    let mut accumulated: u128 = 0;
                    let mut required_signers = 0usize;

                    for stake in sorted {
                        required_signers += 1;
                        accumulated += stake as u128;
                        if accumulated >= required_stake {
                            break;
                        }
                    }

                    required_signers.min(total_operators.max(1)).max(1)
                } else {
                    count_based(total_operators, threshold_bps)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::integration::*;
    use blueprint_client_tangle::ThresholdType;

    // ═══════════════════════════════════════════════════════════════════════════
    // create_signing_message tests
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_create_signing_message_format() {
        let service_id = 1u64;
        let call_id = 42u64;
        let output = b"test output";

        let message = create_signing_message(service_id, call_id, output);

        // Should be 8 + 8 + 32 = 48 bytes
        assert_eq!(message.len(), 48);

        // First 8 bytes should be service_id in big endian
        assert_eq!(&message[0..8], &service_id.to_be_bytes());

        // Next 8 bytes should be call_id in big endian
        assert_eq!(&message[8..16], &call_id.to_be_bytes());

        // Last 32 bytes should be keccak256(output)
        use alloy_primitives::keccak256;
        let expected_hash = keccak256(output);
        assert_eq!(&message[16..48], expected_hash.as_slice());
    }

    #[test]
    fn test_create_signing_message_deterministic() {
        let msg1 = create_signing_message(1, 2, b"hello");
        let msg2 = create_signing_message(1, 2, b"hello");
        assert_eq!(msg1, msg2);
    }

    #[test]
    fn test_create_signing_message_different_outputs() {
        let msg1 = create_signing_message(1, 2, b"hello");
        let msg2 = create_signing_message(1, 2, b"world");
        // Different outputs should produce different messages (different hash suffix)
        assert_ne!(msg1, msg2);
        // But same prefix (service_id and call_id)
        assert_eq!(&msg1[0..16], &msg2[0..16]);
    }

    #[test]
    fn test_create_signing_message_empty_output() {
        let msg = create_signing_message(1, 2, b"");
        assert_eq!(msg.len(), 48);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // calculate_required_signers tests - Count Based
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_calculate_required_signers_count_based_67_percent() {
        // 67% of 3 operators = 2.01 -> 2
        let required = calculate_required_signers(3, 6700, ThresholdType::CountBased, None);
        assert_eq!(required, 2);
    }

    #[test]
    fn test_calculate_required_signers_count_based_50_percent() {
        // 50% of 4 operators = 2
        let required = calculate_required_signers(4, 5000, ThresholdType::CountBased, None);
        assert_eq!(required, 2);
    }

    #[test]
    fn test_calculate_required_signers_count_based_100_percent() {
        // 100% of 5 operators = 5
        let required = calculate_required_signers(5, 10000, ThresholdType::CountBased, None);
        assert_eq!(required, 5);
    }

    #[test]
    fn test_calculate_required_signers_count_based_minimum_one() {
        // Very low threshold should still require at least 1
        let required = calculate_required_signers(10, 100, ThresholdType::CountBased, None); // 1%
        assert_eq!(required, 1);
    }

    #[test]
    fn test_calculate_required_signers_count_based_single_operator() {
        // Single operator, any threshold should require 1
        let required = calculate_required_signers(1, 6700, ThresholdType::CountBased, None);
        assert_eq!(required, 1);
    }

    #[test]
    fn test_calculate_required_signers_count_based_large_set() {
        // 67% of 100 operators = 67
        let required = calculate_required_signers(100, 6700, ThresholdType::CountBased, None);
        assert_eq!(required, 67);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // calculate_required_signers tests - Stake Weighted
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_calculate_required_signers_stake_weighted_no_stakes() {
        // Without stakes, should fall back to count-based
        let required = calculate_required_signers(3, 6700, ThresholdType::StakeWeighted, None);
        assert_eq!(required, 2);
    }

    #[test]
    fn test_calculate_required_signers_stake_weighted_equal_stakes() {
        // 3 operators with equal stakes (10 each), 67% threshold
        // Total stake = 30, required = 20.1, avg = 10, required signers = 2
        let stakes = [10u64, 10, 10];
        let required =
            calculate_required_signers(3, 6700, ThresholdType::StakeWeighted, Some(&stakes));
        assert_eq!(required, 2);
    }

    #[test]
    fn test_calculate_required_signers_stake_weighted_unequal_stakes() {
        // 3 operators: 5, 3, 2 ETH stake (like in contract tests)
        // Total = 10 ETH, 67% = 6.7 ETH required
        // Avg stake = 3.33, signers needed ≈ 2
        let stakes = [5u64, 3, 2];
        let required =
            calculate_required_signers(3, 6700, ThresholdType::StakeWeighted, Some(&stakes));
        assert_eq!(required, 2);
    }

    #[test]
    fn test_calculate_required_signers_stake_weighted_minimum_one() {
        // Very low threshold should still require at least 1
        let stakes = [100u64, 100, 100];
        let required = calculate_required_signers(
            3,
            100, // 1%
            ThresholdType::StakeWeighted,
            Some(&stakes),
        );
        assert_eq!(required, 1);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // URL conversion tests (for discover_operator_services logic)
    // ═══════════════════════════════════════════════════════════════════════════

    /// Helper function to convert RPC address to aggregation URL
    /// This mirrors the logic in discover_operator_services
    fn convert_rpc_to_aggregation_url(rpc: &str, aggregation_path: &str) -> Option<String> {
        if rpc.is_empty() {
            return None;
        }

        if aggregation_path.starts_with(':') {
            // Replace the port in the URL
            if let Some(host_end) = rpc.rfind(':') {
                let before_port = &rpc[..host_end];
                if before_port.contains("://") {
                    return Some(format!("{}{}", before_port, aggregation_path));
                }
            }
            // No port found, just append
            Some(format!("{}{}", rpc, aggregation_path))
        } else {
            // Append as a path
            let base = rpc.trim_end_matches('/');
            Some(format!("{}{}", base, aggregation_path))
        }
    }

    #[test]
    fn test_url_conversion_port_replacement() {
        // Test replacing port
        let url = convert_rpc_to_aggregation_url("http://localhost:8545", ":9090");
        assert_eq!(url, Some("http://localhost:9090".to_string()));

        let url = convert_rpc_to_aggregation_url("https://operator.example.com:8545", ":9090");
        assert_eq!(url, Some("https://operator.example.com:9090".to_string()));
    }

    #[test]
    fn test_url_conversion_port_append() {
        // Test appending port when none exists
        let url = convert_rpc_to_aggregation_url("http://localhost", ":9090");
        assert_eq!(url, Some("http://localhost:9090".to_string()));
    }

    #[test]
    fn test_url_conversion_path_append() {
        // Test appending path
        let url = convert_rpc_to_aggregation_url("http://localhost:8545", "/aggregation");
        assert_eq!(url, Some("http://localhost:8545/aggregation".to_string()));

        let url = convert_rpc_to_aggregation_url("http://localhost:8545/", "/aggregation");
        assert_eq!(url, Some("http://localhost:8545/aggregation".to_string()));
    }

    #[test]
    fn test_url_conversion_empty() {
        let url = convert_rpc_to_aggregation_url("", ":9090");
        assert_eq!(url, None);
    }

    #[test]
    fn test_url_conversion_complex_urls() {
        // IPv6 address
        let url = convert_rpc_to_aggregation_url("http://[::1]:8545", ":9090");
        assert_eq!(url, Some("http://[::1]:9090".to_string()));

        // URL with path
        let url = convert_rpc_to_aggregation_url("http://localhost:8545/rpc", "/v1/aggregate");
        assert_eq!(
            url,
            Some("http://localhost:8545/rpc/v1/aggregate".to_string())
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // AggregationServiceConfig multi-service tests
    // ═══════════════════════════════════════════════════════════════════════════

    #[cfg(feature = "aggregation")]
    mod aggregation_config_tests {
        use crate::AggregationServiceConfig;
        use blueprint_crypto_bn254::ArkBlsBn254;
        use blueprint_crypto_core::KeyType;
        use std::time::Duration;

        fn test_bls_secret() -> blueprint_crypto_bn254::ArkBlsBn254Secret {
            // Generate a deterministic test key
            let seed = [1u8; 32];
            ArkBlsBn254::generate_with_seed(Some(&seed)).unwrap()
        }

        #[test]
        fn test_config_single_service() {
            let config =
                AggregationServiceConfig::new("http://localhost:8080", test_bls_secret(), 0);
            assert_eq!(config.clients.len(), 1);
            assert_eq!(config.operator_index, 0);
            assert!(config.submit_to_chain);
        }

        #[test]
        fn test_config_multiple_services() {
            let config = AggregationServiceConfig::with_multiple_services(
                vec![
                    "http://service1:8080",
                    "http://service2:8080",
                    "http://service3:8080",
                ],
                test_bls_secret(),
                1,
            );
            assert_eq!(config.clients.len(), 3);
            assert_eq!(config.operator_index, 1);
        }

        #[test]
        fn test_config_add_service() {
            let config =
                AggregationServiceConfig::new("http://localhost:8080", test_bls_secret(), 0)
                    .add_service("http://backup:8080")
                    .add_service("http://fallback:8080");

            assert_eq!(config.clients.len(), 3);
        }

        #[test]
        fn test_config_with_submit_to_chain() {
            let config =
                AggregationServiceConfig::new("http://localhost:8080", test_bls_secret(), 0);
            // Default is true
            assert!(config.submit_to_chain);

            let config = config.with_submit_to_chain(false);
            assert!(!config.submit_to_chain);
        }

        #[test]
        fn test_config_with_wait_for_threshold() {
            let config =
                AggregationServiceConfig::new("http://localhost:8080", test_bls_secret(), 0);
            // Default is false
            assert!(!config.wait_for_threshold);

            let config = config.with_wait_for_threshold(true);
            assert!(config.wait_for_threshold);
        }

        #[test]
        fn test_config_with_threshold_timeout() {
            let config =
                AggregationServiceConfig::new("http://localhost:8080", test_bls_secret(), 0)
                    .with_threshold_timeout(Duration::from_secs(120));

            assert_eq!(config.threshold_timeout, Duration::from_secs(120));
        }

        #[test]
        fn test_config_client_accessor() {
            let config = AggregationServiceConfig::with_multiple_services(
                vec!["http://service1:8080", "http://service2:8080"],
                test_bls_secret(),
                0,
            );
            // client() should return the first one
            let _client = config.client();
            // Just verify it doesn't panic
        }

        #[test]
        fn test_config_empty_services() {
            // with_multiple_services should handle empty iterator
            let config = AggregationServiceConfig::with_multiple_services(
                Vec::<String>::new(),
                test_bls_secret(),
                0,
            );
            assert_eq!(config.clients.len(), 0);
        }
    }
}
