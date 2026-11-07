//! Private, durable original authorizations; provider tags are locators, never authority.
use super::{metadata::ManagedMetadata, ownership, quotes::Candidate, *};
use crate::{CloudProvisioner, Error, Result, security::ApiAuthentication};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

fn conflict() -> Error {
    Error::Provider(ProviderErrorCode::Conflict)
}
fn digest(bytes: &[u8]) -> String {
    hex::encode(aws_lc_rs::digest::digest(&aws_lc_rs::digest::SHA256, bytes))
}

pub(crate) fn credential_digest(
    auth: &ApiAuthentication,
    config: impl Serialize,
) -> Result<String> {
    let bytes = zeroize::Zeroizing::new(
        match auth {
            ApiAuthentication::Bearer { token } => serde_json::to_vec(&("bearer", token, config)),
            ApiAuthentication::ApiKey { key, header_name } => {
                serde_json::to_vec(&(header_name, key, config))
            }
            _ => return Err(conflict()),
        }
        .map_err(|_| conflict())?,
    );
    Ok(digest(&bytes))
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Authorization {
    pub intent: String,
    pub public_key: String,
    pub candidate: String,
    pub metadata: ManagedMetadata,
    pub accounts: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Store {
    version: u8,
    namespace: Option<String>,
    pub entries: BTreeMap<String, Authorization>,
}

fn path(root: &Path) -> PathBuf {
    root.join("blueprint/lease-authorizations.json")
}
fn store_identity() -> ManagedLeaseIdentity {
    ManagedLeaseIdentity {
        namespace: "sdk-authorization".into(),
        lease_id: "store".into(),
    }
}

pub(super) fn load(root: &Path) -> Result<Store> {
    if !cfg!(unix) {
        return Err(conflict());
    }
    ownership::directory(root, false)?;
    ownership::directory(&root.join("blueprint"), true)?;
    let mut bytes = Vec::new();
    ownership::open_file(&path(root), false, false)?
        .take(16 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| conflict())?;
    if bytes.len() > 16 * 1024 * 1024 {
        return Err(conflict());
    }
    let store: Store = serde_json::from_slice(&bytes).map_err(|_| conflict())?;
    if store.version != 1 || (store.namespace.is_none() && !store.entries.is_empty()) {
        return Err(conflict());
    }
    for (id, row) in &store.entries {
        row.metadata.validate()?;
        let candidate = Candidate::decode(&row.candidate)?;
        if id != &row.metadata.identity.lease_id
            || Some(&row.metadata.identity.namespace) != store.namespace.as_ref()
            || candidate.allocation_fingerprint()? != row.metadata.request_fingerprint
            || row.intent.len() != 64
            || row.accounts.is_empty()
            || row.accounts.values().any(|v| v.len() != 64)
            || crate::security::canonical_public_key(&row.public_key).is_err()
        {
            return Err(conflict());
        }
    }
    Ok(store)
}

impl CloudProvisioner {
    /// Initialize a new private authorization store, failing if it already exists.
    /// Call only for a deliberately new caller state directory; never repair lost state.
    pub async fn initialize_lease_state_dir(self, root: &Path) -> Result<Self> {
        self.checked_accounts()?;
        if !cfg!(unix) {
            return Err(conflict());
        }
        let root = fs::canonicalize(root).map_err(|_| conflict())?;
        let _guard = ownership::lock_at(&root, "authorization-store", &store_identity()).await?;
        let store = Store {
            version: 1,
            namespace: None,
            entries: BTreeMap::new(),
        };
        let mut file = ownership::open_file(&path(&root), true, true)?;
        file.write_all(&serde_json::to_vec(&store).map_err(|_| conflict())?)
            .map_err(|_| conflict())?;
        file.sync_all().map_err(|_| conflict())?;
        fs::File::open(root.join("blueprint"))
            .and_then(|file| file.sync_all())
            .map_err(|_| conflict())?;
        fs::File::open(&root)
            .and_then(|file| file.sync_all())
            .map_err(|_| conflict())?;
        self.with_lease_state_dir(&root)
    }

    /// Open existing SDK authorizations; missing, corrupt, or unsafe state fails closed.
    /// Carry this directory with caller records and caller-owned SSH keys on restart/move.
    pub fn with_lease_state_dir(mut self, root: &Path) -> Result<Self> {
        self.checked_accounts()?;
        let root = fs::canonicalize(root).map_err(|_| conflict())?;
        load(&root)?;
        self.authorization_dir = Some(root);
        Ok(self)
    }

    pub(super) fn lease_state_dir(&self) -> Result<&Path> {
        self.authorization_dir.as_deref().ok_or_else(conflict)
    }

    pub(super) fn account_binding(&self, provider: &crate::CloudProvider) -> Result<String> {
        let (binding, default) = self.account_bindings.get(provider).ok_or_else(conflict)?;
        let region = self.account_regions.get(provider).or(default.as_ref());
        Ok(digest(
            &serde_json::to_vec(&(binding, region)).map_err(|_| conflict())?,
        ))
    }

    pub(super) fn accounts(&self) -> Result<BTreeMap<String, String>> {
        if !self.init_errors.is_empty()
            || self
                .account_regions
                .keys()
                .any(|key| !self.providers.contains_key(key))
        {
            return Err(Error::Provider(ProviderErrorCode::ProviderNotConfigured));
        }
        self.checked_accounts()
    }

    // Setup needs checked native clients, while missing historical accounts remain
    // per-obligation retirement failures. Allocation still requires full inventory.
    fn checked_accounts(&self) -> Result<BTreeMap<String, String>> {
        let mut result = BTreeMap::new();
        for (provider, adapter) in &self.providers {
            if adapter.managed().is_some() {
                result.insert(
                    provider.catalog_key().into(),
                    self.account_binding(provider)?,
                );
            }
        }
        if result.is_empty() {
            return Err(Error::Provider(ProviderErrorCode::ProviderNotConfigured));
        }
        Ok(result)
    }

    pub(super) async fn reserve(&self, row: Authorization) -> Result<()> {
        let root = self.lease_state_dir()?;
        let _guard = ownership::lock_at(root, "authorization-store", &store_identity()).await?;
        let mut store = load(root)?;
        let identity = &row.metadata.identity;
        if store
            .namespace
            .as_ref()
            .is_some_and(|namespace| namespace != &identity.namespace)
            || store.entries.contains_key(&identity.lease_id)
        {
            return Err(conflict());
        }
        store.namespace = Some(identity.namespace.clone());
        store.entries.insert(identity.lease_id.clone(), row);
        let bytes = serde_json::to_vec(&store).map_err(|_| conflict())?;
        if bytes.len() > 16 * 1024 * 1024 {
            return Err(conflict());
        }
        let temporary = root.join("blueprint").join(format!(
            "lease-authorizations-{}.pending",
            uuid::Uuid::new_v4()
        ));
        let mut file = ownership::open_file(&temporary, true, true)?;
        let result = (|| {
            file.write_all(&bytes).map_err(|_| conflict())?;
            file.sync_all().map_err(|_| conflict())?;
            fs::rename(&temporary, path(root)).map_err(|_| conflict())?;
            fs::File::open(root.join("blueprint"))
                .and_then(|file| file.sync_all())
                .map_err(|_| conflict())
        })();
        // This unique path was created by this call; never remove a preexisting file.
        let _ = fs::remove_file(temporary);
        result
    }

    pub(super) fn authorization(
        &self,
        identity: &ManagedLeaseIdentity,
    ) -> Result<Option<Authorization>> {
        let store = load(self.lease_state_dir()?)?;
        if store
            .namespace
            .as_ref()
            .is_some_and(|namespace| namespace != &identity.namespace)
        {
            return Err(conflict());
        }
        Ok(store.entries.get(&identity.lease_id).cloned())
    }
}

pub(super) fn intent(spec: &ResourceSpec) -> Result<String> {
    let mut intent = spec.clone();
    intent.ssh_public_key = None;
    request_fingerprint(&intent)
}
