//! Integration tests for cloud deployment commands.

#[cfg(test)]
mod cloud_tests {
    use std::process::Command;

    /// Test that cloud help command works
    #[test]
    fn test_cloud_help() {
        let output = Command::new("cargo")
            .args(&[
                "run",
                "-p",
                "cargo-tangle",
                "--",
                "tangle",
                "cloud",
                "--help",
            ])
            .output()
            .expect("Failed to execute command");

        assert!(
            output.status.success() || output.status.code() == Some(0),
            "Cloud help command should work"
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Cloud deployment") || stdout.contains("configure"),
            "Help should mention cloud commands"
        );
    }

    /// Test cloud configure help
    #[test]
    fn test_cloud_configure_help() {
        let output = Command::new("cargo")
            .args(&[
                "run",
                "-p",
                "cargo-tangle",
                "--",
                "tangle",
                "cloud",
                "configure",
                "--help",
            ])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(
            stdout.contains("provider") || stderr.contains("provider"),
            "Configure help should mention provider"
        );
    }

    /// Test cloud estimate help
    #[test]
    fn test_cloud_estimate_help() {
        let output = Command::new("cargo")
            .args(&[
                "run",
                "-p",
                "cargo-tangle",
                "--",
                "tangle",
                "cloud",
                "estimate",
                "--help",
            ])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(
            stdout.contains("cost")
                || stdout.contains("compare")
                || stderr.contains("cost")
                || stderr.contains("compare"),
            "Estimate help should mention cost or compare"
        );
    }

    /// Test that invalid provider is rejected
    #[test]
    fn test_invalid_provider() {
        let output = Command::new("cargo")
            .args(&[
                "run",
                "-p",
                "cargo-tangle",
                "--",
                "tangle",
                "cloud",
                "configure",
                "invalid",
            ])
            .output()
            .expect("Failed to execute command");

        assert!(!output.status.success(), "Invalid provider should fail");
    }

    /// Test cloud status without deployments
    #[test]
    fn test_cloud_status_empty() {
        // This test would normally check actual status
        // For now, just verify the command structure exists
        let output = Command::new("cargo")
            .args(&[
                "run",
                "-p",
                "cargo-tangle",
                "--",
                "tangle",
                "cloud",
                "status",
                "--help",
            ])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(
            stdout.contains("deployment") || stderr.contains("deployment"),
            "Status help should mention deployments"
        );
    }
}
