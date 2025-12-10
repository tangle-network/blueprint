use dialoguer::console::style;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

/// Print a section header with consistent styling
pub fn print_section_header(title: &str) {
    println!("\n{}", style(format!("━━━ {} ━━━", title)).cyan().bold());
}

/// Print a success message with an emoji and optional details
pub fn print_success(message: &str, details: Option<&str>) {
    println!(
        "\n{}  {}{}",
        style("✓").green().bold(),
        style(message).green(),
        details.map_or(String::new(), |d| format!("\n   {}", style(d).dim()))
    );
}

/// Print an info message with consistent styling
pub fn print_info(message: &str) {
    println!("{}", style(format!("ℹ {}", message)).blue());
}

/// Locate the newest `registration_inputs.bin` for a blueprint.
pub fn find_registration_inputs(base_dir: &Path, blueprint_id: u64) -> Option<PathBuf> {
    let prefix = format!("blueprint-{blueprint_id}-");
    WalkDir::new(base_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| entry.file_name() == "registration_inputs.bin")
        .filter(|entry| {
            entry
                .path()
                .parent()
                .and_then(|parent| parent.file_name())
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with(&prefix))
                .unwrap_or(false)
        })
        .filter_map(|entry| {
            let modified = entry
                .metadata()
                .ok()
                .and_then(|meta| meta.modified().ok())
                .unwrap_or(SystemTime::UNIX_EPOCH);
            Some((entry.path().to_path_buf(), modified))
        })
        .max_by_key(|(_, modified)| *modified)
        .map(|(path, _)| path)
}

/// Locate the workspace root by walking up to the directory that contains `Cargo.lock`.
pub fn workspace_root() -> Option<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .find(|path| path.join("Cargo.lock").exists())
        .map(|p| p.to_path_buf())
}
