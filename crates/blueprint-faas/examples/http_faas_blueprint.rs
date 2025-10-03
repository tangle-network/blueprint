//! Example: Blueprint with mixed local/FaaS execution
//!
//! This example demonstrates a blueprint where:
//! - Job 0 (frequent, cheap): Runs locally
//! - Job 1 (infrequent, expensive): Delegated to FaaS
//!
//! Run with: cargo run --example http_faas_blueprint --features custom

use blueprint_core::{JobCall, JobResult};
use blueprint_faas::custom::HttpFaasExecutor;
use blueprint_router::Router;
use blueprint_runner::BlueprintRunner;
use blueprint_runner::config::BlueprintEnvironment;
use bytes::Bytes;
use futures::stream;
use std::time::Duration;
use tokio::time;

/// Local job: Simple echo that runs frequently
async fn local_echo_job(call: JobCall) -> JobResult {
    println!("🏠 LOCAL: Processing job {} locally", call.job_id());

    JobResult::builder(call.job_id())
        .body(call.body().clone())
        .build()
}

/// FaaS job: Expensive computation delegated to serverless
async fn faas_compute_job(call: JobCall) -> JobResult {
    // This function signature is defined, but execution happens on FaaS
    println!("☁️  FAAS: This shouldn't print - job runs remotely!");

    JobResult::builder(call.job_id())
        .body(Bytes::from("should not execute"))
        .build()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("🚀 Starting Mixed Execution Blueprint Example\n");

    // Setup FaaS executor for expensive jobs
    let faas_executor = HttpFaasExecutor::new("http://localhost:8080")
        .with_job_endpoint(1, "http://localhost:8080/compute");

    // Create router with both job types
    let router = Router::new()
        .route(0, local_echo_job)    // Job 0: local
        .route(1, faas_compute_job);  // Job 1: FaaS (won't execute locally)

    // Create test producer that simulates job calls
    let producer = stream::iter(vec![
        Ok(JobCall::builder(0u32).body(Bytes::from("local data")).build()),
        Ok(JobCall::builder(1u32).body(Bytes::from("faas data")).build()),
        Ok(JobCall::builder(0u32).body(Bytes::from("more local")).build()),
    ]);

    // Create simple consumer that prints results
    let consumer = futures::sink::unfold((), |(), result: JobResult| async move {
        println!("✅ Result from job {}: {:?}",
            result.job_id(),
            String::from_utf8_lossy(result.body().as_ref())
        );
        Ok::<_, std::io::Error>(())
    });

    let env = BlueprintEnvironment::default();

    println!("📋 Configuration:");
    println!("  - Job 0: Runs LOCALLY (frequent, cheap)");
    println!("  - Job 1: Runs on FAAS (infrequent, expensive)");
    println!("\n🔄 Processing jobs...\n");

    // Build and run the blueprint
    BlueprintRunner::builder((), env)
        .router(router)
        .producer(producer)
        .consumer(consumer)
        .with_faas_executor(1, faas_executor)  // Only job 1 uses FaaS
        .with_shutdown_handler(async {
            time::sleep(Duration::from_secs(2)).await;
        })
        .run()
        .await?;

    println!("\n✨ Blueprint execution complete!");

    Ok(())
}
