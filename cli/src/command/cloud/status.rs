//! Cloud deployment status and termination, read from the deployment tracker and the provider.

use super::open_tracker;
use blueprint_remote_providers::{
    CloudProvider, CloudProvisioner, Error, deployment::DeploymentRecord,
};
use chrono::Utc;
use color_eyre::{
    Result,
    eyre::{WrapErr, ensure, eyre},
};
use dialoguer::Confirm;
use futures::future::join_all;
use std::time::Duration;

/// Show every tracked deployment or one of them, refreshing every five seconds when watching.
pub async fn show_status(service_id: Option<String>, watch: bool) -> Result<()> {
    loop {
        if watch {
            print!("\x1B[2J\x1B[1;1H"); // Clear screen
        }
        let shown = display_status(service_id.as_deref()).await;
        if !watch {
            return shown;
        }
        if let Err(error) = shown {
            eprintln!("{error:#}");
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

/// Print what the tracker and each provider say; a provider that fails its lookup fails the
/// command after every record is printed.
async fn display_status(service_id: Option<&str>) -> Result<()> {
    let mut records = open_tracker().await?.list_all().await?;
    records.sort_by(|left, right| left.id.cmp(&right.id));
    let provisioner = CloudProvisioner::new().await?;
    if let Some(id) = service_id {
        let record = find(records, id)?;
        let state = provider_state(&provisioner, &record).await;
        print_details(
            &record,
            &state
                .as_ref()
                .map_or_else(|error| format!("unknown ({error:#})"), Clone::clone),
        );
        return state.map(drop);
    }
    if records.is_empty() {
        println!("No tracked deployments.");
        return Ok(());
    }
    let states = join_all(
        records
            .iter()
            .map(|record| provider_state(&provisioner, record)),
    )
    .await;
    println!(
        "{:<40} {:<14} {:<14} {:<12} {:<12} {:<16} {:<10} {:<10}",
        "ID", "Provider", "Region", "Tracked", "Provider", "IP", "Uptime", "TTL"
    );
    let mut failed = 0;
    for (record, state) in records.iter().zip(states) {
        println!(
            "{:<40} {:<14} {:<14} {:<12} {:<12} {:<16} {:<10} {:<10}",
            record.id,
            provider_name(record),
            record.region.as_deref().unwrap_or("unknown"),
            tracked_state(record),
            state.as_deref().unwrap_or("failed"),
            public_ip(record),
            uptime(record),
            ttl(record),
        );
        if let Err(error) = state {
            eprintln!("{}: {error:#}", record.id);
            failed += 1;
        }
    }
    println!("\n{} tracked deployments", records.len());
    println!("  View details:  cargo tangle cloud status <id>");
    println!("  Terminate:     cargo tangle cloud terminate <id>");
    ensure!(
        failed == 0,
        "{failed} of {} deployments could not be read from their provider",
        records.len()
    );
    Ok(())
}

fn print_details(record: &DeploymentRecord, state: &str) {
    println!("Deployment:      {}", record.id);
    println!("Blueprint:       {}", record.blueprint_id);
    println!("Type:            {:?}", record.deployment_type);
    println!("Provider:        {}", provider_name(record));
    println!(
        "Region:          {}",
        record.region.as_deref().unwrap_or("unknown")
    );
    println!(
        "Instance:        {}",
        instance_id(record).unwrap_or("unknown")
    );
    println!("Public IP:       {}", public_ip(record));
    println!("Tracked state:   {}", tracked_state(record));
    println!("Provider state:  {state}");
    println!("Deployed:        {}", record.deployed_at.to_rfc3339());
    println!("Uptime:          {}", uptime(record));
    println!("TTL:             {}", ttl(record));
}

/// Terminate one tracked deployment, or all of them, through its provider.
///
/// A deployment leaves the tracker only after its provider confirms termination, and any
/// failure makes the command fail so a caller cannot read it as a stopped deployment.
pub async fn terminate(service_id: Option<String>, yes: bool) -> Result<()> {
    let tracker = open_tracker().await?;
    let mut records = tracker.list_all().await?;
    if let Some(id) = service_id {
        records = vec![find(records, &id)?];
    }
    if records.is_empty() {
        println!("No tracked deployments.");
        return Ok(());
    }
    records.sort_by(|left, right| left.id.cmp(&right.id));
    let ids: Vec<_> = records.iter().map(|record| record.id.as_str()).collect();
    if !yes
        && !Confirm::new()
            .with_prompt(format!("Terminate {}?", ids.join(", ")))
            .default(false)
            .interact()?
    {
        println!("Termination cancelled.");
        return Ok(());
    }
    let provisioner = CloudProvisioner::new().await?;
    let outcomes = join_all(
        records
            .iter()
            .map(|record| terminate_instance(&provisioner, record)),
    )
    .await;
    let mut failed = 0;
    for (record, outcome) in records.iter().zip(outcomes) {
        let removed = match outcome {
            Ok(()) => tracker
                .remove_by_instance_id(&record.id)
                .await
                .wrap_err("terminated, but its tracker record could not be removed"),
            Err(error) => Err(error),
        };
        match removed {
            Ok(()) => println!("Terminated {}", record.id),
            Err(error) => {
                eprintln!("Failed to terminate {}: {error:#}", record.id);
                failed += 1;
            }
        }
    }
    ensure!(
        failed == 0,
        "{failed} of {} deployments were not terminated",
        records.len()
    );
    Ok(())
}

async fn terminate_instance(
    provisioner: &CloudProvisioner,
    record: &DeploymentRecord,
) -> Result<()> {
    let (provider, instance) = target(record)?;
    Ok(provisioner
        .terminate_in(provider, instance, record.region.as_deref())
        .await?)
}

fn target(record: &DeploymentRecord) -> Result<(CloudProvider, &str)> {
    let provider = record
        .provider
        .clone()
        .ok_or_else(|| eyre!("its record names no cloud provider"))?;
    let instance = instance_id(record).ok_or_else(|| eyre!("its record names no instance"))?;
    Ok((provider, instance))
}

fn find(records: Vec<DeploymentRecord>, id: &str) -> Result<DeploymentRecord> {
    records
        .into_iter()
        .find(|record| record.id == id || record.blueprint_id == id)
        .ok_or_else(|| eyre!("Deployment {id} is not in the deployment tracker"))
}

/// The provider's answer for a record. A record that no provider in this environment can answer
/// for reads as unknown; a provider that fails or refuses the lookup is an error.
async fn provider_state(
    provisioner: &CloudProvisioner,
    record: &DeploymentRecord,
) -> Result<String> {
    let (provider, instance) = match target(record) {
        Ok(target) => target,
        Err(error) => return Ok(format!("unknown ({error})")),
    };
    match provisioner
        .get_status_in(provider, instance, record.region.as_deref())
        .await
    {
        Ok(state) => Ok(format!("{state:?}").to_lowercase()),
        Err(error @ Error::ProviderNotConfigured(_)) => Ok(format!("unknown ({error})")),
        Err(error) => Err(error.into()),
    }
}

fn instance_id(record: &DeploymentRecord) -> Option<&str> {
    record.resource_ids.get("instance_id").map(String::as_str)
}

fn provider_name(record: &DeploymentRecord) -> String {
    record
        .provider
        .as_ref()
        .map_or_else(|| "none".to_owned(), |provider| format!("{provider:?}"))
}

fn tracked_state(record: &DeploymentRecord) -> String {
    format!("{:?}", record.status).to_lowercase()
}

fn public_ip(record: &DeploymentRecord) -> &str {
    record
        .resource_ids
        .get("public_ip")
        .or_else(|| record.metadata.get("public_ip"))
        .map_or("unknown", String::as_str)
}

fn uptime(record: &DeploymentRecord) -> String {
    let elapsed = Utc::now().signed_duration_since(record.deployed_at);
    format!("{}h {}m", elapsed.num_hours(), elapsed.num_minutes() % 60)
}

fn ttl(record: &DeploymentRecord) -> String {
    let Some(expires) = record.expires_at else {
        return "never".to_owned();
    };
    let remaining = expires.signed_duration_since(Utc::now());
    if remaining.num_seconds() > 0 {
        format!(
            "{}h {}m",
            remaining.num_hours(),
            remaining.num_minutes() % 60
        )
    } else {
        "expired".to_owned()
    }
}
