use blueprint_manager::remote::{
    provider_selector::{CloudProvider, ProviderPreferences, ProviderSelector, ResourceSpec},
    service::{RemoteDeploymentPolicy, RemoteDeploymentService},
};
use color_eyre::Result;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 Remote Deployment Demo - Phase 2 Integration");

    // Simulate CLI-configured deployment policy
    let policy = RemoteDeploymentPolicy {
        provider_preferences: ProviderPreferences::default(),
        max_hourly_cost: Some(5.0),
        prefer_spot: true,
        auto_terminate_hours: Some(4),
    };

    info!("✅ Deployment policy loaded:");
    info!("   Max hourly cost: ${:.2}", policy.max_hourly_cost.unwrap_or(0.0));
    info!("   Prefer spot instances: {}", policy.prefer_spot);
    info!("   Auto-terminate: {} hours", policy.auto_terminate_hours.unwrap_or(0));

    // Create remote deployment service
    let service = RemoteDeploymentService::new(policy).await?;
    info!("✅ Remote deployment service initialized");

    // Demo different workload types and provider selection
    let workloads = vec![
        ("GPU Workload", ResourceSpec {
            cpu: 8.0,
            memory_gb: 32.0,
            storage_gb: 200.0,
            gpu_count: Some(2),
            allow_spot: false,
        }),
        ("CPU-Intensive", ResourceSpec {
            cpu: 16.0,
            memory_gb: 64.0,
            storage_gb: 500.0,
            gpu_count: None,
            allow_spot: true,
        }),
        ("Memory-Intensive", ResourceSpec {
            cpu: 8.0,
            memory_gb: 128.0,
            storage_gb: 1000.0,
            gpu_count: None,
            allow_spot: false,
        }),
        ("Cost-Optimized", ResourceSpec {
            cpu: 2.0,
            memory_gb: 4.0,
            storage_gb: 20.0,
            gpu_count: None,
            allow_spot: true,
        }),
    ];

    for (name, spec) in workloads {
        info!("\n📊 Analyzing workload: {}", name);
        info!("   Resources: {:.1} CPU, {:.0} GB RAM, {:.0} GB storage{}", 
              spec.cpu, spec.memory_gb, spec.storage_gb,
              spec.gpu_count.map_or(String::new(), |gpu| format!(", {} GPU", gpu)));
        
        // This uses the same provider selection logic that Blueprint Manager would use
        let selector = ProviderSelector::with_defaults();
        match selector.select_provider(&spec) {
            Ok(provider) => {
                info!("   ✅ Selected provider: {:?}", provider);
                
                let fallbacks = selector.get_fallback_providers(&spec);
                if !fallbacks.is_empty() {
                    info!("   🔄 Fallback providers: {:?}", fallbacks);
                }
            }
            Err(e) => {
                info!("   ❌ Provider selection failed: {}", e);
            }
        }
    }

    // Show deployment registry operations
    info!("\n📋 Deployment Registry Operations:");
    let deployments = service.list_deployments().await;
    info!("   Active deployments: {}", deployments.len());

    // Demo cleanup operations
    service.cleanup_expired_deployments().await?;
    info!("   ✅ Cleanup completed");

    info!("\n🎉 Demo completed successfully!");
    info!("This demonstrates the integrated remote deployment flow:");
    info!("1. CLI loads deployment policy from configuration");
    info!("2. Blueprint Manager creates RemoteDeploymentService");
    info!("3. Provider selection works based on resource requirements");
    info!("4. Deployment registry tracks active instances");
    info!("5. TTL-based cleanup maintains cost control");

    Ok(())
}