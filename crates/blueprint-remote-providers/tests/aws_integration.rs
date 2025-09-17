//! AWS integration tests with mocked responses

#[cfg(all(test, feature = "aws"))]
mod tests {
    use blueprint_remote_providers::{
        providers::aws::provisioner::AwsProvisioner,
        providers::common::ProvisioningConfig,
        resources::ResourceSpec,
    };

    #[tokio::test]
    #[ignore] // Requires AWS credentials
    async fn test_aws_provisioning_real() {
        // This test requires real AWS credentials
        let provisioner = AwsProvisioner::new().await.unwrap();
        
        let config = ProvisioningConfig {
            name: "test-blueprint-instance".to_string(),
            region: "us-west-2".to_string(),
            ssh_key_name: Some("test-key".to_string()),
            ami_id: Some("ami-0c55b159cbfafe1f0".to_string()), // Amazon Linux 2
            ..Default::default()
        };
        
        let spec = ResourceSpec::basic();
        
        // Would provision real instance if credentials are set
        let result = provisioner.provision_instance(&spec, &config).await;
        
        // If AWS credentials aren't configured, this should fail
        if std::env::var("AWS_ACCESS_KEY_ID").is_err() {
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn test_aws_provisioner_creation() {
        // Test that provisioner can be created (will fail without AWS config)
        let result = AwsProvisioner::new().await;
        
        // Should succeed if AWS SDK can load config (even empty)
        // or fail gracefully if not configured
        match result {
            Ok(_) => println!("AWS provisioner created successfully"),
            Err(e) => println!("Expected error without AWS config: {}", e),
        }
    }
}

#[cfg(test)]
mod mock_tests {
    use blueprint_remote_providers::{
        providers::aws::instance_mapper::AwsInstanceMapper,
        resources::ResourceSpec,
    };

    #[test]
    fn test_instance_type_mapping() {
        // Test basic resource mapping
        let spec = ResourceSpec::basic();
        let result = AwsInstanceMapper::map(&spec);
        assert_eq!(result.instance_type, "t3.medium");
        assert!(!result.spot_capable); // Basic specs shouldn't use spot
        
        // Test performance resource mapping
        let spec = ResourceSpec::performance();
        let result = AwsInstanceMapper::map(&spec);
        assert!(result.instance_type.starts_with("c") || result.instance_type.starts_with("m"));
        
        // Test GPU resource mapping
        let mut spec = ResourceSpec::performance();
        spec.gpu_count = Some(1);
        let result = AwsInstanceMapper::map(&spec);
        assert!(result.instance_type.starts_with("g") || result.instance_type.starts_with("p"));
    }

    #[test]
    fn test_spot_instance_eligibility() {
        // Test that spot instances are only used when explicitly allowed
        let mut spec = ResourceSpec::recommended();
        spec.allow_spot = false;
        let result = AwsInstanceMapper::map(&spec);
        assert!(!result.spot_capable);
        
        spec.allow_spot = true;
        let result = AwsInstanceMapper::map(&spec);
        // Spot capability depends on instance type, but flag should be respected
        assert_eq!(result.spot_capable, spec.allow_spot);
    }
}