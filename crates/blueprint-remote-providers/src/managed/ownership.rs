use super::{ManagedLeaseIdentity, ProviderErrorCode};
use crate::{CloudProvider, Error, Result};
use std::fs;
use std::path::{Component, Path, PathBuf};

fn invalid() -> Error {
    Error::Provider(ProviderErrorCode::InvalidRequest)
}

fn directory(path: &Path, private: bool) -> Result<()> {
    let metadata = fs::symlink_metadata(path).map_err(|_| invalid())?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(invalid());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let mask = if private { 0o077 } else { 0o022 };
        if metadata.uid() != unsafe { libc::geteuid() } || metadata.permissions().mode() & mask != 0
        {
            return Err(invalid());
        }
    }
    Ok(())
}

pub(super) async fn lock(
    provider: &CloudProvider,
    identity: &ManagedLeaseIdentity,
) -> Result<fs::File> {
    lock_scope(provider.catalog_key(), identity).await
}

/// Serialize one lease across provider choice, processes and manager restarts.
/// Hold this guard across inventory and allocation; dropping it releases the OS lock.
pub async fn lock_lease(identity: &ManagedLeaseIdentity) -> Result<fs::File> {
    lock_scope("lease", identity).await
}

async fn lock_scope(scope: &str, identity: &ManagedLeaseIdentity) -> Result<fs::File> {
    identity.validate().map_err(|_| invalid())?;
    let base = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|path| PathBuf::from(path).join(".cache")))
        .ok_or_else(invalid)?;
    if !base.is_absolute()
        || base
            .components()
            .any(|part| matches!(part, Component::ParentDir))
    {
        return Err(invalid());
    }
    let digest = hex::encode(aws_lc_rs::digest::digest(
        &aws_lc_rs::digest::SHA256,
        format!(
            "{}:{}:{}",
            scope,
            identity.namespace.len(),
            identity.namespace
        )
        .as_bytes(),
    ));
    let filename = format!("{digest}-{}.lock", identity.lease_id);
    tokio::task::spawn_blocking(move || {
        let mut path = PathBuf::new();
        for component in base.components() {
            path.push(component);
            if let Ok(metadata) = fs::symlink_metadata(&path) {
                if metadata.file_type().is_symlink() || !metadata.is_dir() {
                    return Err(invalid());
                }
                #[cfg(unix)]
                {
                    use std::os::unix::fs::{MetadataExt, PermissionsExt};
                    let mode = metadata.permissions().mode();
                    if (metadata.uid() != 0 && metadata.uid() != unsafe { libc::geteuid() })
                        || (mode & 0o022 != 0 && mode & libc::S_ISVTX == 0)
                    {
                        return Err(invalid());
                    }
                }
            }
        }
        let mut builder = fs::DirBuilder::new();
        builder.recursive(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder.create(&base).map_err(|_| invalid())?;
        directory(&base, false)?;
        let parent = base.join("blueprint");
        builder.create(&parent).map_err(|_| invalid())?;
        directory(&parent, true)?;
        let locks = parent.join("gpu-operation-locks");
        builder.create(&locks).map_err(|_| invalid())?;
        directory(&locks, true)?;
        let mut options = fs::OpenOptions::new();
        options.create(true).read(true).write(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options
                .mode(0o600)
                .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
        }
        let file = options.open(locks.join(filename)).map_err(|_| invalid())?;
        let metadata = file.metadata().map_err(|_| invalid())?;
        if !metadata.is_file() {
            return Err(invalid());
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::{MetadataExt, PermissionsExt};
            if metadata.uid() != unsafe { libc::geteuid() }
                || metadata.permissions().mode() & 0o077 != 0
                || metadata.nlink() != 1
            {
                return Err(invalid());
            }
        }
        file.lock().map_err(|_| invalid())?;
        Ok(file)
    })
    .await
    .map_err(|_| invalid())?
}
