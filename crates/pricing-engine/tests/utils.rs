use blueprint_pricing_engine_lib::OperatorConfig;
use blueprint_pricing_engine_lib::pricing_engine;
use std::path::PathBuf;

pub fn create_test_config() -> OperatorConfig {
    OperatorConfig {
        database_path: "./data/test_benchmark_cache".to_string(),
        benchmark_duration: 10,
        benchmark_interval: 1,
        keystore_path: PathBuf::from("./tests/tmp-keystore"),
        rpc_bind_address: "127.0.0.1".to_string(),
        rpc_port: 9000,
        rpc_timeout: 30,
        rpc_max_connections: 100,
        quote_validity_duration_secs: 300,
    }
}

pub fn create_test_quote_details() -> pricing_engine::QuoteDetails {
    let resource = pricing_engine::ResourcePricing {
        kind: "CPU".to_string(),
        count: 2,
        price_per_unit_rate: 0.000001,
    };

    let security_commitment = pricing_engine::AssetSecurityCommitment {
        asset: Some(pricing_engine::Asset {
            asset_type: Some(pricing_engine::asset::AssetType::Erc20(vec![0u8; 20])),
        }),
        exposure_percent: 50,
    };

    pricing_engine::QuoteDetails {
        blueprint_id: 12345,
        ttl_blocks: 100,
        total_cost_rate: 0.0001,
        timestamp: 1_650_000_000,
        expiry: 1_650_001_000,
        resources: vec![resource],
        security_commitments: vec![security_commitment],
    }
}
