use super::*;

impl RemoteDeploymentService {
    pub(super) async fn deploy_to_cloud(
        &self,
        ctx: &BlueprintManagerContext,
        provider: CloudProvider,
        service_name: &str,
        binary_path: &Path,
        env_vars: BlueprintEnvVars,
        arguments: BlueprintArgs,
        resource_spec: ResourceSpec,
        blueprint_id: Option<u64>,
    ) -> Result<Service> {
        info!("🚀 Deploying to cloud provider: {:?}", provider);
        info!("   Service: {}", service_name);
        info!(
            "   Resources: {:.1} CPU, {:.0} GB RAM (tee_required={})",
            resource_spec.cpu, resource_spec.memory_gb, resource_spec.tee_required
        );
        if resource_spec.tee_required && !provider.supports_tee() {
            return Err(Error::TeePrerequisiteMissing {
                prerequisite: format!("{provider} confidential-compute support"),
                hint: "Select AWS, GCP, or Azure when tee_required=true".to_string(),
            });
        }
        #[cfg(feature = "remote-providers")]
        {
            use blueprint_remote_providers::CloudProvisioner;
            let tracker = ensure_deployment_tracker(ctx, self.deployment_tracker.clone()).await?;
            let tee_attestation_policy = if resource_spec.tee_required {
                Some(TeeAttestationPolicy::from_env()?)
            } else {
                None
            };
            let provisioner = CloudProvisioner::new()
                .await
                .map_err(|e| Error::Other(format!("Failed to create provisioner: {}", e)))?;
            let region = resolve_provider_region(ctx, &provider);
            info!("   Region: {}", region);
            let instance = provisioner
                .provision_with_requirements(
                    provider.clone(),
                    &resource_spec,
                    &region,
                    resource_spec.tee_required,
                )
                .await
                .map_err(|e| Error::Other(format!("Failed to provision instance: {}", e)))?;
            info!(
                "✅ Instance provisioned: {} at {}",
                instance.id,
                instance.public_ip.as_deref().unwrap_or("unknown")
            );
            let blueprint_image = binary_path.to_string_lossy().to_string();
            let mut env_map = HashMap::new();
            for (key, value) in env_vars.encode() {
                env_map.insert(key, value);
            }
            let encoded_args = arguments.encode(false);
            for (i, arg) in encoded_args.iter().enumerate() {
                env_map.insert(format!("ARG_{}", i), arg.clone());
            }
            if resource_spec.tee_required {
                let backend = self
                    .policy
                    .tee_backend
                    .clone()
                    .or_else(|| provider_default_tee_backend(&provider).map(str::to_string))
                    .ok_or_else(|| Error::TeePrerequisiteMissing {
                        prerequisite: "TEE_BACKEND".to_string(),
                        hint: format!(
                            "Set TEE_BACKEND or use a provider with a known default backend. provider={provider}"
                        ),
                    })?;
                env_map.insert("TEE_REQUIRED".to_string(), "true".to_string());
                env_map.entry("TEE_BACKEND".to_string()).or_insert(backend);
            }
            let deployment_result = match provisioner
                .deploy_blueprint_to_instance(
                    &provider,
                    &instance,
                    &blueprint_image,
                    &resource_spec,
                    env_map,
                )
                .await
            {
                Ok(result) => result,
                Err(deployment_error) => {
                    warn!(
                        "Blueprint deployment failed for provider {} (instance={}); attempting best-effort cleanup before returning error: {}",
                        provider, instance.id, deployment_error
                    );
                    if let Err(cleanup_error) =
                        provisioner.terminate(provider.clone(), &instance.id).await
                    {
                        warn!(
                            "Best-effort cleanup failed after deployment failure (instance={}): {}",
                            instance.id, cleanup_error
                        );
                        return Err(Error::Other(format!(
                            "Failed to deploy blueprint: {}. Cleanup after failed deployment also failed for instance {}: {}",
                            deployment_error, instance.id, cleanup_error
                        )));
                    }
                    info!(
                        "✅ Instance {} terminated after deployment failure",
                        instance.id
                    );
                    return Err(Error::Other(format!(
                        "Failed to deploy blueprint: {}",
                        deployment_error
                    )));
                }
            };
            let tee_attestation_proof = if let Some(policy) = tee_attestation_policy {
                match verify_tee_attestation(policy, &provider, &deployment_result).await {
                    Ok(proof) => proof,
                    Err(verification_error) => {
                        warn!(
                            "TEE attestation verification failed for provider {} (instance={}); attempting best-effort cleanup before returning error: {}",
                            provider, instance.id, verification_error
                        );
                        if let Err(cleanup_error) =
                            provisioner.terminate(provider.clone(), &instance.id).await
                        {
                            warn!(
                                "Best-effort cleanup failed after attestation verification failure (instance={}): {}",
                                instance.id, cleanup_error
                            );
                            return Err(Error::Other(format!(
                                "TEE attestation verification failed: {}. Cleanup after attestation failure also failed for instance {}: {}",
                                verification_error, instance.id, cleanup_error
                            )));
                        } else {
                            info!(
                                "✅ Instance {} terminated after attestation verification failure",
                                instance.id
                            );
                        }
                        return Err(verification_error);
                    }
                }
            } else {
                None
            };
            let tee_attestation_verified_at =
                tee_attestation_policy.and_then(|policy| match policy {
                    TeeAttestationPolicy::Cryptographic if tee_attestation_proof.is_some() => {
                        Some(chrono::Utc::now())
                    }
                    _ => None,
                });
            info!("✅ Blueprint deployed with QoS monitoring enabled");
            if let Some(qos_endpoint) = deployment_result.qos_grpc_endpoint() {
                info!("📊 QoS endpoint available: {}", &qos_endpoint);
                let endpoint_str = qos_endpoint.replace("http://", "").replace("https://", "");
                if let Some((host, port_str)) = endpoint_str.rsplit_once(':') {
                    if let Ok(port) = port_str.parse::<u16>() {
                        #[cfg(feature = "qos")]
                        if let Some(ref qos_provider) = self.qos_provider {
                            qos_provider
                                .register_remote_instance(
                                    deployment_result.instance.id.clone(),
                                    host.to_string(),
                                    port,
                                    blueprint_id.unwrap_or(0),
                                    blueprint_id.unwrap_or(0),
                                )
                                .await;
                            info!("✅ QoS endpoint registered: {}:{}", host, port);
                            info!("   Instance: {}", deployment_result.instance.id);
                            info!("   Blueprint metrics will be collected from port {}", port);
                        }
                        #[cfg(not(feature = "qos"))]
                        {
                            info!(
                                "📊 QoS endpoint ready: {}:{} (QoS feature disabled)",
                                host, port
                            );
                        }
                    }
                }
            }
            let deployment_info = RemoteDeploymentInfo {
                instance_id: deployment_result.instance.id.clone(),
                provider: provider.clone(),
                service_name: service_name.to_string(),
                blueprint_id,
                deployed_at: chrono::Utc::now(),
                ttl_expires_at: self
                    .policy
                    .auto_terminate_hours
                    .map(|hours| chrono::Utc::now() + chrono::Duration::hours(hours as i64)),
                public_ip: deployment_result.instance.public_ip.clone(),
                tee_attestation_policy: tee_attestation_policy.map(|p| p.to_string()),
                tee_attestation_verified_at,
                tee_attestation_proof,
                kubernetes_context: None,
                kubernetes_namespace: None,
            };
            let ttl_seconds = ttl_seconds_from_hours(self.policy.auto_terminate_hours);
            let deployment_key = deployment_result.instance.id.clone();
            let mut deployment_record = DeploymentRecord::new(
                deployment_key.clone(),
                deployment_type_for_provider(&provider),
                resource_spec.clone(),
                ttl_seconds,
            );
            deployment_record.id = deployment_result.instance.id.clone();
            deployment_record.provider = Some(provider.clone());
            deployment_record.region = Some(tracker_region_for_provider(&provider, &region));
            deployment_record.resource_ids.insert(
                "instance_id".to_string(),
                deployment_result.instance.id.clone(),
            );
            deployment_record.resource_ids.insert(
                "instance_name".to_string(),
                deployment_result.instance.id.clone(),
            );
            if let Some(public_ip) = &deployment_result.instance.public_ip {
                deployment_record
                    .resource_ids
                    .insert("public_ip".to_string(), public_ip.clone());
            }
            if provider == CloudProvider::GCP {
                if let Ok(project_id) = std::env::var("GCP_PROJECT_ID") {
                    deployment_record
                        .metadata
                        .insert("project_id".to_string(), project_id);
                }
            }
            if let Some(proof) = deployment_info.tee_attestation_proof.clone() {
                deployment_record
                    .metadata
                    .insert("tee_attestation_proof".to_string(), proof);
            }
            if let Err(track_error) = tracker
                .register_deployment(deployment_key.clone(), deployment_record)
                .await
            {
                warn!(
                    "Deployment tracking failed for provider {} (instance={}); attempting best-effort cleanup before returning error: {}",
                    provider, deployment_result.instance.id, track_error
                );
                if let Err(cleanup_error) = provisioner
                    .terminate(provider.clone(), &deployment_result.instance.id)
                    .await
                {
                    return Err(Error::Other(format!(
                        "Failed to track deployment: {}. Cleanup after tracker failure also failed for instance {}: {}",
                        track_error, deployment_result.instance.id, cleanup_error
                    )));
                }
                return Err(Error::Other(format!(
                    "Failed to track deployment: {}",
                    track_error
                )));
            }
            {
                let mut deployments = self.deployments.write().await;
                deployments.insert(
                    deployment_result.instance.id.clone(),
                    deployment_info.clone(),
                );
            }
            info!("✅ Deployment registered with TTL tracking");
            let remote_config = RemoteDeploymentConfig {
                deployment_type: deployment_type_for_provider(&provider),
                provider: Some(provider),
                region: Some(region),
                instance_id: deployment_key,
                resource_spec: resource_spec.clone(),
                ttl_seconds,
                deployed_at: chrono::Utc::now(),
            };
            let remote_instance = RemoteServiceInstance::new(remote_config, tracker);
            let runtime_dir = ctx.data_dir().join("runtime").join(service_name);
            Service::new_remote(ctx, runtime_dir, service_name, remote_instance).await
        }
        #[cfg(not(feature = "remote-providers"))]
        {
            Err(Error::Other(
                "Remote cloud deployment requires the 'remote-providers' feature to be enabled"
                    .into(),
            ))
        }
    }
}

fn provider_default_tee_backend(provider: &CloudProvider) -> Option<&'static str> {
    match provider {
        CloudProvider::AWS => Some("aws-nitro"),
        CloudProvider::GCP => Some("gcp-confidential"),
        CloudProvider::Azure => Some("azure-skr"),
        _ => None,
    }
}

#[cfg(feature = "remote-providers")]
fn resolve_provider_region(ctx: &BlueprintManagerContext, provider: &CloudProvider) -> String {
    if let Some(config) = ctx.cloud_config() {
        let configured = match provider {
            CloudProvider::AWS => config.aws.as_ref().map(|c| c.region.clone()),
            CloudProvider::GCP => config.gcp.as_ref().map(|c| c.region.clone()),
            CloudProvider::Azure => config.azure.as_ref().map(|c| c.region.clone()),
            CloudProvider::DigitalOcean => config.digital_ocean.as_ref().map(|c| c.region.clone()),
            CloudProvider::Vultr => config.vultr.as_ref().map(|c| c.region.clone()),
            CloudProvider::Hetzner
            | CloudProvider::RunPod
            | CloudProvider::LambdaLabs
            | CloudProvider::PrimeIntellect
            | CloudProvider::VastAi
            | CloudProvider::Crusoe
            | CloudProvider::Generic => None,
            _ => None,
        };
        if let Some(region) = configured {
            return region;
        }
    }

    match provider {
        CloudProvider::AWS => "us-east-1".to_string(),
        CloudProvider::GCP => "us-central1".to_string(),
        CloudProvider::Azure => "eastus".to_string(),
        CloudProvider::DigitalOcean => "nyc3".to_string(),
        CloudProvider::Vultr => "ewr".to_string(),
        CloudProvider::Hetzner => "fsn1".to_string(),
        CloudProvider::RunPod => "US".to_string(),
        CloudProvider::LambdaLabs => "us-west-1".to_string(),
        CloudProvider::PrimeIntellect => "us-east".to_string(),
        CloudProvider::VastAi => "any".to_string(),
        CloudProvider::Crusoe => "us-east1".to_string(),
        _ => "default".to_string(),
    }
}

#[cfg(feature = "remote-providers")]
fn deployment_type_for_provider(
    provider: &blueprint_remote_providers::CloudProvider,
) -> DeploymentType {
    match provider {
        blueprint_remote_providers::CloudProvider::AWS => DeploymentType::AwsEc2,
        blueprint_remote_providers::CloudProvider::GCP => DeploymentType::GcpGce,
        blueprint_remote_providers::CloudProvider::Azure => DeploymentType::AzureVm,
        blueprint_remote_providers::CloudProvider::DigitalOcean => {
            DeploymentType::DigitalOceanDroplet
        }
        blueprint_remote_providers::CloudProvider::Vultr => DeploymentType::VultrInstance,
        blueprint_remote_providers::CloudProvider::LambdaLabs => DeploymentType::LambdaLabsInstance,
        blueprint_remote_providers::CloudProvider::RunPod => DeploymentType::RunPodInstance,
        blueprint_remote_providers::CloudProvider::VastAi => DeploymentType::VastAiInstance,
        blueprint_remote_providers::CloudProvider::CoreWeave => DeploymentType::CoreWeaveWorkload,
        blueprint_remote_providers::CloudProvider::Paperspace => DeploymentType::PaperspaceMachine,
        blueprint_remote_providers::CloudProvider::Fluidstack => DeploymentType::FluidstackServer,
        blueprint_remote_providers::CloudProvider::TensorDock => DeploymentType::TensorDockServer,
        blueprint_remote_providers::CloudProvider::Akash => DeploymentType::AkashLease,
        blueprint_remote_providers::CloudProvider::IoNet => DeploymentType::IoNetCluster,
        blueprint_remote_providers::CloudProvider::PrimeIntellect => {
            DeploymentType::PrimeIntellectPod
        }
        blueprint_remote_providers::CloudProvider::Render => DeploymentType::RenderDispersedNode,
        blueprint_remote_providers::CloudProvider::BittensorLium => {
            DeploymentType::BittensorLiumMiner
        }
        blueprint_remote_providers::CloudProvider::Hetzner => DeploymentType::HetznerServer,
        blueprint_remote_providers::CloudProvider::Crusoe => DeploymentType::CrusoeVm,
        blueprint_remote_providers::CloudProvider::Linode
        | blueprint_remote_providers::CloudProvider::Generic
        | blueprint_remote_providers::CloudProvider::DockerLocal
        | blueprint_remote_providers::CloudProvider::DockerRemote(_)
        | blueprint_remote_providers::CloudProvider::BareMetal(_) => DeploymentType::SshRemote,
    }
}

#[cfg(feature = "remote-providers")]
fn tracker_region_for_provider(provider: &CloudProvider, region: &str) -> String {
    match provider {
        CloudProvider::GCP => format!("{region}-a"),
        _ => region.to_string(),
    }
}

#[cfg(feature = "remote-providers")]
async fn verify_tee_attestation(
    policy: TeeAttestationPolicy,
    provider: &CloudProvider,
    deployment_result: &blueprint_remote_providers::infra::traits::BlueprintDeploymentResult,
) -> Result<Option<String>> {
    match policy {
        TeeAttestationPolicy::Structural => {
            info!(
                "TEE structural attestation gate satisfied for provider {} (instance={})",
                provider, deployment_result.instance.id
            );
            Ok(None)
        }
        TeeAttestationPolicy::Cryptographic => {
            let command_spec = std::env::var("BLUEPRINT_REMOTE_TEE_ATTESTATION_VERIFY_CMD")
                .map_err(|_| Error::TeeRuntimeUnavailable {
                    reason: "Cryptographic attestation policy requires BLUEPRINT_REMOTE_TEE_ATTESTATION_VERIFY_CMD".to_string(),
                })?;
            let mut parts = command_spec.split_whitespace();
            let executable = parts.next().ok_or_else(|| Error::TeeRuntimeUnavailable {
                reason: "BLUEPRINT_REMOTE_TEE_ATTESTATION_VERIFY_CMD is empty".to_string(),
            })?;

            let metadata_json = serde_json::to_string(&deployment_result.metadata)?;
            let mut cmd = tokio::process::Command::new(executable);
            cmd.args(parts);
            cmd.env("BLUEPRINT_TEE_PROVIDER", provider.to_string());
            cmd.env(
                "BLUEPRINT_TEE_INSTANCE_ID",
                deployment_result.instance.id.clone(),
            );
            cmd.env(
                "BLUEPRINT_TEE_PUBLIC_IP",
                deployment_result
                    .instance
                    .public_ip
                    .clone()
                    .unwrap_or_default(),
            );
            cmd.env(
                "BLUEPRINT_TEE_DEPLOYMENT_ID",
                deployment_result.blueprint_id.clone(),
            );
            cmd.env("BLUEPRINT_TEE_METADATA_JSON", metadata_json);
            if let Some(backend) = provider_default_tee_backend(provider) {
                cmd.env("BLUEPRINT_TEE_BACKEND", backend);
            }

            let output = cmd
                .output()
                .await
                .map_err(|e| Error::TeeRuntimeUnavailable {
                    reason: format!("Failed to execute cryptographic attestation verifier: {e}"),
                })?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                return Err(Error::TeeRuntimeUnavailable {
                    reason: format!(
                        "Cryptographic attestation verification failed with status {}. {}",
                        output.status,
                        if stderr.is_empty() {
                            "No verifier stderr output provided".to_string()
                        } else {
                            stderr
                        }
                    ),
                });
            }

            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            info!(
                "TEE cryptographic attestation verification succeeded for provider {} (instance={})",
                provider, deployment_result.instance.id
            );
            if stdout.is_empty() {
                Ok(None)
            } else {
                Ok(Some(stdout))
            }
        }
    }
}
