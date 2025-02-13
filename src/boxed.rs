use crate::routing::future::Route;
use crate::routing::future::RouteFuture;
use crate::{Job, JobCall, Router};

use std::{convert::Infallible, fmt};

use tower::Service;

pub(crate) struct BoxedIntoRoute<S, E>(Box<dyn ErasedIntoRoute<S, E>>);

impl<Ctx> BoxedIntoRoute<Ctx, Infallible>
where
    Ctx: Clone + Send + Sync + 'static,
{
    pub(crate) fn from_job<J, T>(job: J) -> Self
    where
        J: Job<T, Ctx>,
        T: 'static,
    {
        Self(Box::new(MakeErasedJob {
            job,
            into_route: |handler, context| Route::new(Job::with_context(handler, context)),
        }))
    }
}

impl<Ctx, E> BoxedIntoRoute<Ctx, E> {
    pub(crate) fn map<F, E2>(self, f: F) -> BoxedIntoRoute<Ctx, E2>
    where
        Ctx: 'static,
        E: 'static,
        F: FnOnce(Route<E>) -> Route<E2> + Clone + Send + Sync + 'static,
        E2: 'static,
    {
        BoxedIntoRoute(Box::new(Map {
            inner: self.0,
            layer: Box::new(f),
        }))
    }

    pub(crate) fn into_route(self, context: Ctx) -> Route<E> {
        self.0.into_route(context)
    }
}

impl<Ctx, E> Clone for BoxedIntoRoute<Ctx, E> {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

impl<Ctx, E> fmt::Debug for BoxedIntoRoute<Ctx, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BoxedIntoRoute").finish()
    }
}

pub(crate) trait ErasedIntoRoute<Ctx, E>: Send + Sync {
    fn clone_box(&self) -> Box<dyn ErasedIntoRoute<Ctx, E>>;

    fn into_route(self: Box<Self>, context: Ctx) -> Route<E>;

    #[allow(dead_code)]
    fn call_with_context(self: Box<Self>, call: JobCall, context: Ctx) -> RouteFuture<E>;
}

pub(crate) struct MakeErasedJob<J, Ctx> {
    pub(crate) job: J,
    pub(crate) into_route: fn(J, Ctx) -> Route,
}

impl<J, Ctx> ErasedIntoRoute<Ctx, Infallible> for MakeErasedJob<J, Ctx>
where
    J: Clone + Send + Sync + 'static,
    Ctx: 'static,
{
    fn clone_box(&self) -> Box<dyn ErasedIntoRoute<Ctx, Infallible>> {
        Box::new(self.clone())
    }

    fn into_route(self: Box<Self>, context: Ctx) -> Route {
        (self.into_route)(self.job, context)
    }

    fn call_with_context(self: Box<Self>, call: JobCall, context: Ctx) -> RouteFuture<Infallible> {
        self.into_route(context).call(call)
    }
}

impl<J, Ctx> Clone for MakeErasedJob<J, Ctx>
where
    J: Clone,
{
    fn clone(&self) -> Self {
        Self {
            job: self.job.clone(),
            into_route: self.into_route,
        }
    }
}

#[allow(dead_code)]
pub(crate) struct MakeErasedRouter<S> {
    pub(crate) router: Router<S>,
    pub(crate) into_route: fn(Router<S>, S) -> Route,
}

impl<Ctx> ErasedIntoRoute<Ctx, Infallible> for MakeErasedRouter<Ctx>
where
    Ctx: Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn ErasedIntoRoute<Ctx, Infallible>> {
        Box::new(self.clone())
    }

    fn into_route(self: Box<Self>, context: Ctx) -> Route {
        (self.into_route)(self.router, context)
    }

    fn call_with_context(self: Box<Self>, call: JobCall, context: Ctx) -> RouteFuture<Infallible> {
        self.router.call_with_context(call, context)
    }
}

impl<Ctx> Clone for MakeErasedRouter<Ctx>
where
    Ctx: Clone,
{
    fn clone(&self) -> Self {
        Self {
            router: self.router.clone(),
            into_route: self.into_route,
        }
    }
}

pub(crate) struct Map<Ctx, E, E2> {
    pub(crate) inner: Box<dyn ErasedIntoRoute<Ctx, E>>,
    pub(crate) layer: Box<dyn LayerFn<E, E2>>,
}

impl<Ctx, E, E2> ErasedIntoRoute<Ctx, E2> for Map<Ctx, E, E2>
where
    Ctx: 'static,
    E: 'static,
    E2: 'static,
{
    fn clone_box(&self) -> Box<dyn ErasedIntoRoute<Ctx, E2>> {
        Box::new(Self {
            inner: self.inner.clone_box(),
            layer: self.layer.clone_box(),
        })
    }

    fn into_route(self: Box<Self>, context: Ctx) -> Route<E2> {
        (self.layer)(self.inner.into_route(context))
    }

    fn call_with_context(self: Box<Self>, call: JobCall, context: Ctx) -> RouteFuture<E2> {
        (self.layer)(self.inner.into_route(context)).call(call)
    }
}

pub(crate) trait LayerFn<E, E2>: FnOnce(Route<E>) -> Route<E2> + Send + Sync {
    fn clone_box(&self) -> Box<dyn LayerFn<E, E2>>;
}

impl<F, E, E2> LayerFn<E, E2> for F
where
    F: FnOnce(Route<E>) -> Route<E2> + Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn LayerFn<E, E2>> {
        Box::new(self.clone())
    }
}
