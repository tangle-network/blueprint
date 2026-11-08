use super::{ManagedLeaseIdentity, ProviderErrorCode};
use crate::{CloudProvider, Error, Result};
use std::fs;
use std::path::{Component, Path, PathBuf};

fn invalid() -> Error {
    Error::Provider(ProviderErrorCode::InvalidRequest)
}

pub(super) fn directory(path: &Path, private: bool) -> Result<()> {
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
    store: Option<&Path>,
    provider: &CloudProvider,
    identity: &ManagedLeaseIdentity,
) -> Result<fs::File> {
    let base = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|path| PathBuf::from(path).join(".cache")))
        .or_else(|| store.map(Path::to_path_buf))
        .ok_or_else(invalid)?;
    lock_scope(base, provider.catalog_key(), identity).await
}

/// Serialize service inventory and allocation in the manager's shared state store.
pub async fn lock_lease(state_dir: &Path, identity: &ManagedLeaseIdentity) -> Result<fs::File> {
    lock_at(state_dir, "lease", identity).await
}

pub(super) async fn lock_at(
    base: &Path,
    scope: &str,
    identity: &ManagedLeaseIdentity,
) -> Result<fs::File> {
    let base = fs::canonicalize(base).map_err(|_| invalid())?;
    lock_scope(base, scope, identity).await
}

pub(super) fn open_file(path: &Path, create: bool, exclusive: bool) -> Result<fs::File> {
    let mut options = fs::OpenOptions::new();
    options
        .read(true)
        .write(create)
        .create(create && !exclusive)
        .create_new(exclusive)
        .truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
    }
    let file = options.open(path).map_err(|_| invalid())?;
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
    Ok(file)
}

async fn lock_scope(
    base: PathBuf,
    scope: &str,
    identity: &ManagedLeaseIdentity,
) -> Result<fs::File> {
    identity.validate().map_err(|_| invalid())?;
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
                    let mode = metadata.permissions().mode() as libc::mode_t;
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
        let file = open_file(&locks.join(filename), true, false)?;
        file.lock().map_err(|_| invalid())?;
        Ok(file)
    })
    .await
    .map_err(|_| invalid())?
}
