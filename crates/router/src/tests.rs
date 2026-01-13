extern crate alloc;
extern crate std;

use crate::Router;
use crate::test_helpers::setup_log;
use alloc::vec;
use alloc::vec::Vec;
use blueprint_core::error::BoxError;
use blueprint_core::job::call::JobCall;
use bytes::Bytes;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use tower::Service;

// =============================================================================
// BASIC ROUTING TESTS
// =============================================================================

#[tokio::test]
async fn route_dispatches_to_correct_handler() {
    setup_log();

    let call_count = Arc::new(AtomicU32::new(0));
    let count_clone = call_count.clone();

    let mut router: Router = Router::new().route(42u32, move || {
        count_clone.fetch_add(1, Ordering::SeqCst);
        async { "handled" }
    });

    let result = router
        .call(JobCall::new(42u32, Bytes::new()))
        .await
        .unwrap();
    assert!(result.is_some());
    assert_eq!(call_count.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn multiple_routes_dispatch_independently() {
    setup_log();

    let job_a_count = Arc::new(AtomicU32::new(0));
    let job_b_count = Arc::new(AtomicU32::new(0));
    let a_clone = job_a_count.clone();
    let b_clone = job_b_count.clone();

    let mut router: Router = Router::new()
        .route(1u32, move || {
            a_clone.fetch_add(1, Ordering::SeqCst);
            async { "job_a" }
        })
        .route(2u32, move || {
            b_clone.fetch_add(1, Ordering::SeqCst);
            async { "job_b" }
        });

    // Call job A twice
    router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();
    router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();
    // Call job B once
    router.call(JobCall::new(2u32, Bytes::new())).await.unwrap();

    assert_eq!(job_a_count.load(Ordering::SeqCst), 2);
    assert_eq!(job_b_count.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn unknown_job_id_returns_none() {
    setup_log();

    let mut router: Router = Router::new().route(1u32, || async { "exists" });

    let result = router
        .call(JobCall::new(999u32, Bytes::new()))
        .await
        .unwrap();

    assert!(result.is_none(), "Unknown job ID should return None");
}

#[tokio::test]
async fn empty_router_returns_none() {
    setup_log();

    let mut router: Router = Router::new();

    let result = router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();

    assert!(result.is_none());
}

// =============================================================================
// FALLBACK ROUTE TESTS
// =============================================================================

#[tokio::test]
async fn fallback_catches_unmatched_job_ids() {
    setup_log();

    let fallback_count = Arc::new(AtomicU32::new(0));
    let count_clone = fallback_count.clone();

    let mut router: Router =
        Router::new()
            .route(1u32, || async { "specific" })
            .fallback(move || {
                count_clone.fetch_add(1, Ordering::SeqCst);
                async { "fallback" }
            });

    // Known route - fallback NOT called
    router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();
    assert_eq!(fallback_count.load(Ordering::SeqCst), 0);

    // Unknown route - fallback IS called
    let result = router
        .call(JobCall::new(999u32, Bytes::new()))
        .await
        .unwrap();
    assert!(result.is_some());
    assert_eq!(fallback_count.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn fallback_replaces_previous_fallback() {
    setup_log();

    let first_fallback = Arc::new(AtomicU32::new(0));
    let second_fallback = Arc::new(AtomicU32::new(0));
    let first_clone = first_fallback.clone();
    let second_clone = second_fallback.clone();

    let mut router: Router = Router::new()
        .fallback(move || {
            first_clone.fetch_add(1, Ordering::SeqCst);
            async { "first" }
        })
        .fallback(move || {
            second_clone.fetch_add(1, Ordering::SeqCst);
            async { "second" }
        });

    router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();

    assert_eq!(
        first_fallback.load(Ordering::SeqCst),
        0,
        "First fallback should be replaced"
    );
    assert_eq!(
        second_fallback.load(Ordering::SeqCst),
        1,
        "Second fallback should be called"
    );
}

// =============================================================================
// ALWAYS ROUTE TESTS
// =============================================================================

#[tokio::test]
async fn always_route_called_for_every_job() {
    setup_log();

    let always_count = Arc::new(AtomicU32::new(0));
    let count_clone = always_count.clone();

    let mut router: Router = Router::new()
        .route(1u32, || async { "job_1" })
        .route(2u32, || async { "job_2" })
        .always(move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
            async { "always" }
        });

    router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();
    router.call(JobCall::new(2u32, Bytes::new())).await.unwrap();
    router
        .call(JobCall::new(999u32, Bytes::new()))
        .await
        .unwrap(); // Unknown ID

    assert_eq!(always_count.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn always_and_specific_route_both_execute() {
    setup_log();

    let specific_count = Arc::new(AtomicU32::new(0));
    let always_count = Arc::new(AtomicU32::new(0));
    let specific_clone = specific_count.clone();
    let always_clone = always_count.clone();

    let mut router: Router = Router::new()
        .route(1u32, move || {
            specific_clone.fetch_add(1, Ordering::SeqCst);
            async { "specific" }
        })
        .always(move || {
            always_clone.fetch_add(1, Ordering::SeqCst);
            async { "always" }
        });

    let result = router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();

    // Both should execute
    assert_eq!(specific_count.load(Ordering::SeqCst), 1);
    assert_eq!(always_count.load(Ordering::SeqCst), 1);

    // Result should contain outputs from both
    let results = result.expect("Should have results");
    assert_eq!(results.len(), 2);
}

#[tokio::test]
async fn always_route_disables_fallback() {
    setup_log();

    let fallback_count = Arc::new(AtomicU32::new(0));
    let always_count = Arc::new(AtomicU32::new(0));
    let fallback_clone = fallback_count.clone();
    let always_clone = always_count.clone();

    // Fallback first, then always - fallback should be disabled
    let mut router: Router = Router::new()
        .fallback(move || {
            fallback_clone.fetch_add(1, Ordering::SeqCst);
            async { "fallback" }
        })
        .always(move || {
            always_clone.fetch_add(1, Ordering::SeqCst);
            async { "always" }
        });

    router
        .call(JobCall::new(999u32, Bytes::new()))
        .await
        .unwrap();

    assert_eq!(
        fallback_count.load(Ordering::SeqCst),
        0,
        "Fallback should not be called when always exists"
    );
    assert_eq!(always_count.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn multiple_always_routes_all_execute() {
    setup_log();

    let first_always = Arc::new(AtomicU32::new(0));
    let second_always = Arc::new(AtomicU32::new(0));
    let first_clone = first_always.clone();
    let second_clone = second_always.clone();

    let mut router: Router = Router::new()
        .always(move || {
            first_clone.fetch_add(1, Ordering::SeqCst);
            async { "first" }
        })
        .always(move || {
            second_clone.fetch_add(1, Ordering::SeqCst);
            async { "second" }
        });

    router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();

    assert_eq!(first_always.load(Ordering::SeqCst), 1);
    assert_eq!(second_always.load(Ordering::SeqCst), 1);
}

// =============================================================================
// ERROR HANDLING TESTS
// =============================================================================

#[tokio::test]
async fn fallible_job_ok_returns_ok_result() {
    setup_log();

    async fn job_ok() -> Result<&'static str, BoxError> {
        Ok("success")
    }

    let mut router: Router = Router::new().route(0u32, job_ok);

    let result = router.call(JobCall::new(0u32, Bytes::new())).await.unwrap();
    let results = result.expect("Should have results");
    assert_eq!(results.len(), 1);
    assert!(results[0].is_ok());
}

#[tokio::test]
async fn fallible_job_err_returns_err_result() {
    setup_log();

    async fn job_err() -> Result<&'static str, BoxError> {
        Err("intentional error".into())
    }

    let mut router: Router = Router::new().route(0u32, job_err);

    let result = router.call(JobCall::new(0u32, Bytes::new())).await.unwrap();
    let results = result.expect("Should have results");
    assert_eq!(results.len(), 1);
    assert!(results[0].is_err());
}

#[tokio::test]
#[should_panic(expected = "intentional panic")]
async fn job_panic_propagates() {
    setup_log();

    async fn panicking_job() -> &'static str {
        panic!("intentional panic")
    }

    let mut router: Router = Router::new().route(0u32, panicking_job);

    // Panics propagate through the router - this tests that panics aren't silently swallowed
    let _ = router.call(JobCall::new(0u32, Bytes::new())).await;
}

// =============================================================================
// CONTEXT TESTS
// =============================================================================

#[tokio::test]
async fn with_context_converts_router_type() {
    setup_log();

    #[derive(Clone)]
    struct TestContext {
        _value: u32,
    }

    // Router<TestContext> requires context to be provided
    let router_needing_context: Router<TestContext> =
        Router::new().route(1u32, || async { "no context needed for this job" });

    // Providing context converts Router<TestContext> to Router<()>
    let ctx = TestContext { _value: 42 };
    let mut router: Router<()> = router_needing_context.with_context(ctx);

    // Now it can be called
    let result = router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();
    assert!(result.is_some());
}

#[tokio::test]
async fn context_accessible_via_arc_state() {
    setup_log();

    // Use Arc to share state that jobs can access
    let shared_counter = Arc::new(AtomicU32::new(0));
    let counter_clone = shared_counter.clone();

    let mut router: Router = Router::new().route(1u32, move || {
        let counter = counter_clone.clone();
        async move {
            counter.fetch_add(1, Ordering::SeqCst);
            "incremented"
        }
    });

    router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();
    router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();

    assert_eq!(shared_counter.load(Ordering::SeqCst), 2);
}

// =============================================================================
// JOB INPUT/OUTPUT TESTS
// =============================================================================

#[tokio::test]
async fn job_receives_call_body_via_bytes_extractor() {
    setup_log();

    let received_body = Arc::new(std::sync::Mutex::new(Vec::new()));
    let body_clone = received_body.clone();

    // Using Bytes extractor - the body is automatically extracted
    let mut router: Router = Router::new().route(1u32, move |body: Bytes| {
        let captured = body_clone.clone();
        async move {
            captured.lock().unwrap().extend_from_slice(&body);
            "received"
        }
    });

    let input = Bytes::from("test input data");
    router
        .call(JobCall::new(1u32, input.clone()))
        .await
        .unwrap();

    let captured = received_body.lock().unwrap();
    assert_eq!(&*captured, &input[..]);
}

#[tokio::test]
async fn job_receives_empty_body() {
    setup_log();

    let mut router: Router = Router::new().route(1u32, move |body: Bytes| async move {
        assert!(body.is_empty());
        "empty"
    });

    router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();
}

// =============================================================================
// ROUTER STATE TESTS
// =============================================================================

#[tokio::test]
async fn has_routes_reports_correctly() {
    let empty_router: Router = Router::new();
    assert!(!empty_router.has_routes());

    let router_with_route: Router = Router::new().route(1u32, || async { "job" });
    assert!(router_with_route.has_routes());
}

#[tokio::test]
async fn has_routes_false_with_only_fallback() {
    // Fallback doesn't count as a "route"
    let router_with_fallback: Router = Router::new().fallback(|| async { "fallback" });
    assert!(!router_with_fallback.has_routes());
}

#[tokio::test]
async fn router_clone_works_independently() {
    setup_log();

    let call_count = Arc::new(AtomicU32::new(0));
    let count_clone = call_count.clone();

    let router: Router = Router::new().route(1u32, move || {
        count_clone.fetch_add(1, Ordering::SeqCst);
        async { "job" }
    });

    let mut router_clone = router.clone();
    let mut original = router;

    original
        .call(JobCall::new(1u32, Bytes::new()))
        .await
        .unwrap();
    router_clone
        .call(JobCall::new(1u32, Bytes::new()))
        .await
        .unwrap();

    assert_eq!(call_count.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn router_is_always_ready() {
    use std::future::poll_fn;
    use std::task::Poll;

    let mut router: Router = Router::new().route(1u32, || async { "job" });

    let ready = poll_fn(|cx| match Service::<JobCall>::poll_ready(&mut router, cx) {
        Poll::Ready(Ok(())) => Poll::Ready(true),
        Poll::Ready(Err(_)) => Poll::Ready(false),
        Poll::Pending => Poll::Ready(false),
    })
    .await;

    assert!(ready, "Router should always be ready");
}

// =============================================================================
// LAYER TESTS
// =============================================================================

#[tokio::test]
async fn layer_wraps_all_routes() {
    setup_log();

    use tower::limit::ConcurrencyLimitLayer;

    let call_count = Arc::new(AtomicU32::new(0));
    let count_clone = call_count.clone();

    let mut router: Router = Router::new()
        .route(1u32, move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
            async { "job" }
        })
        .layer(ConcurrencyLimitLayer::new(1));

    // Should work - layer doesn't block single calls
    router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();
    assert_eq!(call_count.load(Ordering::SeqCst), 1);
}

// =============================================================================
// CONCURRENT EXECUTION TESTS
// =============================================================================

#[tokio::test]
async fn concurrent_job_calls_execute_independently() {
    setup_log();

    use std::time::Duration;
    use tokio::time::sleep;

    let execution_order = Arc::new(std::sync::Mutex::new(Vec::new()));

    let order1 = execution_order.clone();
    let order2 = execution_order.clone();

    let router: Router = Router::new()
        .route(1u32, move || {
            let order = order1.clone();
            async move {
                sleep(Duration::from_millis(50)).await;
                order.lock().unwrap().push(1);
                "slow"
            }
        })
        .route(2u32, move || {
            let order = order2.clone();
            async move {
                order.lock().unwrap().push(2);
                "fast"
            }
        });

    let mut r1 = router.clone();
    let mut r2 = router.clone();

    // Start slow job first, then fast job
    let slow = tokio::spawn(async move { r1.call(JobCall::new(1u32, Bytes::new())).await });
    let fast = tokio::spawn(async move { r2.call(JobCall::new(2u32, Bytes::new())).await });

    let _ = tokio::join!(slow, fast);

    let order = execution_order.lock().unwrap();
    // Fast job (2) should complete before slow job (1)
    assert_eq!(*order, vec![2, 1]);
}

// =============================================================================
// EDGE CASES
// =============================================================================

#[tokio::test]
async fn job_id_zero_works() {
    setup_log();

    let mut router: Router = Router::new().route(0u32, || async { "zero" });

    let result = router.call(JobCall::new(0u32, Bytes::new())).await.unwrap();
    assert!(result.is_some());
}

#[tokio::test]
async fn large_job_id_works() {
    setup_log();

    let large_id = u32::MAX;
    let mut router: Router = Router::new().route(large_id, || async { "max" });

    let result = router
        .call(JobCall::new(large_id, Bytes::new()))
        .await
        .unwrap();
    assert!(result.is_some());
}

#[tokio::test]
async fn empty_body_handled() {
    setup_log();

    let mut router: Router = Router::new().route(1u32, |call: JobCall| async move {
        assert!(call.body().is_empty());
        "handled"
    });

    router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();
}

#[tokio::test]
async fn large_body_handled() {
    setup_log();

    let large_body = Bytes::from(vec![0xABu8; 1024 * 1024]); // 1MB
    let body_clone = large_body.clone();

    let mut router: Router = Router::new().route(1u32, move |call: JobCall| {
        let expected = body_clone.clone();
        async move {
            assert_eq!(call.body().len(), expected.len());
            "handled"
        }
    });

    router.call(JobCall::new(1u32, large_body)).await.unwrap();
}

#[tokio::test]
async fn job_returning_unit_produces_result() {
    setup_log();

    async fn unit_job() {
        // Returns ()
    }

    let mut router: Router = Router::new().route(1u32, unit_job);

    let result = router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();
    assert!(result.is_some());
}

#[tokio::test]
async fn job_returning_option_none_excluded_from_results() {
    setup_log();

    // Option<T> where T: IntoJobResult - using &'static str which implements IntoJobResult
    async fn none_job() -> Option<&'static str> {
        None
    }

    let mut router: Router = Router::new().route(1u32, none_job);

    let result = router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();
    // When job returns None, the result is filtered out
    assert!(result.is_none() || result.unwrap().is_empty());
}

#[tokio::test]
async fn job_returning_option_some_included_in_results() {
    setup_log();

    async fn some_job() -> Option<&'static str> {
        Some("value")
    }

    let mut router: Router = Router::new().route(1u32, some_job);

    let result = router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();
    let results = result.expect("Should have results vec");
    assert!(!results.is_empty());
}

// =============================================================================
// ROUTING BEHAVIOR TESTS
// =============================================================================

#[tokio::test]
async fn same_job_id_overwrites_previous_route() {
    setup_log();

    let first_handler = Arc::new(AtomicU32::new(0));
    let second_handler = Arc::new(AtomicU32::new(0));
    let first_clone = first_handler.clone();
    let second_clone = second_handler.clone();

    // Add route for job 1, then add another route for same job 1
    let mut router: Router = Router::new()
        .route(1u32, move || {
            first_clone.fetch_add(1, Ordering::SeqCst);
            async { "first" }
        })
        .route(1u32, move || {
            second_clone.fetch_add(1, Ordering::SeqCst);
            async { "second" }
        });

    router.call(JobCall::new(1u32, Bytes::new())).await.unwrap();

    // Second handler should be called (overwrites first)
    assert_eq!(first_handler.load(Ordering::SeqCst), 0);
    assert_eq!(second_handler.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn string_job_id_hashes_to_consistent_route() {
    setup_log();

    let call_count = Arc::new(AtomicU32::new(0));
    let count_clone = call_count.clone();

    let mut router: Router = Router::new().route("my_job", move || {
        count_clone.fetch_add(1, Ordering::SeqCst);
        async { "handled" }
    });

    // Same string should route to same handler
    router
        .call(JobCall::new("my_job", Bytes::new()))
        .await
        .unwrap();
    router
        .call(JobCall::new("my_job", Bytes::new()))
        .await
        .unwrap();

    assert_eq!(call_count.load(Ordering::SeqCst), 2);

    // Different string should not match
    let result = router
        .call(JobCall::new("other_job", Bytes::new()))
        .await
        .unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn router_default_is_empty() {
    let router: Router = Router::default();
    assert!(!router.has_routes());
}

#[tokio::test]
async fn router_debug_impl_works() {
    let router: Router = Router::new().route(1u32, || async { "test" });
    let debug_str = std::format!("{:?}", router);
    assert!(debug_str.contains("Router"));
}
