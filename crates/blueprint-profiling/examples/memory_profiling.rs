//! Memory-intensive profiling example to demonstrate memory measurement on macOS.

use blueprint_profiling::{is_faas_compatible, profile_job, FaasProvider, InputGenerator};

/// Input generator that varies sizes to trigger memory allocation
struct VaryingSizeGenerator;

impl InputGenerator for VaryingSizeGenerator {
    fn generate_inputs(&self, count: usize) -> Vec<Vec<u8>> {
        (0..count)
            .map(|i| {
                let size = match i % 3 {
                    0 => 1000,      // 1KB
                    1 => 100_000,   // 100KB
                    _ => 1_000_000, // 1MB
                };
                vec![i as u8; size]
            })
            .collect()
    }
}

#[tokio::main]
async fn main() {
    println!("Blueprint Memory Profiling Example - Testing on macOS\n");

    let generator = VaryingSizeGenerator;

    // Profile a memory-intensive function
    let profile = profile_job(
        0,
        |input| async move {
            // Allocate memory proportional to input size
            let mut buffer = Vec::with_capacity(input.len() * 2);
            buffer.extend_from_slice(&input);
            buffer.extend_from_slice(&input);

            // Do some computation
            let sum: u64 = buffer.iter().map(|&b| b as u64).sum();
            sum.to_le_bytes().to_vec()
        },
        &generator,
        15, // 15 samples
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

    println!("✅ Memory profiling completed successfully on macOS!");
    println!("   Note: Memory measurements use libc::getrusage which works cross-platform");
}
