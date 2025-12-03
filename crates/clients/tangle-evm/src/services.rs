//! Tangle EVM Services Client
//!
//! Service-specific queries and operations for Tangle v2 contracts.

use alloy_primitives::{Address, U256};
use blueprint_std::vec::Vec;

use crate::client::TangleEvmClient;
use crate::contracts::ITangle;
use crate::error::Result;

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

impl From<ITangle::MembershipModel> for MembershipModel {
    fn from(model: ITangle::MembershipModel) -> Self {
        match model {
            ITangle::MembershipModel::Fixed => MembershipModel::Fixed,
            ITangle::MembershipModel::Dynamic => MembershipModel::Dynamic,
            _ => MembershipModel::Fixed, // Default for unknown values
        }
    }
}

impl From<ITangle::PricingModel> for PricingModel {
    fn from(model: ITangle::PricingModel) -> Self {
        match model {
            ITangle::PricingModel::PayOnce => PricingModel::PayOnce,
            ITangle::PricingModel::Subscription => PricingModel::Subscription,
            ITangle::PricingModel::EventDriven => PricingModel::EventDriven,
            _ => PricingModel::PayOnce, // Default for unknown values
        }
    }
}

impl From<ITangle::ServiceStatus> for ServiceStatus {
    fn from(status: ITangle::ServiceStatus) -> Self {
        match status {
            ITangle::ServiceStatus::Pending => ServiceStatus::Pending,
            ITangle::ServiceStatus::Active => ServiceStatus::Active,
            ITangle::ServiceStatus::Terminated => ServiceStatus::Terminated,
            _ => ServiceStatus::Pending, // Default for unknown values
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
            membership: result.membership.into(),
            pricing: result.pricing.into(),
            status: result.status.into(),
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
            membership: result.membership.into(),
            pricing: result.pricing.into(),
            active: result.active,
        })
    }

    /// Get full blueprint configuration
    pub async fn get_blueprint_config_info(&self, blueprint_id: u64) -> Result<BlueprintConfig> {
        let result = self.get_blueprint_config(blueprint_id).await?;

        Ok(BlueprintConfig {
            membership: result.membership.into(),
            pricing: result.pricing.into(),
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
        let service_count = contract
            .serviceCount()
            .call()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?
            ._0;

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
        self.is_operator_registered(blueprint_id, self.account()).await
    }

    /// Check if current operator is active in the restaking system
    pub async fn is_operator_active_in_restaking(&self) -> Result<bool> {
        self.is_operator_active(self.account()).await
    }

    /// Get current operator's stake
    pub async fn get_own_stake(&self) -> Result<U256> {
        self.get_operator_stake(self.account()).await
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
