//! Secure command execution utilities to prevent command injection
//! 
//! This module provides safe alternatives to the vulnerable string interpolation
//! patterns that were identified in the security audit.

use crate::core::error::{Error, Result};
use shell_escape::escape;
use std::collections::HashMap;
use std::path::Path;
use tokio::process::Command as AsyncCommand;

/// Secure container command builder that prevents injection attacks
pub struct SecureContainerCommands;

impl SecureContainerCommands {
    /// Safely build a container pull command with validated image name
    pub fn build_pull_command(runtime: &str, image: &str) -> Result<String> {
        // Validate image name format (basic Docker image name validation)
        if !Self::is_valid_image_name(image) {
            return Err(Error::ConfigurationError(format!(
                "Invalid image name: {}. Image names must follow Docker naming conventions.",
                image
            )));
        }
        
        let escaped_image = escape(image.into());
        let escaped_runtime = escape(runtime.into());
        
        Ok(format!("{} pull {}", escaped_runtime, escaped_image))
    }
    
    /// Safely build a container create command with escaped environment variables
    pub fn build_create_command(
        runtime: &str,
        image: &str,
        env_vars: &HashMap<String, String>,
        cpu_cores: Option<f32>,
        memory_mb: Option<u32>,
        disk_gb: Option<u32>,
    ) -> Result<String> {
        // Validate inputs
        if !Self::is_valid_image_name(image) {
            return Err(Error::ConfigurationError(format!(
                "Invalid image name: {}", image
            )));
        }
        
        Self::validate_env_vars(env_vars)?;
        Self::validate_resource_limits(cpu_cores, memory_mb, disk_gb)?;
        
        let mut cmd = format!("{} create", escape(runtime.into()));
        
        // Add resource limits safely
        if let Some(cpu) = cpu_cores {
            cmd.push_str(&format!(" --cpus={}", Self::format_cpu_limit(cpu)?));
        }
        if let Some(mem) = memory_mb {
            cmd.push_str(&format!(" --memory={}m", mem));
        }
        if let Some(disk) = disk_gb {
            cmd.push_str(&format!(" --storage-opt size={}G", disk));
        }
        
        // Add environment variables with proper escaping
        for (key, value) in env_vars {
            let escaped_key = escape(key.into());
            let escaped_value = escape(value.into());
            cmd.push_str(&format!(" -e {}={}", escaped_key, escaped_value));
        }
        
        // Add security hardening options
        cmd.push_str(" --user 1000:1000"); // Non-root user
        cmd.push_str(" --read-only"); // Read-only filesystem
        cmd.push_str(" --tmpfs /tmp:noexec,nosuid,size=100m"); // Secure tmpfs
        cmd.push_str(" --tmpfs /var/run:noexec,nosuid,size=100m"); 
        cmd.push_str(" --cap-drop ALL"); // Drop all capabilities
        cmd.push_str(" --cap-add NET_BIND_SERVICE"); // Only allow port binding
        cmd.push_str(" --security-opt no-new-privileges"); // Prevent privilege escalation
        cmd.push_str(" --pids-limit 256"); // Limit process count
        cmd.push_str(" --ulimit nproc=256"); // User process limit
        cmd.push_str(" --ulimit nofile=1024"); // File descriptor limit
        cmd.push_str(" --memory-swappiness=0"); // Disable swap
        
        // Network configuration (localhost only for security)
        cmd.push_str(" -p 127.0.0.1:8080:8080"); // Blueprint endpoint
        cmd.push_str(" -p 127.0.0.1:9615:9615"); // QoS gRPC metrics port
        cmd.push_str(" -p 127.0.0.1:9944:9944"); // RPC endpoint for heartbeat
        
        // Add container name and image with timestamp
        let timestamp = chrono::Utc::now().timestamp();
        let escaped_image = escape(image.into());
        cmd.push_str(&format!(" --name blueprint-{} {}", timestamp, escaped_image));
        
        Ok(cmd)
    }
    
    /// Safely build container management commands (start, stop, logs, etc.)
    pub fn build_container_command(
        runtime: &str,
        action: &str,
        container_id: &str,
        follow_logs: Option<bool>,
    ) -> Result<String> {
        // Validate container ID format (Docker container ID validation)
        if !Self::is_valid_container_id(container_id) {
            return Err(Error::ConfigurationError(format!(
                "Invalid container ID: {}. Container IDs must be alphanumeric.", 
                container_id
            )));
        }
        
        // Validate action (whitelist approach)
        let valid_actions = ["start", "stop", "logs", "inspect", "rm"];
        if !valid_actions.contains(&action) {
            return Err(Error::ConfigurationError(format!(
                "Invalid container action: {}. Allowed actions: {:?}",
                action, valid_actions
            )));
        }
        
        let escaped_runtime = escape(runtime.into());
        let escaped_action = escape(action.into());
        let escaped_id = escape(container_id.into());
        
        let mut cmd = format!("{} {} {}", escaped_runtime, escaped_action, escaped_id);
        
        // Add follow flag for logs if specified
        if action == "logs" && follow_logs.unwrap_or(false) {
            cmd = format!("{} {} -f {}", escaped_runtime, escaped_action, escaped_id);
        }
        
        Ok(cmd)
    }
    
    /// Validate image name follows Docker conventions
    fn is_valid_image_name(image: &str) -> bool {
        // Basic Docker image name validation
        // Format: [registry/]namespace/repository[:tag][@digest]
        
        if image.is_empty() || image.len() > 255 {
            return false;
        }
        
        // Check for dangerous characters that could be used for injection
        let dangerous_chars = [';', '&', '|', '`', '$', '(', ')', '{', '}', '[', ']', '<', '>', '"', '\'', '\\'];
        if image.chars().any(|c| dangerous_chars.contains(&c)) {
            return false;
        }
        
        // Must not start with slash, dash, or dot
        if image.starts_with('/') || image.starts_with('-') || image.starts_with('.') {
            return false;
        }
        
        // Basic format validation (simplified)
        image.chars().all(|c| c.is_ascii_alphanumeric() || "-._/:@".contains(c))
    }
    
    /// Validate container ID format
    fn is_valid_container_id(container_id: &str) -> bool {
        if container_id.is_empty() || container_id.len() > 64 {
            return false;
        }
        
        // Container IDs should be hexadecimal or alphanumeric
        container_id.chars().all(|c| c.is_ascii_alphanumeric())
    }
    
    /// Validate environment variables for safety
    fn validate_env_vars(env_vars: &HashMap<String, String>) -> Result<()> {
        for (key, value) in env_vars {
            // Validate environment variable names
            if key.is_empty() || key.len() > 255 {
                return Err(Error::ConfigurationError(format!(
                    "Invalid environment variable name length: {}", key
                )));
            }
            
            // Environment variable names should be alphanumeric + underscore
            if !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                return Err(Error::ConfigurationError(format!(
                    "Invalid environment variable name: {}. Names must be alphanumeric + underscore.", key
                )));
            }
            
            // Validate environment variable values
            if value.len() > 4096 {
                return Err(Error::ConfigurationError(format!(
                    "Environment variable value too long: {} (max 4096 chars)", key
                )));
            }
            
            // Check for suspicious patterns in values
            let suspicious_patterns = [
                ";", "&&", "||", "|", "`", "$(", "${", ")", "}", 
                "curl ", "wget ", "nc ", "netcat", "/bin/", "/usr/bin/",
                "bash", "sh ", "exec", "eval", "base64", "echo '", "cat "
            ];
            
            for pattern in &suspicious_patterns {
                if value.contains(pattern) {
                    return Err(Error::ConfigurationError(format!(
                        "Suspicious pattern '{}' detected in environment variable '{}': {}", 
                        pattern, key, value
                    )));
                }
            }
        }
        
        Ok(())
    }
    
    /// Validate resource limits
    fn validate_resource_limits(
        cpu_cores: Option<f32>,
        memory_mb: Option<u32>,
        disk_gb: Option<u32>,
    ) -> Result<()> {
        if let Some(cpu) = cpu_cores {
            if cpu <= 0.0 || cpu > 32.0 || !cpu.is_finite() {
                return Err(Error::ConfigurationError(format!(
                    "Invalid CPU limit: {}. Must be between 0.1 and 32.0 cores.", cpu
                )));
            }
        }
        
        if let Some(memory) = memory_mb {
            if memory == 0 || memory > 128 * 1024 {
                return Err(Error::ConfigurationError(format!(
                    "Invalid memory limit: {}MB. Must be between 1MB and 128GB.", memory
                )));
            }
        }
        
        if let Some(disk) = disk_gb {
            if disk == 0 || disk > 1024 {
                return Err(Error::ConfigurationError(format!(
                    "Invalid disk limit: {}GB. Must be between 1GB and 1TB.", disk
                )));
            }
        }
        
        Ok(())
    }
    
    /// Format CPU limit safely
    fn format_cpu_limit(cpu: f32) -> Result<String> {
        if !cpu.is_finite() || cpu <= 0.0 {
            return Err(Error::ConfigurationError(format!(
                "Invalid CPU value: {}", cpu
            )));
        }
        
        Ok(format!("{:.2}", cpu))
    }
}

/// Secure configuration file management
pub struct SecureConfigManager;

impl SecureConfigManager {
    /// Safely write configuration file without shell injection
    pub async fn write_config_file<P: AsRef<Path>>(
        config_content: &str,
        target_path: P,
    ) -> Result<()> {
        // Validate configuration content
        Self::validate_config_content(config_content)?;
        
        // Write to temporary file first
        let temp_path = "/tmp/blueprint_config_temp.json";
        tokio::fs::write(temp_path, config_content).await
            .map_err(|e| Error::ConfigurationError(format!("Failed to write temp config: {}", e)))?;
        
        // Use secure file operations instead of shell commands
        let mut cmd = AsyncCommand::new("sudo");
        cmd.args(&["cp", temp_path, target_path.as_ref().to_str().unwrap()]);
        
        let output = cmd.output().await
            .map_err(|e| Error::ConfigurationError(format!("Failed to copy config: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigurationError(format!(
                "Config copy failed: {}", stderr
            )));
        }
        
        // Clean up temporary file
        let _ = tokio::fs::remove_file(temp_path).await;
        
        Ok(())
    }
    
    /// Validate configuration content for safety
    fn validate_config_content(content: &str) -> Result<()> {
        // Validate JSON structure
        let _: serde_json::Value = serde_json::from_str(content)
            .map_err(|e| Error::ConfigurationError(format!("Invalid JSON config: {}", e)))?;
        
        // Check for suspicious patterns in configuration
        let suspicious_patterns = [
            "';", "\";", "`;", "&&", "||", "|", "$(", "${", "`",
            "/bin/", "/usr/bin/", "bash", "sh ", "curl ", "wget ",
            "nc ", "netcat", "exec", "eval", "system", "base64"
        ];
        
        for pattern in &suspicious_patterns {
            if content.contains(pattern) {
                return Err(Error::ConfigurationError(format!(
                    "Suspicious pattern '{}' detected in configuration", pattern
                )));
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_image_names() {
        assert!(SecureContainerCommands::is_valid_image_name("nginx:latest"));
        assert!(SecureContainerCommands::is_valid_image_name("registry.io/namespace/repo:tag"));
        assert!(SecureContainerCommands::is_valid_image_name("ubuntu"));
        assert!(SecureContainerCommands::is_valid_image_name("my-app_v1.0"));
    }
    
    #[test]
    fn test_invalid_image_names() {
        assert!(!SecureContainerCommands::is_valid_image_name("nginx; rm -rf /"));
        assert!(!SecureContainerCommands::is_valid_image_name("image$(curl evil.com)"));
        assert!(!SecureContainerCommands::is_valid_image_name("img`ls`"));
        assert!(!SecureContainerCommands::is_valid_image_name("img && echo pwned"));
        assert!(!SecureContainerCommands::is_valid_image_name(""));
    }
    
    #[test]
    fn test_valid_container_ids() {
        assert!(SecureContainerCommands::is_valid_container_id("abc123"));
        assert!(SecureContainerCommands::is_valid_container_id("1234567890abcdef"));
        assert!(SecureContainerCommands::is_valid_container_id("f1d2e3"));
    }
    
    #[test]
    fn test_invalid_container_ids() {
        assert!(!SecureContainerCommands::is_valid_container_id("abc123; rm -rf /"));
        assert!(!SecureContainerCommands::is_valid_container_id("id$(curl evil.com)"));
        assert!(!SecureContainerCommands::is_valid_container_id(""));
    }
    
    #[test]
    fn test_env_var_validation() {
        let mut valid_vars = HashMap::new();
        valid_vars.insert("API_KEY".to_string(), "valid_value_123".to_string());
        valid_vars.insert("PORT".to_string(), "8080".to_string());
        
        assert!(SecureContainerCommands::validate_env_vars(&valid_vars).is_ok());
        
        let mut malicious_vars = HashMap::new();
        malicious_vars.insert("MALICIOUS".to_string(), "'; rm -rf /; echo '".to_string());
        
        assert!(SecureContainerCommands::validate_env_vars(&malicious_vars).is_err());
    }
}