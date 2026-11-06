//! Status and retirement of deployments recorded by the manager.

use blueprint_remote_providers::{CloudProvisioner, DeploymentTracker};
use color_eyre::{Result, eyre::eyre};
use std::{path::PathBuf, time::Duration};

fn tracker_path() -> Result<PathBuf> {
    Ok(dirs::home_dir()
        .ok_or_else(|| eyre!("Home directory unavailable"))?
        .join(".tangle/remote_deployments"))
}

pub async fn show_status(deployment_id: Option<String>, watch: bool) -> Result<()> {
    loop {
        if watch {
            print!("\x1B[2J\x1B[1;1H");
        }
        let path = tracker_path()?;
        if !path.exists() {
            if deployment_id.is_some() {
                return Err(eyre!("Deployment not found"));
            }
            println!("No recorded deployments.");
        } else {
            let tracker = DeploymentTracker::new(path).await?;
            let provisioner = CloudProvisioner::new().await?;
            let records = tracker.list_all().await?;
            let mut found = false;
            for record in records {
                let instance = record.resource_ids.get("instance_id");
                if deployment_id
                    .as_ref()
                    .is_some_and(|id| id != &record.id && Some(id) != instance)
                {
                    continue;
                }
                found = true;
                let status = match (&record.provider, instance) {
                    (Some(provider), Some(id)) => {
                        format!("{:?}", provisioner.get_status(provider.clone(), id).await?)
                    }
                    _ => format!("recorded {:?}", record.status),
                };
                println!(
                    "{}\t{}\t{}\t{}",
                    record.id,
                    record
                        .provider
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_default(),
                    record.region.as_deref().unwrap_or(""),
                    status
                );
                if let Some(expires_at) = record.expires_at {
                    println!("  expires: {expires_at}");
                }
            }
            if !found {
                if deployment_id.is_some() {
                    return Err(eyre!("Deployment not found"));
                }
                println!("No recorded deployments.");
            }
        }
        if !watch {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

pub async fn terminate(deployment_id: Option<String>, all: bool, yes: bool) -> Result<()> {
    if !all && deployment_id.is_none() {
        return Err(eyre!("Specify a deployment id or --all"));
    }
    let path = tracker_path()?;
    if !path.exists() {
        return Err(eyre!("No deployment tracker found"));
    }
    let tracker = DeploymentTracker::new(path).await?;
    let records = tracker.list_all().await?;
    let selected: Vec<_> = records
        .into_iter()
        .filter(|record| {
            all || deployment_id.as_ref().is_some_and(|id| {
                id == &record.id || record.resource_ids.get("instance_id") == Some(id)
            })
        })
        .collect();
    if selected.is_empty() {
        return Err(eyre!("Deployment not found"));
    }
    if !yes
        && !dialoguer::Confirm::new()
            .with_prompt(format!("Terminate {} deployment(s)?", selected.len()))
            .default(false)
            .interact()?
    {
        println!("Termination cancelled.");
        return Ok(());
    }
    let provisioner = CloudProvisioner::new().await?;
    for record in selected {
        let provider = record
            .provider
            .ok_or_else(|| eyre!("Deployment has no cloud provider"))?;
        let instance_id = record
            .resource_ids
            .get("instance_id")
            .ok_or_else(|| eyre!("Deployment has no provider instance id"))?;
        provisioner.terminate(provider, instance_id).await?;
        tracker.remove_by_instance_id(instance_id).await?;
        println!("Terminated {}", record.id);
    }
    Ok(())
}
