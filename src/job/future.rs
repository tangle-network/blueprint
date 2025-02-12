//! Handler future types.

use crate::JobCall;
use crate::JobResult;
use core::{convert::Infallible, future::Future, pin::Pin, task::Context};
use futures_util::future::Map;
use pin_project_lite::pin_project;
use tower::util::Oneshot;
use tower::Service;

opaque_future! {
    /// The response future for [`IntoService`](super::IntoService).
    pub type IntoServiceFuture<F> =
        Map<
            F,
            fn(JobResult) -> Result<JobResult, Infallible>,
        >;
}

pin_project! {
    /// The response future for [`Layered`](super::Layered).
    pub struct LayeredFuture<S>
    where
        S: Service<JobCall>,
    {
        #[pin]
        inner: Map<Oneshot<S, JobCall>, fn(Result<S::Response, S::Error>) -> JobResult>,
    }
}

impl<S> LayeredFuture<S>
where
    S: Service<JobCall>,
{
    pub(super) fn new(
        inner: Map<Oneshot<S, JobCall>, fn(Result<S::Response, S::Error>) -> JobResult>,
    ) -> Self {
        Self { inner }
    }
}

impl<S> Future for LayeredFuture<S>
where
    S: Service<JobCall>,
{
    type Output = JobResult;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> core::task::Poll<Self::Output> {
        self.project().inner.poll(cx)
    }
}
