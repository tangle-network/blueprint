## Debugging job type errors

For a function to be used as a job it must implement the [`Job`] trait.
`blueprint-sdk` provides blanket implementations for functions that:

- Are `async fn`s.
- Take no more than 16 arguments that all implement `Send`.
    - All except the last argument implement [`FromJobCallParts`].
    - The last argument implements [`FromJobCall`].
- Returns something that implements [`IntoJobResult`].
- If a closure is used it must implement `Clone + Send` and be
  `'static`.
- Returns a future that is `Send`. The most common way to accidentally make a
  future `!Send` is to hold a `!Send` type across an await.

Unfortunately Rust gives poor error messages if you try to use a function
that doesn't quite match what's required by [`Job`].

You might get an error like this:

```not_rust
error[E0277]: the trait bound `fn(u64) -> u64 {job}: blueprint_sdk::Job<_, _>` is not satisfied
  --> src/main.rs:48:40
   |
48 |                 .route(MY_JOB_ID, job)
   |                  -----            ^^^ the trait `blueprint_sdk::Job<_, _>` is not implemented for fn item `fn(u64) -> u64 {job}`
   |                  |
   |                  required by a bound introduced by this call
   |
   = note: Consider using `#[blueprint_sdk::debug_job]` to improve the error message
   = help: the trait `blueprint_sdk::Job<T, Ctx>` is implemented for `blueprint_sdk::job::Layered<L, J, T, Ctx>`
```

This error doesn't tell you _why_ your function doesn't implement
[`Job`]. It's possible to improve the error with the [`debug_job`]
proc-macro from the [blueprint-macros] crate.

[blueprint-macros]: https://docs.rs/blueprint-macros
[`debug_job`]: https://docs.rs/blueprint-macros/latest/blueprint_macros/attr.debug_job.html
