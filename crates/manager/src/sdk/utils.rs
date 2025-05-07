use crate::error::{Error, Result};
use blueprint_core::warn;
use sha2::Digest;
use std::path::{Path, PathBuf};
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::field::BoundedString;

/// Converts a `BoundedString` to a `String`
///
/// # Arguments
/// * `string` - The `BoundedString` to convert
///
/// # Returns
/// The `String` representation of the `BoundedString`
///
/// # Errors
/// * If the `BoundedString` cannot be converted to a `String`
pub fn bounded_string_to_string(string: &BoundedString) -> Result<String> {
    let bytes: &Vec<u8> = &string.0.0;
    let ret = String::from_utf8(bytes.clone())?;
    Ok(ret)
}

pub fn hash_bytes_to_hex<T: AsRef<[u8]>>(input: T) -> String {
    let mut hasher = sha2::Sha256::default();
    hasher.update(input);
    hex::encode(hasher.finalize())
}

pub async fn valid_file_exists(path: impl AsRef<Path>, _expected_hash: &str) -> bool {
    // The hash is sha3_256 of the binary
    if let Ok(_file) = tokio::fs::read(path).await {
        // TODO(HACK): Compute the SHA3-256
        //let retrieved_bytes = hash_bytes_to_hex(file);
        //expected_hash == retrieved_bytes.as_str()
        true
    } else {
        false
    }
}

#[must_use]
pub fn get_formatted_os_string() -> String {
    let os = std::env::consts::OS;

    match os {
        "macos" => "apple-darwin".to_string(),
        "windows" => "pc-windows-msvc".to_string(),
        "linux" => "unknown-linux-gnu".to_string(),
        _ => os.to_string(),
    }
}

/// Makes a file executable by setting the executable permission bits on Unix systems.
/// On Windows, this adds the `.exe` extension if it doesn't already exist.
///
/// # Arguments
/// * `path` - Path to the file to make executable
///
/// # Returns
/// This returns the altered path on Windows, and the original path on Unix.
///
/// # Errors
/// * On Unix, if the file cannot be opened or its metadata cannot be read
pub fn make_executable<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    #[cfg(target_family = "unix")]
    fn unix(path: &Path) -> Result<()> {
        use std::os::unix::fs::PermissionsExt;

        let f = std::fs::File::open(path)?;
        let mut perms = f.metadata()?.permissions();
        perms.set_mode(perms.mode() | 0o111);
        f.set_permissions(perms)?;

        Ok(())
    }

    let mut path = path.as_ref().to_path_buf();
    if cfg!(target_family = "windows") {
        if path.extension().is_none() {
            path.set_extension("exe");
        }
    } else if let Err(err) = unix(&path) {
        let msg = format!("Failed to make the binary executable: {err}");
        warn!("{}", msg);
        return Err(Error::Other(msg));
    }

    Ok(path)
}

#[must_use]
pub fn slice_32_to_sha_hex_string(hash: [u8; 32]) -> String {
    use std::fmt::Write;
    hash.iter().fold(String::new(), |mut acc, byte| {
        write!(&mut acc, "{:02x}", byte).expect("Should be able to write");
        acc
    })
}
