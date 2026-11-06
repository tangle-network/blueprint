//! Tracing subscriber setup for the CLI.

use tracing_subscriber::EnvFilter;

/// Build the CLI's tracing [`EnvFilter`].
///
/// `RUST_LOG` still wins when it is set to something valid. The difference is
/// the fallback: [`EnvFilter::from_default_env`] installs `ERROR` as its
/// default directive, so with `RUST_LOG` unset every `info!` and `warn!` the
/// CLI emits is filtered out before it reaches the fmt layer. Operators then
/// lose warnings that explain a silently degraded result, such as
/// `command::create` skipping a blueprint's forge dependencies because forge
/// is not installed.
///
/// Defaulting to `INFO` instead keeps those warnings visible while leaving
/// `RUST_LOG=debug` / `RUST_LOG=cargo_tangle=trace` as the escape hatch for
/// noisier debugging. `from_env_lossy` is kept so a malformed `RUST_LOG`
/// degrades to the default rather than aborting startup.
pub fn cli_env_filter() -> EnvFilter {
    use tracing_subscriber::filter::LevelFilter;

    EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy()
}
