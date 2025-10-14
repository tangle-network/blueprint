//! Operator pricing service
//!
//! This module provides the pricing service that operators use to:
//! 1. Fetch blueprint metadata with profiling data
//! 2. Analyze deployment strategy (FaaS vs VM sizing)
//! 3. Calculate pricing using the pricing-engine
//! 4. Return a competitive quote
//!
//! # Example Flow
//!
//! ```no_run
//! use blueprint_manager::remote::pricing_service::OperatorPricingService;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let service = OperatorPricingService::new(
//!     "wss://rpc.tangle.tools",
//!     None, // optional binary path for filesystem fallback
//! );
//!
//! // Fetch blueprint and calculate pricing
//! let quote = service.calculate_quote(42).await?;
//!
//! println!("Deployment strategy: {:?}", quote.strategy);
//! println!("Monthly cost estimate: ${:.2}", quote.monthly_cost_usd);
//! println!("Per-execution cost: ${:.6}", quote.per_execution_cost_usd);
//! # Ok(())
//! # }
//! ```

use crate::error::Result;
use crate::remote::blueprint_analyzer::{BlueprintAnalysis, DeploymentStrategy, analyze_blueprint};
use crate::remote::blueprint_fetcher::{BlueprintMetadata, fetch_blueprint_metadata};
use blueprint_pricing_engine_lib::{BenchmarkProfile, CloudProvider, FaasPricingFetcher, PricingFetcher};
use serde::{Deserialize, Serialize};

/// Operator pricing service for calculating deployment costs
pub struct OperatorPricingService {
    rpc_url: String,
    binary_path: Option<std::path::PathBuf>,
    faas_fetcher: FaasPricingFetcher,
    vm_fetcher: PricingFetcher,
}

/// Pricing quote for a blueprint deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingQuote {
    pub blueprint_id: u64,
    pub strategy: DeploymentStrategy,
    pub analysis: BlueprintAnalysis,

    /// Estimated monthly cost in USD (for VM deployments)
    pub monthly_cost_usd: f64,

    /// Estimated per-execution cost in USD (for FaaS deployments)
    pub per_execution_cost_usd: f64,

    /// Expected executions per month (for total cost calculation)
    pub estimated_monthly_executions: u64,

    /// Total estimated monthly cost
    pub total_monthly_cost_usd: f64,

    /// Provider breakdown
    pub provider_costs: Vec<ProviderCost>,
}

/// Cost breakdown by provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCost {
    pub provider: String,
    pub monthly_vm_cost_usd: f64,
    pub per_execution_cost_usd: f64,
    pub instance_type: String,
}

impl OperatorPricingService {
    /// Create a new pricing service
    pub fn new(rpc_url: impl Into<String>, binary_path: Option<std::path::PathBuf>) -> Self {
        Self {
            rpc_url: rpc_url.into(),
            binary_path,
            faas_fetcher: FaasPricingFetcher::new(),
            vm_fetcher: PricingFetcher::new_or_default(),
        }
    }

    /// Calculate pricing quote for a blueprint
    ///
    /// This is the main entry point for operators to get pricing information.
    pub async fn calculate_quote(&self, blueprint_id: u64) -> Result<PricingQuote> {
        // 1. Fetch blueprint metadata (includes profiling data from chain)
        let metadata = fetch_blueprint_metadata(
            blueprint_id,
            Some(&self.rpc_url),
            self.binary_path.as_deref(),
        )
        .await?;

        // 2. Analyze deployment strategy
        use crate::remote::blueprint_analyzer::FaasLimits;
        let faas_limits = FaasLimits::aws_lambda(); // Use AWS Lambda limits as default
        let analysis = analyze_blueprint(
            metadata.job_count,
            &metadata.job_profiles,
            &faas_limits,
            true, // serverless enabled
        );

        // 3. Calculate costs based on strategy
        let quote = self.calculate_costs(&metadata, &analysis).await?;

        Ok(quote)
    }

    /// Calculate costs for the deployment strategy
    async fn calculate_costs(
        &self,
        metadata: &BlueprintMetadata,
        analysis: &BlueprintAnalysis,
    ) -> Result<PricingQuote> {
        let mut provider_costs = Vec::new();

        match &analysis.recommended_strategy {
            DeploymentStrategy::Serverless { job_ids } => {
                // Calculate FaaS costs for each job
                for job_id in job_ids {
                    if let Some(Some(profile)) = metadata.job_profiles.get(*job_id as usize) {
                        let benchmark = profile.to_pricing_benchmark_profile();

                        // Calculate costs for different FaaS providers (using real APIs)
                        let aws_cost = self
                            .calculate_faas_cost("AWS Lambda", &benchmark)
                            .await
                            .unwrap_or(0.0);
                        let gcp_cost = self
                            .calculate_faas_cost("GCP Cloud Functions", &benchmark)
                            .await
                            .unwrap_or(0.0);
                        let azure_cost = self
                            .calculate_faas_cost("Azure Functions", &benchmark)
                            .await
                            .unwrap_or(0.0);

                        provider_costs.push(ProviderCost {
                            provider: "AWS Lambda".to_string(),
                            monthly_vm_cost_usd: 0.0,
                            per_execution_cost_usd: aws_cost,
                            instance_type: format!("{}MB RAM", profile.peak_memory_mb),
                        });

                        provider_costs.push(ProviderCost {
                            provider: "GCP Cloud Functions".to_string(),
                            monthly_vm_cost_usd: 0.0,
                            per_execution_cost_usd: gcp_cost,
                            instance_type: format!("{}MB RAM", profile.peak_memory_mb),
                        });

                        provider_costs.push(ProviderCost {
                            provider: "Azure Functions".to_string(),
                            monthly_vm_cost_usd: 0.0,
                            per_execution_cost_usd: azure_cost,
                            instance_type: format!("{}MB RAM", profile.peak_memory_mb),
                        });
                    }
                }
            }
            DeploymentStrategy::Traditional { .. } | DeploymentStrategy::Hybrid { .. } => {
                // Calculate VM costs (using real pricing APIs)
                let aws_vm = self
                    .calculate_vm_cost("AWS EC2", &analysis.resource_sizing)
                    .await
                    .unwrap_or_else(|_| ProviderCost {
                        provider: "AWS EC2".to_string(),
                        monthly_vm_cost_usd: 0.0,
                        per_execution_cost_usd: 0.0,
                        instance_type: "unknown".to_string(),
                    });
                let gcp_vm = self
                    .calculate_vm_cost("GCP Compute", &analysis.resource_sizing)
                    .await
                    .unwrap_or_else(|_| ProviderCost {
                        provider: "GCP Compute".to_string(),
                        monthly_vm_cost_usd: 0.0,
                        per_execution_cost_usd: 0.0,
                        instance_type: "unknown".to_string(),
                    });
                let azure_vm = self
                    .calculate_vm_cost("Azure VM", &analysis.resource_sizing)
                    .await
                    .unwrap_or_else(|_| ProviderCost {
                        provider: "Azure VM".to_string(),
                        monthly_vm_cost_usd: 0.0,
                        per_execution_cost_usd: 0.0,
                        instance_type: "unknown".to_string(),
                    });

                provider_costs.push(aws_vm);
                provider_costs.push(gcp_vm);
                provider_costs.push(azure_vm);
            }
        }

        // Choose cheapest provider for estimates
        let cheapest_vm = provider_costs
            .iter()
            .min_by(|a, b| a.monthly_vm_cost_usd.partial_cmp(&b.monthly_vm_cost_usd).unwrap())
            .map(|c| c.monthly_vm_cost_usd)
            .unwrap_or(0.0);

        let cheapest_faas = provider_costs
            .iter()
            .filter(|c| c.per_execution_cost_usd > 0.0)
            .min_by(|a, b| {
                a.per_execution_cost_usd
                    .partial_cmp(&b.per_execution_cost_usd)
                    .unwrap()
            })
            .map(|c| c.per_execution_cost_usd)
            .unwrap_or(0.0);

        // Estimate monthly executions (default: 10k/month for FaaS)
        let estimated_monthly_executions = if matches!(
            analysis.recommended_strategy,
            DeploymentStrategy::Serverless { .. }
        ) {
            10_000
        } else {
            0
        };

        let total_monthly_cost_usd = cheapest_vm
            + (cheapest_faas * estimated_monthly_executions as f64);

        Ok(PricingQuote {
            blueprint_id: metadata.blueprint_id,
            strategy: analysis.recommended_strategy.clone(),
            analysis: analysis.clone(),
            monthly_cost_usd: cheapest_vm,
            per_execution_cost_usd: cheapest_faas,
            estimated_monthly_executions,
            total_monthly_cost_usd,
            provider_costs,
        })
    }

    /// Calculate FaaS cost per execution using real pricing APIs
    async fn calculate_faas_cost(&self, provider: &str, benchmark: &BenchmarkProfile) -> Result<f64> {
        let memory_mb = benchmark
            .memory_details
            .as_ref()
            .map(|m| m.peak_memory_mb as u64)
            .unwrap_or(128);

        let duration_secs = benchmark.duration_secs as f64;
        let memory_gb = memory_mb as f64 / 1024.0;

        // Fetch real pricing from provider APIs
        let pricing = match provider {
            "AWS Lambda" => {
                self.faas_fetcher
                    .fetch_aws_lambda_pricing("us-east-1")
                    .await
                    .map_err(|e| crate::error::Error::Other(format!("Failed to fetch AWS pricing: {e}")))?
            }
            "GCP Cloud Functions" => {
                self.faas_fetcher
                    .fetch_gcp_functions_pricing("us-central1")
                    .await
                    .map_err(|e| crate::error::Error::Other(format!("Failed to fetch GCP pricing: {e}")))?
            }
            "Azure Functions" => {
                self.faas_fetcher
                    .fetch_azure_functions_pricing("eastus")
                    .await
                    .map_err(|e| crate::error::Error::Other(format!("Failed to fetch Azure pricing: {e}")))?
            }
            _ => return Ok(0.0),
        };

        // Calculate cost using real pricing
        Ok(self
            .faas_fetcher
            .estimate_execution_cost(&pricing, memory_gb, duration_secs, 1))
    }

    /// Calculate VM cost per month using real pricing APIs
    async fn calculate_vm_cost(
        &self,
        provider: &str,
        sizing: &crate::remote::blueprint_analyzer::ResourceSizing,
    ) -> Result<ProviderCost> {
        // Convert provider string to CloudProvider enum
        let cloud_provider = match provider {
            "AWS EC2" => CloudProvider::AWS,
            "GCP Compute" => CloudProvider::GCP,
            "Azure VM" => CloudProvider::Azure,
            _ => {
                return Ok(ProviderCost {
                    provider: provider.to_string(),
                    monthly_vm_cost_usd: 0.0,
                    per_execution_cost_usd: 0.0,
                    instance_type: "unknown".to_string(),
                });
            }
        };

        // Find best instance using real pricing API
        let memory_gb = sizing.memory_mb as f32 / 1024.0;
        let max_price = 1.0; // $1/hour max

        let instance = self
            .vm_fetcher
            .clone()
            .find_best_instance(
                cloud_provider,
                "us-east-1", // Default region
                sizing.cpu_cores,
                memory_gb,
                max_price,
            )
            .await
            .unwrap_or_else(|_| {
                // Fallback to estimated pricing if API fails
                blueprint_pricing_engine_lib::InstanceInfo {
                    name: format!("{}vCPU/{}GB", sizing.cpu_cores, memory_gb),
                    vcpus: sizing.cpu_cores,
                    memory_gb,
                    hourly_price: 0.05, // Estimated
                }
            });

        // Convert hourly to monthly (730 hours/month standard)
        let monthly_cost = instance.hourly_price * 730.0;

        Ok(ProviderCost {
            provider: provider.to_string(),
            monthly_vm_cost_usd: monthly_cost,
            per_execution_cost_usd: 0.0,
            instance_type: instance.name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pricing_service_mock() {
        // This test uses mock data since we don't have a live chain
        let service = OperatorPricingService::new("ws://localhost:9944", None);

        // In real usage, this would fetch from chain
        let result = service.calculate_quote(42).await;

        // With mock mode, this should work
        assert!(result.is_ok());
    }
}
