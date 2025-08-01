/// The config for a [`Keystore`]
///
/// Depending on the features enabled, this provides methods to enable different storage backends.
///
/// ### Implicit registration
///
/// When no other backends are enabled, an [`InMemoryStorage`] will implicitly be registered by [`Keystore::new()`].
///
/// This means that:
///
/// ```rust
/// use blueprint_keystore::{Keystore, KeystoreConfig};
///
/// # fn main() -> blueprint_keystore::Result<()> {
/// let config = KeystoreConfig::new().in_memory(true);
/// let keystore = Keystore::new(config)?;
/// # Ok(()) }
/// ```
///
/// is equivalent to:
///
/// ```rust
/// use blueprint_keystore::{Keystore, KeystoreConfig};
///
/// # fn main() -> blueprint_keystore::Result<()> {
/// let keystore = Keystore::new(KeystoreConfig::new())?;
/// # Ok(()) }
/// ```
///
/// [`InMemoryStorage`]: crate::storage::InMemoryStorage
/// [`Keystore`]: crate::Keystore
/// [`Keystore::new()`]: crate::Keystore::new
#[derive(Default, Clone)]
pub struct KeystoreConfig {
    pub(crate) in_memory: bool,
    #[cfg(feature = "std")]
    pub(crate) fs_root: Option<std::path::PathBuf>,
    #[cfg(any(
        feature = "aws-signer",
        feature = "gcp-signer",
        feature = "ledger-browser",
        feature = "ledger-node"
    ))]
    pub(crate) remote_configs: Vec<crate::remote::RemoteConfig>,

    #[cfg(feature = "substrate-keystore")]
    pub(crate) substrate: Option<blueprint_std::sync::Arc<sc_keystore::LocalKeystore>>,
}

impl core::fmt::Debug for KeystoreConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut binding = f.debug_struct("KeystoreConfig");
        let c = binding.field("in_memory", &self.in_memory);
        #[cfg(feature = "std")]
        let c = c.field("fs_root", &self.fs_root);

        #[cfg(any(
            feature = "aws-signer",
            feature = "gcp-signer",
            feature = "ledger-browser",
            feature = "ledger-node"
        ))]
        let c = c.field("remote_configs", &self.remote_configs);
        #[cfg(feature = "substrate-keystore")]
        c.field("substrate", &"substrate");
        c.finish()
    }
}

impl KeystoreConfig {
    /// Create a new `KeystoreConfig`
    ///
    /// Alias for [`KeystoreConfig::default()`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use blueprint_keystore::{Keystore, KeystoreConfig};
    ///
    /// # fn main() -> blueprint_keystore::Result<()> {
    /// let config = KeystoreConfig::new();
    /// let keystore = Keystore::new(config)?;
    /// # Ok(()) }
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an [`InMemoryStorage`] backend
    ///
    /// NOTE: This will be enabled by default if no other storage backends are enabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use blueprint_keystore::{Keystore, KeystoreConfig};
    ///
    /// # fn main() -> blueprint_keystore::Result<()> {
    /// let config = KeystoreConfig::new().in_memory(true);
    /// let keystore = Keystore::new(config)?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`InMemoryStorage`]: crate::storage::InMemoryStorage
    #[must_use]
    pub fn in_memory(mut self, value: bool) -> Self {
        self.in_memory = value;
        self
    }

    /// Register a [`FileStorage`] backend
    ///
    /// See [`FileStorage::new()`] for notes on how `path` is used.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use blueprint_keystore::{Keystore, KeystoreConfig};
    ///
    /// # fn main() -> blueprint_keystore::Result<()> {
    /// let config = KeystoreConfig::new().fs_root("path/to/keystore");
    /// let keystore = Keystore::new(config)?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`FileStorage`]: crate::storage::FileStorage
    /// [`FileStorage::new()`]: crate::storage::FileStorage::new
    #[cfg(feature = "std")]
    #[must_use]
    pub fn fs_root<P: AsRef<std::path::Path>>(mut self, path: P) -> Self {
        self.fs_root = Some(path.as_ref().to_path_buf());
        self
    }

    /// Register a [`SubstrateStorage`] backend
    /// See [`SubstrateStorage::new()`] for notes on how `keystore` is used.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use blueprint_keystore::{Keystore, KeystoreConfig};
    /// use blueprint_std::sync::Arc;
    /// use sc_keystore::LocalKeystore;
    ///
    /// # fn main() -> blueprint_keystore::Result<()> {
    /// let keystore = LocalKeystore::in_memory();
    /// let config = KeystoreConfig::new().substrate(Arc::new(keystore));
    /// let keystore = Keystore::new(config)?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`SubstrateStorage`]: crate::storage::SubstrateStorage
    /// [`SubstrateStorage::new()`]: crate::storage::SubstrateStorage::new
    #[cfg(feature = "substrate-keystore")]
    #[must_use]
    pub fn substrate(
        mut self,
        keystore: blueprint_std::sync::Arc<sc_keystore::LocalKeystore>,
    ) -> Self {
        self.substrate = Some(keystore);
        self
    }

    cfg_remote! {
        /// Register a remote backend
        ///
        /// See [`RemoteConfig`] for available options.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// use blueprint_keystore::{Keystore, KeystoreConfig};
        /// use blueprint_keystore::remote::RemoteConfig;
        ///
        /// # fn main() -> blueprint_keystore::Result<()> {
        /// let remote = RemoteConfig::Aws {
        ///     keys: vec![]
        /// };
        ///
        /// let config = KeystoreConfig::new().remote(remote);
        /// let keystore = Keystore::new(config)?;
        /// # Ok(()) }
        /// ```
        ///
        /// [`RemoteConfig`]: crate::remote::RemoteConfig
        #[must_use]
        pub fn remote(mut self, remote_config: crate::remote::RemoteConfig) -> Self {
            self.remote_configs.push(remote_config);
            self
        }
    }

    #[allow(unused_mut)]
    fn is_empty(&self) -> bool {
        let mut is_empty = self.in_memory;
        #[cfg(feature = "std")]
        {
            is_empty |= self.fs_root.is_none();
        }
        #[cfg(any(
            feature = "aws-signer",
            feature = "gcp-signer",
            feature = "ledger-browser",
            feature = "ledger-node"
        ))]
        {
            is_empty |= !self.remote_configs.is_empty();
        }

        is_empty
    }

    pub(crate) fn finalize(self) -> Self {
        if self.is_empty() {
            return self.in_memory(true);
        }

        self
    }
}
