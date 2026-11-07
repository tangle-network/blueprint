//! Secure SSH client with proper host verification and parameter validation
//!
//! Replaces the insecure SSH implementation with proper security controls

use crate::core::error::{Error, Result};
use blueprint_core::warn;
use blueprint_std::path::{Path, PathBuf};
use shell_escape::escape;
use tokio::process::Command;

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
        })
    }

    /// Set SSH port with validation
    pub fn with_port(mut self, port: u16) -> Result<Self> {
        if port == 0 {
            return Err(Error::ConfigurationError(format!(
                "Invalid SSH port: {port}"
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

    /// Validate hostname format and security
    fn validate_hostname(host: &str) -> Result<()> {
        if host.is_empty() || host.len() > 253 {
            return Err(Error::ConfigurationError("Invalid hostname length".into()));
        }

        // Basic hostname format validation
        if !host
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-._".contains(c))
        {
            return Err(Error::ConfigurationError(format!(
                "Invalid hostname format: {host}"
            )));
        }

        Ok(())
    }

    /// Validate username format and security
    fn validate_username(user: &str) -> Result<()> {
        if user.is_empty() || user.len() > 32 {
            return Err(Error::ConfigurationError("Invalid username length".into()));
        }

        // Username should be alphanumeric + underscore/hyphen
        if !user
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_".contains(c))
        {
            return Err(Error::ConfigurationError(format!(
                "Invalid username format: {user}"
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
                Error::ConfigurationError(format!("Cannot read key file metadata: {e}"))
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
#[derive(Clone)]
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

        Self::output(&self.build_secure_ssh_command(command)?).await
    }

    async fn output(command: &str) -> Result<String> {
        let output = Command::new("sh").arg("-c").arg(command).output().await?;
        if !output.status.success() {
            return Err(Error::ConfigurationError(format!(
                "Remote command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// Stream a new owner-only file only after strict host verification.
    pub async fn write_protected_file(&self, remote_path: &str, content: &[u8]) -> Result<()> {
        self.validate_remote_path(remote_path)?;
        if !Path::new(remote_path).is_absolute()
            || Path::new(remote_path)
                .components()
                .any(|part| matches!(part, std::path::Component::ParentDir))
        {
            return Err(Error::ConfigurationError(
                "Invalid protected remote path".into(),
            ));
        }
        let path = escape(remote_path.into());
        let reserve = format!("umask 077; set -C; : > {path}");
        self.run_remote_input(&format!("sh -c {}", escape(reserve.into())), b"")
            .await?;
        let cleanup = escape(format!("rm -f {path}").into());
        let write = format!(
            "set -e; trap {cleanup} EXIT; trap 'exit 1' HUP INT TERM; cat > {path}; test \"$(wc -c < {path})\" -eq {}; trap - EXIT",
            content.len()
        );
        if self
            .run_remote_input(&format!("sh -c {}", escape(write.into())), content)
            .await
            .is_err()
        {
            self.run_remote_input(&format!("rm -f {path}"), b"")
                .await
                .map_err(|_| {
                    Error::ConfigurationError(
                        "Credential transfer failed; owned staging cleanup is unconfirmed".into(),
                    )
                })?;
            return Err(Error::ConfigurationError(
                "SSH input transfer failed".into(),
            ));
        }
        Ok(())
    }

    /// Execute an SDK-generated installation script without placing its contents in argv.
    pub async fn run_script(&self, script: &[u8]) -> Result<()> {
        self.run_remote_input("sh -se", script).await
    }

    async fn run_remote_input(&self, command: &str, content: &[u8]) -> Result<()> {
        use tokio::io::AsyncWriteExt;
        let ssh = self.build_secure_ssh_command(command)?;
        let mut child = Command::new("sh")
            .args(["-c", &ssh])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|_| Error::ConfigurationError("SSH input transfer failed".into()))?;
        let mut input = child
            .stdin
            .take()
            .ok_or_else(|| Error::ConfigurationError("SSH input unavailable".into()))?;
        let written = input.write_all(content).await;
        drop(input);
        let status = child
            .wait()
            .await
            .map_err(|_| Error::ConfigurationError("SSH input transfer failed".into()))?;
        if written.is_err() || !status.success() {
            return Err(Error::ConfigurationError(
                "SSH input transfer failed".into(),
            ));
        }
        Ok(())
    }

    fn transport_command(&self, program: &str, port_flag: &str) -> Result<String> {
        let mut command = format!(
            "{program} -o StrictHostKeyChecking=yes -o ConnectTimeout=30 -o ServerAliveInterval=60 -o ServerAliveCountMax=3 -o BatchMode=yes"
        );
        if let Some(path) = &self.connection.known_hosts_file {
            let path = path
                .to_str()
                .ok_or_else(|| Error::ConfigurationError("Invalid known-hosts path".into()))?;
            command.push_str(&format!(" -o UserKnownHostsFile={}", escape(path.into())));
        }
        if self.connection.port != 22 {
            command.push_str(&format!(" {port_flag} {}", self.connection.port));
        }
        if let Some(path) = &self.connection.key_path {
            let path = path
                .to_str()
                .ok_or_else(|| Error::ConfigurationError("Invalid SSH key path".into()))?;
            command.push_str(&format!(" -i {}", escape(path.into())));
        }
        if let Some(jump) = &self.connection.jump_host {
            command.push_str(&format!(" -J {}", escape(jump.as_str().into())));
        }
        Ok(command)
    }

    fn build_secure_ssh_command(&self, command: &str) -> Result<String> {
        let mut ssh = self.transport_command("ssh", "-p")?;
        ssh.push_str(&format!(
            " {}@{} {}",
            escape(self.connection.user.as_str().into()),
            escape(self.connection.host.as_str().into()),
            escape(command.into())
        ));
        Ok(ssh)
    }

    /// Allowed command prefixes for SSH execution.
    /// Commands must start with one of these prefixes to be accepted.
    const ALLOWED_CMD_PREFIXES: &'static [&'static str] = &[
        "echo ",
        "docker ",
        "podman ",
        "ctr ",
        "sudo ",
        "mkdir ",
        "chmod ",
        "systemctl ",
        "apt-get ",
        "curl ",
        "nginx ",
        "cat ",
        "tee ",
        "install ",
        "test ",
        "ls ",
        "cp ",
        "mv ",
        "rm ",
        "tar ",
        "sysctl ",
        "journalctl ",
        "grep ",
        "head ",
        "tail ",
        "stat ",
        "uname ",
        "whoami",
        "id ",
        "ip ",
        "#",  // shell comments (multi-line scripts)
        "\n", // multi-line scripts starting with newline
    ];

    /// Validate command against an allowlist of permitted prefixes.
    fn validate_command(&self, command: &str) -> Result<()> {
        if command.is_empty() {
            return Err(Error::ConfigurationError(
                "Empty command not allowed".into(),
            ));
        }

        if command.len() > 8192 {
            return Err(Error::ConfigurationError("Command too long".into()));
        }

        // Reject NUL bytes — can truncate strings in C-based tooling
        if command.contains('\0') {
            return Err(Error::ConfigurationError(
                "Command contains null bytes".into(),
            ));
        }

        // Check each line of multi-line commands against the allowlist
        let trimmed = command.trim();
        for line in trimmed.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if !Self::ALLOWED_CMD_PREFIXES
                .iter()
                .any(|prefix| line.starts_with(prefix))
            {
                return Err(Error::ConfigurationError("Command not in allowlist".into()));
            }
        }

        // Reject known destructive patterns even if prefix-allowed
        let dangerous_patterns = [
            "rm -rf /",
            ":(){ :|:& };:",
            "dd if=/dev/zero",
            "mkfs.",
            "fdisk",
            "parted",
        ];
        for pattern in &dangerous_patterns {
            if command.contains(pattern) {
                return Err(Error::ConfigurationError(format!(
                    "Dangerous command pattern detected: {pattern}"
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

        Self::output(&self.build_secure_scp_command(local_path, remote_path)?)
            .await
            .map(|_| ())
    }

    fn build_secure_scp_command(&self, local_path: &Path, remote_path: &str) -> Result<String> {
        let local = local_path
            .to_str()
            .ok_or_else(|| Error::ConfigurationError("Local path contains invalid UTF-8".into()))?;
        let mut scp = self.transport_command("scp", "-P")?;
        scp.push_str(&format!(
            " {} {}@{}:{}",
            escape(local.into()),
            escape(self.connection.user.as_str().into()),
            escape(self.connection.host.as_str().into()),
            escape(remote_path.into())
        ));
        Ok(scp)
    }

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
                "Remote path contains dangerous characters: {path}"
            )));
        }

        Ok(())
    }
}
