// src/benchmark/gpu.rs
//
// GPU benchmarking module for detecting GPU availability and memory

use crate::error::Result;
use blueprint_core::info;
use std::path::Path;
use std::process::{Command, Stdio};

use super::{BenchmarkRunConfig, GpuBenchmarkResult};

// GPU benchmark constants
pub const DEFAULT_UNKNOWN_GPU_MEMORY: f32 = 512.0;
pub const DEFAULT_INTEL_GPU_MEMORY: f32 = 1024.0;
pub const DEFAULT_NVIDIA_GPU_MEMORY: f32 = 2048.0;
pub const DEFAULT_AMD_GPU_MEMORY: f32 = 2048.0;

/// Check for GPU availability and memory
pub fn run_gpu_benchmark(_config: &BenchmarkRunConfig) -> Result<GpuBenchmarkResult> {
    info!("Running GPU benchmark");

    // Try multiple methods to detect GPU
    // 1. Try nvidia-smi for NVIDIA GPUs
    if let Ok(output) = Command::new("nvidia-smi")
        .args(&["--query-gpu=memory.total,name,clocks.max.graphics", "--format=csv,noheader,nounits"])
        .output()
    {
        if output.status.success() {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                let output_str = output_str.trim();
                if !output_str.is_empty() {
                    let parts: Vec<&str> = output_str.split(',').collect();
                    if parts.len() >= 3 {
                        if let Ok(memory) = parts[0].trim().parse::<f32>() {
                            let gpu_model = parts[1].trim().to_string();
                            let gpu_frequency_mhz = parts[2].trim().parse::<f32>().unwrap_or(0.0);
                            
                            info!("Detected NVIDIA GPU: {}, {} MHz with {} MB memory", 
                                  gpu_model, gpu_frequency_mhz, memory);
                            
                            return Ok(GpuBenchmarkResult {
                                gpu_available: true,
                                gpu_memory_mb: memory,
                                gpu_model,
                                gpu_frequency_mhz,
                            });
                        }
                    }
                }
            }
        }
    }

    // 2. Try rocm-smi for AMD GPUs
    if let Ok(output) = Command::new("rocm-smi")
        .args(&["--showmeminfo", "vram", "--showproductname"])
        .output()
    {
        if output.status.success() {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                let memory = parse_amd_gpu_memory(&output_str);
                let gpu_model = parse_amd_gpu_model(&output_str);
                
                if memory > 0.0 {
                    info!("Detected AMD GPU: {}, with {} MB memory", gpu_model, memory);
                    return Ok(GpuBenchmarkResult {
                        gpu_available: true,
                        gpu_memory_mb: memory,
                        gpu_model,
                        gpu_frequency_mhz: 0.0, // AMD frequency not easily available
                    });
                }
            }
        }
    }

    // 3. Try intel_gpu_top for Intel GPUs
    if let Ok(output) = Command::new("intel_gpu_top")
        .args(&["-l"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            std::thread::sleep(std::time::Duration::from_millis(100));
            child.kill()?;
            child.wait_with_output()
        })
    {
        if let Ok(output_str) = String::from_utf8(output.stdout) {
            let memory = parse_intel_gpu_memory(&output_str);
            let (gpu_model, gpu_frequency_mhz) = parse_intel_gpu_info(&output_str);
            
            if memory > 0.0 {
                info!("Detected Intel GPU: {}, {} MHz with {} MB memory", 
                      gpu_model, gpu_frequency_mhz, memory);
                
                return Ok(GpuBenchmarkResult {
                    gpu_available: true,
                    gpu_memory_mb: memory,
                    gpu_model,
                    gpu_frequency_mhz,
                });
            }
        }
    }

    // 4. Try glxinfo as a fallback
    if let Ok(output) = Command::new("glxinfo").args(&["-B"]).output() {
        if output.status.success() {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                // Check if we have a GPU renderer
                if output_str.contains("OpenGL renderer string:") {
                    let renderer_line = output_str
                        .lines()
                        .find(|line| line.contains("OpenGL renderer string:"))
                        .unwrap_or("");

                    // Extract GPU model from renderer string
                    let gpu_model = extract_gpu_model_from_glxinfo(renderer_line);
                    
                    // Extract GPU memory if available
                    let memory = parse_glxinfo_memory(&output_str);

                    // Determine GPU type and set default memory if not detected
                    let gpu_memory = if memory > 0.0 {
                        info!("Detected GPU with {} MB memory via glxinfo", memory);
                        memory
                    } else if renderer_line.contains("NVIDIA") {
                        info!(
                            "Detected NVIDIA GPU via glxinfo, assuming {} MB memory",
                            DEFAULT_NVIDIA_GPU_MEMORY
                        );
                        DEFAULT_NVIDIA_GPU_MEMORY
                    } else if renderer_line.contains("AMD") || renderer_line.contains("ATI") {
                        info!(
                            "Detected AMD GPU via glxinfo, assuming {} MB memory",
                            DEFAULT_AMD_GPU_MEMORY
                        );
                        DEFAULT_AMD_GPU_MEMORY
                    } else if renderer_line.contains("Intel") {
                        info!(
                            "Detected Intel GPU via glxinfo, assuming {} MB memory",
                            DEFAULT_INTEL_GPU_MEMORY
                        );
                        DEFAULT_INTEL_GPU_MEMORY
                    } else {
                        info!(
                            "Detected unknown GPU via glxinfo, assuming {} MB memory",
                            DEFAULT_UNKNOWN_GPU_MEMORY
                        );
                        DEFAULT_UNKNOWN_GPU_MEMORY
                    };

                    return Ok(GpuBenchmarkResult {
                        gpu_available: true,
                        gpu_memory_mb: gpu_memory,
                        gpu_model,
                        gpu_frequency_mhz: 0.0, // Frequency not available from glxinfo
                    });
                }
            }
        }
    }

    // 5. Check for device files as a last resort
    let nvidia_device = Path::new("/dev/nvidia0");
    let amdgpu_device = Path::new("/dev/dri/renderD128");

    if nvidia_device.exists() {
        info!(
            "Detected NVIDIA GPU device file, assuming {} MB memory",
            DEFAULT_NVIDIA_GPU_MEMORY
        );
        return Ok(GpuBenchmarkResult {
            gpu_available: true,
            gpu_memory_mb: DEFAULT_NVIDIA_GPU_MEMORY,
            gpu_model: "NVIDIA GPU (detected via device file)".to_string(),
            gpu_frequency_mhz: 0.0,
        });
    } else if amdgpu_device.exists() {
        info!(
            "Detected AMD GPU device file, assuming {} MB memory",
            DEFAULT_AMD_GPU_MEMORY
        );
        return Ok(GpuBenchmarkResult {
            gpu_available: true,
            gpu_memory_mb: DEFAULT_AMD_GPU_MEMORY,
            gpu_model: "AMD GPU (detected via device file)".to_string(),
            gpu_frequency_mhz: 0.0,
        });
    }

    // No GPU detected
    info!("No GPU detected");
    Ok(GpuBenchmarkResult {
        gpu_available: false,
        gpu_memory_mb: 0.0,
        gpu_model: "No GPU detected".to_string(),
        gpu_frequency_mhz: 0.0,
    })
}

/// Helper function to extract GPU model from glxinfo renderer string
fn extract_gpu_model_from_glxinfo(renderer_line: &str) -> String {
    if renderer_line.contains("OpenGL renderer string:") {
        let parts: Vec<&str> = renderer_line.split(':').collect();
        if parts.len() >= 2 {
            return parts[1].trim().to_string();
        }
    }
    "Unknown GPU".to_string()
}

/// Helper function to parse AMD GPU model from rocm-smi output
pub fn parse_amd_gpu_model(output: &str) -> String {
    // Look for product name in rocm-smi output
    for line in output.lines() {
        if line.contains("GPU") && line.contains("Product name") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                return parts[1].trim().to_string();
            }
        }
    }
    "AMD GPU".to_string()
}

/// Helper function to parse Intel GPU information
pub fn parse_intel_gpu_info(output: &str) -> (String, f32) {
    let mut model = "Intel GPU".to_string();
    let mut frequency = 0.0;
    
    // Try to extract model and frequency information
    for line in output.lines() {
        if line.contains("Device:") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                model = parts[1].trim().to_string();
            }
        }
        
        // Extract frequency if available (format: "1539MHz")
        if let Some(freq_str) = line.split_whitespace().next() {
            if freq_str.ends_with("MHz") {
                if let Ok(freq) = freq_str.trim_end_matches("MHz").parse::<f32>() {
                    frequency = freq;
                }
            }
        }
    }
    
    (model, frequency)
}

/// Helper function to parse AMD GPU memory from rocm-smi output
pub fn parse_amd_gpu_memory(output: &str) -> f32 {
    // Example output format:
    // ============================ ROCm System Management Interface ============================
    // ================================= Memory Usage (VRAM) ==================================
    // GPU  VRAM Total (MB)  VRAM Used (MB)  VRAM Free (MB)  Used %
    // 0    8176            123            8053           2%

    for line in output.lines() {
        if line.trim().starts_with("0") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(memory) = parts[1].parse::<f32>() {
                    return memory;
                }
            }
        }
    }
    0.0
}

/// Helper function to parse Intel GPU memory from intel_gpu_top output
pub fn parse_intel_gpu_memory(output: &str) -> f32 {
    // Example output format:
    // intel-gpu-top -  1539MHz   0% RC6  0.0W   0.0ms   0B/s   0B/s   0% engine 0   0% engine 1   0% engine 2   0% engine 3  0.0% VRAM 0.0% MEDIA 0.0% POWER

    for line in output.lines() {
        if line.contains("VRAM") {
            // Try to extract total memory from other parts of the output
            // This is a simplified approach; actual parsing might be more complex
            if let Some(memory_info) = output.lines().find(|l| l.contains("Memory: ")) {
                if let Some(memory_str) = memory_info.split("Memory: ").nth(1) {
                    if let Some(memory_val) = memory_str.split_whitespace().next() {
                        if let Ok(memory) = memory_val.parse::<f32>() {
                            return memory;
                        }
                    }
                }
            }
            // If we can't find exact memory, return a default value
            return DEFAULT_INTEL_GPU_MEMORY;
        }
    }
    0.0
}

/// Helper function to parse GPU memory from glxinfo output
pub fn parse_glxinfo_memory(output: &str) -> f32 {
    // Look for dedicated video memory
    for line in output.lines() {
        if line.contains("Video memory:") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                let memory_part = parts[1].trim();
                // Handle different formats (e.g., "8192 MB", "8 GB")
                if memory_part.ends_with("MB") {
                    if let Ok(memory) = memory_part.trim_end_matches("MB").trim().parse::<f32>() {
                        return memory;
                    }
                } else if memory_part.ends_with("GB") {
                    if let Ok(memory) = memory_part.trim_end_matches("GB").trim().parse::<f32>() {
                        return memory * 1024.0; // Convert GB to MB
                    }
                }
            }
        }
    }

    // Look for any memory output as a fallback
    for line in output.lines() {
        if line.contains("memory:") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                let memory_part = parts[1].trim();
                if memory_part.ends_with("MB") {
                    if let Ok(memory) = memory_part.trim_end_matches("MB").trim().parse::<f32>() {
                        return memory;
                    }
                } else if memory_part.ends_with("GB") {
                    if let Ok(memory) = memory_part.trim_end_matches("GB").trim().parse::<f32>() {
                        return memory * 1024.0; // Convert GB to MB
                    }
                }
            }
        }
    }

    0.0
}
