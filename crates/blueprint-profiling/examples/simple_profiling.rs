//! Simple example demonstrating blueprint profiling on macOS.

use blueprint_profiling::{is_faas_compatible, profile_job, FaasProvider, InputGenerator};

/// Simple input generator for testing
struct SimpleInputGenerator;

impl InputGenerator for SimpleInputGenerator {
    fn generate_inputs(&self, count: usize) -> Vec<Vec<u8>> {
        (0..count)
            .map(|i| {
                let value = (i as u64) * 10;
                value.to_le_bytes().to_vec()
            })
            .collect()
    }
}

#[tokio::main]
async fn main() {
    println!("Blueprint Profiling Example - Testing on macOS\n");

    let generator = SimpleInputGenerator;

    // Profile a simple square function
    let profile = profile_job(
        0,
        |input| async move {
            let x = u64::from_le_bytes(input[..8].try_into().unwrap());
            let result = x * x;
            result.to_le_bytes().to_vec()
        },
        &generator,
        20, // 20 samples
    )
    .await;

    // Display results
    println!("Profiling Results:");
    println!("  Sample size: {}", profile.sample_size);
    println!("  Min duration: {}ms", profile.min_duration_ms);
    println!("  Avg duration: {}ms", profile.avg_duration_ms);
    println!("  P95 duration: {}ms", profile.p95_duration_ms);
    println!("  Max duration: {}ms", profile.max_duration_ms);
    println!("  Min memory: {}MB", profile.min_memory_mb);
    println!("  Avg memory: {}MB", profile.avg_memory_mb);
    println!("  Peak memory: {}MB", profile.peak_memory_mb);
    println!();

    // Check FaaS compatibility
    println!("FaaS Compatibility:");
    println!(
        "  AWS Lambda: {}",
        is_faas_compatible(&profile, FaasProvider::AwsLambda)
    );
    println!(
        "  GCP Functions: {}",
        is_faas_compatible(&profile, FaasProvider::GcpFunctions)
    );
    println!(
        "  Azure Functions: {}",
        is_faas_compatible(&profile, FaasProvider::AzureFunctions)
    );
    println!(
        "  Custom: {}",
        is_faas_compatible(&profile, FaasProvider::Custom)
    );
    println!();

    // Verify basic properties
    assert_eq!(profile.sample_size, 20);
    assert!(profile.avg_duration_ms >= profile.min_duration_ms);
    assert!(profile.p95_duration_ms >= profile.avg_duration_ms);
    assert!(profile.max_duration_ms >= profile.p95_duration_ms);

    println!("✅ Profiling completed successfully on macOS!");
}
