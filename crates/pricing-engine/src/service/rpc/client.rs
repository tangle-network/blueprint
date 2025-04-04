//! RPC client for interacting with operator RPC servers
//!
//! This module provides a client for users to interact with operator RPC servers
//! to request price quotes for services.

use super::OperatorInfo;
use crate::{
    error::{Error, Result},
    types::ResourceRequirement,
};
use blueprint_crypto::KeyType;
use jsonrpsee::{
    core::client::ClientT,
    http_client::{HttpClient, HttpClientBuilder},
    rpc_params,
};
use std::time::Duration;
use tracing::{debug, error};

/// Client for interacting with the operator RPC server
pub struct RpcClient {
    /// The underlying HTTP client
    client: HttpClient,
}

impl RpcClient {
    /// Create a new RPC client
    ///
    /// # Arguments
    /// * `url` - The URL of the RPC server
    ///
    /// # Returns
    /// A new RPC client
    ///
    /// # Errors
    /// Returns an error if the client cannot be created
    pub async fn new(url: String) -> Result<Self> {
        let client = HttpClientBuilder::default()
            .request_timeout(Duration::from_secs(30))
            .build(url)
            .map_err(|e| Error::RpcClient(format!("Failed to create RPC client: {}", e)))?;

        Ok(Self { client })
    }

    /// Request quotes for a service (standard API method)
    ///
    /// This follows the standard RPC API by initiating an RFQ request
    /// and returning a request ID for later retrieval.
    ///
    /// # Arguments
    /// * `blueprint_id` - The blueprint ID
    /// * `requirements` - The resource requirements
    /// * `max_price` - Maximum price (optional)
    /// * `timeout_seconds` - Timeout in seconds (optional)
    ///
    /// # Returns
    /// A request ID for retrieving results later
    ///
    /// # Errors
    /// Returns an error if the request fails
    pub async fn request_for_quote(
        &self,
        blueprint_id: String,
        requirements: Vec<ResourceRequirement>,
        max_price: Option<u64>,
        timeout_seconds: Option<u64>,
    ) -> Result<String> {
        // Log what we're sending
        debug!("Preparing to send RFQ request to server");
        debug!("  blueprint_id: {}", blueprint_id);
        debug!("  requirements: {:?}", requirements);
        debug!("  max_price: {:?}", max_price);
        debug!("  timeout_seconds: {:?}", timeout_seconds);

        // Construct RPC params - sending as individual parameters
        let params = rpc_params![blueprint_id, requirements, max_price, timeout_seconds];
        debug!("RPC params prepared: {:?}", params);

        debug!("Sending RFQ request via RPC (pricing_requestForQuote)");
        let request_id: String = self
            .client
            .request("pricing_requestForQuote", params)
            .await
            .map_err(|e| {
                error!("RPC request failed: {}", e);
                Error::RpcClient(format!("Failed to send RFQ request: {}", e))
            })?;

        debug!("Received request ID: {}", request_id);
        Ok(request_id)
    }

    /// Get results for an RFQ request (standard API method)
    ///
    /// # Arguments
    /// * `request_id` - The request ID returned from request_for_quote
    ///
    /// # Returns
    /// The RFQ response containing quotes and status
    ///
    /// # Errors
    /// Returns an error if the request fails
    pub async fn get_rfq_results<K: KeyType>(
        &self,
        request_id: String,
    ) -> Result<crate::service::rpc::server::RfqResponse<K>> {
        debug!("Getting RFQ results via RPC (pricing_getRfqResults)");

        let response = self
            .client
            .request("pricing_getRfqResults", rpc_params![request_id])
            .await
            .map_err(|e| Error::RpcClient(format!("Failed to get RFQ results: {}", e)))?;

        debug!("Received RFQ response");
        Ok(response)
    }

    /// Get information about the operator
    ///
    /// # Returns
    /// Information about the operator
    ///
    /// # Errors
    /// Returns an error if the request fails
    pub async fn get_operator_info<K: KeyType>(&self) -> Result<OperatorInfo<K>> {
        let info = self
            .client
            .request("pricing_getOperatorInfo", rpc_params![])
            .await
            .map_err(|e| Error::RpcClient(format!("Failed to request operator info: {}", e)))?;

        Ok(info)
    }

    /// Calculate price for a service with specified requirements
    ///
    /// # Arguments
    /// * `blueprint_id` - The blueprint ID
    /// * `requirements` - The resource requirements
    /// * `duration` - Duration of the service in seconds (optional)
    ///
    /// # Returns
    /// A price quote for the service
    ///
    /// # Errors
    /// Returns an error if the request fails
    pub async fn calculate_price(
        &self,
        blueprint_id: String,
        requirements: Vec<ResourceRequirement>,
        duration: Option<u64>,
    ) -> Result<crate::service::rpc::server::PriceQuote> {
        // Create the request object
        let request = crate::service::rpc::server::PriceCalculationRequest {
            blueprint_id,
            requirements,
            duration,
        };

        debug!("Calculating price via RPC (pricing_calculatePrice)");
        debug!("Request: {:?}", request);

        let quote = self
            .client
            .request("pricing_calculatePrice", rpc_params![request])
            .await
            .map_err(|e| Error::RpcClient(format!("Failed to calculate price: {}", e)))?;

        debug!("Received price quote response");
        Ok(quote)
    }
}
