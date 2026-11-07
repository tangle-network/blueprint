//! Secure command execution utilities to prevent command injection
//!
//! This module provides safe alternatives to the vulnerable string interpolation
//! patterns that were identified in the security audit.

use crate::ResourceSpec;
use crate::core::error::{Error, Result};
use blueprint_std::collections::HashMap;
use shell_escape::escape;

/// Secure container command builder that prevents injection attacks
pub struct SecureContainerCommands;

impl SecureContainerCommands {
    /// Safely build a container pull command with validated image name
    pub fn build_pull_command(runtime: &str, image: &str) -> Result<String> {
        // Validate image name format (basic Docker image name validation)
        if !Self::is_valid_image_name(image) {
            return Err(Error::ConfigurationError(format!(
                "Invalid image name: {image}. Image names must follow Docker naming conventions."
            )));
        }

        let escaped_image = escape(image.into());
        let escaped_runtime = escape(runtime.into());

        Ok(format!("{escaped_runtime} pull {escaped_image}"))
    }

    /// Build a container command with resources and a protected remote environment file.
    pub fn build_create_command(
        runtime: &str,
        image: &str,
        env_file: &str,
        spec: &ResourceSpec,
    ) -> Result<String> {
        spec.validate().map_err(Error::ConfigurationError)?;
        if !Self::is_valid_image_name(image)
            || !std::path::Path::new(env_file).is_absolute()
            || env_file.contains(['\0', '\n', '\r'])
        {
            return Err(Error::ConfigurationError(
                "Invalid container launch request".into(),
            ));
        }
        if !matches!(runtime, "docker" | "podman")
            || (spec.gpu_count.is_some() && runtime != "docker")
        {
            return Err(Error::Provider(
                crate::managed::ProviderErrorCode::Unsupported,
            ));
        }
        let mut cmd = format!(
            "{} create --cpus={} --memory={}m --env-file {}",
            runtime,
            spec.cpu,
            (spec.memory_gb * 1024.0).ceil() as u64,
            escape(env_file.into())
        );
        if let Some(count) = spec.gpu_count {
            cmd.push_str(&format!(" --gpus {count}"));
        }
        let mut ports = spec.ports.clone();
        ports.extend(spec.endpoint_port);
        ports.sort_unstable();
        ports.dedup();
        if ports.contains(&0) {
            return Err(Error::ConfigurationError("Invalid container port".into()));
        }
        for port in ports {
            cmd.push_str(&format!(" -p 127.0.0.1:{port}:{port}"));
        }
        cmd.push_str(" --user 1000:1000 --read-only --tmpfs /tmp:noexec,nosuid,size=100m --tmpfs /var/run:noexec,nosuid,size=100m --cap-drop ALL --cap-add NET_BIND_SERVICE --security-opt no-new-privileges --pids-limit 256 --ulimit nproc=256 --ulimit nofile=1024 --memory-swappiness=0");
        cmd.push_str(&format!(" {}", escape(image.into())));
        Ok(cmd)
    }

    /// Encode environment-file contents without shell interpretation or secret-bearing errors.
    pub fn environment_file(env: &HashMap<String, String>, systemd: bool) -> Result<String> {
        let mut entries = env.iter().collect::<Vec<_>>();
        entries.sort_by_key(|(key, _)| *key);
        let mut content = String::new();
        for (key, value) in entries {
            if key.is_empty()
                || key.len() > 255
                || !key.bytes().enumerate().all(|(index, byte)| {
                    byte == b'_'
                        || byte.is_ascii_alphabetic()
                        || (index > 0 && byte.is_ascii_digit())
                })
                || value.len() > 4096
                || value.contains(['\0', '\r', '\n'])
            {
                return Err(Error::ConfigurationError(
                    "Invalid workload environment".into(),
                ));
            }
            content.push_str(key);
            content.push('=');
            if systemd {
                content.push('"');
                content.push_str(&value.replace('\\', "\\\\").replace('"', "\\\""));
                content.push('"');
            } else {
                content.push_str(value);
            }
            content.push('\n');
        }
        Ok(content)
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
                "Invalid container ID: {container_id}. Container IDs must be alphanumeric."
            )));
        }

        // Validate action (whitelist approach)
        let valid_actions = ["start", "stop", "logs", "inspect", "rm"];
        if !valid_actions.contains(&action) {
            return Err(Error::ConfigurationError(format!(
                "Invalid container action: {action}. Allowed actions: {valid_actions:?}"
            )));
        }

        let escaped_runtime = escape(runtime.into());
        let escaped_action = escape(action.into());
        let escaped_id = escape(container_id.into());

        let mut cmd = format!("{escaped_runtime} {escaped_action} {escaped_id}");

        // Add follow flag for logs if specified
        if action == "logs" && follow_logs.unwrap_or(false) {
            cmd = format!("{escaped_runtime} {escaped_action} -f {escaped_id}");
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
        let dangerous_chars = [
            ';', '&', '|', '`', '$', '(', ')', '{', '}', '[', ']', '<', '>', '"', '\'', '\\',
        ];
        if image.chars().any(|c| dangerous_chars.contains(&c)) {
            return false;
        }

        // Must not start with slash, dash, or dot
        if image.starts_with('/') || image.starts_with('-') || image.starts_with('.') {
            return false;
        }

        // Basic format validation (simplified)
        image
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-._/:@".contains(c))
    }

    /// Validate container ID format
    fn is_valid_container_id(container_id: &str) -> bool {
        if container_id.is_empty() || container_id.len() > 64 {
            return false;
        }

        // Container IDs should be hexadecimal or alphanumeric
        container_id.chars().all(|c| c.is_ascii_alphanumeric())
    }
}
