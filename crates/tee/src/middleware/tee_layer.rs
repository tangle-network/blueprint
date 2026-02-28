//! TEE attestation layer for job results.
//!
//! Follows the same pattern as [`TangleLayer`] to inject TEE attestation
//! metadata into successful job results, enabling downstream consumers
//! to verify TEE provenance.

use crate::attestation::report::AttestationReport;
use blueprint_core::{JobCall, JobResult};
use core::future::Future;
use core::pin::Pin;
use core::task::ready;
use core::task::{Context, Poll};
use pin_project_lite::pin_project;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower::{Layer, Service};

/// Metadata key for the TEE attestation evidence digest.
pub const TEE_ATTESTATION_DIGEST_KEY: &str = "tee.attestation.digest";

/// Metadata key for the TEE provider name.
pub const TEE_PROVIDER_KEY: &str = "tee.provider";

/// Metadata key for the TEE platform measurement.
pub const TEE_MEASUREMENT_KEY: &str = "tee.measurement";

/// A tower layer that attaches TEE attestation metadata to job results.
///
/// When an attestation report is available, this layer injects the following
/// metadata keys into successful `JobResult` responses:
///
/// - `tee.attestation.digest` — SHA-256 digest of the attestation evidence
/// - `tee.provider` — The TEE provider name (e.g., "intel_tdx")
/// - `tee.measurement` — The platform measurement string
///
/// # Examples
///
/// ```rust,ignore
/// use blueprint_tee::middleware::TeeLayer;
/// use blueprint_router::Router;
///
/// let router = Router::new()
///     .route(0, my_job_handler)
///     .layer(TeeLayer::new());
/// ```
#[derive(Clone, Debug)]
pub struct TeeLayer {
    attestation: Arc<Mutex<Option<AttestationReport>>>,
}

impl TeeLayer {
    /// Create a new TEE layer with no initial attestation.
    pub fn new() -> Self {
        Self {
            attestation: Arc::new(Mutex::new(None)),
        }
    }

    /// Create a new TEE layer with an initial attestation report.
    pub fn with_attestation(report: AttestationReport) -> Self {
        Self {
            attestation: Arc::new(Mutex::new(Some(report))),
        }
    }

    /// Update the attestation report used by this layer.
    pub async fn set_attestation(&self, report: AttestationReport) {
        *self.attestation.lock().await = Some(report);
    }

    /// Get a handle to the shared attestation state for external updates.
    pub fn attestation_handle(&self) -> Arc<Mutex<Option<AttestationReport>>> {
        self.attestation.clone()
    }
}

impl Default for TeeLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Layer<S> for TeeLayer {
    type Service = TeeService<S>;

    fn layer(&self, service: S) -> Self::Service {
        TeeService {
            service,
            attestation: self.attestation.clone(),
        }
    }
}

/// The service produced by [`TeeLayer`].
#[derive(Clone, Debug)]
pub struct TeeService<S> {
    service: S,
    attestation: Arc<Mutex<Option<AttestationReport>>>,
}

pin_project! {
    /// Response future for [`TeeService`].
    pub struct TeeServiceFuture<F> {
        #[pin]
        inner: F,
        attestation_digest: Option<String>,
        provider: Option<String>,
        measurement: Option<String>,
    }
}

impl<F, B, E> Future for TeeServiceFuture<F>
where
    F: Future<Output = Result<Option<JobResult<B>>, E>>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let result = ready!(this.inner.poll(cx)?);

        match result {
            Some(mut result) => {
                let JobResult::Ok { head, .. } = &mut result else {
                    return Poll::Ready(Ok(Some(result)));
                };

                if let Some(digest) = this.attestation_digest.take() {
                    head.metadata.insert(TEE_ATTESTATION_DIGEST_KEY, digest);
                }
                if let Some(provider) = this.provider.take() {
                    head.metadata.insert(TEE_PROVIDER_KEY, provider);
                }
                if let Some(measurement) = this.measurement.take() {
                    head.metadata.insert(TEE_MEASUREMENT_KEY, measurement);
                }

                Poll::Ready(Ok(Some(result)))
            }
            None => Poll::Ready(Ok(None)),
        }
    }
}

impl<S> Service<JobCall> for TeeService<S>
where
    S: Service<JobCall, Response = Option<JobResult>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = TeeServiceFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, call: JobCall) -> Self::Future {
        // Try to get the current attestation snapshot synchronously.
        // We use try_lock to avoid blocking the service call path.
        let (attestation_digest, provider, measurement) =
            match self.attestation.try_lock() {
                Ok(guard) => match guard.as_ref() {
                    Some(report) => (
                        Some(report.evidence_digest()),
                        Some(report.provider.to_string()),
                        Some(report.measurement.to_string()),
                    ),
                    None => (None, None, None),
                },
                Err(_) => (None, None, None),
            };

        TeeServiceFuture {
            inner: self.service.call(call),
            attestation_digest,
            provider,
            measurement,
        }
    }
}
