//! Async functions that can be used to handle jobs.
#![doc = include_str!("../../docs/jobs_intro.md")]
//!
//! Some examples of jobs:
//!
//! ```rust
//! use blueprint_sdk::Bytes;
//!
//! // Job that immediately returns an empty result.
//! async fn unit() {}
//!
//! // Job that immediately returns a result with a body of "Hello, World!".
//! async fn string() -> String {
//!     "Hello, World!".to_string()
//! }
//!
//! // Job that buffers the request body and returns it.
//! //
//! // This works because `Bytes` implements `FromJobCall`
//! // and therefore can be used as an extractor.
//! //
//! // `String` implements `IntoJobResult` and therefore `Result<String, String>`
//! // also implements `IntoJobResult`
//! async fn echo(body: Bytes) -> Result<String, String> {
//!     if let Ok(string) = String::from_utf8(body.to_vec()) {
//!         Ok(string)
//!     } else {
//!         Err(String::from("Invalid UTF-8"))
//!     }
//! }
//! ```
//!
//! Instead of a direct `String`, it makes sense to use an intermediate error type
//! that can ultimately be converted to `JobResult`. This allows using the `?` operator
//! in jobs.
#![doc = include_str!("../../docs/debugging_job_type_errors.md")]

use crate::{
    JobCall, JobResult,
    extract::{FromJobCall, FromJobCallParts},
};
use alloc::boxed::Box;
use core::{fmt, future::Future, marker::PhantomData, pin::Pin};
use tower::{Layer, Service, ServiceExt};

pub mod call;
pub mod future;
mod id;
pub use id::*;
pub mod result;
pub mod service;
use result::IntoJobResult;

pub use self::service::JobService;

/// Trait for async functions that can be used to handle requests.
///
/// You shouldn't need to depend on this trait directly. It is automatically
/// implemented for functions of the right types.
///
/// See the [module docs](crate::job) for more details.
///
/// # Converting `Job`s into [`Service`]s
///
/// To convert `Job`s into [`Service`]s you have to call either
/// [`JobWithoutContextExt::into_service`] or [`Job::with_context`]:
///
/// ```
/// use blueprint_sdk::extract::Context;
/// use blueprint_sdk::job::JobWithoutContextExt;
/// use blueprint_sdk::{Job, JobCall};
/// use tower::Service;
///
/// // this job doesn't require any state
/// async fn one() {}
/// // so it can be converted to a service with `JobWithoutContextExt::into_service`
/// assert_service(one.into_service());
///
/// // this job requires a context
/// async fn two(_: Context<String>) {}
/// // so we have to provide it
/// let job_with_state = two.with_context(String::new());
/// // which gives us a `Service`
/// assert_service(job_with_state);
///
/// // helper to check that a value implements `Service`
/// fn assert_service<S>(service: S)
/// where
///     S: Service<JobCall>,
/// {
/// }
/// ```
#[doc = include_str!("../../docs/debugging_job_type_errors.md")]
///
/// # Jobs that aren't functions
///
/// The `Job` trait is also implemented for `T: IntoJobResult`. That allows easily returning
/// fixed data for routes:
///
/// ```
/// use blueprint_sdk::Router;
/// use serde_json::json;
///
/// const HELLO_JOB_ID: u32 = 0;
/// const USERS_JOB_ID: u32 = 1;
///
/// let app = Router::new()
///     // respond with a fixed string
///     .route(HELLO_JOB_ID, "Hello, World!")
///     // or return some mock data
///     .route(USERS_JOB_ID, json!({ "id": 1, "username": "alice" }).to_string());
/// # let _: Router = app;
/// ```
#[diagnostic::on_unimplemented(
    note = "Consider using `#[blueprint_sdk::debug_job]` to improve the error message"
)]
pub trait Job<T, Ctx>: Clone + Send + Sync + Sized + 'static {
    /// The type of future calling this job returns.
    type Future: Future<Output = Option<JobResult>> + Send + 'static;

    /// Call the job with the given request.
    fn call(self, call: JobCall, ctx: Ctx) -> Self::Future;

    /// Apply a [`tower::Layer`] to the job.
    ///
    /// All requests to the job will be processed by the layer's
    /// corresponding middleware.
    ///
    /// This can be used to add additional processing to a request for a single
    /// job.
    ///
    /// Note this differs from [`routing::Router::layer`]
    /// which adds a middleware to a group of routes.
    ///
    /// If you're applying middleware that produces errors you have to handle the errors
    /// so they're converted into responses. You can learn more about doing that
    /// [here](crate::error_handling).
    ///
    /// # Example
    ///
    /// Adding the [`tower::limit::ConcurrencyLimit`] middleware to a job
    /// can be done like so:
    ///
    /// ```rust
    /// use blueprint_sdk::{Job, Router};
    /// use tower::limit::{ConcurrencyLimit, ConcurrencyLimitLayer};
    ///
    /// async fn job() { /* ... */
    /// }
    ///
    /// const MY_JOB_ID: u32 = 0;
    ///
    /// let layered_job = job.layer(ConcurrencyLimitLayer::new(64));
    /// let app = Router::new().route(MY_JOB_ID, layered_job);
    /// # let _: Router = app;
    /// ```
    ///
    /// [`routing::Router::layer`]: https://docs.rs/blueprint-sdk/latest/blueprint_sdk/struct.Router.html#method.layer
    fn layer<L>(self, layer: L) -> Layered<L, Self, T, Ctx>
    where
        L: Layer<JobService<Self, T, Ctx>> + Clone,
        L::Service: Service<JobCall>,
    {
        Layered {
            layer,
            job: self,
            _marker: PhantomData,
        }
    }

    /// Convert the job into a [`Service`] by providing the context
    fn with_context(self, ctx: Ctx) -> JobService<Self, T, Ctx> {
        JobService::new(self, ctx)
    }
}

impl<F, Fut, Res, Ctx> Job<((),), Ctx> for F
where
    F: FnOnce() -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send,
    Res: IntoJobResult,
{
    type Future = Pin<Box<dyn Future<Output = Option<JobResult>> + Send>>;

    fn call(self, _call: JobCall, _ctx: Ctx) -> Self::Future {
        Box::pin(async move { self().await.into_job_result() })
    }
}

macro_rules! impl_job {
    (
        [$($ty:ident),*], $last:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<F, Fut, Ctx, Res, M, $($ty,)* $last> Job<(M, $($ty,)* $last,), Ctx> for F
        where
            F: FnOnce($($ty,)* $last,) -> Fut + Clone + Send + Sync + 'static,
            Fut: Future<Output = Res> + Send,
            Ctx: Send + Sync + 'static,
            Res: IntoJobResult,
            $( $ty: FromJobCallParts<Ctx> + Send, )*
            $last: FromJobCall<Ctx, M> + Send,
        {
            type Future = Pin<Box<dyn Future<Output = Option<JobResult>> + Send>>;

            fn call(self, call: JobCall, context: Ctx) -> Self::Future {
                Box::pin(async move {
                    let (mut parts, body) = call.into_parts();
                    let context = &context;

                    $(
                        let $ty = match $ty::from_job_call_parts(&mut parts, context).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_job_result(),
                        };
                    )*

                    let call = JobCall::from_parts(parts, body);

                    let $last = match $last::from_job_call(call, context).await {
                        Ok(value) => value,
                        Err(rejection) => return rejection.into_job_result(),
                    };

                    let res = self($($ty,)* $last,).await;

                    res.into_job_result()
                })
            }
        }
    };
}

all_the_tuples!(impl_job);

mod private {
    // Marker type for `impl<T: IntoJobResult> Job for T`
    #[allow(missing_debug_implementations)]
    pub enum IntoJobResultHandler {}
}

impl<T, Ctx> Job<private::IntoJobResultHandler, Ctx> for T
where
    T: IntoJobResult + Clone + Send + Sync + 'static,
{
    type Future = core::future::Ready<Option<JobResult>>;

    fn call(self, _call: JobCall, _ctx: Ctx) -> Self::Future {
        core::future::ready(self.into_job_result())
    }
}

/// A [`Service`] created from a [`Job`] by applying a Tower middleware.
///
/// Created with [`Job::layer`]. See that method for more details.
pub struct Layered<L, J, T, Ctx> {
    layer: L,
    job: J,
    _marker: PhantomData<fn() -> (T, Ctx)>,
}

impl<L, J, T, Ctx> fmt::Debug for Layered<L, J, T, Ctx>
where
    L: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Layered")
            .field("layer", &self.layer)
            .finish()
    }
}

impl<L, J, T, Ctx> Clone for Layered<L, J, T, Ctx>
where
    L: Clone,
    J: Clone,
{
    fn clone(&self) -> Self {
        Self {
            layer: self.layer.clone(),
            job: self.job.clone(),
            _marker: PhantomData,
        }
    }
}

impl<L, J, Ctx, T> Job<T, Ctx> for Layered<L, J, T, Ctx>
where
    L: Layer<JobService<J, T, Ctx>> + Clone + Send + Sync + 'static,
    L::Service: Service<JobCall> + Clone + Send + 'static,
    <L::Service as Service<JobCall>>::Response: IntoJobResult,
    <L::Service as Service<JobCall>>::Future: Send,
    J: Job<T, Ctx>,
    T: 'static,
    Ctx: 'static,
{
    type Future = future::LayeredFuture<L::Service>;

    fn call(self, call: JobCall, context: Ctx) -> Self::Future {
        use futures_util::future::{FutureExt, Map};

        let svc = self.job.with_context(context);
        let svc = self.layer.layer(svc);

        #[allow(clippy::type_complexity)]
        let future: Map<
            _,
            fn(
                Result<
                    <L::Service as Service<JobCall>>::Response,
                    <L::Service as Service<JobCall>>::Error,
                >,
            ) -> _,
        > = svc.oneshot(call).map(|result| match result {
            Ok(res) => res.into_job_result(),
            Err(_err) => todo!("JobService needs to return a result"),
        });

        future::LayeredFuture::new(future)
    }
}

/// Extension trait for [`Job`]s that don't have context.
///
/// This provides convenience methods to convert the [`Job`] into a [`Service`].
pub trait JobWithoutContextExt<T>: Job<T, ()> {
    /// Convert the handler into a [`Service`] and no context.
    fn into_service(self) -> JobService<Self, T, ()>;
}

impl<H, T> JobWithoutContextExt<T> for H
where
    H: Job<T, ()>,
{
    fn into_service(self) -> JobService<Self, T, ()> {
        self.with_context(())
    }
}
