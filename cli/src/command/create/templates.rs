//! Embedded templates for blueprint creation.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Embedded template files for a Tangle EVM blueprint.
pub struct TangleEvmTemplate;

impl TangleEvmTemplate {
    const CARGO_GENERATE_TOML: &'static str =
        include_str!("../../../templates/tangle-evm/cargo-generate.toml");
    const CARGO_TOML: &'static str = include_str!("../../../templates/tangle-evm/Cargo.toml");
    const LIB_CARGO_TOML: &'static str =
        include_str!("../../../templates/tangle-evm/{{project-name}}-lib/Cargo.toml");
    const LIB_RS: &'static str =
        include_str!("../../../templates/tangle-evm/{{project-name}}-lib/src/lib.rs");
    const BIN_CARGO_TOML: &'static str =
        include_str!("../../../templates/tangle-evm/{{project-name}}-bin/Cargo.toml");
    const MAIN_RS: &'static str =
        include_str!("../../../templates/tangle-evm/{{project-name}}-bin/src/main.rs");

    /// Write the embedded template to a temporary directory.
    ///
    /// Returns the path to the template directory.
    ///
    /// # Errors
    ///
    /// Returns an error if file operations fail.
    pub fn write_to_temp_dir() -> io::Result<PathBuf> {
        let temp_dir = std::env::temp_dir()
            .join(format!("tangle-blueprint-template-{}", std::process::id()));

        // Clean up any existing temp dir
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }

        Self::write_to_dir(&temp_dir)?;
        Ok(temp_dir)
    }

    /// Write the embedded template to the specified directory.
    ///
    /// # Errors
    ///
    /// Returns an error if file operations fail.
    pub fn write_to_dir(dir: &Path) -> io::Result<()> {
        // Create directory structure
        let lib_src_dir = dir.join("{{project-name}}-lib").join("src");
        let bin_src_dir = dir.join("{{project-name}}-bin").join("src");

        fs::create_dir_all(&lib_src_dir)?;
        fs::create_dir_all(&bin_src_dir)?;

        // Write files
        fs::write(dir.join("cargo-generate.toml"), Self::CARGO_GENERATE_TOML)?;
        fs::write(dir.join("Cargo.toml"), Self::CARGO_TOML)?;
        fs::write(
            dir.join("{{project-name}}-lib").join("Cargo.toml"),
            Self::LIB_CARGO_TOML,
        )?;
        fs::write(lib_src_dir.join("lib.rs"), Self::LIB_RS)?;
        fs::write(
            dir.join("{{project-name}}-bin").join("Cargo.toml"),
            Self::BIN_CARGO_TOML,
        )?;
        fs::write(bin_src_dir.join("main.rs"), Self::MAIN_RS)?;

        Ok(())
    }

    /// Clean up the temporary template directory.
    ///
    /// # Errors
    ///
    /// Returns an error if removal fails.
    pub fn cleanup_temp_dir() -> io::Result<()> {
        let temp_dir = std::env::temp_dir()
            .join(format!("tangle-blueprint-template-{}", std::process::id()));
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_to_temp_dir() {
        let temp_dir = TangleEvmTemplate::write_to_temp_dir().unwrap();
        assert!(temp_dir.exists());
        assert!(temp_dir.join("Cargo.toml").exists());
        assert!(temp_dir.join("cargo-generate.toml").exists());
        assert!(temp_dir
            .join("{{project-name}}-lib")
            .join("Cargo.toml")
            .exists());
        assert!(temp_dir
            .join("{{project-name}}-lib")
            .join("src")
            .join("lib.rs")
            .exists());
        assert!(temp_dir
            .join("{{project-name}}-bin")
            .join("Cargo.toml")
            .exists());
        assert!(temp_dir
            .join("{{project-name}}-bin")
            .join("src")
            .join("main.rs")
            .exists());

        TangleEvmTemplate::cleanup_temp_dir().unwrap();
        assert!(!temp_dir.exists());
    }
}
