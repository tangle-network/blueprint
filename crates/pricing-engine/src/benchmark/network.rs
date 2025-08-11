// Network benchmarking module for measuring network performance

use crate::error::{PricingError, Result};
use blueprint_core::info;
use std::process::Command;
use std::str;
use std::time::{Duration, Instant};

use super::{BenchmarkRunConfig, NetworkBenchmarkResult};

/// Run a network-intensive benchmark to measure network performance
pub fn run_network_benchmark(_config: &BenchmarkRunConfig) -> Result<NetworkBenchmarkResult> {
    info!("Running network benchmark");

    // Start timing
    let start_time = Instant::now();

    // Get initial network stats
    let (initial_rx_bytes, initial_tx_bytes) = super::utils::get_network_stats()?;

    // Measure latency first with ping
    let (latency_ms, jitter_ms, packet_loss) = measure_network_latency()?;

    info!(
        "Network latency: {:.2} ms, jitter: {:.2} ms, packet loss: {:.2}%",
        latency_ms, jitter_ms, packet_loss
    );

    // Create a network-intensive workload for download
    // We'll use curl to download a file from a reliable server
    let download_start = Instant::now();
    let download_size_bytes = 100_000_000; // 100MB download
    let download_url = format!("https://speed.cloudflare.com/__down?bytes={download_size_bytes}",);

    info!("Starting download test ({download_size_bytes} bytes)...");

    let network_command = "curl";
    let network_args = vec![
        "-s".to_string(),
        "-o".to_string(),
        "/dev/null".to_string(),
        download_url,
    ];

    // Run the download command
    let status = Command::new(network_command)
        .args(&network_args)
        .status()
        .map_err(|e| {
            PricingError::Benchmark(format!("Failed to run network download benchmark: {e}"))
        })?;

    if !status.success() {
        return Err(PricingError::Benchmark(
            "Network download benchmark command failed".to_string(),
        ));
    }

    let download_duration = download_start.elapsed();
    let download_duration_secs = download_duration.as_secs_f32();

    // Calculate download speed in Mbps (megabits per second)
    let download_speed_mbps = if download_duration_secs > 0.0 {
        (download_size_bytes as f32 * 8.0 / 1_000_000.0) / download_duration_secs
    } else {
        0.0
    };

    info!(
        "Download test completed in {:.2} seconds ({:.2} Mbps)",
        download_duration_secs, download_speed_mbps
    );

    // Create a network-intensive workload for upload
    // We'll use curl to upload data to a reliable server
    let upload_start = Instant::now();
    let upload_size_bytes = 10_000_000; // 10MB upload

    info!("Starting upload test ({} bytes)...", upload_size_bytes);

    // Create a temporary file with random data for upload
    let temp_file = "/tmp/network_benchmark_upload.dat";
    let _ = Command::new("dd")
        .args([
            "if=/dev/urandom",
            &format!("of={temp_file}"),
            "bs=1M",
            "count=10", // 10MB file
        ])
        .output()
        .map_err(|e| PricingError::Benchmark(format!("Failed to create upload test file: {e}")))?;

    // Upload the file
    let upload_status = Command::new("curl")
        .args([
            "-s",
            "-X",
            "POST",
            "-F",
            &format!("file=@{temp_file}"),
            "https://httpbin.org/post", // This endpoint accepts file uploads
        ])
        .output()
        .map_err(|e| {
            PricingError::Benchmark(format!("Failed to run network upload benchmark: {e}"))
        })?;

    // Clean up the temporary file
    let _ = std::fs::remove_file(temp_file);

    if !upload_status.status.success() {
        return Err(PricingError::Benchmark(
            "Network upload benchmark command failed".to_string(),
        ));
    }

    let upload_duration = upload_start.elapsed();
    let upload_duration_secs = upload_duration.as_secs_f32();

    // Calculate upload speed in Mbps (megabits per second)
    let upload_speed_mbps = if upload_duration_secs > 0.0 {
        (upload_size_bytes as f32 * 8.0 / 1_000_000.0) / upload_duration_secs
    } else {
        0.0
    };

    info!(
        "Upload test completed in {:.2} seconds ({:.2} Mbps)",
        upload_duration_secs, upload_speed_mbps
    );

    // Wait a moment for network stats to update
    std::thread::sleep(Duration::from_secs(1));

    // Get final network stats
    let (final_rx_bytes, final_tx_bytes) = super::utils::get_network_stats()?;

    // Calculate network usage in MB
    let rx_mb = (final_rx_bytes - initial_rx_bytes) as f32 / 1024.0 / 1024.0;
    let tx_mb = (final_tx_bytes - initial_tx_bytes) as f32 / 1024.0 / 1024.0;

    // Calculate total duration
    let total_duration = start_time.elapsed();
    let duration_ms = total_duration.as_millis() as u64;

    info!(
        "Network benchmark completed in {} ms: RX: {:.2} MB, TX: {:.2} MB",
        duration_ms, rx_mb, tx_mb
    );
    info!(
        "Download: {:.2} Mbps, Upload: {:.2} Mbps, Latency: {:.2} ms",
        download_speed_mbps, upload_speed_mbps, latency_ms
    );

    Ok(NetworkBenchmarkResult {
        network_rx_mb: rx_mb,
        network_tx_mb: tx_mb,
        download_speed_mbps,
        upload_speed_mbps,
        latency_ms,
        duration_ms,
        packet_loss_percent: packet_loss,
        jitter_ms,
    })
}

/// Measure network latency, jitter, and packet loss using ping
fn measure_network_latency() -> Result<(f32, f32, f32)> {
    // Use ping to measure latency to a reliable server (Google's DNS)
    let ping_output = Command::new("ping")
        .args(["-c", "10", "-i", "0.2", "8.8.8.8"])
        .output()
        .map_err(|e| PricingError::Benchmark(format!("Failed to run ping: {e}")))?;

    let ping_output_str = String::from_utf8_lossy(&ping_output.stdout);

    // Parse ping statistics
    let mut latency_ms = 0.0;
    let mut jitter_ms = 0.0;
    let mut packet_loss = 0.0;

    // Extract packet loss percentage
    if let Some(loss_line) = ping_output_str
        .lines()
        .find(|line| line.contains("packet loss"))
    {
        if let Some(loss_str) = loss_line.split_whitespace().find(|s| s.contains('%')) {
            packet_loss = loss_str.trim_end_matches('%').parse::<f32>().unwrap_or(0.0);
        }
    }

    // Extract latency statistics
    if let Some(stats_line) = ping_output_str
        .lines()
        .find(|line| line.contains("min/avg/max/mdev"))
    {
        let parts: Vec<&str> = stats_line.split('=').collect();
        if parts.len() >= 2 {
            let stats_parts: Vec<&str> = parts[1].trim().split('/').collect();
            if stats_parts.len() >= 4 {
                // Average latency
                latency_ms = stats_parts[1].parse::<f32>().unwrap_or(0.0);
                // Jitter (mdev in ping output)
                jitter_ms = stats_parts[3]
                    .split_whitespace()
                    .next()
                    .unwrap_or("0.0")
                    .parse::<f32>()
                    .unwrap_or(0.0);
            }
        }
    }

    Ok((latency_ms, jitter_ms, packet_loss))
}
