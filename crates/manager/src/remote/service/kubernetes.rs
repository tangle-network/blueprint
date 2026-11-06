use super::*;

impl RemoteDeploymentService {
    pub(super) async fn deploy_to_kubernetes(
        &self,
        ctx: &BlueprintManagerContext,
        context: &str,
        namespace: &str,
        service_name: &str,
        binary_path: &Path,
        env_vars: BlueprintEnvVars,
        arguments: BlueprintArgs,
        resource_spec: ResourceSpec,
    ) -> Result<Service> {
        info!("🚀 Deploying to Kubernetes cluster");
        info!("   Context: {}", context);
        info!("   Namespace: {}", namespace);
        info!("   Service: {}", service_name);
        let _ = binary_path;

        #[cfg(feature = "remote-providers")]
        {
            use blueprint_remote_providers::{
                CloudProvisioner, core::deployment_target::DeploymentTarget,
            };

            let tracker = ensure_deployment_tracker(ctx, self.deployment_tracker.clone()).await?;

            // Create provisioner
            let provisioner = CloudProvisioner::new()
                .await
                .map_err(|e| Error::Other(format!("Failed to create provisioner: {}", e)))?;

            // Create Kubernetes deployment target
            let target = DeploymentTarget::GenericKubernetes {
                context: Some(context.to_string()),
                namespace: namespace.to_string(),
            };

            // For Kubernetes, we need a container image
            let blueprint_image = format!("ghcr.io/tangle-network/blueprint/{service_name}:latest");

            // Convert env_vars to HashMap
            let mut env_map = HashMap::new();
            for (key, value) in env_vars.encode() {
                env_map.insert(key, value);
            }

            // Add arguments as environment variables
            let encoded_args = arguments.encode(false);
            for (i, arg) in encoded_args.iter().enumerate() {
                env_map.insert(format!("ARG_{}", i), arg.clone());
            }

            // Deploy to Kubernetes using either an inferred provider or a single configured provider.
            let inferred_provider = infer_remote_provider_from_context(context);
            let deployment_result = if let Some(provider) = inferred_provider.clone() {
                provisioner
                    .deploy_with_target_for_provider(
                        &provider,
                        &target,
                        &blueprint_image,
                        &resource_spec,
                        env_map,
                    )
                    .await
                    .map_err(|e| Error::Other(format!("Failed to deploy to Kubernetes: {}", e)))?
            } else {
                provisioner
                    .deploy_with_target(&target, &blueprint_image, &resource_spec, env_map)
                    .await
                    .map_err(|e| Error::Other(format!("Failed to deploy to Kubernetes: {}", e)))?
            };
            let deployment_provider = inferred_provider
                .clone()
                .or_else(|| Some(deployment_result.instance.provider.clone()));

            info!(
                "✅ Blueprint deployed to Kubernetes: {}",
                deployment_result.blueprint_id
            );

            // Register deployment
            let deployment_info = RemoteDeploymentInfo {
                instance_id: deployment_result.blueprint_id.clone(),
                provider: CloudProvider::Generic,
                service_name: service_name.to_string(),
                blueprint_id: None,
                deployed_at: chrono::Utc::now(),
                ttl_expires_at: self
                    .policy
                    .auto_terminate_hours
                    .map(|hours| chrono::Utc::now() + chrono::Duration::hours(hours as i64)),
                public_ip: deployment_result.instance.public_ip.clone(),
                tee_attestation_policy: None,
                tee_attestation_verified_at: None,
                tee_attestation_proof: None,
                kubernetes_context: Some(context.to_string()),
                kubernetes_namespace: Some(namespace.to_string()),
            };

            info!("✅ Kubernetes deployment registered");
            let ttl_seconds = ttl_seconds_from_hours(self.policy.auto_terminate_hours);
            let deployment_key = deployment_result.blueprint_id.clone();

            let mut deployment_record = DeploymentRecord::new(
                deployment_key.clone(),
                DeploymentType::LocalKubernetes,
                resource_spec.clone(),
                ttl_seconds,
            );
            deployment_record.id = deployment_result.blueprint_id.clone();
            if let Some(provider) = deployment_provider.clone() {
                deployment_record.provider = Some(provider);
            }
            deployment_record
                .resource_ids
                .insert("namespace".to_string(), namespace.to_string());
            deployment_record.resource_ids.insert(
                "deployment".to_string(),
                deployment_result.blueprint_id.clone(),
            );
            deployment_record.resource_ids.insert(
                "service".to_string(),
                format!("{}-service", deployment_result.blueprint_id),
            );
            deployment_record
                .resource_ids
                .insert("context".to_string(), context.to_string());
            deployment_record
                .metadata
                .insert("k8s_context".to_string(), context.to_string());
            deployment_record
                .metadata
                .insert("k8s_namespace".to_string(), namespace.to_string());
            if let Err(track_error) = tracker
                .register_deployment(deployment_key.clone(), deployment_record)
                .await
            {
                warn!(
                    "Kubernetes deployment tracking failed (deployment={}); attempting best-effort cleanup before returning error: {}",
                    deployment_result.blueprint_id, track_error
                );
                if let Err(cleanup_error) = best_effort_kubernetes_cleanup(
                    context,
                    namespace,
                    &deployment_result.blueprint_id,
                )
                .await
                {
                    return Err(Error::Other(format!(
                        "Failed to track Kubernetes deployment: {}. Cleanup after tracker failure also failed for deployment {}: {}",
                        track_error, deployment_result.blueprint_id, cleanup_error
                    )));
                }
                return Err(Error::Other(format!(
                    "Failed to track Kubernetes deployment: {}",
                    track_error
                )));
            }

            {
                let mut deployments = self.deployments.write().await;
                deployments.insert(
                    deployment_result.blueprint_id.clone(),
                    deployment_info.clone(),
                );
            }

            let remote_config = RemoteDeploymentConfig {
                deployment_type: DeploymentType::LocalKubernetes,
                provider: deployment_provider,
                region: None,
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
                "Kubernetes deployment requires the 'remote-providers' feature to be enabled"
                    .into(),
            ))
        }
    }
}

#[cfg(feature = "remote-providers")]
fn infer_remote_provider_from_context(
    context: &str,
) -> Option<blueprint_remote_providers::CloudProvider> {
    let context = context.to_ascii_lowercase();
    if context.contains("aws") || context.contains("eks") {
        return Some(blueprint_remote_providers::CloudProvider::AWS);
    }
    if context.contains("gcp") || context.contains("gke") || context.contains("google") {
        return Some(blueprint_remote_providers::CloudProvider::GCP);
    }
    if context.contains("azure") || context.contains("aks") {
        return Some(blueprint_remote_providers::CloudProvider::Azure);
    }
    if context.contains("digitalocean") || context.contains("doks") {
        return Some(blueprint_remote_providers::CloudProvider::DigitalOcean);
    }
    if context.contains("vultr") || context.contains("vke") {
        return Some(blueprint_remote_providers::CloudProvider::Vultr);
    }
    if context.contains("hetzner") {
        return Some(blueprint_remote_providers::CloudProvider::Hetzner);
    }
    if context.contains("runpod") {
        return Some(blueprint_remote_providers::CloudProvider::RunPod);
    }
    if context.contains("lambda") {
        return Some(blueprint_remote_providers::CloudProvider::LambdaLabs);
    }
    if context.contains("prime") || context.contains("primeintellect") {
        return Some(blueprint_remote_providers::CloudProvider::PrimeIntellect);
    }
    if context.contains("vast") {
        return Some(blueprint_remote_providers::CloudProvider::VastAi);
    }
    if context.contains("crusoe") {
        return Some(blueprint_remote_providers::CloudProvider::Crusoe);
    }
    None
}

#[cfg(feature = "remote-providers")]
pub(super) async fn best_effort_kubernetes_cleanup(
    context: &str,
    namespace: &str,
    deployment_id: &str,
) -> std::result::Result<(), String> {
    run_kubectl_delete(context, namespace, "deployment", deployment_id).await?;

    let mut service_errors = Vec::new();
    let compat_service_name = format!("{deployment_id}-service");
    if let Err(error) =
        run_kubectl_delete(context, namespace, "service", &compat_service_name).await
    {
        service_errors.push(error);
    }
    if let Err(error) = run_kubectl_delete(context, namespace, "service", deployment_id).await {
        service_errors.push(error);
    }
    if service_errors.len() == 2 {
        return Err(format!(
            "kubectl cleanup failed for Kubernetes services associated with deployment {}: {}; {}",
            deployment_id, service_errors[0], service_errors[1]
        ));
    }

    Ok(())
}

#[cfg(feature = "remote-providers")]
async fn run_kubectl_delete(
    context: &str,
    namespace: &str,
    resource_kind: &str,
    resource_name: &str,
) -> std::result::Result<(), String> {
    let mut command = tokio::process::Command::new("kubectl");
    if !context.trim().is_empty() {
        command.args(["--context", context]);
    }
    let output = command
        .args([
            "delete",
            resource_kind,
            resource_name,
            "-n",
            namespace,
            "--ignore-not-found=true",
        ])
        .output()
        .await
        .map_err(|error| {
            format!(
                "failed to execute kubectl cleanup for {resource_kind}/{resource_name}: {error}"
            )
        })?;
    if output.status.success() {
        return Ok(());
    }

    Err(format!(
        "kubectl cleanup failed for {resource_kind}/{resource_name}: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}
