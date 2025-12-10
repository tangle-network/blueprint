//! Tangle EVM Services Client
//!
//! Service-specific queries and operations for Tangle v2 contracts.

#![allow(missing_docs)]

extern crate alloc;

use alloc::string::ToString;
use alloy_primitives::{Address, Bytes, U256};
use blueprint_std::vec::Vec;

use crate::client::TangleEvmClient;
use crate::contracts::ITangleTypes;
use crate::error::{Error, Result};

/// Service information from the Tangle contract
#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub blueprint_id: u64,
    pub owner: Address,
    pub created_at: u64,
    pub ttl: u64,
    pub terminated_at: u64,
    pub last_payment_at: u64,
    pub operator_count: u32,
    pub min_operators: u32,
    pub max_operators: u32,
    pub membership: MembershipModel,
    pub pricing: PricingModel,
    pub status: ServiceStatus,
}

/// Blueprint information from the Tangle contract
#[derive(Debug, Clone)]
pub struct BlueprintInfo {
    pub owner: Address,
    pub manager: Address,
    pub created_at: u64,
    pub operator_count: u32,
    pub membership: MembershipModel,
    pub pricing: PricingModel,
    pub active: bool,
}

/// Blueprint configuration
#[derive(Debug, Clone)]
pub struct BlueprintConfig {
    pub membership: MembershipModel,
    pub pricing: PricingModel,
    pub min_operators: u32,
    pub max_operators: u32,
    pub subscription_rate: U256,
    pub subscription_interval: u64,
    pub event_rate: U256,
}

/// Membership model
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MembershipModel {
    Fixed,
    Dynamic,
}

/// Pricing model
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PricingModel {
    PayOnce,
    Subscription,
    EventDriven,
}

/// Service status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    Pending,
    Active,
    Terminated,
}

/// Parameters for submitting a new service request.
#[derive(Debug, Clone)]
pub struct ServiceRequestParams {
    /// Blueprint being requested.
    pub blueprint_id: u64,
    /// Candidate operators for the service.
    pub operators: Vec<Address>,
    /// Optional exposure (in basis points) for each operator.
    pub operator_exposures: Option<Vec<u16>>,
    /// Additional permitted callers (beyond the requester).
    pub permitted_callers: Vec<Address>,
    /// Arbitrary configuration blob expected by the blueprint manager.
    pub config: Bytes,
    /// Time-to-live in blocks.
    pub ttl: u64,
    /// Payment token (zero address for ETH).
    pub payment_token: Address,
    /// Payment amount.
    pub payment_amount: U256,
    /// Optional security requirements (falls back to simple exposure flow when empty).
    pub security_requirements: Vec<ITangleTypes::AssetSecurityRequirement>,
}

impl ServiceRequestParams {
    /// Create a new parameter set without security requirements.
    pub fn new(
        blueprint_id: u64,
        operators: Vec<Address>,
        permitted_callers: Vec<Address>,
        config: Bytes,
        ttl: u64,
        payment_token: Address,
        payment_amount: U256,
    ) -> Self {
        Self {
            blueprint_id,
            operators,
            operator_exposures: None,
            permitted_callers,
            config,
            ttl,
            payment_token,
            payment_amount,
            security_requirements: Vec::new(),
        }
    }
}

/// Details about a service request stored on-chain.
#[derive(Debug, Clone)]
pub struct ServiceRequestInfo {
    /// Request identifier.
    pub request_id: u64,
    /// Blueprint being requested.
    pub blueprint_id: u64,
    /// Address that created the request.
    pub requester: Address,
    /// Block timestamp when request was created.
    pub created_at: u64,
    /// Request time-to-live in blocks.
    pub ttl: u64,
    /// Number of operators requested.
    pub operator_count: u32,
    /// Number of approvals the request has received.
    pub approval_count: u32,
    /// ERC-20 token used for payment (zero address for ETH).
    pub payment_token: Address,
    /// Payment amount for the request.
    pub payment_amount: U256,
    /// Membership model requested.
    pub membership: MembershipModel,
    /// Minimum operators allowed.
    pub min_operators: u32,
    /// Maximum operators allowed.
    pub max_operators: u32,
    /// Whether the request has been rejected.
    pub rejected: bool,
}

impl ServiceRequestInfo {
    fn from_contract(id: u64, request: ITangleTypes::ServiceRequest) -> Self {
        Self {
            request_id: id,
            blueprint_id: request.blueprintId,
            requester: request.requester,
            created_at: request.createdAt,
            ttl: request.ttl,
            operator_count: request.operatorCount,
            approval_count: request.approvalCount,
            payment_token: request.paymentToken,
            payment_amount: request.paymentAmount,
            membership: ITangleTypes::MembershipModel::from_underlying(request.membership).into(),
            min_operators: request.minOperators,
            max_operators: request.maxOperators,
            rejected: request.rejected,
        }
    }
}

impl From<ITangleTypes::MembershipModel> for MembershipModel {
    fn from(model: ITangleTypes::MembershipModel) -> Self {
        match model.into_underlying() {
            0 => MembershipModel::Fixed,
            1 => MembershipModel::Dynamic,
            _ => MembershipModel::Fixed,
        }
    }
}

impl From<ITangleTypes::PricingModel> for PricingModel {
    fn from(model: ITangleTypes::PricingModel) -> Self {
        match model.into_underlying() {
            0 => PricingModel::PayOnce,
            1 => PricingModel::Subscription,
            2 => PricingModel::EventDriven,
            _ => PricingModel::PayOnce,
        }
    }
}

impl From<ITangleTypes::ServiceStatus> for ServiceStatus {
    fn from(status: ITangleTypes::ServiceStatus) -> Self {
        match status.into_underlying() {
            0 => ServiceStatus::Pending,
            1 => ServiceStatus::Active,
            2 => ServiceStatus::Terminated,
            _ => ServiceStatus::Pending,
        }
    }
}

/// Extension trait for service-related operations on TangleEvmClient
impl TangleEvmClient {
    /// Get full service information
    pub async fn get_service_info(&self, service_id: u64) -> Result<ServiceInfo> {
        let result = self.get_service(service_id).await?;

        Ok(ServiceInfo {
            blueprint_id: result.blueprintId,
            owner: result.owner,
            created_at: result.createdAt,
            ttl: result.ttl,
            terminated_at: result.terminatedAt,
            last_payment_at: result.lastPaymentAt,
            operator_count: result.operatorCount,
            min_operators: result.minOperators,
            max_operators: result.maxOperators,
            membership: ITangleTypes::MembershipModel::from_underlying(result.membership).into(),
            pricing: ITangleTypes::PricingModel::from_underlying(result.pricing).into(),
            status: ITangleTypes::ServiceStatus::from_underlying(result.status).into(),
        })
    }

    /// Get full blueprint information
    pub async fn get_blueprint_info(&self, blueprint_id: u64) -> Result<BlueprintInfo> {
        let result = self.get_blueprint(blueprint_id).await?;

        Ok(BlueprintInfo {
            owner: result.owner,
            manager: result.manager,
            created_at: result.createdAt,
            operator_count: result.operatorCount,
            membership: ITangleTypes::MembershipModel::from_underlying(result.membership).into(),
            pricing: ITangleTypes::PricingModel::from_underlying(result.pricing).into(),
            active: result.active,
        })
    }

    /// Get full blueprint configuration
    pub async fn get_blueprint_config_info(&self, blueprint_id: u64) -> Result<BlueprintConfig> {
        let result = self.get_blueprint_config(blueprint_id).await?;

        Ok(BlueprintConfig {
            membership: ITangleTypes::MembershipModel::from_underlying(result.membership).into(),
            pricing: ITangleTypes::PricingModel::from_underlying(result.pricing).into(),
            min_operators: result.minOperators,
            max_operators: result.maxOperators,
            subscription_rate: result.subscriptionRate,
            subscription_interval: result.subscriptionInterval,
            event_rate: result.eventRate,
        })
    }

    /// Get services for an operator
    ///
    /// Queries all services and filters for those where the operator is active
    pub async fn get_operator_services(&self, operator: Address) -> Result<Vec<u64>> {
        let contract = self.tangle_contract();
        let service_count: u64 = contract
            .serviceCount()
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;

        let mut services = Vec::new();

        for service_id in 0..service_count {
            if self.is_service_operator(service_id, operator).await? {
                services.push(service_id);
            }
        }

        Ok(services)
    }

    /// Check if current operator is registered for the configured blueprint
    pub async fn is_registered_for_blueprint(&self) -> Result<bool> {
        let blueprint_id = self.config.settings.blueprint_id;
        self.is_operator_registered(blueprint_id, self.account())
            .await
    }

    /// Check if current operator is active in the restaking system
    pub async fn is_operator_active_in_restaking(&self) -> Result<bool> {
        self.is_operator_active(self.account()).await
    }

    /// Get current operator's stake
    pub async fn get_own_stake(&self) -> Result<U256> {
        self.get_operator_stake(self.account()).await
    }

    /// Fetch all blueprint summaries.
    pub async fn list_blueprints(&self) -> Result<Vec<(u64, BlueprintInfo)>> {
        let total = self.blueprint_count().await?;
        let capacity = usize::try_from(total).unwrap_or(usize::MAX);
        let mut blueprints = Vec::with_capacity(capacity);

        for blueprint_id in 0..total {
            match self.get_blueprint_info(blueprint_id).await {
                Ok(info) => blueprints.push((blueprint_id, info)),
                Err(err) => {
                    tracing::warn!(
                        %blueprint_id,
                        error = %err,
                        "failed to fetch blueprint info"
                    );
                }
            }
        }

        Ok(blueprints)
    }

    /// Fetch all services registered on-chain.
    pub async fn list_services(&self) -> Result<Vec<(u64, ServiceInfo)>> {
        let total = self.service_count().await?;
        let capacity = usize::try_from(total).unwrap_or(usize::MAX);
        let mut services = Vec::with_capacity(capacity);

        for service_id in 0..total {
            match self.get_service_info(service_id).await {
                Ok(info) => services.push((service_id, info)),
                Err(err) => {
                    tracing::warn!(
                        %service_id,
                        error = %err,
                        "failed to fetch service info"
                    );
                }
            }
        }

        Ok(services)
    }

    /// Fetch a single service request.
    pub async fn get_service_request_info(&self, request_id: u64) -> Result<ServiceRequestInfo> {
        let request = self.get_service_request(request_id).await?;
        Ok(ServiceRequestInfo::from_contract(request_id, request))
    }

    /// List all service requests ever recorded on-chain.
    pub async fn list_service_requests(&self) -> Result<Vec<ServiceRequestInfo>> {
        let total = self.service_request_count().await?;
        let capacity = usize::try_from(total).unwrap_or(usize::MAX);
        let mut requests = Vec::with_capacity(capacity);

        for request_id in 0..total {
            match self.get_service_request(request_id).await {
                Ok(info) => requests.push(ServiceRequestInfo::from_contract(request_id, info)),
                Err(err) => {
                    tracing::warn!(
                        %request_id,
                        error = %err,
                        "failed to fetch service request"
                    );
                }
            }
        }

        Ok(requests)
    }
}

/// Operator security commitment for a service
#[derive(Debug, Clone)]
pub struct OperatorSecurityCommitment {
    pub operator: Address,
    pub exposure_bps: u16,
}

impl TangleEvmClient {
    /// Get operators with their security commitments for a service
    pub async fn get_service_operators_with_exposure(
        &self,
        service_id: u64,
    ) -> Result<Vec<OperatorSecurityCommitment>> {
        let operators = self.get_service_operators(service_id).await?;

        // For now, we return operators without exposure info
        // The full implementation would query the contract for exposure data
        Ok(operators
            .into_iter()
            .map(|operator| OperatorSecurityCommitment {
                operator,
                exposure_bps: 10000, // 100% default
            })
            .collect())
    }
}
