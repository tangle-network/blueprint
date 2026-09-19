//! Regression coverage for fail-closed aggregation policy queries.

use alloy_primitives::Address;
use alloy_sol_types::SolCall;
use blueprint_client_tangle::contracts::{ITangle, ITangleTypes};
use blueprint_client_tangle::{Error, TangleClient, TangleClientConfig, TangleSettings};
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::{K256Ecdsa, K256SigningKey};
use blueprint_keystore::backends::Backend;
use blueprint_keystore::{Keystore, KeystoreConfig};
use mockito::{Matcher, Server};
use serde_json::{Value, json};
use url::Url;

const BLUEPRINT_ID: u64 = 7;
const SERVICE_ID: u64 = 9;

#[tokio::test]
async fn manager_policy_query_errors_are_not_silently_downgraded() {
    let mut server = Server::new_async().await;
    let tangle = Address::repeat_byte(0xaa);
    let manager = Address::repeat_byte(0xbb);

    let service = ITangleTypes::Service {
        blueprintId: BLUEPRINT_ID,
        owner: Address::ZERO,
        createdAt: 0,
        ttl: 0,
        terminatedAt: 0,
        lastPaymentAt: 0,
        operatorCount: 0,
        minOperators: 0,
        maxOperators: 0,
        membership: ITangleTypes::MembershipModel::from_underlying(0).into(),
        pricing: ITangleTypes::PricingModel::from_underlying(0).into(),
        status: ITangleTypes::ServiceStatus::from_underlying(0).into(),
        confidentiality: ITangleTypes::ConfidentialityPolicy::from_underlying(0).into(),
    };
    let blueprint = ITangleTypes::Blueprint {
        owner: Address::ZERO,
        manager,
        createdAt: 0,
        membership: ITangleTypes::MembershipModel::from_underlying(0).into(),
        pricing: ITangleTypes::PricingModel::from_underlying(0).into(),
        active: true,
    };
    let service_response = format!(
        "0x{}",
        hex::encode(ITangle::getServiceCall::abi_encode_returns(&service))
    );
    let blueprint_response = format!(
        "0x{}",
        hex::encode(ITangle::getBlueprintCall::abi_encode_returns(&blueprint))
    );

    let tangle_hex = format!("{tangle:#x}");
    let manager_hex = format!("{manager:#x}");
    let service_response_for_rpc = service_response.clone();
    let blueprint_response_for_rpc = blueprint_response.clone();
    let policy_mock = server
        .mock("POST", "/")
        .match_body(Matcher::Any)
        .with_body_from_request(move |request| {
            let body: Value = serde_json::from_slice(request.body().unwrap()).unwrap();
            let id = body["id"].clone();

            if body["method"] != "eth_call" {
                return serde_json::to_vec(&json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": "0x1"
                }))
                .unwrap();
            }

            let call = &body["params"][0];
            let to = call["to"].as_str().unwrap();
            let data = call
                .get("data")
                .or_else(|| call.get("input"))
                .and_then(Value::as_str)
                .unwrap();

            let response = if to.eq_ignore_ascii_case(&tangle_hex) && data.starts_with("0x3dc0d5fe")
            {
                json!({"jsonrpc": "2.0", "id": id, "result": service_response_for_rpc})
            } else if to.eq_ignore_ascii_case(&tangle_hex) && data.starts_with("0xb7696dbb") {
                json!({"jsonrpc": "2.0", "id": id, "result": blueprint_response_for_rpc})
            } else if to.eq_ignore_ascii_case(&manager_hex) {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {"code": 3, "message": "manager policy unavailable"}
                })
            } else {
                panic!("unexpected JSON-RPC call: {body}");
            };

            serde_json::to_vec(&response).unwrap()
        })
        .expect(6)
        .create_async()
        .await;

    let keystore = Keystore::new(KeystoreConfig::new().in_memory(true)).unwrap();
    let secret = K256SigningKey::from_bytes(&[1u8; 32]).unwrap();
    keystore.insert::<K256Ecdsa>(&secret).unwrap();
    let rpc_url = Url::parse(&server.url()).unwrap();
    let settings = TangleSettings {
        blueprint_id: BLUEPRINT_ID,
        service_id: Some(SERVICE_ID),
        tangle_contract: tangle,
        staking_contract: Address::ZERO,
        status_registry_contract: Address::ZERO,
    };
    let config = TangleClientConfig::new(rpc_url.clone(), rpc_url, "memory://", settings);
    let client = TangleClient::with_keystore(config, keystore).await.unwrap();

    let error = client
        .requires_aggregation(SERVICE_ID, 4)
        .await
        .expect_err("a failed manager query must not mean no aggregation");
    assert!(
        matches!(error, Error::Contract(message) if message.contains("manager policy unavailable"))
    );

    let error = client
        .get_aggregation_threshold(SERVICE_ID, 4)
        .await
        .expect_err("a failed threshold query must not use a guessed threshold");
    assert!(
        matches!(error, Error::Contract(message) if message.contains("manager policy unavailable"))
    );

    policy_mock.assert_async().await;
}
