//! Backend registry for TEE provider discovery.
//!
//! Allows registering multiple [`TeeRuntimeBackend`] implementations and
//! looking them up by provider type.

use crate::config::TeeProvider;
use crate::errors::TeeError;
use crate::runtime::backend::{
    TeeDeployRequest, TeeDeploymentHandle, TeeDeploymentStatus, TeePublicKey, TeeRuntimeBackend,
};
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Internal trait for type-erased backends (dyn compatible via boxed futures).
trait ErasedBackend: Send + Sync {
    fn deploy(&self, req: TeeDeployRequest)
    -> BoxFuture<'_, Result<TeeDeploymentHandle, TeeError>>;

    fn get_attestation<'a>(
        &'a self,
        handle: &'a TeeDeploymentHandle,
    ) -> BoxFuture<'a, Result<crate::attestation::report::AttestationReport, TeeError>>;

    fn cached_attestation<'a>(
        &'a self,
        handle: &'a TeeDeploymentHandle,
    ) -> BoxFuture<'a, Result<Option<crate::attestation::report::AttestationReport>, TeeError>>;

    fn derive_public_key<'a>(
        &'a self,
        handle: &'a TeeDeploymentHandle,
    ) -> BoxFuture<'a, Result<TeePublicKey, TeeError>>;

    fn status<'a>(
        &'a self,
        handle: &'a TeeDeploymentHandle,
    ) -> BoxFuture<'a, Result<TeeDeploymentStatus, TeeError>>;

    fn stop<'a>(&'a self, handle: &'a TeeDeploymentHandle) -> BoxFuture<'a, Result<(), TeeError>>;

    fn destroy<'a>(
        &'a self,
        handle: &'a TeeDeploymentHandle,
    ) -> BoxFuture<'a, Result<(), TeeError>>;
}

impl<T: TeeRuntimeBackend + 'static> ErasedBackend for T {
    fn deploy(
        &self,
        req: TeeDeployRequest,
    ) -> BoxFuture<'_, Result<TeeDeploymentHandle, TeeError>> {
        Box::pin(TeeRuntimeBackend::deploy(self, req))
    }

    fn get_attestation<'a>(
        &'a self,
        handle: &'a TeeDeploymentHandle,
    ) -> BoxFuture<'a, Result<crate::attestation::report::AttestationReport, TeeError>> {
        Box::pin(TeeRuntimeBackend::get_attestation(self, handle))
    }

    fn cached_attestation<'a>(
        &'a self,
        handle: &'a TeeDeploymentHandle,
    ) -> BoxFuture<'a, Result<Option<crate::attestation::report::AttestationReport>, TeeError>>
    {
        Box::pin(TeeRuntimeBackend::cached_attestation(self, handle))
    }

    fn derive_public_key<'a>(
        &'a self,
        handle: &'a TeeDeploymentHandle,
    ) -> BoxFuture<'a, Result<TeePublicKey, TeeError>> {
        Box::pin(TeeRuntimeBackend::derive_public_key(self, handle))
    }

    fn status<'a>(
        &'a self,
        handle: &'a TeeDeploymentHandle,
    ) -> BoxFuture<'a, Result<TeeDeploymentStatus, TeeError>> {
        Box::pin(TeeRuntimeBackend::status(self, handle))
    }

    fn stop<'a>(&'a self, handle: &'a TeeDeploymentHandle) -> BoxFuture<'a, Result<(), TeeError>> {
        Box::pin(TeeRuntimeBackend::stop(self, handle))
    }

    fn destroy<'a>(
        &'a self,
        handle: &'a TeeDeploymentHandle,
    ) -> BoxFuture<'a, Result<(), TeeError>> {
        Box::pin(TeeRuntimeBackend::destroy(self, handle))
    }
}

/// Type-erased backend wrapper.
struct DynBackend {
    inner: Arc<dyn ErasedBackend>,
}

impl core::fmt::Debug for DynBackend {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DynBackend").finish()
    }
}

/// Registry of TEE runtime backends, keyed by provider.
///
/// This provides type-erased dispatch over multiple [`TeeRuntimeBackend`]
/// implementations. Since `TeeRuntimeBackend` uses RPITIT and is not
/// `dyn`-compatible directly, the registry wraps backends in an internal
/// `ErasedBackend` trait with boxed futures.
///
/// Register backends with [`register`](Self::register), then call lifecycle
/// methods (deploy, stop, destroy, etc.) which dispatch to the correct backend
/// based on the provider.
#[derive(Debug, Default)]
pub struct BackendRegistry {
    backends: BTreeMap<TeeProvider, DynBackend>,
}

impl BackendRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a backend for a provider.
    pub fn register(&mut self, provider: TeeProvider, backend: impl TeeRuntimeBackend + 'static) {
        self.backends.insert(
            provider,
            DynBackend {
                inner: Arc::new(backend),
            },
        );
    }

    /// Check if a provider is registered.
    pub fn has_provider(&self, provider: TeeProvider) -> bool {
        self.backends.contains_key(&provider)
    }

    /// List all registered providers.
    pub fn providers(&self) -> Vec<TeeProvider> {
        self.backends.keys().copied().collect()
    }

    /// Deploy using the backend registered for the given provider.
    pub async fn deploy(
        &self,
        provider: TeeProvider,
        req: TeeDeployRequest,
    ) -> Result<TeeDeploymentHandle, TeeError> {
        let backend = self.backends.get(&provider).ok_or_else(|| {
            TeeError::UnsupportedProvider(format!("no backend registered for {provider}"))
        })?;
        backend.inner.deploy(req).await
    }

    fn get_backend(&self, provider: TeeProvider) -> Result<&DynBackend, TeeError> {
        self.backends.get(&provider).ok_or_else(|| {
            TeeError::UnsupportedProvider(format!("no backend registered for {provider}"))
        })
    }

    /// Get attestation from the backend registered for a deployment's provider.
    pub async fn get_attestation(
        &self,
        handle: &TeeDeploymentHandle,
    ) -> Result<crate::attestation::report::AttestationReport, TeeError> {
        self.get_backend(handle.provider)?
            .inner
            .get_attestation(handle)
            .await
    }

    /// Get cached attestation from the backend registered for a deployment's provider.
    pub async fn cached_attestation(
        &self,
        handle: &TeeDeploymentHandle,
    ) -> Result<Option<crate::attestation::report::AttestationReport>, TeeError> {
        self.get_backend(handle.provider)?
            .inner
            .cached_attestation(handle)
            .await
    }

    /// Derive the public key for a deployment.
    pub async fn derive_public_key(
        &self,
        handle: &TeeDeploymentHandle,
    ) -> Result<TeePublicKey, TeeError> {
        self.get_backend(handle.provider)?
            .inner
            .derive_public_key(handle)
            .await
    }

    /// Get the current status of a deployment.
    pub async fn status(
        &self,
        handle: &TeeDeploymentHandle,
    ) -> Result<TeeDeploymentStatus, TeeError> {
        self.get_backend(handle.provider)?
            .inner
            .status(handle)
            .await
    }

    /// Gracefully stop a running deployment.
    pub async fn stop(&self, handle: &TeeDeploymentHandle) -> Result<(), TeeError> {
        self.get_backend(handle.provider)?.inner.stop(handle).await
    }

    /// Destroy a deployment and release all resources.
    pub async fn destroy(&self, handle: &TeeDeploymentHandle) -> Result<(), TeeError> {
        self.get_backend(handle.provider)?
            .inner
            .destroy(handle)
            .await
    }

    /// Create a registry populated from the `TEE_BACKEND` environment variable.
    ///
    /// The `TEE_BACKEND` variable accepts a comma-separated list of backend names:
    /// `direct`, `aws-nitro`, `gcp-confidential`, `azure-snp`.
    ///
    /// If `TEE_BACKEND` is not set, returns an empty registry.
    ///
    /// # Errors
    ///
    /// Returns an error if a requested backend's feature is not compiled in, or
    /// if a backend's configuration is invalid (e.g., missing required env vars).
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Register only the direct backend
    /// TEE_BACKEND=direct
    ///
    /// # Register AWS Nitro and GCP
    /// TEE_BACKEND=aws-nitro,gcp-confidential
    /// ```
    pub async fn from_env() -> Result<Self, TeeError> {
        let mut registry = Self::new();

        let backends = match std::env::var("TEE_BACKEND") {
            Ok(val) => val,
            Err(_) => return Ok(registry),
        };

        for backend in backends.split(',').map(str::trim) {
            match backend {
                "direct" | "direct-tdx" => {
                    registry.register(
                        TeeProvider::IntelTdx,
                        crate::runtime::direct::DirectBackend::tdx(),
                    );
                }
                "direct-sev-snp" => {
                    registry.register(
                        TeeProvider::AmdSevSnp,
                        crate::runtime::direct::DirectBackend::sev_snp(),
                    );
                }
                #[cfg(feature = "aws-nitro")]
                "aws-nitro" => {
                    let backend = crate::runtime::aws_nitro::NitroBackend::from_env().await?;
                    registry.register(TeeProvider::AwsNitro, backend);
                }
                #[cfg(not(feature = "aws-nitro"))]
                "aws-nitro" => {
                    return Err(TeeError::UnsupportedProvider(
                        "aws-nitro backend requested but the 'aws-nitro' feature is not enabled; \
                         recompile with --features aws-nitro"
                            .to_string(),
                    ));
                }
                #[cfg(feature = "gcp-confidential")]
                "gcp-confidential" => {
                    let backend =
                        crate::runtime::gcp_confidential::GcpConfidentialBackend::from_env()
                            .await?;
                    registry.register(TeeProvider::GcpConfidential, backend);
                }
                #[cfg(not(feature = "gcp-confidential"))]
                "gcp-confidential" => {
                    return Err(TeeError::UnsupportedProvider(
                        "gcp-confidential backend requested but the 'gcp-confidential' feature \
                         is not enabled; recompile with --features gcp-confidential"
                            .to_string(),
                    ));
                }
                #[cfg(feature = "azure-snp")]
                "azure-snp" => {
                    let backend = crate::runtime::azure_skr::AzureSkrBackend::from_env()?;
                    registry.register(TeeProvider::AzureSnp, backend);
                }
                #[cfg(not(feature = "azure-snp"))]
                "azure-snp" => {
                    return Err(TeeError::UnsupportedProvider(
                        "azure-snp backend requested but the 'azure-snp' feature is not enabled; \
                         recompile with --features azure-snp"
                            .to_string(),
                    ));
                }
                other => {
                    return Err(TeeError::Config(format!(
                        "unknown TEE_BACKEND value: '{other}'; \
                         valid options: direct, direct-tdx, direct-sev-snp, \
                         aws-nitro, gcp-confidential, azure-snp"
                    )));
                }
            }
        }

        Ok(registry)
    }
}
