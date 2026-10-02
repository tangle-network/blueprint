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

#[cfg(test)]
mod tests {
    use super::cli_env_filter;
    use std::io;
    use std::sync::{Arc, Mutex};

    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::prelude::*;

    /// Captures fmt-layer output so assertions are about what an operator
    /// actually sees, not about a filter's internal representation.
    #[derive(Clone, Default)]
    struct Buffer(Arc<Mutex<Vec<u8>>>);

    impl io::Write for Buffer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for Buffer {
        type Writer = Self;
        fn make_writer(&'a self) -> Self::Writer {
            self.clone()
        }
    }

    /// The same subscriber shape `main.rs` installs, writing into a buffer.
    fn rendered(filter: EnvFilter) -> String {
        let out = Buffer::default();
        let subscriber = tracing_subscriber::registry().with(filter).with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_span_events(FmtSpan::CLOSE)
                .pretty()
                .with_writer(out.clone()),
        );

        tracing::subscriber::with_default(subscriber, || {
            tracing::error!("MARKER_ERROR");
            tracing::warn!("MARKER_WARN");
            tracing::info!("MARKER_INFO");
            tracing::debug!("MARKER_DEBUG");
            tracing::trace!("MARKER_TRACE");
        });

        String::from_utf8(out.0.lock().unwrap().clone()).unwrap()
    }

    /// The CLI reports user-visible degradation at `WARN` and `INFO`, so with
    /// `RUST_LOG` unset those must reach the fmt layer.
    ///
    /// This is the regression test for the `from_default_env` default of
    /// `ERROR`, which dropped them. Each marker is emitted at a distinct level
    /// and the buffer is a real fmt layer, so the assertion fails on pristine
    /// `main` for the right reason rather than on a formatting detail.
    #[test]
    fn default_filter_surfaces_warn_and_info() {
        with_env("RUST_LOG", None, || {
            let text = rendered(cli_env_filter());

            assert!(
                text.contains("MARKER_WARN"),
                "WARN must reach the fmt layer with RUST_LOG unset, otherwise the \
                 CLI silently drops warnings that explain a degraded result"
            );
            assert!(
                text.contains("MARKER_INFO"),
                "INFO must reach the fmt layer with RUST_LOG unset"
            );
        });
    }

    /// Control case: `ERROR` was already admitted before this change, so it must
    /// still be. Guards against a filter that admits nothing at all.
    #[test]
    fn default_filter_still_surfaces_error() {
        with_env("RUST_LOG", None, || {
            assert!(rendered(cli_env_filter()).contains("MARKER_ERROR"));
        });
    }

    /// The default must not open `DEBUG` or `TRACE`, or every CLI invocation
    /// becomes unusably noisy. `DEBUG` is the boundary that moves.
    #[test]
    fn default_filter_excludes_debug_and_trace() {
        with_env("RUST_LOG", None, || {
            let text = rendered(cli_env_filter());

            assert!(
                !text.contains("MARKER_DEBUG"),
                "DEBUG must stay off by default; RUST_LOG is the opt-in"
            );
            assert!(!text.contains("MARKER_TRACE"));
        });
    }

    /// `RUST_LOG` must remain an override in both directions: asking for more
    /// detail still works, and narrowing the filter still narrows it rather
    /// than being ignored in favour of the new default.
    #[test]
    fn rust_log_overrides_the_default_directive() {
        with_env("RUST_LOG", Some("cargo_tangle=trace"), || {
            assert!(rendered(cli_env_filter()).contains("MARKER_TRACE"));
        });

        with_env("RUST_LOG", Some("error"), || {
            let text = rendered(cli_env_filter());
            assert!(text.contains("MARKER_ERROR"));
            assert!(
                !text.contains("MARKER_WARN"),
                "RUST_LOG=error must still narrow the filter"
            );
        });
    }

    /// A malformed `RUST_LOG` must not abort startup or silently widen the
    /// filter; it falls back to the default.
    #[test]
    fn malformed_rust_log_falls_back_to_default() {
        with_env("RUST_LOG", Some("not a valid directive ==="), || {
            let text = rendered(cli_env_filter());
            assert!(text.contains("MARKER_INFO"));
            assert!(!text.contains("MARKER_TRACE"));
        });
    }

    /// Run `f` with `key` set to `value` (or removed when `None`), restoring the
    /// prior value afterwards.
    ///
    /// Tests in this module are the only readers of `RUST_LOG`, and CI runs the
    /// crate's serial profile, so the process env is mutated without a lock.
    fn with_env(key: &str, value: Option<&str>, f: impl FnOnce()) {
        let prior = std::env::var_os(key);
        // SAFETY: no other thread in this test binary reads this key; see above.
        unsafe {
            match value {
                Some(v) => std::env::set_var(key, v),
                None => std::env::remove_var(key),
            }
        }

        f();

        // SAFETY: as above.
        unsafe {
            match prior {
                Some(v) => std::env::set_var(key, v),
                None => std::env::remove_var(key),
            }
        }
    }
}
