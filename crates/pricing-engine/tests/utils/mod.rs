#![allow(dead_code)]

use blueprint_pricing_engine_lib::OperatorConfig;
use blueprint_pricing_engine_lib::pricing_engine;
use blueprint_pricing_engine_lib::utils::u32_to_u128_bytes;
use blueprint_runner::BackgroundService;
use blueprint_runner::error::RunnerError;
use blueprint_tangle_extra::extract::TangleArg;
use blueprint_tangle_extra::extract::TangleResult;
use std::path::PathBuf;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;

// Square job ID
pub const XSQUARE_JOB_ID: u8 = 0;

/// A copy of the `square` function from the `incredible-squaring` crate used for testing
pub async fn square(TangleArg(x): TangleArg<u64>) -> TangleResult<u64> {
    let result = x * x;

    // The result is then converted into a `JobResult` to be sent back to the caller.
    TangleResult(result)
}

#[derive(Clone)]
pub struct FooBackgroundService;

impl BackgroundService for FooBackgroundService {
    async fn start(&self) -> Result<Receiver<Result<(), RunnerError>>, RunnerError> {
        let (tx, rx) = oneshot::channel();
        tokio::spawn(async move {
            let _ = tx.send(Ok(()));
        });
        Ok(rx)
    }
}

// Helper function to create a test configuration
pub fn create_test_config() -> OperatorConfig {
    OperatorConfig {
        keystore_path: PathBuf::from("/tmp/test-keystore"),
        database_path: "./data/test_benchmark_cache".to_string(),
        rpc_port: 9000,
        rpc_bind_address: "127.0.0.1:9000".to_string(),
        benchmark_duration: 10,
        benchmark_interval: 1,
        rpc_timeout: 30,
        rpc_max_connections: 100,
        quote_validity_duration_secs: 300,
    }
}

/// Helper function to create a test QuoteDetails message with deterministic values
pub fn create_test_quote_details() -> pricing_engine::QuoteDetails {
    let resource = pricing_engine::ResourcePricing {
        kind: "CPU".to_string(),
        count: 2,
        price_per_unit_rate: 0.000001,
    };

    let security_commitment = pricing_engine::AssetSecurityCommitment {
        asset: Some(pricing_engine::Asset {
            asset_type: Some(pricing_engine::asset::AssetType::Custom(u32_to_u128_bytes(
                1234,
            ))),
        }),
        exposure_percent: 50,
    };

    pricing_engine::QuoteDetails {
        blueprint_id: 12345,
        ttl_blocks: 100,
        total_cost_rate: 0.0001,
        timestamp: 1650000000,
        expiry: 1650001000,
        resources: vec![resource],
        security_commitments: Some(security_commitment),
    }
}
