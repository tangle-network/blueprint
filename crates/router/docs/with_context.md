Provide the context for the router. Context passed to this method is global and will be used for all requests this router receives.

```rust
use blueprint_sdk::{Router, extract::Context, runner::BlueprintRunner};

const MY_JOB_ID: u8 = 0;

#[derive(Clone)]
struct AppContext {}

let routes = Router::new()
    .route(MY_JOB_ID, |Context(ctx): Context<AppContext>| async {
        // use context
    })
    .with_context(AppContext {});

# async {
let config = /* ... */
# ();
let env = /* ... */
# blueprint_sdk::runner::config::BlueprintEnvironment::default();

let runner = BlueprintRunner::builder(config, env).router(routes);
let result = runner.run().await;
# };
```

# Returning routers with contexts from functions

When returning `Router`s from functions, it is generally recommended not to set the
context directly:

```rust
use blueprint_sdk::{Router, extract::Context, runner::BlueprintRunner};

const MY_JOB_ID: u8 = 0;

#[derive(Clone)]
struct AppContext {}

// Don't call `Router::with_context` here
fn routes() -> Router<AppContext> {
    Router::new()
        .route(MY_JOB_ID, |_: Context<AppContext>| async {})
}

// Instead, do it before you run the server
let routes = routes().with_context(AppContext {});

# async {
let config = /* ... */
# ();
let env = /* ... */
# blueprint_sdk::runner::config::BlueprintEnvironment::default();

let runner = BlueprintRunner::builder(config, env).router(routes);
let result = runner.run().await;
# };
```

If you do need to provide the context, then return `Router` without any type parameters:

```rust
# use blueprint_sdk::{Router, extract::Context, runner::BlueprintRunner};
# const MY_JOB_ID: u8 = 0;
# #[derive(Clone)]
# struct AppContext {}
#
// Don't return `Router<AppContext>`
fn routes(context: AppContext) -> Router {
    Router::new()
        .route(MY_JOB_ID, |_: Context<AppContext>| async {})
        .with_context(context)
}

let routes = routes(AppContext {});

# async {
let config = /* ... */
# ();
let env = /* ... */
# blueprint_sdk::runner::config::BlueprintEnvironment::default();

let runner = BlueprintRunner::builder(config, env).router(routes);
let result = runner.run().await;
# };
```

This is because we can only call [`BlueprintRunnerBuilder::router()`] on `Router<()>`,
not `Router<AppContext>`. See below for more details about why that is.

Note that the context defaults to `()` so `Router` and `Router<()>` is the same.

# What `Ctx` in `Router<Ctx>` means

`Router<Ctx>` means a router that is _missing_ a context of type `Ctx` to be able to
handle requests. It does _not_ mean a `Router` that _has_ a context of type `Ctx`.

For example:

```rust
# use blueprint_sdk::{Router, extract::Context, runner::BlueprintRunner};
# const MY_JOB_ID: u8 = 0;
# #[derive(Clone)]
# struct AppContext {}
# 
// A router that _needs_ an `AppContext` to handle requests
let router: Router<AppContext> = Router::new()
    .route(MY_JOB_ID, |_: Context<AppContext>| async {});

// Once we call `Router::with_context` the router isn't missing
// the context anymore, because we just provided it
//
// Therefore the router type becomes `Router<()>`, i.e a router
// that is not missing any context
let router: Router<()> = router.with_context(AppContext {});

// Only `Router<()>` can be used in a `BlueprintRunner`.
//
// You cannot call `BlueprintRunnerBuilder::router` with a `Router<AppContext>`
// because it is still missing an `AppContext`.
# async {
let config = /* ... */
# ();
let env = /* ... */
# blueprint_sdk::runner::config::BlueprintEnvironment::default();

let runner = BlueprintRunner::builder(config, env).router(router);
let result = runner.run().await;
# };
```

Perhaps a little counter intuitively, `Router::with_context` doesn't always return a
`Router<()>`. Instead, you get to pick what the new missing context type is:

```rust
# use blueprint_sdk::{Router, extract::Context, runner::BlueprintRunner};
# const MY_JOB_ID: u8 = 0;
# #[derive(Clone)]
# struct AppContext {}
# 
let router: Router<AppContext> = Router::new()
    .route(MY_JOB_ID, |_: Context<AppContext>| async {});

// When we call `with_context` we're able to pick what the next missing context type is.
// Here we pick `String`.
let string_router: Router<String> = router.with_context(AppContext {});

// That allows us to add new routes that uses `String` as the context type
const NEEDS_STRING_JOB_ID: u8 = 1;

let string_router = string_router
    .route(NEEDS_STRING_JOB_ID, |_: Context<String>| async {});

// Provide the `String` and choose `()` as the new missing context.
let final_router: Router<()> = string_router.with_context("foo".to_owned());

// Since we have a `Router<()>` we can run it.
# async {
let config = /* ... */
# ();
let env = /* ... */
# blueprint_sdk::runner::config::BlueprintEnvironment::default();

let runner = BlueprintRunner::builder(config, env).router(final_router);
let result = runner.run().await;
# };
```

This why this returning `Router<AppContext>` after calling `with_context` doesn't
work:

```rust,compile_fail
# use blueprint_sdk::{Router, extract::Context, runner::BlueprintRunner};
# #[derive(Clone)]
# struct AppContext {}
# 
// This won't work because we're returning a `Router<AppContext>`
// i.e. we're saying we're still missing an `AppContext`
fn routes(context: AppContext) -> Router<AppContext> {
    Router::new()
        .route("/", |_: Context<AppContext>| async {})
        .with_context(context)
}

let app = routes(AppContext {});

// We can only call `BlueprintRunnerBuilder::router` with a `Router<()>`
// but `app` is a `Router<AppContext>`
# async {
let config = /* ... */
# ();
let env = /* ... */
# blueprint_sdk::runner::config::BlueprintEnvironment::default();

let runner = BlueprintRunner::builder(config, env).router(app);
let result = runner.run().await;
# };
```

Instead, return `Router<()>` since we have provided all the context needed:

```rust
# use blueprint_sdk::{Router, extract::Context, runner::BlueprintRunner};
# const MY_JOB_ID: u8 = 0;
# #[derive(Clone)]
# struct AppContext {}
# 
// We've provided all the context necessary so return `Router<()>`
fn routes(context: AppContext) -> Router<()> {
    Router::new()
        .route(MY_JOB_ID, |_: Context<AppContext>| async {})
        .with_context(context)
}

let app = routes(AppContext {});

// We can now call `BlueprintRunnerBuilder::router`
# async {
let config = /* ... */
# ();
let env = /* ... */
# blueprint_sdk::runner::config::BlueprintEnvironment::default();

let runner = BlueprintRunner::builder(config, env).router(app);
let result = runner.run().await;
# };
```

# A note about performance

If you need a `Router` that implements `Service` but you don't need any context (perhaps
you're making a library that uses `blueprint-router` internally) then it is recommended to call this
method before you start serving requests:

```rust
use blueprint_sdk::Router;

const MY_JOB_ID: u8 = 0;

let app = Router::new()
    .route(MY_JOB_ID, || async { /* ... */ })
    // even though we don't need any context, call `with_context(())` anyway
    .with_context(());
# let _: Router = app;
```

This is not required but it gives `blueprint-router` a chance to update some internals in the router
which may impact performance and reduce allocations.

[`Extension`]: blueprint_core::extensions::Extension
[`BlueprintRunnerBuilder::router()`]: https://docs.rs/blueprint-runner/latest/blueprint_runner/struct.BlueprintRunnerBuilder.html#method.router
