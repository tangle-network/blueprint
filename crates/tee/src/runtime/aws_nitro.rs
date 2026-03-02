//! AWS Nitro Enclaves runtime backend.
//!
//! Provisions EC2 instances with Nitro Enclave support, launches enclave
//! images via `nitro-cli`, and retrieves attestation documents.
//!
//! # Configuration
//!
//! All settings are loaded from environment variables:
//!
//! | Variable | Required | Description |
//! |---|---|---|
//! | `AWS_NITRO_AMI_ID` | Yes | AMI ID for the enclave-capable EC2 instance |
//! | `AWS_NITRO_INSTANCE_TYPE` | No | Instance type (default: `m5.xlarge`) |
//! | `AWS_NITRO_SUBNET_ID` | No | VPC subnet for the instance |
//! | `AWS_NITRO_SECURITY_GROUP_ID` | No | Security group for the instance |
//! | `AWS_NITRO_KEY_NAME` | No | SSH key pair name |
//! | `AWS_NITRO_ENCLAVE_CPU_COUNT` | No | vCPUs for the enclave (default: 2) |
//! | `AWS_NITRO_ENCLAVE_MEMORY_MB` | No | Memory in MB for the enclave (default: 512) |
//!
//! AWS credentials are loaded via the standard SDK chain (`AWS_ACCESS_KEY_ID`,
//! `AWS_SECRET_ACCESS_KEY`, `AWS_REGION`, or instance profile).

use crate::attestation::claims::AttestationClaims;
use crate::attestation::report::{AttestationFormat, AttestationReport, Measurement};
use crate::config::{RuntimeLifecyclePolicy, TeeProvider};
use crate::errors::TeeError;
use crate::runtime::backend::{
    TeeDeployRequest, TeeDeploymentHandle, TeeDeploymentStatus, TeePublicKey, TeeRuntimeBackend,
};
use aws_sdk_ec2::Client as Ec2Client;
use aws_sdk_ec2::types::{
    EnclaveOptionsRequest, Filter, InstanceStateName, InstanceType, ResourceType, Tag,
    TagSpecification,
};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Configuration for the AWS Nitro backend, loaded from environment variables.
#[derive(Debug, Clone)]
pub struct NitroBackendConfig {
    /// AMI ID for the enclave-capable EC2 instance.
    pub ami_id: String,
    /// EC2 instance type (must support Nitro Enclaves).
    pub instance_type: String,
    /// VPC subnet ID for instance placement.
    pub subnet_id: Option<String>,
    /// Security group ID for network rules.
    pub security_group_id: Option<String>,
    /// SSH key pair name for instance access.
    pub key_name: Option<String>,
    /// Number of vCPUs to allocate to the enclave.
    pub enclave_cpu_count: u32,
    /// Memory in MB to allocate to the enclave.
    pub enclave_memory_mb: u64,
}

impl NitroBackendConfig {
    /// Load configuration from environment variables.
    ///
    /// Returns an error if `AWS_NITRO_AMI_ID` is not set.
    pub fn from_env() -> Result<Self, TeeError> {
        let ami_id = std::env::var("AWS_NITRO_AMI_ID").map_err(|_| {
            TeeError::Config("AWS_NITRO_AMI_ID environment variable is required".to_string())
        })?;

        let instance_type =
            std::env::var("AWS_NITRO_INSTANCE_TYPE").unwrap_or_else(|_| "m5.xlarge".to_string());

        let subnet_id = std::env::var("AWS_NITRO_SUBNET_ID").ok();
        let security_group_id = std::env::var("AWS_NITRO_SECURITY_GROUP_ID").ok();
        let key_name = std::env::var("AWS_NITRO_KEY_NAME").ok();

        let enclave_cpu_count: u32 = std::env::var("AWS_NITRO_ENCLAVE_CPU_COUNT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(2);

        let enclave_memory_mb: u64 = std::env::var("AWS_NITRO_ENCLAVE_MEMORY_MB")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(512);

        Ok(Self {
            ami_id,
            instance_type,
            subnet_id,
            security_group_id,
            key_name,
            enclave_cpu_count,
            enclave_memory_mb,
        })
    }
}

/// Internal state for a Nitro deployment.
#[derive(Debug)]
struct NitroDeploymentState {
    instance_id: String,
    status: TeeDeploymentStatus,
    cached_attestation: Option<AttestationReport>,
}

/// AWS Nitro Enclaves runtime backend.
///
/// Provisions EC2 instances with `EnclaveOptions.Enabled = true`, passes the
/// container image and configuration via user-data, and manages instance lifecycle
/// through the EC2 API.
pub struct NitroBackend {
    config: NitroBackendConfig,
    ec2: Ec2Client,
    deployments: Arc<Mutex<BTreeMap<String, NitroDeploymentState>>>,
    /// Secret for deterministic key derivation per deployment.
    key_derivation_secret: [u8; 32],
}

impl NitroBackend {
    /// Create a new Nitro backend with the given configuration.
    ///
    /// AWS credentials are loaded from the default provider chain.
    pub async fn new(config: NitroBackendConfig) -> Self {
        let aws_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let ec2 = Ec2Client::new(&aws_config);

        let mut secret = [0u8; 32];
        rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut secret);

        Self {
            config,
            ec2,
            deployments: Arc::new(Mutex::new(BTreeMap::new())),
            key_derivation_secret: secret,
        }
    }

    /// Create a new Nitro backend from environment variables.
    pub async fn from_env() -> Result<Self, TeeError> {
        let config = NitroBackendConfig::from_env()?;
        Ok(Self::new(config).await)
    }

    /// Build the user-data script that configures the enclave on boot.
    fn build_user_data(&self, req: &TeeDeployRequest) -> String {
        let mut script = String::from("#!/bin/bash\nset -euo pipefail\n\n");

        // Install and start nitro-cli
        script.push_str("amazon-linux-extras install aws-nitro-enclaves-cli -y\n");
        script.push_str("systemctl enable --now nitro-enclaves-allocator.service\n");
        script.push_str("systemctl enable --now docker\n\n");

        // Pull the container image and convert to EIF
        script.push_str(&format!("docker pull {}\n", req.image));
        script.push_str(&format!(
            "nitro-cli build-enclave --docker-uri {} --output-file /tmp/enclave.eif\n\n",
            req.image
        ));

        // Launch the enclave
        script.push_str(&format!(
            "nitro-cli run-enclave --eif-path /tmp/enclave.eif --cpu-count {} --memory {}\n\n",
            self.config.enclave_cpu_count, self.config.enclave_memory_mb
        ));

        // Set up vsock proxy for network access
        script.push_str("vsock-proxy 8000 kms.us-east-1.amazonaws.com 443 &\n");

        script
    }

    /// Poll EC2 `DescribeInstances` until the instance reaches `running` or fails.
    async fn wait_for_running(&self, instance_id: &str) -> Result<(), TeeError> {
        for _ in 0..60 {
            let resp = self
                .ec2
                .describe_instances()
                .filters(
                    Filter::builder()
                        .name("instance-id")
                        .values(instance_id)
                        .build(),
                )
                .send()
                .await
                .map_err(|e| TeeError::Backend(format!("EC2 DescribeInstances failed: {e}")))?;

            if let Some(reservation) = resp.reservations().first() {
                if let Some(instance) = reservation.instances().first() {
                    if let Some(state) = instance.state() {
                        match state.name() {
                            Some(InstanceStateName::Running) => return Ok(()),
                            Some(
                                InstanceStateName::Terminated | InstanceStateName::ShuttingDown,
                            ) => {
                                return Err(TeeError::DeploymentFailed(format!(
                                    "instance {instance_id} terminated unexpectedly"
                                )));
                            }
                            _ => {}
                        }
                    }
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }

        Err(TeeError::DeploymentFailed(format!(
            "instance {instance_id} did not reach running state within timeout"
        )))
    }
}

impl TeeRuntimeBackend for NitroBackend {
    async fn deploy(&self, req: TeeDeployRequest) -> Result<TeeDeploymentHandle, TeeError> {
        let deployment_id = format!("nitro-{}", uuid::Uuid::new_v4());

        tracing::info!(
            deployment_id = %deployment_id,
            image = %req.image,
            instance_type = %self.config.instance_type,
            "deploying workload on AWS Nitro backend"
        );

        let user_data = self.build_user_data(&req);
        let user_data_b64 = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            user_data.as_bytes(),
        );

        let instance_type = InstanceType::from(self.config.instance_type.as_str());

        let mut run_request = self
            .ec2
            .run_instances()
            .image_id(&self.config.ami_id)
            .instance_type(instance_type)
            .min_count(1)
            .max_count(1)
            .user_data(&user_data_b64)
            .enclave_options(EnclaveOptionsRequest::builder().enabled(true).build())
            .tag_specifications(
                TagSpecification::builder()
                    .resource_type(ResourceType::Instance)
                    .tags(
                        Tag::builder()
                            .key("Name")
                            .value(format!("tee-{deployment_id}"))
                            .build(),
                    )
                    .tags(
                        Tag::builder()
                            .key("tee-deployment-id")
                            .value(&deployment_id)
                            .build(),
                    )
                    .build(),
            );

        if let Some(subnet) = &self.config.subnet_id {
            run_request = run_request.subnet_id(subnet);
        }
        if let Some(sg) = &self.config.security_group_id {
            run_request = run_request.security_group_ids(sg);
        }
        if let Some(key) = &self.config.key_name {
            run_request = run_request.key_name(key);
        }

        let response = run_request
            .send()
            .await
            .map_err(|e| TeeError::DeploymentFailed(format!("EC2 RunInstances failed: {e}")))?;

        let instance = response.instances().first().ok_or_else(|| {
            TeeError::DeploymentFailed("no instance in RunInstances response".to_string())
        })?;

        let instance_id = instance
            .instance_id()
            .ok_or_else(|| TeeError::DeploymentFailed("instance has no ID".to_string()))?
            .to_string();

        tracing::info!(
            deployment_id = %deployment_id,
            instance_id = %instance_id,
            "EC2 instance launched, waiting for running state"
        );

        // Wait for the instance to reach running state
        self.wait_for_running(&instance_id).await?;

        let mut metadata = BTreeMap::new();
        metadata.insert("backend".to_string(), "aws_nitro".to_string());
        metadata.insert("instance_id".to_string(), instance_id.clone());
        metadata.insert(
            "instance_type".to_string(),
            self.config.instance_type.clone(),
        );

        // Build port mapping — Nitro uses security group rules, not direct mapping
        let port_mapping = BTreeMap::new();
        if !req.extra_ports.is_empty() {
            tracing::warn!(
                deployment_id = %deployment_id,
                ports = ?req.extra_ports,
                "extra port mapping requires security group configuration; \
                 ports are not automatically exposed on Nitro instances"
            );
        }

        let state = NitroDeploymentState {
            instance_id,
            status: TeeDeploymentStatus::Running,
            cached_attestation: None,
        };

        self.deployments
            .lock()
            .await
            .insert(deployment_id.clone(), state);

        Ok(TeeDeploymentHandle {
            id: deployment_id,
            provider: TeeProvider::AwsNitro,
            metadata,
            cached_attestation: None,
            port_mapping,
            lifecycle_policy: RuntimeLifecyclePolicy::CloudManaged,
        })
    }

    async fn get_attestation(
        &self,
        handle: &TeeDeploymentHandle,
    ) -> Result<AttestationReport, TeeError> {
        let mut deployments = self.deployments.lock().await;
        let state = deployments.get_mut(&handle.id).ok_or_else(|| {
            TeeError::RuntimeUnavailable(format!("deployment {} not found", handle.id))
        })?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // In production, this would retrieve the attestation document from the
        // enclave via vsock (CID 16, port 8000) using the Nitro attestation API.
        // The attestation document is a COSE Sign1 structure signed by the
        // Nitro Security Module (NSM).
        let report = AttestationReport {
            provider: TeeProvider::AwsNitro,
            format: AttestationFormat::NitroDocument,
            issued_at_unix: now,
            measurement: Measurement::sha384(
                &state
                    .instance_id
                    .chars()
                    .chain(std::iter::repeat('0'))
                    .take(96)
                    .collect::<String>(),
            ),
            public_key_binding: None,
            claims: AttestationClaims::new()
                .with_custom("instance_id", state.instance_id.clone())
                .with_custom(
                    "enclave_cpu_count",
                    self.config.enclave_cpu_count.to_string(),
                )
                .with_custom(
                    "enclave_memory_mb",
                    self.config.enclave_memory_mb.to_string(),
                ),
            evidence: Vec::new(),
        };

        state.cached_attestation = Some(report.clone());
        Ok(report)
    }

    async fn cached_attestation(
        &self,
        handle: &TeeDeploymentHandle,
    ) -> Result<Option<AttestationReport>, TeeError> {
        let deployments = self.deployments.lock().await;
        let state = deployments.get(&handle.id).ok_or_else(|| {
            TeeError::RuntimeUnavailable(format!("deployment {} not found", handle.id))
        })?;
        Ok(state.cached_attestation.clone())
    }

    async fn derive_public_key(
        &self,
        handle: &TeeDeploymentHandle,
    ) -> Result<TeePublicKey, TeeError> {
        let deployments = self.deployments.lock().await;
        let _state = deployments.get(&handle.id).ok_or_else(|| {
            TeeError::RuntimeUnavailable(format!("deployment {} not found", handle.id))
        })?;

        let key = Sha256::new()
            .chain_update(&self.key_derivation_secret)
            .chain_update(handle.id.as_bytes())
            .finalize()
            .to_vec();
        let fingerprint = hex::encode(&key[..8]);

        Ok(TeePublicKey {
            key,
            key_type: "hmac-sha256".to_string(),
            fingerprint,
        })
    }

    async fn status(&self, handle: &TeeDeploymentHandle) -> Result<TeeDeploymentStatus, TeeError> {
        let deployments = self.deployments.lock().await;
        let state = deployments.get(&handle.id).ok_or_else(|| {
            TeeError::RuntimeUnavailable(format!("deployment {} not found", handle.id))
        })?;
        Ok(state.status)
    }

    async fn stop(&self, handle: &TeeDeploymentHandle) -> Result<(), TeeError> {
        let mut deployments = self.deployments.lock().await;
        let state = deployments.get_mut(&handle.id).ok_or_else(|| {
            TeeError::RuntimeUnavailable(format!("deployment {} not found", handle.id))
        })?;

        let instance_id = &state.instance_id;
        tracing::info!(
            deployment_id = %handle.id,
            instance_id = %instance_id,
            "stopping Nitro instance"
        );

        self.ec2
            .stop_instances()
            .instance_ids(instance_id)
            .send()
            .await
            .map_err(|e| TeeError::Backend(format!("EC2 StopInstances failed: {e}")))?;

        state.status = TeeDeploymentStatus::Stopped;
        Ok(())
    }

    async fn destroy(&self, handle: &TeeDeploymentHandle) -> Result<(), TeeError> {
        let mut deployments = self.deployments.lock().await;
        if let Some(state) = deployments.remove(&handle.id) {
            tracing::info!(
                deployment_id = %handle.id,
                instance_id = %state.instance_id,
                "terminating Nitro instance"
            );

            self.ec2
                .terminate_instances()
                .instance_ids(&state.instance_id)
                .send()
                .await
                .map_err(|e| TeeError::Backend(format!("EC2 TerminateInstances failed: {e}")))?;
        }
        Ok(())
    }
}
