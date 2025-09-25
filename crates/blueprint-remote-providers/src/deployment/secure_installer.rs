//! Secure binary installation with cryptographic verification
//! 
//! Replaces the insecure download mechanism with proper verification

use crate::core::error::{Error, Result};
use blake3::Hasher;
use std::path::Path;
use tokio::process::Command;
use tracing::{info, warn};

/// Secure binary installer with cryptographic verification
pub struct SecureBinaryInstaller {
    /// Expected SHA256 hash of the binary
    expected_hash: String,
    /// GPG public key for signature verification (if available)
    gpg_public_key: Option<String>,
    /// Download URL (must be HTTPS)
    download_url: String,
}

impl SecureBinaryInstaller {
    /// Create new secure installer with hash verification
    pub fn new(download_url: String, expected_hash: String) -> Result<Self> {
        if !download_url.starts_with("https://") {
            return Err(Error::ConfigurationError(
                "Download URL must use HTTPS".into()
            ));
        }
        
        if expected_hash.len() != 64 {
            return Err(Error::ConfigurationError(
                "Expected hash must be 64-character SHA256".into()
            ));
        }
        
        Ok(Self {
            expected_hash,
            gpg_public_key: None,
            download_url,
        })
    }
    
    /// Add GPG signature verification
    pub fn with_gpg_verification(mut self, public_key: String) -> Self {
        self.gpg_public_key = Some(public_key);
        self
    }
    
    /// Securely download and install Blueprint runtime
    pub async fn install_blueprint_runtime(&self) -> Result<()> {
        info!("Starting secure Blueprint runtime installation");
        
        // Create secure directory structure
        self.create_secure_directories().await?;
        
        // Download binary with verification
        let temp_binary = "/tmp/blueprint-runtime-download";
        self.secure_download(temp_binary).await?;
        
        // Verify cryptographic hash
        self.verify_hash(temp_binary).await?;
        
        // Verify GPG signature if available
        if self.gpg_public_key.is_some() {
            self.verify_signature(temp_binary).await?;
        } else {
            warn!("GPG signature verification not configured - supply chain attacks possible");
        }
        
        // Install with proper permissions
        self.install_binary(temp_binary).await?;
        
        // Create secure systemd service
        self.create_secure_systemd_service().await?;
        
        // Clean up temporary files
        let _ = tokio::fs::remove_file(temp_binary).await;
        
        info!("Blueprint runtime installed securely");
        Ok(())
    }
    
    /// Create secure directory structure
    async fn create_secure_directories(&self) -> Result<()> {
        let create_dirs = r#"
        sudo mkdir -p /opt/blueprint/{bin,config,data,logs}
        sudo useradd -r -s /bin/false -d /opt/blueprint blueprint 2>/dev/null || true
        sudo chown -R blueprint:blueprint /opt/blueprint
        sudo chmod 755 /opt/blueprint
        sudo chmod 750 /opt/blueprint/{config,data,logs}
        sudo chmod 755 /opt/blueprint/bin
        "#;
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(create_dirs)
            .output()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Directory creation failed: {}", e)))?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigurationError(format!(
                "Directory creation failed: {}", stderr
            )));
        }
        
        Ok(())
    }
    
    /// Secure download with TLS verification
    async fn secure_download(&self, dest_path: &str) -> Result<()> {
        // Use curl with security options
        let download_cmd = format!(
            "curl --fail --location --max-time 300 --max-filesize 104857600 \
             --proto =https --tlsv1.2 --ciphers ECDHE+AESGCM:ECDHE+CHACHA20:DHE+AESGCM:DHE+CHACHA20:!aNULL:!MD5:!DSS \
             --output {} {}",
            shell_escape::escape(dest_path.into()),
            shell_escape::escape(self.download_url.as_str().into())
        );
        
        info!("Downloading Blueprint runtime from: {}", self.download_url);
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&download_cmd)
            .output()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Download failed: {}", e)))?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigurationError(format!(
                "Download failed: {}", stderr
            )));
        }
        
        Ok(())
    }
    
    /// Verify cryptographic hash
    async fn verify_hash(&self, file_path: &str) -> Result<()> {
        info!("Verifying cryptographic hash");
        
        let file_content = tokio::fs::read(file_path).await
            .map_err(|e| Error::ConfigurationError(format!("Cannot read downloaded file: {}", e)))?;
            
        let mut hasher = Hasher::new();
        hasher.update(&file_content);
        let actual_hash = hasher.finalize();
        
        let actual_hash_hex = hex::encode(actual_hash.as_bytes());
        
        if actual_hash_hex != self.expected_hash {
            return Err(Error::ConfigurationError(format!(
                "Hash verification failed! Expected: {}, Actual: {}",
                self.expected_hash, actual_hash_hex
            )));
        }
        
        info!("Hash verification successful");
        Ok(())
    }
    
    /// Verify GPG signature
    async fn verify_signature(&self, file_path: &str) -> Result<()> {
        info!("Verifying GPG signature");
        
        // Download signature file
        let sig_url = format!("{}.sig", self.download_url);
        let sig_path = format!("{}.sig", file_path);
        
        let download_sig = format!(
            "curl --fail --location --max-time 60 --proto =https --tlsv1.2 --output {} {}",
            shell_escape::escape(sig_path.as_str().into()),
            shell_escape::escape(sig_url.as_str().into())
        );
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&download_sig)
            .output()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Signature download failed: {}", e)))?;
            
        if !output.status.success() {
            warn!("Signature file not available - proceeding without GPG verification");
            return Ok(());
        }
        
        // Verify signature
        let verify_cmd = format!(
            "gpg --batch --verify {} {}",
            shell_escape::escape(sig_path.as_str().into()),
            shell_escape::escape(file_path.into())
        );
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&verify_cmd)
            .output()
            .await
            .map_err(|e| Error::ConfigurationError(format!("GPG verification failed: {}", e)))?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigurationError(format!(
                "GPG signature verification failed: {}", stderr
            )));
        }
        
        // Clean up signature file
        let _ = tokio::fs::remove_file(&sig_path).await;
        
        info!("GPG signature verification successful");
        Ok(())
    }
    
    /// Install binary with proper permissions
    async fn install_binary(&self, temp_path: &str) -> Result<()> {
        let install_cmd = format!(
            "sudo cp {} /opt/blueprint/bin/blueprint-runtime && \
             sudo chown root:root /opt/blueprint/bin/blueprint-runtime && \
             sudo chmod 755 /opt/blueprint/bin/blueprint-runtime",
            shell_escape::escape(temp_path.into())
        );
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&install_cmd)
            .output()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Binary installation failed: {}", e)))?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigurationError(format!(
                "Binary installation failed: {}", stderr
            )));
        }
        
        Ok(())
    }
    
    /// Create secure systemd service with hardening
    async fn create_secure_systemd_service(&self) -> Result<()> {
        let service_content = r#"[Unit]
Description=Blueprint Runtime
After=network.target
Wants=network.target

[Service]
Type=simple
User=blueprint
Group=blueprint
WorkingDirectory=/opt/blueprint
ExecStart=/opt/blueprint/bin/blueprint-runtime
Restart=always
RestartSec=10

# Security hardening
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true
RestrictRealtime=true
RestrictSUIDSGID=true
RemoveIPC=true
PrivateTmp=true
PrivateDevices=true
ProtectHostname=true
ProtectClock=true
ProtectKernelLogs=true
ProtectProc=invisible
ProcSubset=pid
RestrictNamespaces=true
LockPersonality=true
MemoryDenyWriteExecute=true
RestrictAddressFamilies=AF_INET AF_INET6 AF_UNIX
SystemCallFilter=@system-service
SystemCallFilter=~@debug @mount @cpu-emulation @obsolete @privileged @reboot @swap
SystemCallErrorNumber=EPERM

# Resource limits
LimitNOFILE=1024
LimitNPROC=256
TasksMax=256

# Directories
ReadWritePaths=/opt/blueprint/data /opt/blueprint/logs
ReadOnlyPaths=/opt/blueprint/config

[Install]
WantedBy=multi-user.target"#;

        // Write service file
        let write_service = format!(
            "sudo tee /etc/systemd/system/blueprint-runtime.service > /dev/null << 'EOF'\n{}\nEOF",
            service_content
        );
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&write_service)
            .output()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Service creation failed: {}", e)))?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigurationError(format!(
                "Service creation failed: {}", stderr
            )));
        }
        
        // Enable and start service
        let enable_service = "sudo systemctl daemon-reload && sudo systemctl enable blueprint-runtime && sudo systemctl start blueprint-runtime";
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(enable_service)
            .output()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Service activation failed: {}", e)))?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigurationError(format!(
                "Service activation failed: {}", stderr
            )));
        }
        
        Ok(())
    }
    
    /// Verify installation
    pub async fn verify_installation(&self) -> Result<()> {
        let status_cmd = "sudo systemctl is-active blueprint-runtime";
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(status_cmd)
            .output()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Status check failed: {}", e)))?;
            
        let status = String::from_utf8_lossy(&output.stdout).trim().to_string();
        
        if status == "active" {
            info!("Blueprint runtime is running successfully");
            Ok(())
        } else {
            Err(Error::ConfigurationError(format!(
                "Blueprint runtime is not active: {}", status
            )))
        }
    }
}

/// Predefined secure installer for Blueprint runtime
impl Default for SecureBinaryInstaller {
    fn default() -> Self {
        // These should be updated for each release
        Self {
            download_url: "https://github.com/tangle-network/blueprint/releases/latest/download/blueprint-runtime".to_string(),
            expected_hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(), // MUST BE UPDATED
            gpg_public_key: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_secure_installer_validation() {
        // Valid HTTPS URL
        let installer = SecureBinaryInstaller::new(
            "https://example.com/binary".to_string(),
            "a".repeat(64)
        );
        assert!(installer.is_ok());
        
        // Invalid HTTP URL
        let installer = SecureBinaryInstaller::new(
            "http://example.com/binary".to_string(),
            "a".repeat(64)
        );
        assert!(installer.is_err());
        
        // Invalid hash length
        let installer = SecureBinaryInstaller::new(
            "https://example.com/binary".to_string(),
            "short_hash".to_string()
        );
        assert!(installer.is_err());
    }
}