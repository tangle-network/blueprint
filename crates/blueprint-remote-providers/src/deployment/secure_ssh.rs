//! Secure SSH client with proper host verification and parameter validation
//!
//! Replaces the insecure SSH implementation with proper security controls

use crate::core::error::{Error, Result};
use shell_escape::escape;
use std::path::{Path, PathBuf};
use tokio::process::Command;
use tracing::{debug, info, warn};

/// Secure SSH connection configuration with validation
#[derive(Debug, Clone)]
pub struct SecureSshConnection {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub key_path: Option<PathBuf>,
    pub jump_host: Option<String>,
    /// Known hosts file path for host key verification
    pub known_hosts_file: Option<PathBuf>,
    /// Whether to perform strict host key checking
    pub strict_host_checking: bool,
}

impl SecureSshConnection {
    /// Create new secure SSH connection with validation
    pub fn new(host: String, user: String) -> Result<Self> {
        Self::validate_hostname(&host)?;
        Self::validate_username(&user)?;

        Ok(Self {
            host,
            port: 22,
            user,
            key_path: None,
            jump_host: None,
            known_hosts_file: None,
            strict_host_checking: true, // SECURE DEFAULT
        })
    }

    /// Set SSH port with validation
    pub fn with_port(mut self, port: u16) -> Result<Self> {
        if port == 0 {
            return Err(Error::ConfigurationError(format!(
                "Invalid SSH port: {}",
                port
            )));
        }
        self.port = port;
        Ok(self)
    }

    /// Set SSH key path with validation
    pub fn with_key_path<P: AsRef<Path>>(mut self, key_path: P) -> Result<Self> {
        let path = key_path.as_ref();
        Self::validate_key_path(path)?;
        self.key_path = Some(path.to_path_buf());
        Ok(self)
    }

    /// Set jump host with validation
    pub fn with_jump_host(mut self, jump_host: String) -> Result<Self> {
        Self::validate_hostname(&jump_host)?;
        self.jump_host = Some(jump_host);
        Ok(self)
    }

    /// Set known hosts file for host verification
    pub fn with_known_hosts<P: AsRef<Path>>(mut self, known_hosts: P) -> Result<Self> {
        let path = known_hosts.as_ref();
        if !path.exists() {
            warn!("Known hosts file does not exist: {}", path.display());
        }
        self.known_hosts_file = Some(path.to_path_buf());
        Ok(self)
    }

    /// Enable or disable strict host key checking (DANGEROUS if disabled)
    pub fn with_strict_host_checking(mut self, strict: bool) -> Self {
        if !strict {
            warn!("SECURITY WARNING: Disabling strict host key checking - MITM attacks possible!");
        }
        self.strict_host_checking = strict;
        self
    }

    /// Validate hostname format and security
    fn validate_hostname(host: &str) -> Result<()> {
        if host.is_empty() || host.len() > 253 {
            return Err(Error::ConfigurationError("Invalid hostname length".into()));
        }

        // Check for dangerous characters that could be used for injection
        let dangerous_chars = [
            ';', '&', '|', '`', '$', '(', ')', '{', '}', '<', '>', '"', '\'', '\\',
        ];
        if host.chars().any(|c| dangerous_chars.contains(&c)) {
            return Err(Error::ConfigurationError(format!(
                "Hostname contains dangerous characters: {}",
                host
            )));
        }

        // Basic hostname format validation
        if !host
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-._".contains(c))
        {
            return Err(Error::ConfigurationError(format!(
                "Invalid hostname format: {}",
                host
            )));
        }

        Ok(())
    }

    /// Validate username format and security
    fn validate_username(user: &str) -> Result<()> {
        if user.is_empty() || user.len() > 32 {
            return Err(Error::ConfigurationError("Invalid username length".into()));
        }

        // Check for dangerous characters
        let dangerous_chars = [
            ';', '&', '|', '`', '$', '(', ')', '{', '}', '<', '>', '"', '\'', '\\',
        ];
        if user.chars().any(|c| dangerous_chars.contains(&c)) {
            return Err(Error::ConfigurationError(format!(
                "Username contains dangerous characters: {}",
                user
            )));
        }

        // Username should be alphanumeric + underscore/hyphen
        if !user
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_".contains(c))
        {
            return Err(Error::ConfigurationError(format!(
                "Invalid username format: {}",
                user
            )));
        }

        Ok(())
    }

    /// Validate SSH key path
    fn validate_key_path(path: &Path) -> Result<()> {
        // Check that path doesn't contain dangerous patterns
        let path_str = path
            .to_str()
            .ok_or_else(|| Error::ConfigurationError("Invalid UTF-8 in key path".into()))?;

        if path_str.contains("../") || path_str.contains("..\\") {
            return Err(Error::ConfigurationError(
                "Path traversal detected in key path".into(),
            ));
        }

        if !path.exists() {
            return Err(Error::ConfigurationError(format!(
                "SSH key file does not exist: {}",
                path.display()
            )));
        }

        // Check file permissions (should be readable only by owner)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = path.metadata().map_err(|e| {
                Error::ConfigurationError(format!("Cannot read key file metadata: {}", e))
            })?;
            let perms = metadata.permissions().mode();

            // SSH keys should be 600 or 400 (owner read/write or read-only)
            if perms & 0o077 != 0 {
                warn!(
                    "SSH key file has overly permissive permissions: {:o}",
                    perms
                );
            }
        }

        Ok(())
    }
}

/// Secure SSH client with proper security controls
pub struct SecureSshClient {
    connection: SecureSshConnection,
}

impl SecureSshClient {
    /// Create new secure SSH client
    pub fn new(connection: SecureSshConnection) -> Self {
        Self { connection }
    }

    /// Execute command on remote host with security validation
    pub async fn run_remote_command(&self, command: &str) -> Result<String> {
        // Validate command for basic safety
        self.validate_command(command)?;

        let ssh_cmd = self.build_secure_ssh_command(command)?;

        debug!("Executing SSH command: {}", ssh_cmd);

        let output = Command::new("sh")
            .arg("-c")
            .arg(&ssh_cmd)
            .output()
            .await
            .map_err(|e| Error::ConfigurationError(format!("SSH command failed: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigurationError(format!(
                "Remote command failed: {}",
                stderr
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Build secure SSH command with proper escaping and validation
    fn build_secure_ssh_command(&self, command: &str) -> Result<String> {
        let mut ssh_cmd = String::from("ssh");

        // Add security options based on configuration
        if self.connection.strict_host_checking {
            ssh_cmd.push_str(" -o StrictHostKeyChecking=yes");

            // Use known hosts file if provided, otherwise use default
            if let Some(ref known_hosts) = self.connection.known_hosts_file {
                ssh_cmd.push_str(&format!(
                    " -o UserKnownHostsFile={}",
                    escape(known_hosts.to_str().unwrap().into())
                ));
            }
        } else {
            // DANGEROUS: Only allow if explicitly configured
            warn!("Using insecure SSH configuration - MITM attacks possible!");
            ssh_cmd.push_str(" -o StrictHostKeyChecking=no");
            ssh_cmd.push_str(" -o UserKnownHostsFile=/dev/null");
        }

        // Add connection timeout and other security options
        ssh_cmd.push_str(" -o ConnectTimeout=30");
        ssh_cmd.push_str(" -o ServerAliveInterval=60");
        ssh_cmd.push_str(" -o ServerAliveCountMax=3");
        ssh_cmd.push_str(" -o BatchMode=yes"); // Disable interactive prompts

        // Add port if not default (with validation)
        if self.connection.port != 22 {
            ssh_cmd.push_str(&format!(" -p {}", self.connection.port));
        }

        // Add identity file if provided (with validation and escaping)
        if let Some(ref key_path) = self.connection.key_path {
            let escaped_path = escape(key_path.to_str().unwrap().into());
            ssh_cmd.push_str(&format!(" -i {}", escaped_path));
        }

        // Add jump host if provided (with validation and escaping)
        if let Some(ref jump_host) = self.connection.jump_host {
            let escaped_jump = escape(jump_host.into());
            ssh_cmd.push_str(&format!(" -J {}", escaped_jump));
        }

        // Add user@host with proper escaping
        let escaped_user = escape(self.connection.user.as_str().into());
        let escaped_host = escape(self.connection.host.as_str().into());
        ssh_cmd.push_str(&format!(" {}@{}", escaped_user, escaped_host));

        // Add the command to execute with proper escaping
        let escaped_command = escape(command.into());
        ssh_cmd.push_str(&format!(" {}", escaped_command));

        Ok(ssh_cmd)
    }

    /// Validate command for basic security
    fn validate_command(&self, command: &str) -> Result<()> {
        if command.is_empty() {
            return Err(Error::ConfigurationError(
                "Empty command not allowed".into(),
            ));
        }

        if command.len() > 8192 {
            return Err(Error::ConfigurationError("Command too long".into()));
        }

        // Check for extremely dangerous patterns
        let dangerous_patterns = [
            "rm -rf /",
            ":(){ :|:& };:", // Fork bomb
            "dd if=/dev/zero",
            "mkfs.",
            "fdisk",
            "parted",
        ];

        for pattern in &dangerous_patterns {
            if command.contains(pattern) {
                return Err(Error::ConfigurationError(format!(
                    "Dangerous command pattern detected: {}",
                    pattern
                )));
            }
        }

        Ok(())
    }

    /// Secure file copy with validation
    pub async fn copy_files(&self, local_path: &Path, remote_path: &str) -> Result<()> {
        // Validate paths
        self.validate_local_path(local_path)?;
        self.validate_remote_path(remote_path)?;

        let scp_cmd = self.build_secure_scp_command(local_path, remote_path)?;

        info!(
            "Copying files via SCP: {} -> {}",
            local_path.display(),
            remote_path
        );

        let output = Command::new("sh")
            .arg("-c")
            .arg(&scp_cmd)
            .output()
            .await
            .map_err(|e| Error::ConfigurationError(format!("SCP failed: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigurationError(format!(
                "File copy failed: {}",
                stderr
            )));
        }

        info!("Files copied successfully");
        Ok(())
    }

    /// Build secure SCP command
    fn build_secure_scp_command(&self, local_path: &Path, remote_path: &str) -> Result<String> {
        let mut scp_cmd = String::from("scp");

        // Add security options (same as SSH)
        if self.connection.strict_host_checking {
            scp_cmd.push_str(" -o StrictHostKeyChecking=yes");
            if let Some(ref known_hosts) = self.connection.known_hosts_file {
                scp_cmd.push_str(&format!(
                    " -o UserKnownHostsFile={}",
                    escape(known_hosts.to_str().unwrap().into())
                ));
            }
        } else {
            warn!("Using insecure SCP configuration");
            scp_cmd.push_str(" -o StrictHostKeyChecking=no");
            scp_cmd.push_str(" -o UserKnownHostsFile=/dev/null");
        }

        // Add port if not default
        if self.connection.port != 22 {
            scp_cmd.push_str(&format!(" -P {}", self.connection.port));
        }

        // Add identity file if provided
        if let Some(ref key_path) = self.connection.key_path {
            let escaped_path = escape(key_path.to_str().unwrap().into());
            scp_cmd.push_str(&format!(" -i {}", escaped_path));
        }

        // Add source and destination with proper escaping
        let escaped_local = escape(local_path.to_str().unwrap().into());
        let escaped_user = escape(self.connection.user.as_str().into());
        let escaped_host = escape(self.connection.host.as_str().into());
        let escaped_remote = escape(remote_path.into());

        scp_cmd.push_str(&format!(
            " {} {}@{}:{}",
            escaped_local, escaped_user, escaped_host, escaped_remote
        ));

        Ok(scp_cmd)
    }

    /// Validate local file path
    fn validate_local_path(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(Error::ConfigurationError(format!(
                "Local file does not exist: {}",
                path.display()
            )));
        }

        // Check for path traversal
        let path_str = path
            .to_str()
            .ok_or_else(|| Error::ConfigurationError("Invalid UTF-8 in local path".into()))?;

        if path_str.contains("../") || path_str.contains("..\\") {
            return Err(Error::ConfigurationError(
                "Path traversal detected in local path".into(),
            ));
        }

        Ok(())
    }

    /// Validate remote path
    fn validate_remote_path(&self, path: &str) -> Result<()> {
        if path.is_empty() {
            return Err(Error::ConfigurationError("Empty remote path".into()));
        }

        if path.len() > 4096 {
            return Err(Error::ConfigurationError("Remote path too long".into()));
        }

        // Check for dangerous characters
        let dangerous_chars = [
            ';', '&', '|', '`', '$', '(', ')', '{', '}', '<', '>', '"', '\\',
        ];
        if path.chars().any(|c| dangerous_chars.contains(&c)) {
            return Err(Error::ConfigurationError(format!(
                "Remote path contains dangerous characters: {}",
                path
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_ssh_connection_validation() {
        // Valid connection
        let conn = SecureSshConnection::new("example.com".to_string(), "user".to_string()).unwrap();
        assert_eq!(conn.host, "example.com");
        assert_eq!(conn.user, "user");
        assert!(conn.strict_host_checking); // Secure default

        // Invalid hostname
        assert!(
            SecureSshConnection::new("host; rm -rf /".to_string(), "user".to_string()).is_err()
        );

        // Invalid username
        assert!(
            SecureSshConnection::new("example.com".to_string(), "user; id".to_string()).is_err()
        );
    }

    #[test]
    fn test_command_validation() {
        let conn = SecureSshConnection::new("example.com".to_string(), "user".to_string()).unwrap();
        let client = SecureSshClient::new(conn);

        // Valid command
        assert!(client.validate_command("ls -la").is_ok());

        // Dangerous commands
        assert!(client.validate_command("rm -rf /").is_err());
        assert!(client.validate_command(":(){ :|:& };:").is_err());

        // Empty command
        assert!(client.validate_command("").is_err());
    }

    #[test]
    fn test_hostname_validation() {
        assert!(SecureSshConnection::validate_hostname("example.com").is_ok());
        assert!(SecureSshConnection::validate_hostname("192.168.1.1").is_ok());

        assert!(SecureSshConnection::validate_hostname("host; rm -rf /").is_err());
        assert!(SecureSshConnection::validate_hostname("host$(curl evil.com)").is_err());
        assert!(SecureSshConnection::validate_hostname("").is_err());
    }
}
