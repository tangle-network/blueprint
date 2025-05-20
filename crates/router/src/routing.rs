//! Routing between [`Service`]s and jobs.

use crate::future::{Route, RouteFuture};
use alloc::boxed::Box;

use crate::job_id_router::JobIdRouter;
use crate::util::try_downcast;
use blueprint_core::{IntoJobResult, Job, JobCall, JobId, JobResult};

use alloc::sync::Arc;
use alloc::vec::Vec;
use bytes::Bytes;
use core::marker::PhantomData;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::{fmt, iter};
use futures::StreamExt;
use futures::stream::FuturesUnordered;
use tower::{BoxError, Layer, Service};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct RouteId(pub u32);

/// The router type for composing jobs and services.
///
/// `Router<Ctx>` means a router that is missing a context of type `Ctx` to be able to handle requests.
/// Thus, only `Router<()>` (i.e. without missing context) can be passed to a [`BlueprintRunner`]. See [`Router::with_context()`] for more details.
///
/// [`BlueprintRunner`]: https://docs.rs/blueprint-runner/latest/blueprint_runner/struct.BlueprintRunner.html
#[must_use]
pub struct Router<Ctx = ()> {
    inner: Arc<JobIdRouter<Ctx>>,
}

impl<Ctx> Clone for Router<Ctx> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<Ctx> Default for Router<Ctx>
where
    Ctx: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Ctx> fmt::Debug for Router<Ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<Ctx> Router<Ctx>
where
    Ctx: Clone + Send + Sync + 'static,
{
    /// Create a new `Router`.
    ///
    /// Unless you add additional routes this will ignore all requests.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(JobIdRouter::default()),
        }
    }

    fn into_inner(self) -> JobIdRouter<Ctx> {
        Arc::try_unwrap(self.inner).unwrap_or_else(|arc| arc.as_ref().clone())
    }

    /// Add a [`Job`] to the router, with the given job ID.
    ///
    /// The job will be called when a [`JobCall`] with the given job ID is received by the router.
    #[track_caller]
    pub fn route<I, J, T>(self, job_id: I, job: J) -> Self
    where
        I: Into<JobId>,
        J: Job<T, Ctx>,
        T: 'static,
    {
        let mut inner = self.into_inner();
        inner.route(job_id, job);
        Router {
            inner: Arc::new(inner),
        }
    }

    /// Add a [`Service`] to the router, with the given job ID.
    ///
    /// # Panics
    ///
    /// Panics if `service` is a `Router`.
    pub fn route_service<T>(self, job_id: u32, service: T) -> Self
    where
        T: Service<JobCall, Error = BoxError> + Clone + Send + Sync + 'static,
        T::Response: IntoJobResult,
        T::Future: Send + 'static,
    {
        let service = match try_downcast::<Router<Ctx>, _>(service) {
            Ok(_) => {
                panic!("Invalid route: `Router::route_service` cannot be used with `Router`s.");
            }
            Err(service) => service,
        };

        let mut inner = self.into_inner();
        inner.route_service(job_id, service);
        Router {
            inner: Arc::new(inner),
        }
    }

    /// Add a [`Job`] that *always* gets called, regardless of the job ID
    ///
    /// This is useful for jobs that want to watch for certain events. Any [`JobCall`] received by
    /// router will be passed to the `job`, regardless if another route matches.
    #[track_caller]
    pub fn always<J, T>(self, job: J) -> Self
    where
        J: Job<T, Ctx>,
        T: 'static,
    {
        let mut inner = self.into_inner();
        inner.always(job);
        Router {
            inner: Arc::new(inner),
        }
    }

    /// Add a [`Job`] that gets called if no other route matches
    ///
    /// NOTE: This will replace any existing fallback route.
    ///
    /// This will **only** be called when:
    /// - No other route matches the job ID
    /// - No [`always`] route is present
    ///
    /// [`always`]: Router::always
    #[track_caller]
    pub fn fallback<J, T>(self, job: J) -> Self
    where
        J: Job<T, Ctx>,
        T: 'static,
    {
        let mut inner = self.into_inner();
        inner.fallback(job);
        Router {
            inner: Arc::new(inner),
        }
    }

    /// Apply a [`tower::Layer`] to all routes in this `Router`
    ///
    /// See [`Job::layer()`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use blueprint_sdk::{Job, Router};
    /// use tower::limit::{ConcurrencyLimit, ConcurrencyLimitLayer};
    ///
    /// async fn job() { /* ... */
    /// }
    ///
    /// async fn another_job() { /* ... */
    /// }
    ///
    /// const JOB_ID: u32 = 0;
    /// const ANOTHER_JOB_ID: u32 = 1;
    ///
    /// let app = Router::new()
    ///     .route(JOB_ID, job)
    ///     .route(ANOTHER_JOB_ID, another_job)
    ///     // Limit concurrent calls to both `job` and `another_job` to 64
    ///     .layer(ConcurrencyLimitLayer::new(64));
    /// # let _: Router = app;
    /// ```
    pub fn layer<L>(self, layer: L) -> Router<Ctx>
    where
        L: Layer<Route> + Clone + Send + Sync + 'static,
        L::Service: Service<JobCall> + Clone + Send + Sync + 'static,
        <L::Service as Service<JobCall>>::Response: IntoJobResult + 'static,
        <L::Service as Service<JobCall>>::Error: Into<BoxError> + 'static,
        <L::Service as Service<JobCall>>::Future: Send + 'static,
    {
        let inner = self.into_inner().layer(layer);
        Router {
            inner: Arc::new(inner),
        }
    }

    /// Whether the router currently has at least one route added.
    #[must_use]
    pub fn has_routes(&self) -> bool {
        self.inner.has_routes()
    }

    #[doc = include_str!("../docs/with_context.md")]
    pub fn with_context<Ctx2>(self, context: Ctx) -> Router<Ctx2> {
        let inner = self.into_inner().with_context(context);
        Router {
            inner: Arc::new(inner),
        }
    }

    pub(crate) fn call_with_context(
        &self,
        call: JobCall,
        context: Ctx,
    ) -> Option<FuturesUnordered<RouteFuture<BoxError>>> {
        blueprint_core::trace!(
            target: "blueprint-router",
            job_id = %call.job_id(),
            metadata = ?call.metadata(),
            body = ?call.body(),
            "routing a job call to inner routers"
        );
        let (call, context) = match self.inner.call_with_context(call, context) {
            Ok(matched_call_future) => {
                blueprint_core::trace!(
                    target: "blueprint-router",
                    matched_calls = matched_call_future.len(),
                    "A route matched this job call"
                );
                return Some(matched_call_future);
            }
            Err((call, context)) => (call, context),
        };

        // At this point, no route matched the job ID, and there are no always routes
        blueprint_core::trace!(
            target: "blueprint-router",
            ?call,
            "No explicit or always route caught this job call, passing to fallback"
        );

        self.inner
            .call_fallback(call, context)
            .map(|future| iter::once(future).collect::<FuturesUnordered<_>>())
    }

    /// Convert the router into a borrowed [`Service`] with a fixed request body type, to aid type
    /// inference.
    ///
    /// In some cases when calling methods from [`tower::ServiceExt`] on a [`Router`] you might get
    /// type inference errors along the lines of
    ///
    /// ```not_rust
    /// let response = router.ready().await?.call(request).await?;
    ///                       ^^^^^ cannot infer type for type parameter `B`
    /// ```
    ///
    /// This happens because `Router` implements [`Service`] with `impl<B> Service<Request<B>> for Router<()>`.
    ///
    /// For example:
    ///
    /// ```compile_fail
    /// use blueprint_sdk::{Router, JobCall, Bytes};
    /// use tower::{Service, ServiceExt};
    ///
    /// const MY_JOB_ID: u8 = 0;
    ///
    /// # async fn async_main() -> Result<(), blueprint_sdk::core::error::BoxError> {
    /// let mut router = Router::new().route(MY_JOB_ID, || async {});
    /// let request = JobCall::new(MY_JOB_ID, Bytes::new());
    /// let response = router.ready().await?.call(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Calling `Router::as_service` fixes that:
    ///
    /// ```
    /// use blueprint_sdk::{JobCall, Router};
    /// use bytes::Bytes;
    /// use tower::{Service, ServiceExt};
    ///
    /// const MY_JOB_ID: u32 = 0;
    ///
    /// # async fn async_main() -> Result<(), blueprint_sdk::core::error::BoxError> {
    /// let mut router = Router::new().route(MY_JOB_ID, || async {});
    /// let request = JobCall::new(MY_JOB_ID, Bytes::new());
    /// let response = router.as_service().ready().await?.call(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// This is mainly used when calling `Router` in tests. It shouldn't be necessary when running
    /// the `Router` normally via the blueprint runner.
    pub fn as_service<B>(&mut self) -> RouterAsService<'_, B, Ctx> {
        RouterAsService {
            router: self,
            _marker: PhantomData,
        }
    }
}

impl<B> Service<JobCall<B>> for Router<()>
where
    B: Into<Bytes>,
{
    type Response = Option<Vec<JobResult>>;
    type Error = BoxError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    #[inline]
    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    #[allow(clippy::needless_continue)]
    fn call(&mut self, call: JobCall<B>) -> Self::Future {
        let Some(mut futures) = self.call_with_context(call.map(Into::into), ()) else {
            return Box::pin(async { Ok(None) });
        };

        Box::pin(async move {
            let mut results = Vec::with_capacity(futures.len());
            while let Some(item) = futures.next().await {
                blueprint_core::trace!(target: "blueprint-router", outcome = ?item, "Job finished with outcome");
                match item {
                    Ok(Some(job)) => results.push(job),
                    // Job produced nothing, and didn't error. Don't include it.
                    Ok(None) => continue,
                    Err(e) => {
                        blueprint_core::error!(?e, "Job failed");
                        return Err(e);
                    }
                }
            }

            Ok(Some(results))
        })
    }
}

/// A [`Router`] converted into a borrowed [`Service`] with a fixed body type.
///
/// See [`Router::as_service`] for more details.
pub struct RouterAsService<'a, B, Ctx = ()> {
    router: &'a mut Router<Ctx>,
    _marker: PhantomData<B>,
}

impl<B> Service<JobCall<B>> for RouterAsService<'_, B, ()>
where
    B: Into<Bytes>,
{
    type Response = Option<Vec<JobResult>>;
    type Error = BoxError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        <Router as Service<JobCall<B>>>::poll_ready(self.router, cx)
    }

    #[inline]
    fn call(&mut self, call: JobCall<B>) -> Self::Future {
        self.router.call(call)
    }
}

impl<B, Ctx> fmt::Debug for RouterAsService<'_, B, Ctx>
where
    Ctx: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RouterAsService")
            .field("router", &self.router)
            .finish()
    }
}

#[test]
fn traits() {
    use crate::test_helpers::*;
    assert_send::<Router<()>>();
    assert_sync::<Router<()>>();
}
