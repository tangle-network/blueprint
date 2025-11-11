/// List all registered EigenLayer AVS services
use blueprint_eigenlayer_extra::registration::{RegistrationStateManager, RegistrationStatus};
use color_eyre::Result;

/// List all registered EigenLayer AVS services
///
/// # Arguments
///
/// * `active_only` - If true, only show active registrations
/// * `format` - Output format ("table" or "json")
///
/// # Errors
///
/// Returns error if registration state cannot be loaded
pub async fn list_avs_registrations(active_only: bool, format: &str) -> Result<()> {
    // Load registration state
    let state_manager = RegistrationStateManager::load().unwrap_or_else(|e| {
        eprintln!("⚠️  Warning: Could not load registrations: {}", e);
        eprintln!("   No AVS registrations found");
        std::process::exit(0);
    });

    let registrations = state_manager.registrations();

    // Filter registrations
    let filtered: Vec<_> = if active_only {
        registrations
            .registrations
            .values()
            .filter(|r| r.status == RegistrationStatus::Active)
            .collect()
    } else {
        registrations.registrations.values().collect()
    };

    if filtered.is_empty() {
        println!("No AVS registrations found");
        return Ok(());
    }

    match format {
        "json" => print_json(&filtered)?,
        _ => print_table(&filtered),
    }

    Ok(())
}

fn print_table(registrations: &[&blueprint_eigenlayer_extra::registration::AvsRegistration]) {
    println!();
    println!("📋 Registered EigenLayer AVS Services");
    println!("{}", "=".repeat(120));
    println!(
        "{:<20} {:<44} {:<12} Operator",
        "Blueprint ID", "Service Manager", "Status"
    );
    println!("{}", "-".repeat(120));

    for registration in registrations {
        let status_icon = match registration.status {
            RegistrationStatus::Active => "✅",
            RegistrationStatus::Deregistered => "🔴",
            RegistrationStatus::Pending => "⏳",
        };

        println!(
            "{:<20} {:#44x} {:<12} {:#x}",
            registration.blueprint_id(),
            registration.config.service_manager,
            format!("{} {:?}", status_icon, registration.status),
            registration.operator_address
        );
    }

    println!("{}", "=".repeat(120));
    println!();
    println!("Total: {} registration(s)", registrations.len());

    // Print summary statistics
    let active_count = registrations
        .iter()
        .filter(|r| r.status == RegistrationStatus::Active)
        .count();
    let deregistered_count = registrations
        .iter()
        .filter(|r| r.status == RegistrationStatus::Deregistered)
        .count();
    let pending_count = registrations
        .iter()
        .filter(|r| r.status == RegistrationStatus::Pending)
        .count();

    println!();
    println!("Summary:");
    println!("  Active:        {}", active_count);
    println!("  Deregistered:  {}", deregistered_count);
    println!("  Pending:       {}", pending_count);
    println!();
}

fn print_json(
    registrations: &[&blueprint_eigenlayer_extra::registration::AvsRegistration],
) -> Result<()> {
    let json = serde_json::to_string_pretty(registrations)?;
    println!("{}", json);
    Ok(())
}
